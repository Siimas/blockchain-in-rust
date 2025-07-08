use crate::{address::Address, blockchain::Blockchain, utils::{colorize, Color, Effect}};

mod transaction;
mod blockchain;
mod address;
mod block;
mod utils;
mod node;

#[warn(unused_variables)] // ! DELETE

fn main() {

  let mut chain = Blockchain::new();

  let alice = Address::new("alice_pub");
  let bob = Address::new("bob_pub");

  println!("Genesis Block {:?}", chain.blocks.last().unwrap());

  chain.new_transaction(10, bob, &alice.public);
  println!("{} {:?}", colorize("New Transaction", Some(Color::GREEN), Some(Effect::BOLD)), chain.transaction_buffer.last());

}