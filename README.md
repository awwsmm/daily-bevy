# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows entry #22 in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Low-Power Windows

Today is day #22 of Daily Bevy.

This kata uses [Bevy `v0.13.0`](https://github.com/bevyengine/bevy/tree/v0.13.0).

### Today's Kata

Today, we're looking at [the `low_power` example](https://github.com/bevyengine/bevy/blob/v0.13.0/examples/window/low_power.rs) from the Bevy repo.

#### The Code

Here's the `main.rs` for this example

```rust
//! This example illustrates how to run a winit window in a reactive, low power mode.
//!
//! This is useful for making desktop applications, or any other program that doesn't need to be
//! running the event loop non-stop.

use bevy::{
    prelude::*,
    utils::Duration,
    window::{PresentMode, RequestRedraw, WindowPlugin},
    winit::WinitSettings,
};

fn main() {
    App::new()
        // Continuous rendering for games - bevy's default.
        .insert_resource(WinitSettings::game())
        // Power-saving reactive rendering for applications.
        .insert_resource(WinitSettings::desktop_app())
        // You can also customize update behavior with the fields of [`WinitConfig`]
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
                wait: Duration::from_millis(10),
            },
        })
        .insert_resource(ExampleMode::Game)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Turn off vsync to maximize CPU/GPU usage
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, test_setup::setup)
        .add_systems(
            Update,
            (
                test_setup::cycle_modes,
                test_setup::rotate_cube,
                test_setup::update_text,
                update_winit,
            ),
        )
        .run();
}

#[derive(Resource, Debug)]
enum ExampleMode {
    Game,
    Application,
    ApplicationWithRedraw,
}

/// Update winit based on the current `ExampleMode`
fn update_winit(
    mode: Res<ExampleMode>,
    mut event: EventWriter<RequestRedraw>,
    mut winit_config: ResMut<WinitSettings>,
) {
    use ExampleMode::*;
    *winit_config = match *mode {
        Game => {
            // In the default `WinitConfig::game()` mode:
            //   * When focused: the event loop runs as fast as possible
            //   * When not focused: the event loop runs as fast as possible
            WinitSettings::game()
        }
        Application => {
            // While in `WinitConfig::desktop_app()` mode:
            //   * When focused: the app will update any time a winit event (e.g. the window is
            //     moved/resized, the mouse moves, a button is pressed, etc.), a [`RequestRedraw`]
            //     event is received, or after 5 seconds if the app has not updated.
            //   * When not focused: the app will update when the window is directly interacted with
            //     (e.g. the mouse hovers over a visible part of the out of focus window), a
            //     [`RequestRedraw`] event is received, or one minute has passed without the app
            //     updating.
            WinitSettings::desktop_app()
        }
        ApplicationWithRedraw => {
            // Sending a `RequestRedraw` event is useful when you want the app to update the next
            // frame regardless of any user input. For example, your application might use
            // `WinitConfig::desktop_app()` to reduce power use, but UI animations need to play even
            // when there are no inputs, so you send redraw requests while the animation is playing.
            event.send(RequestRedraw);
            WinitSettings::desktop_app()
        }
    };
}

/// Everything in this module is for setting up and animating the scene, and is not important to the
/// demonstrated features.
pub(crate) mod test_setup {
    use crate::ExampleMode;
    use bevy::{prelude::*, window::RequestRedraw};

    /// Switch between update modes when the mouse is clicked.
    pub(crate) fn cycle_modes(
        mut mode: ResMut<ExampleMode>,
        mouse_button_input: Res<ButtonInput<KeyCode>>,
    ) {
        if mouse_button_input.just_pressed(KeyCode::Space) {
            *mode = match *mode {
                ExampleMode::Game => ExampleMode::Application,
                ExampleMode::Application => ExampleMode::ApplicationWithRedraw,
                ExampleMode::ApplicationWithRedraw => ExampleMode::Game,
            };
        }
    }

    #[derive(Component)]
    pub(crate) struct Rotator;

    /// Rotate the cube to make it clear when the app is updating
    pub(crate) fn rotate_cube(
        time: Res<Time>,
        mut cube_transform: Query<&mut Transform, With<Rotator>>,
    ) {
        for mut transform in &mut cube_transform {
            transform.rotate_x(time.delta_seconds());
            transform.rotate_local_y(time.delta_seconds());
        }
    }

    #[derive(Component)]
    pub struct ModeText;

    pub(crate) fn update_text(
        mut frame: Local<usize>,
        mode: Res<ExampleMode>,
        mut query: Query<&mut Text, With<ModeText>>,
    ) {
        *frame += 1;
        let mode = match *mode {
            ExampleMode::Game => "game(), continuous, default",
            ExampleMode::Application => "desktop_app(), reactive",
            ExampleMode::ApplicationWithRedraw => "desktop_app(), reactive, RequestRedraw sent",
        };
        let mut text = query.single_mut();
        text.sections[1].value = mode.to_string();
        text.sections[3].value = frame.to_string();
    }

    /// Set up a scene with a cube and some text
    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut event: EventWriter<RequestRedraw>,
    ) {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
                ..default()
            },
            Rotator,
        ));

        commands.spawn(DirectionalLightBundle {
            transform: Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
        commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
        event.send(RequestRedraw);
        commands.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "Press spacebar to cycle modes\n",
                    TextStyle {
                        font_size: 50.0,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: 50.0,
                    color: Color::GREEN,
                    ..default()
                }),
                TextSection::new(
                    "\nFrame: ",
                    TextStyle {
                        font_size: 50.0,
                        color: Color::YELLOW,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: 50.0,
                    color: Color::YELLOW,
                    ..default()
                }),
            ])
                .with_style(Style {
                    align_self: AlignSelf::FlexStart,
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                }),
            ModeText,
        ));
    }
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.13.0"
```

#### Discussion

I chose this example for today because some of the things I'm interested in building include: UIs for games, and full websites in Bevy. In both of these cases, we don't want to be constantly redrawing the screen if there is no user input. Bevy allows us to enter a "low power" mode, which will only re-render the screen when there is user input. This is what this example explores.

Let's start by just having a peek at the imports

```rust
use bevy::{
    prelude::*,
    utils::Duration,
    window::{PresentMode, RequestRedraw, WindowPlugin},
    winit::WinitSettings,
};
```

In most examples, we only `use bevy::prelude::*;`, but in this example, we're bringing in some things from `bevy::window` and `bevy::winit`, as well as `utils::Duration`.

`main()` is also a bit more complex than usual; we start by adding a `WinitSettings` resource

```rust
fn main() {
    App::new()
        // Continuous rendering for games - bevy's default.
        .insert_resource(WinitSettings::game())
        // Power-saving reactive rendering for applications.
        .insert_resource(WinitSettings::desktop_app())
        // You can also customize update behavior with the fields of [`WinitConfig`]
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
                wait: Duration::from_millis(10),
            },
        })
        // -- snip --
        .run();
}
```

Remember that `Resource`s are singletons, and calling `insert_resource` with a type of `Resource` which has already been added to the `World` will just overwrite the existing `Resource` of that type. So the above is equivalent to

```rust
fn main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::ReactiveLowPower {
                wait: Duration::from_millis(10),
            },
        })
        // -- snip --
        .run();
}
```

`WinitSettings` has just these two fields, `focused_mode` and `unfocused_mode`, and the documentation does a good job explaining what they're for

```rust
/// Settings for the [`WinitPlugin`](super::WinitPlugin).
#[derive(Debug, Resource)]
pub struct WinitSettings {
    /// Determines how frequently the application can update when it has focus.
    pub focused_mode: UpdateMode,
    /// Determines how frequently the application can update when it's out of focus.
    pub unfocused_mode: UpdateMode,
}
```

These fields are both of type `UpdateMode`, which has some great documentation, as well

```rust
/// Determines how frequently an [`App`](bevy_app::App) should update.
///
/// **Note:** This setting is independent of VSync. VSync is controlled by a window's
/// [`PresentMode`](bevy_window::PresentMode) setting. If an app can update faster than the refresh
/// rate, but VSync is enabled, the update rate will be indirectly limited by the renderer.
#[derive(Debug, Clone, Copy)]
pub enum UpdateMode {
    /// The [`App`](bevy_app::App) will update over and over, as fast as it possibly can, until an
    /// [`AppExit`](bevy_app::AppExit) event appears.
    Continuous,
    /// The [`App`](bevy_app::App) will update in response to the following, until an
    /// [`AppExit`](bevy_app::AppExit) event appears:
    /// - `wait` time has elapsed since the previous update
    /// - a redraw has been requested by [`RequestRedraw`](bevy_window::RequestRedraw)
    /// - new [window](`winit::event::WindowEvent`) or [raw input](`winit::event::DeviceEvent`)
    /// events have appeared
    Reactive {
        /// The approximate time from the start of one update to the next.
        ///
        /// **Note:** This has no upper limit.
        /// The [`App`](bevy_app::App) will wait indefinitely if you set this to [`Duration::MAX`].
        wait: Duration,
    },
    /// The [`App`](bevy_app::App) will update in response to the following, until an
    /// [`AppExit`](bevy_app::AppExit) event appears:
    /// - `wait` time has elapsed since the previous update
    /// - a redraw has been requested by [`RequestRedraw`](bevy_window::RequestRedraw)
    /// - new [window events](`winit::event::WindowEvent`) have appeared
    ///
    /// **Note:** Unlike [`Reactive`](`UpdateMode::Reactive`), this mode will ignore events that
    /// don't come from interacting with a window, like [`MouseMotion`](winit::event::DeviceEvent::MouseMotion).
    /// Use this mode if, for example, you only want your app to update when the mouse cursor is
    /// moving over a window, not just moving in general. This can greatly reduce power consumption.
    ReactiveLowPower {
        /// The approximate time from the start of one update to the next.
        ///
        /// **Note:** This has no upper limit.
        /// The [`App`](bevy_app::App) will wait indefinitely if you set this to [`Duration::MAX`].
        wait: Duration,
    },
}
```

But let's look at what this predefined `WinitSettings` -- `game` and `desktop_app` -- look like

```rust
impl WinitSettings {
    /// Default settings for games.
    ///
    /// [`Continuous`](UpdateMode::Continuous) if windows have focus,
    /// [`ReactiveLowPower`](UpdateMode::ReactiveLowPower) otherwise.
    pub fn game() -> Self {
        WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::ReactiveLowPower {
                wait: Duration::from_secs_f64(1.0 / 60.0), // 60Hz
            },
        }
    }

    /// Default settings for desktop applications.
    ///
    /// [`Reactive`](UpdateMode::Reactive) if windows have focus,
    /// [`ReactiveLowPower`](UpdateMode::ReactiveLowPower) otherwise.
    pub fn desktop_app() -> Self {
        WinitSettings {
            focused_mode: UpdateMode::Reactive {
                wait: Duration::from_secs(5),
            },
            unfocused_mode: UpdateMode::ReactiveLowPower {
                wait: Duration::from_secs(60),
            },
        }
    }
    // -- snip --
}
```

So when a `game` window has focus, Bevy will update it `Continuous`ly, as quickly as it can. But when a `desktop_app` window has focus, if there is no user input, it will update only once every five seconds by default. You can see this by running this example, hitting the space bar to enter `desktop_app` mode, and doing nothing for a while. Eventually you will see the animation jump to a new frame, even with no user input.

When a `game` window does not have focus, it will be limited to 60 [FPS](https://www.lenovo.com/ca/en/glossary/fps/) by default. When a `desktop_app` does not have focus, it will be updated once per minute (0.0167 FPS).

The `Default` `WinitSettings` is `game`

```rust
impl Default for WinitSettings {
    fn default() -> Self {
        WinitSettings::game()
    }
}
```

---

Next in `main()`, we insert the `ExampleMode` `Resource`, which is mutated when the user of this example hits the space bar

```rust
fn main() {
    App::new()
        // -- snip --
        .insert_resource(ExampleMode::Game)
        // -- snip --
        .run();
}

#[derive(Resource, Debug)]
enum ExampleMode {
    Game,
    Application,
    ApplicationWithRedraw,
}
```

---

Then, we make a change to the `Window` settings

```rust
fn main() {
    App::new()
        // -- snip --
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Turn off vsync to maximize CPU/GPU usage
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // -- snip --
        .run();
}
```

This `AutoNoVsync` is for demonstration purposes. When I run this example and allow Bevy to update `Continuous`ly, my CPU/GPU usage is about 266%/55% respectively. That means Bevy is using two and a half entire cores of my laptop, and about half of the GPU, to render a rotating cube as quickly as it possibly can. On my machine, that's 120Hz.

When I press the space bar in this example to enter low power mode, that usage drops to 5%/15% CPU/GPU usage. (Weird that GPU usage is still so high, but that's an investigation for another day.)

Something I picked up along the way... When running this example locally, you might notice that your screen "flickers" while the example is being rendered. This flickering is caused by precisely this `AutoNoVsync` setting. The default `PresentMode` is `Fifo` or ["VSync On"](https://www.intel.com/content/www/us/en/support/articles/000005552/graphics.html), and this default will ensure that the Bevy window is re-rendered in sync with the screen refresh, so no "flickering" will occur.

---

Finally, we add our systems to our app

```rust
fn main() {
    App::new()
        // -- snip --
        .add_systems(Startup, test_setup::setup)
        .add_systems(
            Update,
            (
                test_setup::cycle_modes,
                test_setup::rotate_cube,
                test_setup::update_text,
                update_winit,
            ),
        )
        .run();
}
```

In the `Startup` schedule, we run a `setup` system, and in the `Update` schedule, we run the systems that change the `WinitSettings` when the space bar is pressed (`cycle_modes`), rotate the cube (`rotate_cube`), and update the text in the window (`update_text`).

The author of this example has also helpfully separated the _relevant_ parts of this example from the _irrelevant_ parts. Everything in `test_setup` is required only to set up the example. But the purpose of this example is to demonstrate how different `WinitSettings` affect the behaviour of a Bevy app. This logic is contained in `update_winit`.

---

`update_winit` again has fantastic documentation

```rust
/// Update winit based on the current `ExampleMode`
fn update_winit(
    mode: Res<ExampleMode>,
    mut event: EventWriter<RequestRedraw>,
    mut winit_config: ResMut<WinitSettings>,
) {
    use ExampleMode::*;
    *winit_config = match *mode {
        Game => {
            // In the default `WinitConfig::game()` mode:
            //   * When focused: the event loop runs as fast as possible
            //   * When not focused: the event loop runs as fast as possible
            WinitSettings::game()
        }
        Application => {
            // While in `WinitConfig::desktop_app()` mode:
            //   * When focused: the app will update any time a winit event (e.g. the window is
            //     moved/resized, the mouse moves, a button is pressed, etc.), a [`RequestRedraw`]
            //     event is received, or after 5 seconds if the app has not updated.
            //   * When not focused: the app will update when the window is directly interacted with
            //     (e.g. the mouse hovers over a visible part of the out of focus window), a
            //     [`RequestRedraw`] event is received, or one minute has passed without the app
            //     updating.
            WinitSettings::desktop_app()
        }
        ApplicationWithRedraw => {
            // Sending a `RequestRedraw` event is useful when you want the app to update the next
            // frame regardless of any user input. For example, your application might use
            // `WinitConfig::desktop_app()` to reduce power use, but UI animations need to play even
            // when there are no inputs, so you send redraw requests while the animation is playing.
            event.send(RequestRedraw);
            WinitSettings::desktop_app()
        }
    };
}
```

...my only complaint is that it looks like `WinitSettings` used to be called `WinitConfig`, and this hasn't been updated in this example.

---

Everything else in `test_setup`, we've seen in previous katas, so I won't rehash those points again, but I will point out another small typo in this example, in case it confuses you, as it did me

```rust
/// Switch between update modes when the mouse is clicked.
pub(crate) fn cycle_modes(
    mut mode: ResMut<ExampleMode>,
    mouse_button_input: Res<ButtonInput<KeyCode>>,
) {
    if mouse_button_input.just_pressed(KeyCode::Space) {
        *mode = match *mode {
            ExampleMode::Game => ExampleMode::Application,
            ExampleMode::Application => ExampleMode::ApplicationWithRedraw,
            ExampleMode::ApplicationWithRedraw => ExampleMode::Game,
        };
    }
}
```

`mouse_button_input` here should be renamed, as, of course, it's not mouse button input that we're listening for, but the space bar (`KeyCode::Space`). It looks like this was also changed in this example at some point, but the variable name was never updated.

---

Today we learned how to reduce the resource consumption of a Bevy app when we don't need frames to be re-rendered as quickly as possible. If you're building a UI, or a website, or some other relatively static interface, maybe you don't need it to be rendered at 120 FPS. Maybe you don't need the default `game` settings.

Use `WinitSettings` to refresh only as quickly as you need, or only in response to user input, and save yourself from using two and a half entire CPUs to render something relatively simple and static.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
