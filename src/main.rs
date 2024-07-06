#![feature(iter_array_chunks)]

use bevy::prelude::*;

use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

mod frame;
mod debug;
mod ecoli;
mod asset_loader;
mod camera;
use ecoli::EColiPlugin;
use camera::CameraPlugin;
use asset_loader::AssetLoaderPlugin;
use frame::FramePlugin;
use debug::DebugPlugin;


pub const CENTER: Vec3 = Vec3::ZERO;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        // .insert_resource(AmbientLight {
        //     color: Color::default(),
        //     brightness: 300.,
        // })
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(DefaultPlugins)
        // .add_plugins(PanOrbitCameraPlugin)
        //custom
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        // .add_plugins(FramePlugin)
        .add_plugins(EColiPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
