use bevy::prelude::*;
use bevy::utils::HashSet;
use crate::components::*;
use crate::init_data::*;
use rand::Rng;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
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
    // commands.insert_resource(GridRestriction(vec![vec![100; CANVAS_SIZE]; CANVAS_SIZE]));
    // commands.insert_resource(GridBoundary(green_channel));
    // commands.insert_resource(GridIdealShape(blue_channel));
}

pub fn update_fungi(
    mut fungi_spawn_position_list: ResMut<FungiSpawnPositionList>,
    fungi_exist_position_list: Res<FungiExistPositionList>,
    mut fungi: Query<(&Transform, &mut IsAlive, &mut Sprite), With<FungiDefault>>,
){
    for (transform, mut is_alive, mut sprite) in fungi.iter_mut(){
        let mut x = transform.translation.x as i32;
        let mut y = transform.translation.y as i32;

        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..=1);
        let dy = rng.gen_range(-1..=1);

        x += dx;
        y += dy;

        fungi_spawn_position_list.0.insert((x,y));

        sprite.color = Color::rgba(rng.gen(), rng.gen(), rng.gen(), 1.0);
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
                    sprite: Sprite {
                        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
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