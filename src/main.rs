mod block;
mod blockchain;
mod transaction;

use blockchain::Blockchain;
use transaction::Transaction;

use ed25519_dalek::{Keypair, PublicKey, Signer};
use rand::rngs::OsRng;

fn main() {
    let mut rng = OsRng;

    let alice = Keypair::generate(&mut rng);
    let bob = Keypair::generate(&mut rng);
    let carol = Keypair::generate(&mut rng);

    let mut blockchain = Blockchain::new(3);

    let tx1 = Transaction::new(&alice, bob.public, 100);
    let tx2 = Transaction::new(&bob, carol.public, 50);

    blockchain.add_block(vec![tx1]);
    blockchain.add_block(vec![tx2]);

    println!("\n🔗 Blockchain:");
    for block in &blockchain.chain {
        println!("{:#?}", block);
    }

    println!("\n✔️ Blockchain válida? {}", blockchain.is_valid());
}
