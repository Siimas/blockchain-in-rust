use crate::{block::Block, transaction::Transaction};

pub struct Blockchain {
  pub blocks : Vec<Block>,
  pub transaction_buffer : Vec<Transaction>
}

impl Blockchain {
  pub fn new() -> Blockchain {
    Blockchain{
      blocks: vec!(Block::genesis()),
      transaction_buffer: Vec::new(),
    }
  }

  pub fn new_transaction(&mut self, amount: u64, sender_address: &Address, recipient_address_str: &str) {
    self.transaction_buffer.push(
      Transaction::new(
        amount, 
        sender_address.public.clone(), 
        sender_address.get_signature(),
        recipient_address_str.to_string(), 
      )
    );
  }
}