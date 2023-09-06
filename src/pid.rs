/*
 *  @name Config: the configuration struct for a PID controller
 *  @param p: the proportion value the PID loop
 *  @param i: the integral value of the PID loop
 *  @param d: the derivative value of the PID loop
 *  @param izone: the distance in which the loop will begin slowing to increase accuracy
 *  @param error: the acceptable error margin of the loop
 */
pub struct Config {
    p: f32,
    i: f32,
    d: f32,
    izone: f32,
    error: f32,
}

/*
 *  @name Default: an implementation of the default trait for the Config struct
 */
impl Default for Config {
    /*
     *  @name default: the default values
     */
    fn default() -> Self { return Config::from(1.0, 1.0, 1.0, 1.0, 1.0); }
}

/*
 *  @name Config methods
 */
impl Config {
    /*
     *  @name get_p: returns the p value
     */
    pub fn get_p(&self) -> f32 { return self.p; }
    /*
     *  @name get_i: returns the i value
     */
    pub fn get_i(&self) -> f32 { return self.i; }
    /*
     *  @name get_d: returns the d value
     */
    pub fn get_d(&self) -> f32 { return self.d; }
    /*
     *  @name get_izone: returns the izone value
     */
    pub fn get_izone(&self) -> f32 { return self.izone; }
    /*
     *  @name get_error: returns the error margin value
     */
    pub fn get_error(&self) -> f32 { return self.error; }

    /*
     *  @name from: the function to create a new pid controller instance
     */
    pub fn from(p: f32, i: f32, d: f32, izone: f32, error: f32) -> Self {
        return Config {
            p: p,
            i: i,
            d: d,
            izone: izone,
            error: error,
        }
    }
}

