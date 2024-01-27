// source: https://github.com/bevyengine/bevy/blob/main/examples/hello_world.rs

use bevy::prelude::*;

fn main() {
    App::new().add_systems(Update, hello_world_system).run();
}

fn hello_world_system() {
    println!("hello world");
}
