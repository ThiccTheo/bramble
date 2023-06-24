use {bevy::prelude::*, maplit::hashmap, regex::Regex, std::collections::HashMap};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {}
}

// #[derive(PartialEq, Eq, Hash)]
// pub struct CraftingRecipe {
//     pattern: Regex,
// }

// impl CraftingRecipe {
//     pub fn new(pattern: &str) -> Self {
//         Self {
//             pattern: Regex::new(pattern).unwrap(),
//         }
//     }
// }

// #[derive(Resource)]
// pub struct CraftingTable {
//     pub recipes: HashMap<CraftingRecipe, &'static str>,
// }

// impl Default for CraftingTable {
//     fn default() -> Self {
//         Self {
//             recipes: hashmap! {
//                 CraftingRecipe::new("Test") => "Fuck",
//             },
//         }
//     }
// }
