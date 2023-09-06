pub struct Config {
    p: f32,
    i: f32,
    d: f32,
    izone: f32,
    error: f32,
}

impl Default for Config {
    fn default() -> Self { return Config::from(1.0, 1.0, 1.0, 1.0, 1.0); }
}

impl Config {
    // getters
    pub fn get_p(&self) -> f32 { return self.p; }
    pub fn get_i(&self) -> f32 { return self.i; }
    pub fn get_d(&self) -> f32 { return self.d; }
    pub fn get_izone(&self) -> f32 { return self.izone; }
    pub fn get_error(&self) -> f32 { return self.error; }

    // method to create a new struct
    pub fn from(p: f32, i: f32, d: f32, izone: f32, error: f32) -> Config {
        return Config {
            p: p,
            i: i,
            d: d,
            izone: izone,
            error: error,
        }
    }
}

