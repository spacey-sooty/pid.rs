/*
 *  @name config: the module for all of the PID Controllers configuration
 */
pub mod config {
    /*
     *  @name Config: the configuration struct for a PID controller
     *  @param path: for network tables
     *  @param p: the proportion value the PID loop
     *  @param i: the integral value of the PID loop
     *  @param d: the derivative value of the PID loop
     *  @param izone: the distance in which the loop will begin slowing to increase accuracy
     *  @param error: the acceptable error margin of the loop
     */
    pub struct Config {
        path: String,
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
        fn default() -> Self {
            Config::from(String::from("/pid"), 1.0, 1.0, 1.0, 1.0, 1.0)
        }
    }

    /*
     *  @name Config methods
     */
    impl Config {
        /*
         *  @name get_p: returns the p value
         */
        pub fn get_p(&self) -> f32 {
            self.p
        }
        /*
         *  @name get_i: returns the i value
         */
        pub fn get_i(&self) -> f32 {
            self.i
        }
        /*
         *  @name get_d: returns the d value
         */
        pub fn get_d(&self) -> f32 {
            self.d
        }
        /*
         *  @name get_izone: returns the izone value
         */
        pub fn get_izone(&self) -> f32 {
            self.izone
        }
        /*
         *  @name get_error: returns the error margin value
         */
        pub fn get_error(&self) -> f32 {
            self.error
        }
        /*
         *  @name get_error: returns the error margin value
         */
        pub fn get_path(&self) -> &String {
            &self.path
        }

        /*
         *  @name from: the function to create a new pid controller instance
         *  @param p: the p value to be assigned
         *  @param i: the i value to be assigned
         *  @param d: the d value to be assigned
         *  @param izone: the izone value to be assigned
         *  @param error: the error margin to be assigned
         */
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

        pub fn from() -> Self {
            StateInfo { is_active: false }
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
