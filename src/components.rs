use bevy::prelude::*;
use bevy::utils::HashSet;

/// bundle
#[derive(Bundle)]
pub struct Fungi {
    pub fungi_type: FungiDefault,
    pub food_consumption_speed: FoodConsumptionSpeed,
    pub is_alive: IsAlive,
}

/// components
#[derive(Component)]
pub struct FungiDefault;

#[derive(Component)]
pub struct IsAlive (pub bool);

#[derive(Component)]
pub struct FoodConsumptionSpeed (pub i32);

#[derive(Component)]
pub struct Restriction;


/// resources
#[derive(Resource)]
pub  struct GridFood(pub Vec<Vec<i32>>);

#[derive(Resource)]
pub struct GridRestriction(pub Vec<Vec<i32>>);

#[derive(Resource)]
pub struct GridIdealShape(pub Vec<Vec<i32>>);

#[derive(Resource)]
pub struct GridBoundary(pub Vec<Vec<i32>>);

#[derive(Resource)]
pub struct FungiSpawnPositionList(pub HashSet<(i32, i32)>);

#[derive(Resource)]
pub struct FungiExistPositionList(pub HashSet<(i32, i32)>);

#[derive(Resource)]
pub struct FungiTextureHandle(pub Handle<Image>);