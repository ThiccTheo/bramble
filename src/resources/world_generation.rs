use {bevy::prelude::*, noise::utils::NoiseMap};

#[derive(Resource)]
pub struct PerlinMap(pub NoiseMap);
