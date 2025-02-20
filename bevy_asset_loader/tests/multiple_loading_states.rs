#![allow(dead_code, unused_imports)]

use bevy::app::AppExit;
use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

#[cfg(all(
    not(feature = "2d"),
    not(feature = "3d"),
    not(feature = "progress_tracking")
))]
#[test]
fn multiple_loading_states() {
    App::new()
        .add_state::<MyStates>()
        .add_plugins(MinimalPlugins)
        .add_plugin(AssetPlugin::default())
        .add_plugin(AudioPlugin::default())
        .add_loading_state(LoadingState::new(MyStates::Splash).continue_to_state(MyStates::Load))
        .add_collection_to_loading_state::<_, SplashAssets>(MyStates::Splash)
        .add_loading_state(LoadingState::new(MyStates::Load).continue_to_state(MyStates::Play))
        .add_collection_to_loading_state::<_, MyAssets>(MyStates::Load)
        .add_collection_to_loading_state::<_, MyOtherAssets>(MyStates::Load)
        .add_system(timeout)
        .add_system(use_splash_assets.in_schedule(OnEnter(MyStates::Load)))
        .add_system(use_loading_assets.in_schedule(OnEnter(MyStates::Play)))
        .add_system(quit.run_if(in_state(MyStates::Play)))
        .run();
}

fn timeout(time: Res<Time>) {
    if time.elapsed_seconds_f64() > 30. {
        panic!("The app did not finish in 30 seconds");
    }
}

fn use_splash_assets(_splash_assets: Res<SplashAssets>) {
    // I could do something with the splash assets here
}

fn use_loading_assets(_my_assets: Res<MyAssets>, _my_other_assets: Res<MyOtherAssets>) {
    // I could do something with the `MyAssets` and `MyOtherAssets` collections here
}

fn quit(mut exit: EventWriter<AppExit>) {
    info!("Everything fine, quitting the app");
    exit.send(AppExit);
}

#[derive(AssetCollection, Resource)]
#[allow(dead_code)]
struct MyAssets {
    #[asset(path = "audio/background.ogg")]
    background: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
#[allow(dead_code)]
struct MyOtherAssets {
    #[asset(path = "audio/yipee.ogg")]
    yipee: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
#[allow(dead_code)]
struct SplashAssets {
    #[asset(path = "audio/plop.ogg")]
    plop: Handle<AudioSource>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum MyStates {
    #[default]
    Splash,
    Load,
    Play,
}
