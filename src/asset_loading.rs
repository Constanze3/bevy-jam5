use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Spawning)
                .load_collection::<GltfAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource, Clone)]
pub struct GltfAssets {
    #[asset(path = "town.glb")]
    pub world: Handle<Gltf>,
}
