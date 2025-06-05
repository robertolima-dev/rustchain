use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct WalletData {
    pub secret: Vec<u8>,
    pub public: Vec<u8>,
}

pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    /// Cria uma nova carteira
    pub fn generate() -> Self {
        let mut rng = OsRng;
        let keypair = Keypair::generate(&mut rng);
        Wallet { keypair }
    }

    /// Salva a carteira em arquivo JSON
    pub fn save_to_file(&self, name: &str) {
        let wallet = WalletData {
            secret: self.keypair.secret.to_bytes().to_vec(),
            public: self.keypair.public.to_bytes().to_vec(),
        };

        create_dir_all("wallets").unwrap();
        let path = format!("wallets/{}_wallet.json", name);
        let json = serde_json::to_string_pretty(&wallet).unwrap();
        let mut file = File::create(&path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        println!("💾 Carteira salva em {}", path);

        // Salva chave pública separada
        let pub_path = format!("wallets/{}.pub", name);
        let mut pub_file = File::create(&pub_path).unwrap();
        pub_file.write_all(&wallet.public).unwrap();
        println!("🔑 Chave pública salva em {}", pub_path);
    }

    /// Carrega uma carteira de arquivo JSON
    pub fn load_from_file(name: &str) -> Option<Self> {
        let path = format!("wallets/{}_wallet.json", name);
        if !Path::new(&path).exists() {
            println!("❌ Carteira '{}' não encontrada.", name);
            return None;
        }

        let mut file = File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let wallet_data: WalletData = serde_json::from_str(&contents).unwrap();

        let secret = SecretKey::from_bytes(&wallet_data.secret).unwrap();
        let public = PublicKey::from_bytes(&wallet_data.public).unwrap();
        let keypair = Keypair { secret, public };

        Some(Wallet { keypair })
    }

    /// Exibe chave pública em hex
    pub fn public_hex(&self) -> String {
        hex::encode(self.keypair.public.as_bytes())
    }
}
