pub const WIDTH: u32 = 350;
pub const HEIGHT: u32 = 200;
pub const SAMPLES: u32 = 200;

pub const LOOK_FROM: (f64, f64, f64) = (13.0, 2.0, 3.0);
pub const LOOK_TO: (f64, f64, f64) = (0.0, 0.0, 0.0);
pub const VERTICAL: (f64, f64, f64) = (0.0, 1.0, 0.0);
pub const FOV: f64 = 20.0;
pub const APERTURE: f64 = 0.1;
pub const FOCUS_DISTANCE: f64 = 10.0;
pub const T0: f64 = 0.0;
pub const T1: f64 = 1.0;
pub const OUTPUT_FILENAME: &str = "output.png";

pub const ENABLE_LIGHTS: bool = true;

// I got the defaults from https://github.com/rudolphalmeida/raytrac so I can compare the
// performance of mine. I also changed the random scene so it matches.
