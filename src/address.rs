#[derive(Hash, Debug)]
pub struct Address {
  pub public: String,
  private: String,
}

impl Address {
  pub fn new(public_str: &str) -> Address {
    // TODO: generate this key pair
    Address{
      public: public_str.to_string(),
      private: "priv_x1".to_string(),
    }
  }

  pub fn get_signature(&self) -> String {
    "sig_x1".to_string()
  }
}