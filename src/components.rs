use bevy::prelude::*;
use bevy::utils::HashSet;

/// bundle
#[derive(Bundle)]
pub struct Fungi {
    pub fungi_type: FungiDefault,
    pub food_consumption_speed: FoodConsumptionSpeed,
    pub is_alive: IsAlive,
}

/// bundle
#[derive(Bundle)]
pub struct FungiExperimental {
    pub fungi_type: FungiExperimentalType,
    pub food_consumption_speed: FoodConsumptionSpeed,
    pub is_alive: IsAlive,
    pub growth_direction: GrowthDirection,
    pub growth_curve: GrowthCurve,
    pub split_probability: SplitProbability,
}

#[derive(Bundle)]
pub struct Light {
    pub light_type: LightDefault,
    pub open_counting: OpenCounting,
    pub is_alive: IsAlive,
}

/// components
#[derive(Component)]
pub struct FungiDefault;

#[derive(Component)]
pub struct FungiExperimentalType;

#[derive(Component)]
pub struct IsAlive (pub bool);

#[derive(Component)]
pub struct FoodConsumptionSpeed (pub f32);

#[derive(Component)]
pub struct Restriction;

#[derive(Component)]
pub struct LightDefault;

#[derive(Component)]
pub struct OpenCounting(pub u32);

#[derive(Component)]
pub struct GrowthDirection(pub f32);

#[derive(Component)]
pub struct GrowthCurve(pub f32);

#[derive(Component)]
pub struct SplitProbability(pub f32);


/// resources
#[derive(Resource)]
pub  struct GridFood(pub Vec<Vec<f32>>);

#[derive(Resource)]
pub struct GridRestriction(pub Vec<Vec<i32>>);

#[derive(Resource)]
pub struct FungiSpawnPositionList(pub HashSet<(i32, i32)>);

#[derive(Resource)]
pub struct FungiExistPositionList(pub HashSet<(i32, i32)>);

#[derive(Resource)]
pub struct PixelImageHandle(pub Handle<Image>);