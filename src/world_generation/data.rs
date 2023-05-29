use {bevy::prelude::*, noise::utils::NoiseMap};

#[derive(Component)]
pub struct BackgroundTilemap;

#[derive(Component)]
pub struct ForegroundTilemap;

#[derive(Resource)]
pub struct PerlinMap(pub NoiseMap);

#[derive(Resource)]
pub struct WorldSeed(pub u32);
