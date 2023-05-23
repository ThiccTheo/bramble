use {bevy::prelude::*, noise::utils::NoiseMap};

#[derive(Resource)]
pub struct PerlinMap(pub NoiseMap);

#[derive(Resource)]
pub struct WorldSeed(pub u32);
