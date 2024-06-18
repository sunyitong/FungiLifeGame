use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::components::*;
use crate::init_data::*;
use rand::Rng;
use image::{GenericImageView, Pixel};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use ndarray::Array2;
use ndarray_stats::DeviationExt;
use std::collections:: VecDeque;


fn process_image_to_restriction(
    file_path: &str,
    channel_index: usize,
    target_value: u8,
    restriction: &mut Vec<Vec<i32>>,
    canvas_size: usize,
) {
    let img = image::open(file_path).expect("Failed to open image");

    let (width, height) = img.dimensions();
    assert!(width == canvas_size as u32 && height == canvas_size as u32, "Image dimensions do not match canvas size");

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let channels = pixel.channels();

            let new_y = canvas_size - 1 - y as usize;

            if channels[channel_index] == target_value {
                restriction[x as usize][new_y] = 1;
            }
        }
    }
}


fn fill_square(grid: &mut Vec<Vec<i32>>, start_x: usize, start_y: usize, x_length: usize, y_length: usize, fill: bool) {
    let canvas_size = grid.len(); 

    for x in start_x..(start_x + x_length) {
        if x >= canvas_size {
            break; 
        }
        for y in start_y..(start_y + y_length) {
            if y >= canvas_size {
                break;
            }
            if fill || x == start_x || x == start_x + x_length - 1 || y == start_y || y == start_y + y_length - 1 {
                grid[x][y] = 1;
            }
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut init_fungi_hashset= HashSet::new();
    init_fungi_hashset.insert((256, 256));

    let mut restriction = vec![vec![0; CANVAS_SIZE]; CANVAS_SIZE];
    // fill_square(&mut restriction, 200, 220, 250,150, false);
    process_image_to_restriction(
        RESTRICTION_IMAGE,
        1,
        255,
        &mut restriction,
        CANVAS_SIZE,
    );

    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz((CANVAS_SIZE/2) as f32, (CANVAS_SIZE/2) as f32, 0.0),
        ..default()
    });

    let pixel_image = Image::new_fill(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    let pixel_sprite_handle:Handle<Image> = asset_server.add(pixel_image);

    commands.insert_resource(PixelImageHandle(pixel_sprite_handle));
    commands.insert_resource(ClearColor(Color::rgb(BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2)));
    commands.insert_resource(FungiSpawnPositionList(init_fungi_hashset));
    commands.insert_resource(FungiExistPositionList(HashSet::new()));
    commands.insert_resource(GridFood(vec![vec![100; CANVAS_SIZE]; CANVAS_SIZE]));
    commands.insert_resource(GridRestriction(restriction));
}

pub fn init_restriction(
    mut commands: Commands,
    restriction: Res<GridRestriction>,
    fungi_sprite: Res<PixelImageHandle>
){
    for (x, row) in restriction.0.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            if *value != 0 {
                commands.spawn((
                    SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(RESTRICTION_COLOR.0, RESTRICTION_COLOR.1, RESTRICTION_COLOR.2, 1.0),
                        ..default()
                    },
                    texture: fungi_sprite.0.clone(),
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                    ..default()
                }, 
                Restriction));
            }
        }
    }
}

pub fn update_fungi(
    mut commands: Commands,
    fungi_sprite: Res<PixelImageHandle>,
    mut fungi_spawn_position_list: ResMut<FungiSpawnPositionList>,
    mut grid_food: ResMut<GridFood>,
    restriction: Res<GridRestriction>,
    mut fungi: Query<(&Transform, &mut IsAlive, &FoodConsumptionSpeed, &mut Sprite), With<FungiDefault>>,
){
    let mut rng = rand::thread_rng();
    for (transform, mut is_alive, food_consumption_speed, mut sprite) in fungi.iter_mut(){
        
        // is alive
        if is_alive.0 {
            let x = transform.translation.x as usize;
            let y = transform.translation.y as usize;

            // random death
            if rng.gen::<f32>() < 0.0004 {
                is_alive.0 = false;
                sprite.color = Color::rgba(DEAD_RANDOM_FUNGI_COLOR.0, DEAD_RANDOM_FUNGI_COLOR.1, DEAD_RANDOM_FUNGI_COLOR.2, 1.0);
                continue;
            }

            // spawn new fungi
            let dx = rng.gen_range(-1..=1);
            let dy = rng.gen_range(-1..=1);

            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if restriction.0[new_x as usize][new_y as usize] == 0 {
                fungi_spawn_position_list.0.insert((new_x, new_y));
            } else {
                commands.spawn((
                    SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(LIGHT_COLOR.0, LIGHT_COLOR.1, LIGHT_COLOR.2, 1.0),
                        ..default()
                    },
                    texture: fungi_sprite.0.clone(),
                    transform: Transform::from_xyz(new_x as f32, new_y as f32, 0.0),
                    ..default()
                }, Light {
                    light_type: LightDefault,
                    open_counting: OpenCounting(LIGHT_LIFE_TIME),
                    is_alive: IsAlive(true)
                }
                ));
            }

            // food consumption
            grid_food.0[x][y] -= food_consumption_speed.0;
            if grid_food.0[x][y] <= 0 {
                is_alive.0 = false;
                grid_food.0[x][y] = 0;
            }

            // display fungi status
            if is_alive.0 {
                if  70 < grid_food.0[x][y] && grid_food.0[x][y] <= 100 {
                    sprite.color = Color::rgba(ALIVE_FUNGI_COLOR_1.0 , ALIVE_FUNGI_COLOR_1.1, ALIVE_FUNGI_COLOR_1.2, 1.0);
                } else if  30 < grid_food.0[x][y] && grid_food.0[x][y] <= 70 {
                    sprite.color = Color::rgba(ALIVE_FUNGI_COLOR_2.0 , ALIVE_FUNGI_COLOR_2.1, ALIVE_FUNGI_COLOR_2.2, 1.0);
                } else {
                    sprite.color = Color::rgba(ALIVE_FUNGI_COLOR_3.0 , ALIVE_FUNGI_COLOR_3.1, ALIVE_FUNGI_COLOR_3.2, 1.0);
                }
            } else {
                sprite.color = Color::rgba(DEAD_FUNGI_COLOR.0, DEAD_FUNGI_COLOR.1, DEAD_FUNGI_COLOR.2, 1.0);
            }
        }
    }
}

pub fn spawn_fungi(
    mut commands: Commands,
    mut fungi_spawn_position_list: ResMut<FungiSpawnPositionList>,
    mut fungi_exist_position_list: ResMut<FungiExistPositionList>,
    fungi_sprite: Res<PixelImageHandle>
) {
    if !fungi_spawn_position_list.0.is_empty() {
        for pos in fungi_spawn_position_list.0.drain() {
            if !fungi_exist_position_list.0.contains(&pos) {
                commands.spawn((SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(NEWBORN_FUNGI_COLOR.0, NEWBORN_FUNGI_COLOR.1, NEWBORN_FUNGI_COLOR.2, 1.0),
                        ..default()
                    },
                    texture: fungi_sprite.0.clone(),
                    transform: Transform::from_xyz(pos.0 as f32, pos.1 as f32, 0.0),
                    ..default()
                },Fungi {
                    fungi_type: FungiDefault,
                    food_consumption_speed: FoodConsumptionSpeed(1),
                    is_alive: IsAlive(true),
                }));
                fungi_exist_position_list.0.insert(pos);
            }
        }
    }
}

pub fn update_light(
    mut light: Query<(&mut IsAlive, &mut OpenCounting, &mut Sprite), With<LightDefault>>,
){
    for (mut is_alive, mut open_counting, mut sprite) in light.iter_mut() {
        if is_alive.0 {
            open_counting.0 -= 1;
            if open_counting.0 <= 0 {
                is_alive.0 = false;
                sprite.color = Color::rgba(0.5, 0.0, 0.5, 1.0);
            }
        }
    }
}

pub fn sort_light_path(
    light: Query<(&IsAlive, &Transform), With<LightDefault>>,
){
    let mut active_light_list: Vec<[f32;2]> = Vec::new();

    for (is_alive, position) in light.iter(){
        if is_alive.0 {
            active_light_list.push([position.translation.x, position.translation.y]);
        }
    }
    // info!("active light list length is: {}", active_light_list.len());
    // info!("{:?}", active_light_list);
    let components = find_connected_components(&active_light_list, LIGHT_PATH_SORT_THRESHOLD);
    info!("{}", components.len());
}

fn find_connected_components(points: &Vec<[f32;2]>, threshold: f32) -> Vec<Vec<usize>> {
    let n = points.len();
    let mut distances = Array2::<f32>::zeros((n, n));
    for i in 0..n {
        for j in 0..n {
            distances[[i, j]] = ((points[i][0] - points[j][0]).powi(2) + (points[i][1] - points[j][1]).powi(2)).sqrt();
        }
    }

    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if distances[[i, j]] < threshold {
                graph[i].push(j);
            }
        }
    }

    let mut visited = vec![false; n];
    let mut components = Vec::new();
    for i in 0..n {
        if !visited[i] {
            let component = bfs(i, &graph, &mut visited);
            components.push(component);
        }
    }
    components
}

fn bfs(start: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> Vec<usize> {
    let mut queue = VecDeque::new();
    let mut component = Vec::new();
    queue.push_back(start);
    visited[start] = true;
    while let Some(node) = queue.pop_front() {
        component.push(node);
        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    component
}