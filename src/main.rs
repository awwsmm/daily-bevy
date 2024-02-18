// adapted from the 2d/text2d.rs example here: https://github.com/bevyengine/bevy/blob/v0.12.1/examples/2d/text2d.rs

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (mouse_coordinates, keyboard_input_system))
        .run();
}

#[derive(Component)]
struct CursorPosition;

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let text_alignment = JustifyText::Center;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::BLACK,
    };

    commands.spawn((
        Camera2dBundle::default(),
        MainCamera
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Hello, Bevy!", text_style.clone())
                .with_justify(text_alignment),
            ..default()
        },
        CursorPosition
    ));

}

fn mouse_coordinates(
    window_query: Query<&Window>,
    mut text_query: Query<&mut Text, With<CursorPosition>>
) {
    let window = window_query.single();

    if let Some(world_position) = window.cursor_position() {
        let mut text = text_query.single_mut();
        if let Some(text) = text.sections.iter_mut().next() {
            text.value = format!("World coords: {}/{}", world_position.x, world_position.y);
        }
    }

}

fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        let mut camera = camera_query.single_mut();
        camera.translation.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        let mut camera = camera_query.single_mut();
        camera.translation.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        let mut camera = camera_query.single_mut();
        camera.translation.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        let mut camera = camera_query.single_mut();
        camera.translation.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ShiftRight) {
        let mut camera = camera_query.single_mut();
        camera.scale.x *= 0.999;
        camera.scale.y *= 0.999;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        let mut camera = camera_query.single_mut();
        camera.scale.x *= 1.001;
        camera.scale.y *= 1.001;
    }

    if keyboard_input.pressed(KeyCode::SuperRight) {
        let mut camera = camera_query.single_mut();
        camera.rotate_z(0.1);
    }

    if keyboard_input.pressed(KeyCode::SuperLeft) {
        let mut camera = camera_query.single_mut();
        camera.rotate_z(-0.1);
    }

    if keyboard_input.pressed(KeyCode::AltRight) {
        let mut camera = camera_query.single_mut();
        camera.rotate_x(0.1);
    }

    if keyboard_input.pressed(KeyCode::AltLeft) {
        let mut camera = camera_query.single_mut();
        camera.rotate_x(-0.1);
    }

    if keyboard_input.pressed(KeyCode::BracketRight) {
        let mut camera = camera_query.single_mut();
        camera.rotate_y(0.1);
    }

    if keyboard_input.pressed(KeyCode::BracketLeft) {
        let mut camera = camera_query.single_mut();
        camera.rotate_y(-0.1);
    }
}