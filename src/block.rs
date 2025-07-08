
use crate::transaction::{Transaction};
use crate::utils::{get_timestamp};

const DIFFICULTY_TARGET: i32 = 1;

struct Merkle {}

struct Header {
  // version: i32,
  timestamp: u64,
  merkle_root: Merkle,
  previous_block_hash: String,
  nonce: i32,
  difficulty_target: i32,
}

impl Header {
  pub fn new(previous_block_hash : String) -> Header {
    Header{
      nonce: 0,
      previous_block_hash,
      merkle_root: Merkle{},
      timestamp: get_timestamp(),
      difficulty_target: DIFFICULTY_TARGET,
    }
  }
}

pub struct Block {
  header: Header,
  data: Vec<Transaction>,
}

impl Block {
  pub fn new(previous_block_hash : String) -> Block {
    Block{
      header: Header::new(previous_block_hash),
      data: Vec::new(),
    }
  }

  pub fn add_transaction(&mut self, tx : Transaction) {
    self.data.push(tx);
  }
}