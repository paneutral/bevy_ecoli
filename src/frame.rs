use bevy::prelude::*;
use crate::asset_loader::SAssets;

pub struct FramePlugin;

impl Plugin for FramePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_frames);
    }
}

#[derive(Component, Debug)]
pub struct SphereFrame;



fn add_frames(mut commands: Commands, assets: Res<SAssets>) {
    commands.spawn((
        
    ));
}