use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::components::*;
use crate::init_data::*;
use image::open;
use std::path::Path;
use rand::Rng;

fn bmp_to_rgb_arrays<P: AsRef<Path>>(path: P) -> Result<(Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>), String> {
    let img = open(path).map_err(|e| e.to_string())?;
    let rgb_img = img.to_rgb8();

    let canvas_size = CANVAS_SIZE as u32;
    if rgb_img.width() < canvas_size || rgb_img.height() < canvas_size {
        return Err("Image dimensions are too small".into());
    }

    let mut red_channel = vec![vec![0; CANVAS_SIZE]; CANVAS_SIZE];
    let mut green_channel = vec![vec![0; CANVAS_SIZE]; CANVAS_SIZE];
    let mut blue_channel = vec![vec![0; CANVAS_SIZE]; CANVAS_SIZE];

    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        if x >= canvas_size || y >= canvas_size {
            break;
        }
        let [r, g, b] = pixel.0;
        red_channel[y as usize][x as usize] = r as i32;
        green_channel[y as usize][x as usize] = g as i32;
        blue_channel[y as usize][x as usize] = b as i32;
    }

    Ok((red_channel, green_channel, blue_channel))
}

fn find_specific_value_positions(channel: &[Vec<i32>], value: i32) -> HashSet<(u32, u32)> {
    let mut positions = HashSet::new();
    for (y, row) in channel.iter().enumerate() {
        for (x, &color_value) in row.iter().enumerate() {
            if color_value == value {
                positions.insert((x as u32, y as u32));
            }
        }
    }
    positions
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // let (red_channel, green_channel, blue_channel) = bmp_to_rgb_arrays(Path::new(INIT_SETUP_BMP_PATH)).expect("Failed to process image");
    // let init_fungi_hashset = find_specific_value_positions(&red_channel, INIT_BMP_FUNGI_COLOR_VALUE);

    let mut init_fungi_hashset= HashSet::new();
    init_fungi_hashset.insert((0,0));
    // init_fungi_hashset.insert((100,100));

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    let sprite_handle:Handle<Image> = asset_server.load(FUNGI_IMAGE_PATH);
    commands.insert_resource(FungiTextureHandle(sprite_handle.clone()));
    commands.insert_resource(FungiSpawnPositionList(init_fungi_hashset));
    commands.insert_resource(FungiExistPositionList(HashSet::new()));
    commands.insert_resource(GridFood(vec![vec![100; CANVAS_SIZE]; CANVAS_SIZE]));
    commands.insert_resource(GridRestriction(vec![vec![100; CANVAS_SIZE]; CANVAS_SIZE]));
    // commands.insert_resource(GridBoundary(green_channel));
    // commands.insert_resource(GridIdealShape(blue_channel));
}

pub fn update_fungi(
    mut fungi_spawn_position_list: ResMut<FungiSpawnPositionList>,
    fungi: Query<&Transform, With<FungiDefault>>,
){
    for transform in fungi.iter(){
        if transform.translation.x < 50.0 && transform.translation.y < 30.0 {
            let mut x = transform.translation.x as i32;
            let mut y = transform.translation.y as i32;

            let mut rng = rand::thread_rng();
            let dx = rng.gen_range(-1..=1);
            let dy = rng.gen_range(-1..=1);

            x += dx;
            y += dy;

            fungi_spawn_position_list.0.insert((x,y));
        }
    }

}

pub fn spawn_fungi(
    mut commands: Commands,
    mut fungi_spawn_position_list: ResMut<FungiSpawnPositionList>,
    mut fungi_exist_position_list: ResMut<FungiExistPositionList>,
    fungi_sprite: Res<FungiTextureHandle>
) {
    if !fungi_spawn_position_list.0.is_empty() {
        for pos in fungi_spawn_position_list.0.drain() {
            if !fungi_exist_position_list.0.contains(&pos) {
                commands.spawn((SpriteBundle {
                    texture: fungi_sprite.0.clone(),
                    transform: Transform::from_xyz(pos.0 as f32, pos.1 as f32, 0.0),
                    ..default()
                },Fungi {
                    fungi_type: FungiDefault,
                    is_alive: IsAlive(true),
                }));
                fungi_exist_position_list.0.insert(pos);
            }
        }
    }
}