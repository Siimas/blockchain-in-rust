use crate::{block::Block, colors::ColorizeExt, transaction::{Transaction, TransactionValidationError}};

pub struct Node {
  mempool : Vec<Transaction>,
  blockchain : Vec<Block>,
}

impl Node {
  pub fn new() -> Self {
    Node{
      mempool: Vec::new(),
      blockchain: Vec::new(),
    }
  }

  pub fn get_last_block(&self) -> &Block {
    self.blockchain.last().expect("Failed to get the last block")
  }

  pub fn add_transaction(&mut self, tx : Transaction) {
    match tx.validate() {
      Ok(()) => {
        self.mempool.push(tx);
        println!(
          "\n{} {:?}",
          "New Transaction:".success(),
          self.get_last_block()
        );
      },
      Err(e) => {
        match e {
          TransactionValidationError::InvalidSignature => {
            println!("{}", "Signature mismatch!".error());
          },
          TransactionValidationError::InsufficientFunds => {
            println!("{}", "Sender has insufficient funds!".error());
          },
          _ => {
            println!("{}", "Other validation error.".error());
          }
        }
      }
    }
  }

  pub fn mine_block() {
    
  }
}