
use crate::transaction::{Transaction};
use crate::utils::{get_timestamp, calculate_object_hash};

const DIFFICULTY_TARGET: i32 = 1;

#[derive(Hash, Debug)]
struct Merkle {}

#[derive(Hash, Debug)]
struct Header {
  // version: i32,
  pub timestamp: u64,
  pub merkle_root:Option<Merkle>,
  pub previous_block_hash: Option<u64>,
  pub nonce: i32,
  pub difficulty_target: i32,
}

impl Header {
  pub fn new(previous_block_hash : u64) -> Header {
    Header{
      nonce: 0,
      timestamp: get_timestamp(),
      merkle_root: Some(Merkle{}),
      difficulty_target: DIFFICULTY_TARGET,
      previous_block_hash: Some(previous_block_hash),
    }
  }

  pub fn genesis() -> Header {
    Header{
      nonce: 0,
      merkle_root: None,
      difficulty_target: 0,
      previous_block_hash: None,
      timestamp: get_timestamp(),
    }
  }
}

#[derive(Hash, Debug)]
pub struct Block {
  pub header: Header,
  pub data: Vec<Transaction>,
}

impl Block {
  pub fn new(previous_block_hash : u64) -> Block {
    Block{
      header: Header::new(previous_block_hash),
      data: Vec::new(),
    }
  }

  pub fn genesis() -> Block {
    Block { 
      header: Header::genesis(), 
      data: Vec::new(),
    }
  }

  pub fn add_transaction(&mut self, tx : Transaction) {
    self.data.push(tx);
  }

  pub fn get_hash(&self) -> u64 {
    calculate_object_hash(self)
  }
}