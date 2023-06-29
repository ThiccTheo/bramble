use {super::item::ItemType, bevy::prelude::*, regex::Regex};

pub(super) struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {}
}

pub type CraftingRecipe = Regex;

#[derive(Resource)]
pub struct CraftingRecipes {
    pub recipes: Vec<(CraftingRecipe, (ItemType, usize))>,
}

impl Default for CraftingRecipes {
    fn default() -> Self {
        let recipes = vec![(
            CraftingRecipe::new(format!("(Empty)*({:?})(Empty)*", ItemType::WoodLog).as_str())
                .unwrap(),
            (ItemType::WoodPlank, 4),
        )];

        Self { recipes }
    }
}
