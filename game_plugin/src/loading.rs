use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/kenney-future.ttf")]
    pub kenney_future: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/coin3.ogg")]
    pub coin3: Handle<AudioSource>,
    #[asset(path = "audio/engine4.ogg")]
    pub engine4: Handle<AudioSource>,
    #[asset(path = "audio/explosion1.ogg")]
    pub explosion1: Handle<AudioSource>,
    #[asset(path = "audio/gameover2.ogg")]
    pub gameover2: Handle<AudioSource>,
    #[asset(path = "audio/hit1.ogg")]
    pub hit1: Handle<AudioSource>,
    #[asset(path = "audio/hit2.ogg")]
    pub hit2: Handle<AudioSource>,
    #[asset(path = "audio/music-comedy.ogg")]
    pub music_comedy: Handle<AudioSource>,
    #[asset(path = "audio/upgrade4.ogg")]
    pub upgrade4: Handle<AudioSource>,
    #[asset(path = "audio/upgrade5.ogg")]
    pub upgrade5: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 8.0, tile_size_y = 8.0, columns = 12, rows = 10, padding_x = 0.0, padding_y = 0.0))]
    #[asset(path = "textures/tiles_packed.png")]
    tiles: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 2, padding_x = 0.0, padding_y = 0.0))]
    #[asset(path = "textures/ships_packed.png")]
    ships: Handle<TextureAtlas>,
}
