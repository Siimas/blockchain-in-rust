use crate::address::Address;
use crate::utils::{get_timestamp, calculate_object_hash};

#[derive(Hash, Debug)]
pub struct Transaction {
  id: u64, // Hash of the previous transaction
  amount: u64,
  sender_address: String,    // public key
  recipient_address: String, // public key
  signature: String,
  timestamp: u64,
}

impl Transaction {
  pub fn new(amount: u64, sender_address_str: String, recipient_address_str: String, signature_str: String) -> Transaction {
    let timestamp = get_timestamp();
    // TODO: Before hashing, all this varied transaction data (which might include numbers, strings, arrays, etc.) must be converted into a single, standardized, deterministic sequence of bytes. This process is called serialization.
    let serialized_tx = format!(
      "{}{}{}{}", 
      amount,
      sender_address_str,
      recipient_address_str,
      timestamp
    );

    Transaction{
      id: calculate_object_hash(&serialized_tx),
      amount,
      sender_address: sender_address_str,
      recipient_address: recipient_address_str,
      signature: signature_str,
      timestamp,
    }
  }

  pub fn validate() -> bool {
    true
  }
}