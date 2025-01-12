use crate::games::Game;
use async_trait::async_trait;
use sp1_sdk::network::proto::network::ProofMode;
use sp1_sdk::{NetworkProverV1, Prover, SP1ProofWithPublicValues, SP1Stdin};
use std::env::var;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, fs};

pub struct SudokuGame {
    pub initial_state: Vec<(u8, u8)>,
    pub answer: Vec<u8>
}

#[async_trait]
impl Game for SudokuGame {
    async fn generate_proof(&self) -> anyhow::Result<SP1ProofWithPublicValues> {
        env::set_var(
            "SP1_PRIVATE_KEY",
            var("SP1_PROVER_KEY")?,
        );
        let mut stdin = SP1Stdin::new();
        stdin.write(&self.initial_state);
        stdin.write(&self.answer);
        
        let client = NetworkProverV1::new();
        
        let elf = self.elf()?;
        let elf = elf.as_slice();
        let (_pk, vk) = client.setup(elf);
        let proof = client
            .prove(elf, stdin, ProofMode::Compressed, Some(Duration::from_secs(3600)))
            .await?;

        // let proof: SP1ProofWithPublicValues = serde_json::from_slice(include_bytes!("../../proof.json"))?;
        // let vk: SP1VerifyingKey = serde_json::from_slice(include_bytes!("../../vk.json"))?;
        fs::write("proof.json", serde_json::to_string(&proof)?)?;
        fs::write("vk.json", serde_json::to_string(&vk)?)?;
        println!("Proof created successfully");

        client.verify(&proof, &vk)?;

        Ok(proof)
    }

    fn elf(&self) -> anyhow::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        File::open(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../games/sudoku/elf/riscv32im-succinct-zkvm-elf"),
        )?
            .read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use crate::games::sudoku::SudokuGame;
    use crate::games::Game;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_generate_proof() {
        dotenv().ok();
        let initial_state = vec![(0, 8), (1, 7), (7, 9), (14, 8), (17, 1)];
        let answer = vec![
            1, 4, 5, 6, 2, 3,
            4, 5, 9, 2, 3, 6, 7,
            2, 3, 6, 1, 7, 9, 4, 5, 8,
            1, 2, 5, 8, 4, 3, 9, 6, 7,
            7, 6, 4, 9, 1, 5, 3, 8, 2,
            3, 9, 8, 6, 2, 7, 5, 1, 4,
            5, 8, 2, 3, 6, 1, 7, 4, 9,
            6, 1, 3, 7, 9, 4, 8, 2, 5,
            9, 4, 7, 5, 8, 2, 1, 3, 6
        ];
        let game = SudokuGame {
            initial_state,
            answer,
        };
        game.generate_proof().await.unwrap();
    }
}