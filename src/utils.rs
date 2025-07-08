use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn calculate_object_hash<T: Hash + ?Sized>(t: &T) -> u64 {
  let mut s = DefaultHasher::new();
  t.hash(&mut s);
  s.finish()
}

pub fn get_timestamp() -> u64 {
  let now = SystemTime::now();
  now.duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs()
}