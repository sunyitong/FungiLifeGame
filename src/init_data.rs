
pub const CANVAS_SIZE: usize = 512;
pub const FUNGI_IMAGE_PATH: &str = "images/fungi_sprite.png";
pub const INIT_SETUP_BMP_PATH: &str = "C:/Users/sunyi/RustroverProjects/life_game/assets/images/init_setup.bmp";
pub const RESTRICTION_IMAGE: &str = "C:/Users/sunyi/RustroverProjects/life_game/assets/images/shape_test_1.png";
pub const INIT_BMP_FUNGI_COLOR_VALUE: i32 = 100;


// color palette (RGB 0-256 -> RGB 0.0-1.0)
pub const NEWBORN_FUNGI_COLOR: (f32,f32,f32) = (1.0, 1.0, 1.0);

pub const ALIVE_FUNGI_COLOR_1: (f32,f32,f32) = (1.0, 0.0, 0.0); // status 100%
pub const ALIVE_FUNGI_COLOR_2: (f32,f32,f32) = (0.5, 0.0, 0.0); // status 70%
pub const ALIVE_FUNGI_COLOR_3: (f32,f32,f32) = (0.2, 0.0, 0.0); // status 30%

pub const DEAD_FUNGI_COLOR: (f32,f32,f32) = (0.0, 0.0, 0.3);
pub const DEAD_RANDOM_FUNGI_COLOR: (f32,f32,f32) = (0.0, 0.2, 0.5);

pub const RESTRICTION_COLOR: (f32,f32,f32) = (0.0, 1.0, 0.0);

pub const BACKGROUND_COLOR: (f32,f32,f32) = (0.0, 0.0, 0.0);