use sha2::{Digest, Sha512};
use std::fmt;
pub trait Hashable {
    fn hash(&self) -> Vec<u8> {
        let mut hasher: Sha512 = Sha512::new();
        hasher.update(self.hash_repr());
        let h = hasher.finalize();
        let a: Vec<u8> = h.into_iter().collect();
        println!("{:?}", a);
        // return format!("{:x}", h);
        return a;
    }
    fn hash_repr(&self) -> String;
}
pub struct Transaction {
    pub id: u64,
    pub origin: String,
    pub destination: String,
    pub amount: f64,
    pub message: String,
    pub identity_proof: String,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Transaction {{ id: {}, origin: {}, destination: {}, amount: {}, message: {}, identity: {} }}", 
            self.id, self.origin, self.destination,self.amount, self.message, self.identity_proof

        )
    }
}

impl Hashable for Transaction {
    fn hash_repr(&self) -> String {
        return format!(
            "{},{},{},{},{},{}",
            self.id, self.origin, self.destination, self.amount, self.message, self.identity_proof
        );
    }
}

pub struct PreviousBlock {
    pub id: u64,
    pub hash: Vec<u8>,
    pub block: Option<Box<Block>>,
}

impl Hashable for PreviousBlock {
    fn hash(&self) -> Vec<u8> {
        return self.hash.clone();
    }

    fn hash_repr(&self) -> String {
        return format!("{:?}", self.hash);
    }
}

pub struct Block {
    pub id: String,
    pub nonce: String,
    pub transactions: Vec<Transaction>,
    pub previous: PreviousBlock,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block:
        id: {},
        nonce: {},
        transactions: {{
            \t{}
        \t}}
         previous: {}",
            self.id,
            self.nonce,
            self.transactions
                .iter()
                .map(|x| x.to_string())
                .fold("".to_string(), |r, x| { x + ",\n\t\t" + &r }),
            self.previous.id
        )
    }
}

impl Hashable for Block {
    fn hash_repr(&self) -> String {
        return format!(
            "{},{},{:?},{:?}",
            self.id,
            self.nonce,
            self.transactions
                .iter()
                .map(|x| x.hash())
                .collect::<Vec<Vec<u8>>>(),
            self.previous.hash()
        );
    }
}

pub trait ModifiableBlock {
    fn change_nonce(&mut self, new_nonce: String);
}

impl ModifiableBlock for Block {
    fn change_nonce(&mut self, new_nonce: String) {
        self.nonce = new_nonce;
    }
}

pub fn get_leading(bytes: &[u8]) -> u64 {
    let mut zeros = 0u64;
    for &byte in bytes {
        zeros += byte.leading_zeros() as u64;
        if byte != 0 {
            break;
        }
    }

    zeros
}
trait ProofOfWork {
    fn check_hash(&self, difficulty: u64, hash: Vec<u8>) -> bool {
        get_leading(&hash) <= difficulty
    }

    fn proof_of_work(&self, difficulty: u64) -> String;

    fn check_nonce(&mut self, difficulty: u64, nonce: String) -> bool;
}

impl ProofOfWork for Block {
    fn proof_of_work(&self, difficulty: u64) -> String {
        return "s".to_string();
    }

    fn check_nonce(&mut self, difficulty: u64, nonce: String) -> bool {
        self.change_nonce(nonce);
        return self.check_hash(difficulty, self.hash());
    }
}
