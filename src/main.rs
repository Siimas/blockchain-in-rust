use crate::{colors::ColorizeExt, node::Node, transaction::Transaction, utils::io_input, wallet::Wallet};

mod transaction;
mod wallet;
mod block;
mod node;

mod colors;
mod utils;

#[warn(unused_variables)] // ! DELETE

// Node:
// - mantains a full or partial copy of the blockchain ledger
// - verifies incoming transactions
// - validates and mines blocks (consensus)
// - relays transactions and blocks to other nodes
fn main() {
  let mut node = Node::new();

  let bob_wallet = Wallet::new();
  
  let mut option : String;
  loop {
    println!("\nOptions:");
    println!("  1 - New Transaction");
    println!("  2 - Mine Block");
    println!("  3 - View Chain");
    println!("  0 - Exit");

    option = io_input(">");

    match option.trim() {
      "0" => {
        println!("\n{}", "Exiting...".error());
        break;
      },
      "1" => new_transaction_screen(&mut node),
      "2" => (),
      "3" => (),
      _ => (),
    }

    option.clear();
  }

}

fn new_transaction_screen(node: &mut Node) {
  let amount = io_input("Amount");
  let amount_parsed: u64 = amount.trim().parse().expect("Failed to parse amount as u64");

  let sender_address = io_input("Sender Address");
  let recipient_address = io_input("Recipient Address");
  let signature_str = String::from("test");
  
  node.add_transaction(
    Transaction::new(
      amount_parsed, 
      sender_address, 
      recipient_address, 
      signature_str
    )
  );
}

