use chrono::prelude::*;
use sha2::{Digest, Sha256};
use std::fmt::Write;

// Definición de un bloque

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    //Crear un nuevo bloque
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let hash = Self::calculate_hash(index, &timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(
        index: u64,
        timestamp: &DateTime<Utc>,
        data: &str,
        previous_hash: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_rfc3339());
        hasher.update(data);
        hasher.update(previous_hash);

        let result = hasher.finalize();
        let mut hash_hex = String::new();

        for byte in result {
            write!(&mut hash_hex, "{:02x}", byte).unwrap();
        }

        hash_hex
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    // inicializar la cadena con el bloque genésis
    fn new() -> Self {
        let genesis_block = Block::new(0, "Bloque Génesis".to_string(), "0".to_string());

        Blockchain {
            chain: vec![genesis_block],
        }
    }

    //Añadir un nuevo bloque a la cadena
    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, data, previous_block.hash.clone());

        self.chain.push(new_block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    // Añadir nuestro "Hola Mundo Blockchain" al blockchain
    blockchain.add_block("Hola mundo Blockchain".to_string());

    // Añadir nuestro "test one Blockchain" al blockchain
    blockchain.add_block("test one in Blockchain".to_string());

    // Añadir nuestro "otro mensaje" al blockchain
    blockchain.add_block("Aqui va un mensaje motivacional".to_string());

    // imprimir la cadena
    for block in blockchain.chain {
        println!(
            "Bloque {}: {} | Hash: {}",
            block.index, block.data, block.hash
        );
    }
}
