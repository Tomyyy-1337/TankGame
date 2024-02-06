use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub tank: Handle<Scene>,
    pub floor: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(
    mut assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>
) {
    *assets = SceneAssets {
        tank: asset_server.load("tank2.glb#Scene0"),
        floor: asset_server.load("floor.glb#Scene0")
    }
}