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

pub enum Color {
  RED,
  GREEN,
  YELLOW,
  BLUE,
  MAGENTA,
  CYAN,
  // Removed RESET and BOLD from Color enum as they are effects, not primary colors
}

pub enum Effect {
  BOLD,
  ITALIC,
  RESET, // Add RESET here as a general effect
}

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";

const RESET_CODE: &str = "\x1b[0m"; // Renamed to avoid confusion with Effect::RESET
const BOLD_CODE: &str = "\x1b[1m";   // Renamed to avoid confusion with Effect::BOLD
const ITALIC_CODE: &str = "\x1b[3m"; // Added for italic effect


// Changed return type to String
pub fn colorize(s: &str, color: Option<Color>, effect: Option<Effect>) -> String {
  let mut color_code = "";
  let mut effect_code = ""; // Initialize to empty string

  match color {
    Some(Color::RED) => color_code = RED,
    Some(Color::GREEN) => color_code = GREEN,
    Some(Color::YELLOW) => color_code = YELLOW,
    Some(Color::BLUE) => color_code = BLUE,
    Some(Color::MAGENTA) => color_code = MAGENTA,
    Some(Color::CYAN) => color_code = CYAN,
    None => color_code = ""
  }

  match effect {
    Some(Effect::BOLD) => effect_code = BOLD_CODE,
    Some(Effect::ITALIC) => effect_code = ITALIC_CODE,
    Some(Effect::RESET) => effect_code = RESET_CODE, // If you want to apply reset as an effect
    None => effect_code = ""
  }

  // format! returns a String, which now matches the function's return type
  format!("{}{}{}{}", effect_code, color_code, s, RESET_CODE)
}
