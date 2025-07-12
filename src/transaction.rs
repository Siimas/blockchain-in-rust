use k256::ecdsa::VerifyingKey;

use crate::utils::{get_timestamp, calculate_object_hash};

#[derive(Hash, Debug)]
pub struct Transaction {
  pub id: u64, // Hash of the previous transaction
  pub amount: u64,
  pub sender_public_key: VerifyingKey,
  pub sender_address: String, 
  pub recipient_address: String,
  pub signature: String,
  pub timestamp: u64,
}

pub enum TransactionValidationError {
  InvalidSignature,
  InsufficientFunds,
  DoubleSpendAttempt,
  GenericError(String),
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
      sender_public_key: ,
      sender_address: sender_address_str,
      recipient_address: recipient_address_str,
      signature: signature_str,
      timestamp,
    }
  }

  // TODO: improve
  pub fn validate(&self) -> Result<(), TransactionValidationError> {
    // validate signature
    self.
  }
}