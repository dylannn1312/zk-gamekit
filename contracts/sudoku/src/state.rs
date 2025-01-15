use cosmwasm_std::{Addr, HexBinary};
use cw_storage_plus::{Item, Map};
use crate::game::SudokuGame;

pub(crate) const GAME_STORAGE: Map<u64, SudokuGame> = Map::new("sudoku game");
pub(crate) const ROOM_ID: Item<u64> = Item::new("sudoku room id");
pub(crate) const OWNER: Item<Addr> = Item::new("sudoku owner");
pub(crate) const VK: Item<HexBinary> = Item::new("sudoku verifying key");