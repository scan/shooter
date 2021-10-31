use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(GameState::Loading, GameState::Playing)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}

#[derive(Debug, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/kenney-future.ttf")]
    pub kenney_future: Handle<Font>,
}

#[derive(Debug, AssetCollection)]
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

#[derive(Debug, AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 8.0, tile_size_y = 8.0, columns = 12, rows = 10, padding_x = 0.0, padding_y = 0.0))]
    #[asset(path = "textures/tiles_packed.png")]
    pub tiles: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 2, padding_x = 0.0, padding_y = 0.0))]
    #[asset(path = "textures/ships_packed.png")]
    pub ships: Handle<TextureAtlas>,
}
