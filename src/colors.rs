// Colors
const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";

// Effects
const BOLD_CODE: &str = "\x1b[1m";
const DIM_CODE: &str = "\x1b[2m";
const ITALIC_CODE: &str = "\x1b[3m";
const UNDERLINE_CODE: &str = "\x1b[4m";
const BLINK_CODE: &str = "\x1b[5m";
const REVERSE_CODE: &str = "\x1b[7m";
const HIDDEN_CODE: &str = "\x1b[8m";
const STRIKETHROUGH_CODE: &str = "\x1b[9m";

const RESET_CODE: &str = "\x1b[0m";

pub enum Color {
  BLACK,
  RED,
  GREEN,
  YELLOW,
  BLUE,
  MAGENTA,
  CYAN,
  WHITE,
}

pub enum Effect {
  BOLD,
  DIM,
  ITALIC,
  UNDERLINE,
  BLINK,
  REVERSE,
  HIDDEN,
  STRIKETHROUGH,
}


fn colorize(s: &str, color: Option<Color>, effect: Option<Effect>) -> String {
  let mut ansi_parts = Vec::new();

  if let Some(eff) = effect {
    match eff {
      Effect::BOLD => ansi_parts.push("1"),
      Effect::DIM => ansi_parts.push("2"),
      Effect::ITALIC => ansi_parts.push("3"),
      Effect::UNDERLINE => ansi_parts.push("4"),
      Effect::BLINK => ansi_parts.push("5"),
      Effect::REVERSE => ansi_parts.push("7"),
      Effect::HIDDEN => ansi_parts.push("8"),
      Effect::STRIKETHROUGH => ansi_parts.push("9"),
    }
  }

  if let Some(col) = color {
    match col {
      Color::BLACK => ansi_parts.push("30"),
      Color::RED => ansi_parts.push("31"),
      Color::GREEN => ansi_parts.push("32"),
      Color::YELLOW => ansi_parts.push("33"),
      Color::BLUE => ansi_parts.push("34"),
      Color::MAGENTA => ansi_parts.push("35"),
      Color::CYAN => ansi_parts.push("36"),
      Color::WHITE => ansi_parts.push("37"),
    }
  }

  if ansi_parts.is_empty() {
    s.to_string()
  } else {
    let ansi_prefix = format!("\x1b[{}m", ansi_parts.join(";"));
    format!("{}{}{}", ansi_prefix, s, RESET_CODE)
  }
}

pub trait ColorizeExt {
  fn green(self) -> String;
  fn red(self) -> String;
  fn yellow(self) -> String;
  fn blue(self) -> String;
  fn magenta(self) -> String;
  fn cyan(self) -> String;
  fn white(self) -> String;
  fn black(self) -> String;

  fn bold(self) -> String;
  fn dim(self) -> String;
  fn italic(self) -> String;
  fn underline(self) -> String;

  // Combos
  fn success(self) -> String;
  fn error(self) -> String;
  fn warning(self) -> String;
}

impl ColorizeExt for &str {
  fn green(self) -> String {
    colorize(self, Some(Color::GREEN), None)
  }
  fn red(self) -> String {
    colorize(self, Some(Color::RED), None)
  }
  fn yellow(self) -> String {
    colorize(self, Some(Color::YELLOW), None)
  }
  fn blue(self) -> String {
    colorize(self, Some(Color::BLUE), None)
  }
  fn magenta(self) -> String {
    colorize(self, Some(Color::MAGENTA), None)
  }
  fn cyan(self) -> String {
    colorize(self, Some(Color::CYAN), None)
  }
  fn white(self) -> String {
    colorize(self, Some(Color::WHITE), None)
  }
  fn black(self) -> String {
    colorize(self, Some(Color::BLACK), None)
  }

  fn bold(self) -> String {
    colorize(self, None, Some(Effect::BOLD))
  }
  fn dim(self) -> String {
    colorize(self, None, Some(Effect::DIM))
  }
  fn italic(self) -> String {
    colorize(self, None, Some(Effect::ITALIC))
  }
  fn underline(self) -> String {
    colorize(self, None, Some(Effect::UNDERLINE))
  }

  fn success(self) -> String {
    colorize(self, Some(Color::GREEN), Some(Effect::BOLD))
  }
  fn error(self) -> String {
    colorize(self, Some(Color::RED), Some(Effect::BOLD))
  }
  fn warning(self) -> String {
    colorize(self, Some(Color::YELLOW), Some(Effect::BOLD))
  }
}