use crate::block::Block;
use crate::transaction::Transaction;
use ed25519_dalek::PublicKey;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    /// Cria uma nova blockchain ou carrega do arquivo blockchain.json
    pub fn new(difficulty: usize) -> Self {
        match Self::load_from_file("blockchain.json") {
            Some(chain) => {
                println!("📂 Blockchain carregada do disco.");
                Blockchain { chain, difficulty }
            }
            None => {
                println!("🆕 Criando nova blockchain...");
                let mut bc = Blockchain {
                    chain: Vec::new(),
                    difficulty,
                };
                bc.create_genesis_block();
                bc.save_to_file("blockchain.json");
                bc
            }
        }
    }

    /// Cria o primeiro bloco da cadeia
    fn create_genesis_block(&mut self) {
        let genesis = Block::new(0, Vec::new(), "0".to_string(), self.difficulty);
        self.chain.push(genesis);
    }

    /// Adiciona um novo bloco à cadeia e salva no arquivo
    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let last_hash = self.chain.last().unwrap().hash.clone();
        let block = Block::new(
            self.chain.len() as u64,
            transactions,
            last_hash,
            self.difficulty,
        );
        self.chain.push(block);
        self.save_to_file("blockchain.json");
    }

    /// Valida a integridade da blockchain
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Valida o hash do bloco atual
            if current.hash != current.calculate_hash() {
                println!("❌ Hash inválido no bloco {}", current.index);
                return false;
            }

            // Verifica se o bloco aponta corretamente para o anterior
            if current.previous_hash != previous.hash {
                println!("❌ Hash anterior inválido no bloco {}", current.index);
                return false;
            }

            // Valida todas as transações do bloco
            for tx in &current.transactions {
                if !tx.is_valid() {
                    println!("❌ Transação inválida no bloco {}", current.index);
                    return false;
                }
            }
        }

        true
    }

    /// Salva a blockchain no disco como JSON
    fn save_to_file(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self.chain)
            .expect("❌ Falha ao serializar a blockchain.");
        let mut file = File::create(path).expect("❌ Não foi possível criar o arquivo.");
        file.write_all(json.as_bytes())
            .expect("❌ Falha ao escrever no arquivo.");
        println!("💾 Blockchain salva em '{}'.", path);
    }

    /// Tenta carregar a blockchain de um arquivo JSON
    fn load_from_file(path: &str) -> Option<Vec<Block>> {
        if let Ok(mut file) = OpenOptions::new().read(true).open(path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(chain) = serde_json::from_str::<Vec<Block>>(&contents) {
                    return Some(chain);
                }
            }
        }
        None
    }
    pub fn get_balance(&self, address: &PublicKey) -> u64 {
        let mut balance: i64 = 0;
        let target_bytes = address.as_bytes();

        for block in &self.chain {
            for tx in &block.transactions {
                if tx.to.as_bytes() == target_bytes {
                    balance += tx.amount as i64;
                }
                if tx.from.as_bytes() == target_bytes {
                    balance -= tx.amount as i64;
                }
            }
        }

        balance.max(0) as u64
    }
}
