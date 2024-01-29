// source: https://github.com/bevyengine/bevy/blob/main/examples/app/drag_and_drop.rs

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, file_drag_and_drop_system)
        .run();
}

fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>) {
    for event in events.read() {
        info!("{:?}", event);
    }
}