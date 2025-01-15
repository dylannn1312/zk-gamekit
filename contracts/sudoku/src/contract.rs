use crate::error::ContractError;
use crate::error::ContractError::{InvalidAction, InvalidSolution};
use crate::game::{GameSolution, SudokuGame};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{GAME_STORAGE, OWNER, ROOM_ID, VK};
use crate::verifier::verify_proof;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coins, ensure, ensure_eq, to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError};
use cw_utils::must_pay;

// instantiate the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    ROOM_ID.save(deps.storage, &0)?;
    OWNER.save(deps.storage, &info.sender)?;
    VK.save(deps.storage, &msg.vk)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::QueryRoom { room_id } => {
            let game = GAME_STORAGE.load(_deps.storage, room_id)?;
            Ok(to_json_binary(&game)?)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateNewRoom {
            initial_state,
            deposit_price,
            denom,
        } => {
            let new_room_id = ROOM_ID.load(deps.storage)? + 1;
            GAME_STORAGE.save(
                deps.storage,
                new_room_id,
                &SudokuGame {
                    initial_state,
                    creator: info.sender,
                    deposit_price,
                    denom,
                    players: vec![],
                    solution: None,
                    winner: None,
                    claimed: false,
                },
            )?;
            Ok(Response::new().add_attribute("id", new_room_id.to_string()))
        }
        ExecuteMsg::JoinRoom { room_id } => {
            let mut game = GAME_STORAGE.load(deps.storage, room_id)?;
            ensure!(
                game.winner.is_none(),
                InvalidAction("the game is overed".to_string())
            );
            ensure!(
                !game.players.contains(&info.sender),
                InvalidAction("player already joined".to_string())
            );
            ensure!(
                must_pay(&info, &game.denom)?.u128() == game.deposit_price,
                InvalidAction("invalid deposit price".to_string())
            );
            game.players.push(info.sender);
            GAME_STORAGE.save(deps.storage, room_id, &game)?;
            Ok(Response::default())
        }
        ExecuteMsg::SubmitSolution { room_id, solution } => {
            let mut game = GAME_STORAGE.load(deps.storage, room_id)?;
            ensure!(
                game.winner.is_none(),
                InvalidAction("the game is overed".to_string())
            );
            match &solution {
                GameSolution::Public(solution) => {
                    check_solution(&game.initial_state, solution)?;
                }
                GameSolution::Private(proof) => {
                    verify_proof(proof, &VK.load(deps.storage)?)?;
                }
            };
            game.solution = Some(solution);
            game.winner = Some(info.sender);
            GAME_STORAGE.save(deps.storage, room_id, &game)?;
            Ok(Response::default())
        }
        ExecuteMsg::ClaimReward { room_id } => {
            let mut game = GAME_STORAGE.load(deps.storage, room_id)?;
            ensure!(
                game.winner.is_some(),
                InvalidAction("the game is not over yet".to_string())
            );
            ensure!(
                !game.claimed,
                InvalidAction("reward already claimed".to_string())
            );
            ensure!(
                info.sender == game.winner.clone().unwrap(),
                InvalidSolution("only winner can claim reward".to_string())
            );

            game.claimed = true;
            GAME_STORAGE.save(deps.storage, room_id, &game)?;

            let total_reward = game.deposit_price * (game.players.len() as u128);
            let resp = Response::new()
                .add_message(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: coins(total_reward, game.denom),
                })
                .add_attribute("action", "claim_reward")
                .add_attribute("total_reward", total_reward.to_string());

            Ok(resp)
        }
    }
}

fn check_solution(initial_state: &[(u8, u8)], answer: &[u8]) -> Result<(), ContractError> {
    ensure_eq!(
        initial_state.len() + answer.len(),
        81,
        InvalidSolution("sum of initial state and answer length must be 81".to_string())
    );

    let mut grid = [[0; 9]; 9];
    for (compressed_coordinate, value) in initial_state {
        ensure!(
            (1..=9).contains(value),
            InvalidSolution("values in initial state must be between 1 and 9".to_string())
        );
        let x = compressed_coordinate / 9;
        let y = compressed_coordinate % 9;
        grid[x as usize][y as usize] = *value;
    }
    let mut ptr = 0;
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j] == 0 {
                ensure!(
                    1 <= answer[ptr] && answer[ptr] <= 9,
                    InvalidSolution("values in answer state must be between 1 and 9".to_string())
                );
                grid[i][j] = answer[ptr];
                ptr += 1;
            }
        }
    }

    // check rows
    for (i, row) in grid.iter().enumerate() {
        ensure!(
            is_valid_group(row),
            InvalidSolution(format!("invalid row {}", i + 1))
        )
    }

    // check columns
    for j in 0..9 {
        let mut column = [0; 9];
        for i in 0..9 {
            column[i] = grid[i][j];
        }
        ensure!(
            is_valid_group(&column),
            InvalidSolution(format!("invalid column {}", j + 1))
        )
    }

    // check squares
    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            let mut square = [0; 9];
            let mut ptr = 0;
            for x in 0..3 {
                for y in 0..3 {
                    square[ptr] = grid[i + x][j + y];
                    ptr += 1;
                }
            }
            ensure!(
                is_valid_group(&square),
                InvalidSolution(format!("invalid square at ({}, {})", i + 1, j + 1))
            )
        }
    }
    Ok(())
}

fn is_valid_group(group: &[u8; 9]) -> bool {
    let mut seen = [false; 9];
    for x in group {
        if seen[(x - 1) as usize] {
            return false;
        }
        seen[(x - 1) as usize] = true;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::game::GameSolution;
    use crate::msg::ExecuteMsg::{JoinRoom, SubmitSolution};
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use cosmwasm_std::{coins, Addr, HexBinary};
    use cw_multi_test::{App, BasicApp, ContractWrapper, Executor};

    #[test]
    fn test_full_flow() {
        let deposit_price = 5;
        let denom = "xion".to_string();
        let (mut app, addr) = init();
        let mut players = vec![];
        for i in 0..5 {
            players.push(app.api().addr_make(&i.to_string()));
            app.init_modules(|router, _, storage| {
                router
                    .bank
                    .init_balance(storage, &players[i], coins(1000, &denom))
                    .unwrap()
            });
        }

        let room_id: u64 = app
            .execute_contract(
                players[0].clone(),
                addr.clone(),
                &ExecuteMsg::CreateNewRoom {
                    initial_state: initial_state(),
                    deposit_price,
                    denom: denom.clone(),
                },
                &[],
            )
            .unwrap()
            .events[1]
            .attributes[1]
            .value
            .parse()
            .unwrap();
        for player in &players {
            app.execute_contract(
                player.clone(),
                addr.clone(),
                &JoinRoom { room_id },
                &coins(deposit_price, &denom),
            )
            .unwrap();
        }
        app.execute_contract(
            players[0].clone(),
            addr.clone(),
            &SubmitSolution {
                room_id,
                solution: GameSolution::Public(solution()),
            },
            &[],
        )
        .unwrap();
        dbg!(app
            .wrap()
            .query_balance(&players[0], &denom)
            .unwrap()
            .amount
            .u128());
        app.execute_contract(
            players[0].clone(),
            addr,
            &ExecuteMsg::ClaimReward { room_id },
            &[],
        )
        .unwrap();
        dbg!(app
            .wrap()
            .query_balance(&players[0], &denom)
            .unwrap()
            .amount
            .u128());
    }

    fn init() -> (BasicApp, Addr) {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    vk: HexBinary::from_hex(
                        &include_str!("../../../games/sudoku/elf/verifying_key_hex")[2..],
                    )
                    .unwrap(),
                },
                &[],
                "verifier",
                None,
            )
            .unwrap();
        (app, addr)
    }

    fn initial_state() -> Vec<(u8, u8)> {
        vec![(0, 8), (1, 7), (7, 9), (14, 8), (17, 1)]
    }

    fn solution() -> Vec<u8> {
        vec![
            1, 4, 5, 6, 2, 3, 4, 5, 9, 2, 3, 6, 7, 2, 3, 6, 1, 7, 9, 4, 5, 8, 1, 2, 5, 8, 4, 3, 9,
            6, 7, 7, 6, 4, 9, 1, 5, 3, 8, 2, 3, 9, 8, 6, 2, 7, 5, 1, 4, 5, 8, 2, 3, 6, 1, 7, 4, 9,
            6, 1, 3, 7, 9, 4, 8, 2, 5, 9, 4, 7, 5, 8, 2, 1, 3, 6,
        ]
    }
}
