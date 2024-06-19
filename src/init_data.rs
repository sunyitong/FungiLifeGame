
pub const CANVAS_SIZE: usize = 512;
pub const RESTRICTION_IMAGE: &str = "./assets/images/shape_test_1.png";


// light configuration
pub const LIGHT_PATH_SORT_THRESHOLD: f32 = 10.0;
pub const LIGHT_LIFE_TIME: u32 = 20;


// color palette (RGB 0-256 -> RGB 0.0-1.0)
pub const NEWBORN_FUNGI_COLOR: (f32,f32,f32) = (1.0, 1.0, 1.0);

pub const ALIVE_FUNGI_COLOR_1: (f32,f32,f32) = (1.0, 0.0, 0.0); // status 100%
pub const ALIVE_FUNGI_COLOR_2: (f32,f32,f32) = (0.5, 0.0, 0.0); // status 70%
pub const ALIVE_FUNGI_COLOR_3: (f32,f32,f32) = (0.2, 0.0, 0.0); // status 30%

pub const DEAD_FUNGI_COLOR: (f32,f32,f32) = (0.0, 0.0, 0.3);
pub const DEAD_RANDOM_FUNGI_COLOR: (f32,f32,f32) = (0.0, 0.2, 0.5);

pub const RESTRICTION_COLOR: (f32,f32,f32) = (0.1, 0.1, 0.1);
pub const LIGHT_COLOR: (f32,f32,f32) = (1.0, 1.0, 0.0);

pub const BACKGROUND_COLOR: (f32,f32,f32) = (0.0, 0.0, 0.0);


// light bloom configuration
pub const LIGHT_INTENSITY: f32 = 0.6;
pub const LOW_FREQUENCY_BOOST: f32 = 0.7;
pub const LOW_FREQUENCY_BOOST_CURVATURE: f32 = 0.95;
pub const HIGH_PASS_FREQUENCY: f32 = 1.0;
pub const THRESHOLD: f32 = 0.8;
pub const THRESHOLD_SOFTNESS: f32 = 0.0;