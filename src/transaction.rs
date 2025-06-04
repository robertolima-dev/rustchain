use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};

// Serialização manual para tipos binários (PublicKey, Signature)
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: PublicKey,
    pub to: PublicKey,
    pub amount: u64,
    pub signature: Signature,
}

impl Transaction {
    pub fn new(from: &Keypair, to: PublicKey, amount: u64) -> Self {
        let message = Self::message_bytes(&from.public, &to, amount);
        let signature = from.sign(&message);
        Transaction {
            from: from.public,
            to,
            amount,
            signature,
        }
    }

    pub fn is_valid(&self) -> bool {
        let message = Self::message_bytes(&self.from, &self.to, self.amount);
        self.from.verify(&message, &self.signature).is_ok()
    }

    fn message_bytes(from: &PublicKey, to: &PublicKey, amount: u64) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(from.as_bytes());
        bytes.extend(to.as_bytes());
        bytes.extend(&amount.to_be_bytes());
        bytes
    }
}

// ✅ Serialização personalizada para tipos binários

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Transaction", 4)?;
        s.serialize_field("from", &hex::encode(self.from.as_bytes()))?;
        s.serialize_field("to", &hex::encode(self.to.as_bytes()))?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("signature", &hex::encode(self.signature.to_bytes()))?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Transaction, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TransactionHelper {
            from: String,
            to: String,
            amount: u64,
            signature: String,
        }

        let helper = TransactionHelper::deserialize(deserializer)?;
        let from_bytes = hex::decode(&helper.from).map_err(de::Error::custom)?;
        let to_bytes = hex::decode(&helper.to).map_err(de::Error::custom)?;
        let sig_bytes = hex::decode(&helper.signature).map_err(de::Error::custom)?;

        let from = PublicKey::from_bytes(&from_bytes).map_err(de::Error::custom)?;
        let to = PublicKey::from_bytes(&to_bytes).map_err(de::Error::custom)?;
        let signature = Signature::from_bytes(&sig_bytes).map_err(de::Error::custom)?;

        Ok(Transaction {
            from,
            to,
            amount: helper.amount,
            signature,
        })
    }
}
