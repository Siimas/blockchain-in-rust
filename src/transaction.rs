use crate::utils::{get_timestamp, calculate_object_hash};

#[derive(Hash, Debug)]
pub struct Address {
  public: String,
}

impl Address {
  pub fn new(public_str: &str) -> Address {
    Address{
      public: public_str.to_string(),
    }
  }
}

#[derive(Hash, Debug)]
pub struct Transaction {
  id: String, // Hash of the previous transaction
  amount: u64,
  sender_address: Address,    // public key
  recipient_address: Address, // public key
  signature: String,
  timestamp: u64,
} 

impl Transaction {
  pub fn new(id_str: &str, amount: u64, sender_address: Address, recipient_address: Address, signature_str: &str) -> Transaction {
    Transaction{
      id: id_str.to_string(),
      amount,
      sender_address,
      recipient_address,
      signature: signature_str.to_string(),
      timestamp: get_timestamp(),
    }
  }

  pub fn calculate_hash(&mut self) -> u64 {
    let serialized_tx = format!(
      "{}{}{}{}{}", 
      self.id, 
      self.amount, 
      self.sender_address.public,
      self.recipient_address.public,
      self.timestamp
    );
    // ? println!("Serialized Tx: {}", serialized_tx);
    return calculate_object_hash(&serialized_tx);
  }

  pub fn validate() -> bool {
    true
  }
}