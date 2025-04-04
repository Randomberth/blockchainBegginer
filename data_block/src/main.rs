use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Write;

// Definición de un bloque

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: DateTime<Utc>,
    data: Vec<Transaccion>,
    previous_hash: String,
    hash: String,
}

// 1. Define una estructura para los datos complejos
//#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Transaccion {
    sender: String,           // Remitente del mensaje
    receiver: String,         // Receptor del mensaje
    message: String,          // Contenido (ej: "Hola Mundo")
    timestamp: DateTime<Utc>, // Fecha de creación
}

impl Block {
    //Crear un nuevo bloque
    fn new(index: u64, data: Vec<Transaccion>, previous_hash: String) -> Self {
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
        data: &[Transaccion],
        previous_hash: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_rfc3339());

        // serializar datos a JSON para hashear
        let data_json = serde_json::to_string(data).unwrap();
        hasher.update(data_json);

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

    let transaccion = Transaccion {
        sender: "Alicia".to_string(),
        receiver: "White Rabbit".to_string(),
        message: "Test de uso de blockchain y almacenaje de datos".to_string(),
        timestamp: Utc::now(),
    };

    blockchain.add_block(vec![transaccion]); // se añade el bloque con la data

    // imprimir la cadena
    for block in blockchain.chain {
        println!(
            "Bloque {}: {} | Hash: {}",
            block.index, block.data, block.hash
        );
    }
}
