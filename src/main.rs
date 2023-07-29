mod data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transaction = data::Transaction {
        id: 1,
        origin: "Alice".to_string(),
        destination: "Bob".to_string(),
        amount: 100.0,
        message: "TestTransaction".to_string(),
        identity_proof: "SecretKey".to_string(),
    };
    let t2 = data::Transaction {
        id: 2,
        origin: "a".to_string(),
        destination: "b".to_string(),
        amount: 10.0,
        message: "Test2".to_string(),
        identity_proof: "Proof".to_string(),
    };
    let a = data::Block {
        id: "id".to_string(),
        nonce: "".to_string(),
        transactions: vec![transaction, t2],
        previous: data::PreviousBlock {
            id: 24,
            hash: Vec::new(),
            block: None,
        },
    };
    // let transaction_hash = transaction.hash();
    println!("{}", a);
    Ok(())
}
