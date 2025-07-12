use k256::{ecdsa::{SigningKey, VerifyingKey}, elliptic_curve::rand_core::OsRng};

pub struct Wallet {
  private_key: SigningKey,
  public_key: VerifyingKey,
  address: String,
}

impl Wallet {
  pub fn new() -> Self {
    let (private_key, public_key)= generate_key_pair();
    let address = generate_address(&public_key); 
    Wallet{
      private_key,
      public_key,
      address,
    }
  }
}

// TODO
fn generate_address(public_key : &VerifyingKey) -> String {
  String::from("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa")
}

fn generate_key_pair() -> (SigningKey, VerifyingKey) {
  let signing_key = SigningKey::random(&mut OsRng);
  let verifying_key = VerifyingKey::from(&signing_key);
  (signing_key, verifying_key)
}