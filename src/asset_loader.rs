use bevy::prelude::*;

/// Resource to store 3D assets.
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub tank: Handle<Scene>,
    pub floor: Handle<Scene>,
}

/// Resource to store font assets.
#[derive(Resource, Debug, Default)]
pub struct FontAssets {
    pub menu_font: Handle<Font>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .init_resource::<FontAssets>()
            .add_systems(PreStartup, (
                load_3d_assets,
                load_font_assets,
            ));
    }
}

/// System to load 3D assets from the asset server and store them in the `SceneAssets` resource.
fn load_3d_assets(
    mut assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>
) {
    *assets = SceneAssets {
        tank: asset_server.load("models/tank2.glb#Scene0"),
        floor: asset_server.load("models/floor.glb#Scene0")
    }
}

/// System to load font assets from the asset server and store them in the `FontAssets` resource.
fn load_font_assets(
    mut assets: ResMut<FontAssets>,
    asset_server: Res<AssetServer>
) {
    *assets = FontAssets {
        menu_font: asset_server.load("fonts/LonelyNight.otf")
    }
}