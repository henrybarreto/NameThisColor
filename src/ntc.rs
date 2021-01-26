#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64
}
pub struct NTC {
  pub names: Vec<String>,
  pub colors: Vec<Color>
}

impl NTC {
  pub fn new() -> Self {
    NTC {
      names: vec![],
      colors: vec![]
    }
  }
  pub fn save_color(&mut self, color: Color, name: String) -> Result<(), String> {
    if self.colors.contains(&color) || self.names.contains(&name) {
      Err("The color was already saved!".to_string())
    } else {
      self.colors.push(color);
      self.names.push(name);
      Ok(())
    }
  }
}
