mod block;
mod blockchain;
mod transaction;
mod wallet;

use blockchain::Blockchain;
use ed25519_dalek::PublicKey;
use std::fs::File;
use std::io::Read;
use transaction::Transaction;
use wallet::Wallet;

fn main() {
    // 1️⃣ Carrega carteiras salvas do disco
    let alice_wallet = Wallet::load_from_file("alice").expect("Carteira de Alice não encontrada");
    let bob_pub_key =
        load_public_key("wallets/bob.pub").expect("Chave pública do Bob não encontrada");

    // 2️⃣ Cria blockchain com persistência automática
    let mut blockchain = Blockchain::new(3);

    // 3️⃣ Cria uma transação assinada pela Alice para Bob
    let tx = Transaction::new(&alice_wallet.keypair, bob_pub_key, 100);

    // 4️⃣ Adiciona o bloco contendo essa transação
    blockchain.add_block(vec![tx]);

    // 5️⃣ Exibe blockchain completa e verifica validade
    println!("\n🔗 Blockchain:");
    for block in &blockchain.chain {
        println!("{:#?}", block);
    }

    println!("\n✔️ Blockchain válida? {}", blockchain.is_valid());

    let alice_balance = blockchain.get_balance(&alice_wallet.keypair.public);
    println!("💰 Saldo da Alice: {}", alice_balance);
}

/// Função auxiliar para carregar chave pública de arquivo .pub
fn load_public_key(path: &str) -> Option<PublicKey> {
    let mut file = File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;
    PublicKey::from_bytes(&bytes).ok()
}
