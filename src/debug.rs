use bevy::prelude::*;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    //logging
    query.iter().for_each(|(entity, position)| {
        info!("Entity {:?} is at Position {:?},", entity, position);
    })
}
