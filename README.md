# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the seventh entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Bonus: Camera2dBundle 3

Today is the seventh day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, I will be wrapping up our deep dive into `Camera2dBundle`.

#### Discussion

Reflecting on the last few days, I was wondering if there were any better way to introduce `Camera2dBundle`. We need

1. to introduce `Query`s and marker `Component`s, so we can mutate the camera
2. to talk about `Camera2dBundle` itself
3. to render something to look at with the camera (I used `Text2dBundle`)
4. to transform the camera, so we can see the effect of moving it

The way I organized this across these last three katas is to

- dig into the details of the `Camera2dBundle` without rendering anything
- render a `Text2dBundle`, introduce marker `Component`s, gloss over `Query`s and `Transform` the camera
- (today I will) talk more about `Query`s and `Transform`s and move the camera in other ways

In the future, I might do (1) in a single example, then (2) and (3) together, and save all the transforms (translation, rotation, etc.) for a single follow-up example.

---

Today's kata will build on yesterday's example. Here's the code we'll start with

```rust
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

    let text_alignment = TextAlignment::Center;
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
                .with_alignment(text_alignment),
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

The `Cargo.toml` is the same minimal one we've been using for all the examples so far

```toml
[package]
name = "daily_bevy"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
bevy = "0.12.1"
```

We've also got some `*.ttf` font files in the `assets/` directory.

---

Let's rewind a bit and talk about [entities](https://bevy-cheatbook.github.io/programming/ec.html) first.

In [an ECS (Entity-Component-System) architecture](https://en.wikipedia.org/wiki/Entity_component_system), _entities_ are just unique collections of components, with an ID attached to the collection. You might represent a specific enemy character in a video game with an ID, a health component, and a stamina component.

Suppose, in your video game, there is some environmental effect that reduces the health of any entity exposed to it -- maybe poison gas. It would be helpful if you could select all entities with a health component and continually reduce their hit points, so long as they are exposed to poison gas.

This is what `Query`s are used for. `Query`s let us select all entities with a particular kind of component.

```rust
// NOTE: extremely abridged

/// [System parameter] that provides selective access to the [`Component`] data stored in a [`World`].
///
/// Enables access to [entity identifiers] and [components] from a system, without the need to directly access the world.
/// Its iterators and getter methods return *query items*.
/// Each query item is a type containing data relative to an entity.
///
/// `Query` is a generic data structure that accepts two type parameters, both of which must implement the [`WorldQuery`] trait:
///
/// - **`Q` (query fetch).**
///   The type of data contained in the query item.
///   Only entities that match the requested data will generate an item.
/// - **`F` (query filter).**
///   A set of conditions that determines whether query items should be kept or discarded.
///   This type parameter is optional.
// -- snip --
pub struct Query<'world, 'state, Q: WorldQuery, F: ReadOnlyWorldQuery = ()> {
    // -- snip --
}
```

(While it is possible to directly create a Bevy `Entity`, this is ["rarely the right choice"](https://docs.rs/bevy/latest/bevy/prelude/struct.Entity.html#method.from_raw). "Most apps should favor `Commands::spawn`", as we have done above.)

In the last kata, we defined a `mouse_coordinates` system that took two different `Query`s as arguments

```rust
fn mouse_coordinates(
    window_query: Query<&Window>,
    mut text_query: Query<&mut Text, With<CursorPosition>>
) {
    // -- snip --
}
```

The `window_query` will return a read-only reference to any entities with a `Window` component in our app.

Where is our `Window` defined? In the `DefaultPlugins`

```rust
impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            // -- snip --
            .add(bevy_window::WindowPlugin::default())
        
        // -- snip --
    }
}
```

```rust
impl Default for WindowPlugin {
    fn default() -> Self {
        WindowPlugin {
            primary_window: Some(Window::default()),
            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
        }
    }
}
```

So this `Query` will return a read-only reference to this default `Window`.

The `text_query` will return a _mutable_ reference to any entity with a `Text` component which _also_ has a `CursorPosition` component. That's what `With` does

```rust
/// Filter that selects entities with a component `T`.
///
/// This can be used in a [`Query`](crate::system::Query) if entities are required to have the
/// component `T` but you don't actually care about components value.
///
/// This is the negation of [`Without`].
///
/// # Examples
///
/// ```
/// # use bevy_ecs::component::Component;
/// # use bevy_ecs::query::With;
/// # use bevy_ecs::system::IntoSystem;
/// # use bevy_ecs::system::Query;
/// #
/// # #[derive(Component)]
/// # struct IsBeautiful;
/// # #[derive(Component)]
/// # struct Name { name: &'static str };
/// #
/// fn compliment_entity_system(query: Query<&Name, With<IsBeautiful>>) {
///     for name in &query {
///         println!("{} is looking lovely today!", name.name);
///     }
/// }
/// # bevy_ecs::system::assert_is_system(compliment_entity_system);
/// ```
pub struct With<T>(PhantomData<T>);
```

(Note there is also a `Without` selector.)

As we only have one entity with a `CursorPosition` `Component`, just this one entity will be returned from this `Query`.

Note that `CursorPosition` doesn't _add_ anything to the entity in terms of data or functionality -- it has no fields and no `impl`ementation. It simply acts as a label which we can use to filter our `Query` of the world. This is what is meant by "marker component" -- all it does is attach a label to an entity.

The `text_query` is mutable because we want to change the `value` of the text, updating it with the latest cursor position.

---

In our other system, the `keyboard_input_system`, we also had a `Query`

```rust
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>
) {
```

`camera_query` is a `mut`able `Query` which returns all entities with a `Transform` component, filtered to only entities which also have a `MainCamera` component. `MainCamera` is another one of the marker components that we defined. The only new thing here is `Transform` -- let's talk about that.

[The `Transform` component](https://bevy-cheatbook.github.io/fundamentals/transforms.html#transform) is attached to any entity which has a position in the world.

> "All of Bevy's [built-in bundle types](https://bevy-cheatbook.github.io/builtins.html#bundles)" include a `Transform` component. [[source]](https://bevy-cheatbook.github.io/fundamentals/transforms.html#transform-components)

We can scale, rotate, or translate any entity with a `Transform` component

```rust
pub struct Transform {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    ///
    /// See the [`translations`] example for usage.
    ///
    /// [`translations`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/translation.rs
    pub translation: Vec3,
    /// Rotation of the entity.
    ///
    /// See the [`3d_rotation`] example for usage.
    ///
    /// [`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs
    pub rotation: Quat,
    /// Scale of the entity.
    ///
    /// See the [`scale`] example for usage.
    ///
    /// [`scale`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/scale.rs
    pub scale: Vec3,
}
```

So to bring this all together: `keyboard_input_system` is a system with arguments that include `camera_query`, which is a `Query` for all entities in the world with a `Transform` component, filtered to only those entities which also have a `MainCamera` component. The entities returned from this query can be translated, rotated, or scaled by `mut`ating their `Transform` components.

And that's what we did! We mutated the position and scale of the `MainCamera` `Camera2dBundle` in the previous kata, in response to keyboard input.

So let's add some easy things first: let's rotate the camera in response to keyboard input. Naively, you might try adding something like...

```rust
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>
) {
    // -- snip --

    if keyboard_input.pressed(KeyCode::SuperRight) {
        let mut camera = camera_query.single_mut();
        camera.rotation.z += 0.01;
    }

    if keyboard_input.pressed(KeyCode::SuperLeft) {
        let mut camera = camera_query.single_mut();
        camera.rotation.z -= 0.01;
    }
}
```

While this _does_ rotate the text, it will also make it smaller as it rotates -- why is that?

The short answer is that `rotation` is a `Quat`ernion, and quaternions behave in complex and possibly unintuitive ways.

The long answer is that quaternions in Bevy are normalized. Their [coefficients](https://en.wikipedia.org/wiki/Quaternion) are always scaled so that the square root of the sum of the squared coefficients is 1.0. So if you _increase the magnitude_ (whether in the positive or the negative direction) of the `z` component, you are necessarily decreasing the magnitudes of the `x`, `y`, and `w` components. In other words, you are "zooming out". Since `w` is decreasing as well, as we "zoom out", we also rotate less and less. With this implementation, we will never be able to rotate the text more than 180 degrees.

Luckily, Bevy provides convenience methods for rotating around an axis without accidentally causing this scaling

```rust
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>
) {
    // -- snip --

    if keyboard_input.pressed(KeyCode::SuperRight) {
        let mut camera = camera_query.single_mut();
        camera.rotate_z(0.1);
    }

    if keyboard_input.pressed(KeyCode::SuperLeft) {
        let mut camera = camera_query.single_mut();
        camera.rotate_z(-0.1);
    }
}
```

You can now rotate to your heart's content.

Rotating around other axes is also fun

```rust
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>
) {
    // -- snip --

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
```

---

Phew, that was a lot about cameras. Now that we know how to use them, we can explore many more [examples](https://github.com/bevyengine/bevy/tree/v0.12.1/examples). Let's get back to those tomorrow. 

The complete source code for this example can be found in `main.rs`.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).