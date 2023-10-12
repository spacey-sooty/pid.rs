/// The module for all of the PID Controllers configuration
pub mod config {
    /// A struct which holds a PID controllers configuration information. Takes in a path, p, i and
    /// d values, izone and error margin. An instance can be created using the from method eg
    /// ```
    /// use pid::config::Config;
    /// let config = Config::from("/arm".to_string(), 1.0, 1.0, 1.0, 0.1, 0.001);
    /// ```
    pub struct Config {
        path: String,
        p: f32,
        i: f32,
        d: f32,
        izone: f32,
        error: f32,
    }

    /// An implementation of the default trait for the config struct.
    impl Default for Config {
        /// The default values for a config struct.
        fn default() -> Self {
            Config::from("/pid".to_string(), 1.0, 1.0, 1.0, 1.0, 1.0)
        }
    }

    /// Methods for the config struct
    impl Config {
        /// Returns the structs p value
        pub fn get_p(&self) -> f32 {
            self.p
        }

        /// Returns the structs i value
        pub fn get_i(&self) -> f32 {
            self.i
        }

        /// Returns the structs d value
        pub fn get_d(&self) -> f32 {
            self.d
        }

        /// Returns the structs izone value
        pub fn get_izone(&self) -> f32 {
            self.izone
        }

        /// returns the structs error margin
        pub fn get_error(&self) -> f32 {
            self.error
        }

        /// returns the structs path
        pub fn get_path(&self) -> &String {
            &self.path
        }

        /// the from method used to intialise the struct
        pub fn from(path: String, p: f32, i: f32, d: f32, izone: f32, error: f32) -> Self {
            Config {
                path,
                p,
                i,
                d,
                izone,
                error,
            }
        }
    }
}

pub mod controller {
    pub struct StateInfo {
        is_active: bool,
    }

    impl StateInfo {
        pub fn is_active(&self) -> bool {
            self.is_active
        }

        pub fn from(is_active: bool) -> Self {
            StateInfo { is_active }
        }
    }

    impl Default for StateInfo {
        fn default() -> Self {
            StateInfo { is_active: false }
        }
    }

    pub enum State {
        Running(StateInfo),
        Finished(StateInfo),
        Err(StateInfo),
    }

    impl Default for State {
        fn default() -> Self {
            let info = StateInfo { is_active: false };
            State::Err(info)
        }
    }

    impl StateInfo {}
}
