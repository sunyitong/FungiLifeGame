use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::rngs::ThreadRng;
use crate::init_data::*;

/// bundle
#[derive(Bundle)]
pub struct Fungi {
    pub fungi_type: FungiDefault,
    pub is_alive: IsAlive,
}

/// components
#[derive(Component)]
pub struct FungiDefault;

#[derive(Component)]
pub struct IsAlive (pub bool);

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