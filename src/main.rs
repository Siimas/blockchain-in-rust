use crate::{blockchain::Blockchain, utils::{colorize, io_input, Color, Effect}};

mod transaction;
mod blockchain;
mod wallet;
mod block;
mod utils;
mod node;

#[warn(unused_variables)] // ! DELETE

// Node:
// - mantains a full or partial copy of the blockchain ledger
// - verifies incoming transactions
// - validates and mines blocks (consensus)
// - relays transactions and blocks to other nodes
fn main() {
  let mut chain = Blockchain::new();
  println!("{} {:?}", colorize("Genesis Block", Some(Color::YELLOW), Some(Effect::BOLD)), chain.blocks.last().unwrap());

  let mut bob = Address::new("bob_pub");

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
        println!("\n{}", colorize("Exiting...", Some(Color::RED), Some(Effect::BOLD)));
        break;
      },
      "1" => new_transaction_screen(&mut chain, &mut bob),
      "2" => (),
      "3" => (),
      _ => (),
    }

    option.clear();
  }

}

fn new_transaction_screen(chain: &mut Blockchain, sender_address: &mut Address) {
  let amount = io_input("Amount");
  let recipient_address = io_input("Recipient Address");
    
  let amount_parsed: u64 = amount.trim().parse().expect("Failed to parse amount as u64");

  chain.new_transaction(amount_parsed, sender_address, &recipient_address);
  println!("\n{} {:?}", colorize("New Transaction:", Some(Color::GREEN), Some(Effect::BOLD)), chain.transaction_buffer.last());
}

