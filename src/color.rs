pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Rgba {
    pub fn red(&self) -> u8 {
        self.red
    }
    pub fn green(&self) -> u8 {
        self.green
    }
    pub fn blue(&self) -> u8 {
        self.blue
    }
    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    pub fn red_f64(&self) -> f64 {
        From::from(self.red)
    }
    pub fn green_f64(&self) -> f64 {
        From::from(self.green)
    }
    pub fn blue_f64(&self) -> f64 {
        From::from(self.blue)
    }
    pub fn alpha_f64(&self) -> f64 {
        From::from(self.alpha)
    }

    pub fn min_value() -> u8 {
        0
    }
    pub fn min_value_f64() -> f64 {
        0.0
    }

    pub fn max_value() -> u8 {
        255
    }
    pub fn max_value_f64() -> f64 {
        255.0
    }
}
