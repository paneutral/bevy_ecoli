use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SAssets {
    pub ecoli: Handle<Scene>,
    // pub sphere: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SAssets {
        ecoli: asset_server.load("ecoli_low.glb#Scene0"),
        // sphere: asset_server.load("Spaceship.glb#Scene0"),
    }
}