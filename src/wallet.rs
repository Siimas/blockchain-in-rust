pub struct PrivateKey(Vec<u8>);
pub struct PublicKey(Vec<u8>);
pub struct Address (String);

impl PrivateKey {
  pub fn generate_new() -> Self {
    PrivateKey(Vec::new())
  }
}

impl PublicKey {
  pub fn from_private_key(private_key: &Vec<u8>) -> Self {
    PublicKey(Vec::new())
  }
}

impl Address {
  pub fn from_public_key(public_key: &Vec<u8>) -> Self {
    Address(String::new())
  }
}

pub struct Wallet {
  private_key: PrivateKey,
  public_key: PublicKey,
  address: Address,
}

impl Wallet {
  pub fn new() -> Self {
    let private_key= PrivateKey(Vec::new());
    let public_key= PublicKey(Vec::new());
    let address = Address::from_public_key(&public_key); 

    Wallet{
      private_key,
      public_key, 
      address,
    }
  }
}