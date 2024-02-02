# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the sixth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Bonus: Camera2dBundle 2

Today is the sixth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, I will be exploring `Camera2dBundle` a bit more.

#### Discussion

Yesterday, we had an introduction to `Camera2dBundle` and read about all the things we can do with it. Today, let's do some of those things.

Let's start by rendering some stuff. Let's put some text on the screen (we can dig into the details of all of this in later katas).

Here is a minimal example which shows how to render text in a window. Note that we need the `*.ttf` file for the font we're using

```rust
// adapted from the 2d/text2d.rs example here: https://github.com/bevyengine/bevy/blob/v0.12.1/examples/2d/text2d.rs

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let text_alignment = TextAlignment::Center;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::BLACK,
    };

    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        Text2dBundle {
            text: Text::from_section("Hello, Bevy!", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        }
    );

}
```

The `Cargo.toml` only has the basic `bevy` dependency

```toml
[dependencies]
bevy = "0.12.1"
```

Instead of just displaying static text, let's display the position of our cursor. [This article](https://taintedcoders.com/bevy/cameras/) explains how to do this

Let's start simple by displaying this in the terminal. We need to add an `Update` schedule to our `App`...

```rust
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_coordinates) // <- this is new
        .run();
```

...and we need to define this new system, which will query for information about the window (this will be explained in later katas), from which we can get the cursor position

```rust
fn mouse_coordinates(window_query: Query<&Window>) {
    let window = window_query.single();
    if let Some(world_position) = window.cursor_position() {
        info!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
```

Running the example with the above two changes will print the cursor position as quickly as possible, over and over

```
...
2024-02-02T02:12:06.634735Z  INFO daily_bevy: World coords: 13.707031/1.7265625
2024-02-02T02:12:06.642307Z  INFO daily_bevy: World coords: 14.644531/1.0976563
2024-02-02T02:12:06.650489Z  INFO daily_bevy: World coords: 15.269531/0.15625
...
```

Now, let's get this rendering as text in the window, rather than as text in the terminal. (Based on [this example](https://taintedcoders.com/bevy/text/).)

First, add [a marker `Component`](https://bevy-cheatbook.github.io/programming/ec.html#marker-components), in the same scope as the `main` method

```rust
#[derive(Component)]
struct CursorPosition;
```

Next, add the marker component to the `Text2dBundle`

```rust
    commands.spawn(( // <- added ( here
        Text2dBundle {
            text: Text::from_section("Hello, Bevy!", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        CursorPosition // <- this has been added
    )); // <- added ) here
```

Finally, change the `mouse_coordinates` system to the following

```rust
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
```

This will update the text in the window to reflect the current position of the cursor.

Let's now add some event handling to mutate the camera. Let's use the arrow keys to translate the camera left, right, up, and down.

To do this, we need another system to listen to keyboard input events. Let's copy and paste our keyboard input system from the [`keyboard_input` kata](https://github.com/awwsmm/daily-bevy/blob/input/keyboard_input/README.md)...

```rust
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}
```

...but tweak it a bit. For now, we'll just print to the terminal

```rust
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Left) {
        info!("camera should translate left");
    }

    if keyboard_input.pressed(KeyCode::Up) {
        info!("camera should translate up");
    }

    if keyboard_input.pressed(KeyCode::Right) {
        info!("camera should translate right");
    }

    if keyboard_input.pressed(KeyCode::Down) {
        info!("camera should translate down");
    }
}
```

Don't forget to add this system to our `App`

```rust
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (mouse_coordinates, keyboard_input_system)) // <- this line has changed
        .run();
```

Running this, we should see text appear in the terminal when we press the arrow keys

```
...
2024-02-02T02:31:54.345678Z  INFO daily_bevy: camera should translate right
2024-02-02T02:31:54.353978Z  INFO daily_bevy: camera should translate right
2024-02-02T02:31:55.345615Z  INFO daily_bevy: camera should translate down
2024-02-02T02:31:55.353753Z  INFO daily_bevy: camera should translate down
2024-02-02T02:31:55.362352Z  INFO daily_bevy: camera should translate down
2024-02-02T02:31:56.612476Z  INFO daily_bevy: camera should translate left
2024-02-02T02:31:56.620860Z  INFO daily_bevy: camera should translate left
...
```

Let's now use this information to translate the camera.

First, we need to add another marker component

```rust
#[derive(Component)]
struct MainCamera;
```

We tag the `Camera2dBundle` with this marker component

```rust
    commands.spawn(( // <- added ( here
        Camera2dBundle::default(),
        MainCamera // <- this has been added
    )); // <- added ) here
```

Finally, we add a `Query` for the `Transform` `Component` of the `Camera2dBundle`, which we've tagged with the `MainCamera` marker component.

```rust
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>
) {
    if keyboard_input.pressed(KeyCode::Left) {
        let mut camera = camera_query.single_mut();
        camera.translation.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        let mut camera = camera_query.single_mut();
        camera.translation.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        let mut camera = camera_query.single_mut();
        camera.translation.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        let mut camera = camera_query.single_mut();
        camera.translation.y -= 1.0;
    }
}
```

Running this example, you should see that pressing the left arrow moves the camera to the left (the text moves to the right), and so on.

One more small thing we can add to this example is zooming the camera in and out. Let's make right shift move the camera in and left shift move the camera out

```rust
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>
) {
    // -- snip --

    if keyboard_input.pressed(KeyCode::ShiftRight) {
        let mut camera = camera_query.single_mut();
        camera.translation.z -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        let mut camera = camera_query.single_mut();
        camera.translation.z += 1.0;
    }
}
```

Hmmm... this one doesn't seem to work. Do you know why?

Remember that, by default, Bevy uses an orthogonal projection for 2D cameras. This means there is no perspective. Closer objects are not "bigger" and farther objects are not "smaller". They are just [layered according to z-indexing](https://en.wikipedia.org/wiki/Z-order).

We can simulate this effect, though, by scaling instead of translating the camera

```rust
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>
) {
    // -- snip --

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
}
```

Ta da! Camera translation in "three" dimensions!

Rotating the camera is just as easy -- I'll leave that as an exercise for the reader.

This is pretty fun, eh? Let's continue playing around with cameras for the next few days.

The complete source code for this example can be found in `main.rs`.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).