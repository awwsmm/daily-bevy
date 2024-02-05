# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the eighth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Text 2D

Today is the eighth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, I'll be digging into the [`text2d` example](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/2d/text2d.rs) found in the Bevy repo.

#### The Code

Here's the `main.rs` for this example

```rust
use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (animate_translation, animate_rotation, animate_scale),
        )
        .run();
}

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
    // 2d camera
    commands.spawn(Camera2dBundle::default());
    // Demonstrate changing translation
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("translation", text_style.clone())
                .with_alignment(text_alignment),
            ..default()
        },
        AnimateTranslation,
    ));
    // Demonstrate changing rotation
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("rotation", text_style.clone()).with_alignment(text_alignment),
            ..default()
        },
        AnimateRotation,
    ));
    // Demonstrate changing scale
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("scale", text_style).with_alignment(text_alignment),
            ..default()
        },
        AnimateScale,
    ));
    // Demonstrate text wrapping
    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 42.0,
        color: Color::WHITE,
    };
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(Unicode linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    let other_box_size = Vec2::new(300.0, 200.0);
    let other_box_position = Vec2::new(320.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.20, 0.3, 0.70),
                custom_size: Some(Vec2::new(other_box_size.x, other_box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(other_box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(AnyCharacter linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: BreakLineOn::AnyCharacter,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: other_box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    for (text_anchor, color) in [
        (Anchor::TopLeft, Color::RED),
        (Anchor::TopRight, Color::GREEN),
        (Anchor::BottomRight, Color::BLUE),
        (Anchor::BottomLeft, Color::YELLOW),
    ] {
        commands.spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!(" Anchor::{text_anchor:?} "),
                    TextStyle {
                        color,
                        ..slightly_smaller_text_style.clone()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(250. * Vec3::Y),
            text_anchor,
            ..default()
        });
    }
}

fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
        transform.translation.y = 100.0 * time.elapsed_seconds().cos();
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos());
    }
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in &mut query {
        transform.translation = Vec3::new(400.0, 0.0, 0.0);

        let scale = (time.elapsed_seconds().sin() + 1.1) * 2.0;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
```

(By the end of this kata, I hope you'll understand what _all_ of that does! Really!)

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.12.1"
```

We also need an `assets/fonts/` directory with `FiraCode`, like in [this kata](https://github.com/awwsmm/daily-bevy/tree/bonus/Camera2dBundle_2).

#### Discussion

We're back to the `examples/` directory in the Bevy repo. Now that we understand a bit of how cameras and `Transform`s work, today's example looks at transforming text, rather than transforming the camera itself.

By now, we're familiar with everything happening in `main`

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (animate_translation, animate_rotation, animate_scale),
        )
        .run();
}
```

We

- add the `DefaultPlugins`
- add a `setup` system to the `Startup` `Schedule`
- add a few systems to the `Update` schedule
- and `run` the `App`

Though what does it actually _mean_ to "add a plugin" or "add a system" to an `App`?

Here's `add_plugins`

```rust
pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
    if matches!(
        self.plugins_state(),
        PluginsState::Cleaned | PluginsState::Finished
    ) {
        panic!(
            "Plugins cannot be added after App::cleanup() or App::finish() has been called."
        );
    }
    plugins.add_to_app(self);
    self
}
```

It `panic!`s if the provided `Plugin`s are all `Cleaned` or `Finished`, with an error message explaining how this has happened.

Note that while `App::cleanup()` and `App::finish()` _sound like_ they should happen at the _end_ of app execution, they actually seem to happen just before app exeuction begins. For instance, `App::finish()` looks like this

```rust
/// Run [`Plugin::finish`] for each plugin. This is usually called by the event loop once all
/// plugins are ready, but can be useful for situations where you want to use [`App::update`].
pub fn finish(&mut self) {
    // temporarily remove the plugin registry to run each plugin's setup function on app.
    let plugin_registry = std::mem::take(&mut self.plugin_registry);
    for plugin in &plugin_registry {
        plugin.finish(self);
    }
    self.plugin_registry = plugin_registry;
    self.plugins_state = PluginsState::Finished;
}
```

For each `plugin`, we call its `finish` method, which looks like this by default (note the documentation comment)

```rust
/// Finish adding this plugin to the [`App`], once all plugins registered are ready. This can
/// be useful for plugins that depends on another plugin asynchronous setup, like the renderer.
fn finish(&self, _app: &mut App) {
    // do nothing
}
```

So "finish" should be read like "finish adding this to the app" and not like "finish using this plugin". Similarly, for `cleanup`

```rust
/// Runs after all plugins are built and finished, but before the app schedule is executed.
/// This can be useful if you have some resource that other plugins need during their build step,
/// but after build you want to remove it and send it to another thread.
fn cleanup(&self, _app: &mut App) {
    // do nothing
}
```

Read this as: "clean up the setup process _for_ the plugin", not "clean up the plugin itself".

So anyway, if the plugins to be added haven't already been added, we run `add_to_app`. This comes from the `Plugins` `trait`

```rust
pub trait Plugins<Marker> {
    fn add_to_app(self, app: &mut App);
}
```

which is implemented via macro for tuples of plugins and accordingly for an individual `Plugin`

```rust
impl<P: Plugin> Plugins<PluginMarker> for P {
    #[track_caller]
    fn add_to_app(self, app: &mut App) {
        if let Err(AppError::DuplicatePlugin { plugin_name }) =
            app.add_boxed_plugin(Box::new(self))
        {
            panic!(
                "Error adding plugin {plugin_name}: : plugin was already added in application"
            )
        }
    }
}
```

This calls `add_boxed_plugin`

```rust
/// Boxed variant of [`add_plugins`](App::add_plugins) that can be used from a
/// [`PluginGroup`](super::PluginGroup)
pub(crate) fn add_boxed_plugin(
    &mut self,
    plugin: Box<dyn Plugin>,
) -> Result<&mut Self, AppError> {
    // -- snip --

    // Reserve that position in the plugin registry. if a plugin adds plugins, they will be correctly ordered
    let plugin_position_in_registry = self.plugin_registry.len();
    self.plugin_registry.push(Box::new(PlaceholderPlugin));

    self.building_plugin_depth += 1;
    let result = catch_unwind(AssertUnwindSafe(|| plugin.build(self)));
    self.building_plugin_depth -= 1;
    if let Err(payload) = result {
        resume_unwind(payload);
    }
    self.plugin_registry[plugin_position_in_registry] = plugin;
    Ok(self)
}
```

which pushes the plugin to a `plugin_registry`, which is just a field on the `App` `struct`

```rust
pub struct App {
    // -- snip --
    plugin_registry: Vec<Box<dyn Plugin>>,
    // -- snip --
}
```

And where does the `Plugin` actually... do its thing? Get configured? This happens in `add_boxed_plugin`, above, when `plugin.build` is called

```rust
/// Configures the [`App`] to which this plugin is added.
fn build(&self, app: &mut App);
```

For example, recall the `InputPlugin` we looked at in a previous kata

```rust
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            // keyboard
            .add_event::<KeyboardInput>()
            .init_resource::<Input<KeyCode>>()
            .init_resource::<Input<ScanCode>>()
            .add_systems(PreUpdate, keyboard_input_system.in_set(InputSystem))
        // -- snip --
    }
}
```

Let's save the deep dive into `add_systems` for a future kata, this is already getting a bit long, and we've got a lot of code to work through today.

---

After the `main` method, we have some marker `Component`s

```rust
#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;
```

Each of these is only used in two places -- in a tuple with another `Component` (as a "bundle") which is then used to spawn an entity, and then later to query for that entity.

`AnimateTranslation`, for example, is used here to create an entity

```rust
commands.spawn((
    Text2dBundle {
        text: Text::from_section("translation", text_style.clone())
            .with_alignment(text_alignment),
        ..default()
    },
    AnimateTranslation,
));
```

We've seen `Text2dBundle` before, this is just a bit of text which will be rendered in the window.

`text_style` and `text_alignment` are defined just above this...

```rust
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Center;
```

...and are used to create the other two entities as well. The other two entities are identical to this first one, except they use

- the `AnimateRotation` marker component with the text `"rotation"`, and
- the `AnimateScale` marker component with the text `"scale"`

These three entities are queried at the bottom of this file in three corresponding systems.

```rust
fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    // -- snip --
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    // -- snip --
}

fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    // -- snip --
}
```

Each of these systems performs a `Query` of the `World` for entities which
- contain `Transform` components, filtered to only those entities which also
- have a `Text` component, and
- have one of the three marker components we defined above: `AnimateTranslation`, `AnimateRotation`, or `AnimateScale`

Each of these `query`s returns a `mut`able reference to the set of entities which satisfies those conditions. In our case, we know that each of these sets contains only a single entity.

Each of these systems also takes a `time: Res<Time>` argument. We've seen `Res` before: it gives us an immutable reference to a `Resource` defined in our `World`. `Time` is new, though.

`Time` is a pretty simple `struct`, but it has a ton of great documentation. Here is an abridged version

```rust
/// A generic clock resource that tracks how much it has advanced since its
/// previous update and since its creation.
///
/// Multiple instances of this resource are inserted automatically by
/// [`TimePlugin`](crate::TimePlugin):
///
/// - [`Time<Real>`](crate::real::Real) tracks real wall-clock time elapsed.
/// - [`Time<Virtual>`](crate::virt::Virtual) tracks virtual game time that may
///   be paused or scaled.
/// - [`Time<Fixed>`](crate::fixed::Fixed) tracks fixed timesteps based on
///   virtual time.
/// - [`Time`] is a generic clock that corresponds to "current" or "default"
///   time for systems. It contains [`Time<Virtual>`](crate::virt::Virtual)
///   except inside the [`FixedUpdate`](bevy_app::FixedUpdate) schedule when it
///   contains [`Time<Fixed>`](crate::fixed::Fixed).
// -- snip --
pub struct Time<T: Default = ()> {
    context: T,
    wrap_period: Duration,
    delta: Duration,
    delta_seconds: f32,
    delta_seconds_f64: f64,
    elapsed: Duration,
    elapsed_seconds: f32,
    elapsed_seconds_f64: f64,
    elapsed_wrapped: Duration,
    elapsed_seconds_wrapped: f32,
    elapsed_seconds_wrapped_f64: f64,
}
```

So when we use `Res<Time>` in our app, we are using the "generic" `Time` (and therefore, by default, we are using `Time<Virtual>`).

The way in which `Time` is used in this example is interesting: we always do `time.elapsed_seconds()` and then call `.sin()` or `.cos()` on the result.

`elapsed_seconds` is just

```rust
/// Returns how much time has advanced since [`startup`](#method.startup), as [`f32`] seconds.
///
/// **Note:** This is a monotonically increasing value. It's precision will degrade over time.
/// If you need an `f32` but that precision loss is unacceptable,
/// use [`elapsed_seconds_wrapped`](#method.elapsed_seconds_wrapped).
#[inline]
pub fn elapsed_seconds(&self) -> f32 {
    self.elapsed_seconds
}
```

Where `self` above is the instance of the `Time` struct, so `elapsed_seconds` is just the `f32` field of the same name on that struct.

Taking the sine or the cosine of a linearly increasing value will result in a never-ending "bouncing back and forth" from -1.0 to 1.0. So `animate_translation`...

```rust
fn animate_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
        transform.translation.y = 100.0 * time.elapsed_seconds().cos();
    }
}
```

...will cause the `x` coordinate of the `AnimateTranslation` text to move back and forth between -500.0 and -300.0, while the `y` coordinate will move back and forth between -100.0 and 100.0. As `x` uses `sin` and `y` uses `cos`, the result is that the text traces our a circular path of radius 100.0, over and over.

`animate_rotation` uses our old friend the `Quat`ernion...

```rust
fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds().cos());
    }
}
```

...to rotate the `AnimateRotation` text around the z-axis (that is the axis "coming out of the screen") over and over.

And `animate_scale`...

```rust
fn animate_scale(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateScale>)>,
) {
    // Consider changing font-size instead of scaling the transform. Scaling a Text2D will scale the
    // rendered quad, resulting in a pixellated look.
    for mut transform in &mut query {
        transform.translation = Vec3::new(400.0, 0.0, 0.0);

        let scale = (time.elapsed_seconds().sin() + 1.1) * 2.0;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
```

This one might look a bit weird at first... why are we setting the `translation` of this text in every `Update` schedule?

The reason is... there is no reason. This is definitely wonky. We should probably have set this translation up where we defined this text

```rust
// Demonstrate changing scale
commands.spawn((
    Text2dBundle {
        text: Text::from_section("scale", text_style)
            .with_alignment(text_alignment),
        transform: Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)), // <- here
        ..default()
    },
    AnimateScale,
));
```

The thing that actually changes in this system with respect to time is the scale. We scale up 4.2x ((1.0 + 1.1) * 2.0) and then down to 0.2x ((-1.0 + 1.1) * 2.0) of the initial size. Note the comment here, which suggests changing the font size instead of scaling the rendered text itself. This is not vector text, it is a bitmap. Scaling up the rendered text will cause it to look pixellated. Scaling the font-size will eliminate this pixellation, but is very slightly harder to implement. (This is left as an exercise for the reader.)

So that's all the transformed text, but there are a few other things to explore in this example.

---

Here's everything else in the `setup` system which we haven't already covered

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // -- snip --

    // 2d camera
    commands.spawn(Camera2dBundle::default());

    // -- snip --

    // Demonstrate text wrapping
    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 42.0,
        color: Color::WHITE,
    };
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(Unicode linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    let other_box_size = Vec2::new(300.0, 200.0);
    let other_box_position = Vec2::new(320.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.20, 0.3, 0.70),
                custom_size: Some(Vec2::new(other_box_size.x, other_box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(other_box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(AnyCharacter linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: BreakLineOn::AnyCharacter,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: other_box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });

    for (text_anchor, color) in [
        (Anchor::TopLeft, Color::RED),
        (Anchor::TopRight, Color::GREEN),
        (Anchor::BottomRight, Color::BLUE),
        (Anchor::BottomLeft, Color::YELLOW),
    ] {
        commands.spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!(" Anchor::{text_anchor:?} "),
                    TextStyle {
                        color,
                        ..slightly_smaller_text_style.clone()
                    },
                )],
                ..Default::default()
            },
            transform: Transform::from_translation(250. * Vec3::Y),
            text_anchor,
            ..default()
        });
    }
}
```

We've got the now-very-familiar `Camera2dBundle`, followed by two quite long text wrapping examples. Both use the `slightly_smaller_text_style`

```rust
let slightly_smaller_text_style = TextStyle {
    font,
    font_size: 42.0,
    color: Color::WHITE,
};
```

...which uses the same `font` and `color` as the earlier `text_style` but is, well, slightly smaller.

```rust
let text_style = TextStyle {
    font: font.clone(),
    font_size: 60.0,
    color: Color::WHITE,
};
```

Here is the first example of text wrapping

```rust
    let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(box_size.x, box_size.y)),
                ..default()
            },
            transform: Transform::from_translation(box_position.extend(0.0)),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "this text wraps in the box\n(Unicode linebreaks)",
                        slightly_smaller_text_style.clone(),
                    )],
                    alignment: TextAlignment::Left,
                    linebreak_behavior: BreakLineOn::WordBoundary,
                },
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: box_size,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_translation(Vec3::Z),
                ..default()
            });
        });
```

`box_size` and `box_position` are pretty self-evident. These are 2D vectors (really, 2-tuples of floating-point values) which define the size and position of the box which contains the text.

Then we do a bunch of new things
- we `spawn(...).with_children(...)`
- we create a `SpriteBundle`, which we haven't seen before
- we define `linebreak_behavior` for the `text` in our `Text2dBundle`, and
- we define `text_2d_bounds` for our `Text2dBundle`

The second text wrapping example is very similar, except it uses
- a different position (`other_box_position`)
- a different `Sprite` `color`
- a different `text` value
- a different `text` `linebreak_behavior`

The "important" difference is the `linebreak_behavior`, which is what this pair of examples is demonstrating. In the first one, we use `BreakLineOn::WordBoundary`, and in the second, we use `BreakLineOn::AnyCharacter`. There is a third option: `NoWrap`. Here is where these are defined

```rust
/// Determines how lines will be broken when preventing text from running out of bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum BreakLineOn {
    /// Uses the [Unicode Line Breaking Algorithm](https://www.unicode.org/reports/tr14/).
    /// Lines will be broken up at the nearest suitable word boundary, usually a space.
    /// This behavior suits most cases, as it keeps words intact across linebreaks.
    WordBoundary,
    /// Lines will be broken without discrimination on any character that would leave bounds.
    /// This is closer to the behavior one might expect from text in a terminal.
    /// However it may lead to words being broken up across linebreaks.
    AnyCharacter,
    /// No soft wrapping, where text is automatically broken up into separate lines when it overflows a boundary, will ever occur.
    /// Hard wrapping, where text contains an explicit linebreak such as the escape sequence `\n`, is still enabled.
    NoWrap,
}
```

What about `spawn(...).with_children(...)`, what does that do?

```rust
/// Takes a closure which builds children for this entity using [`ChildBuilder`].
fn with_children(&mut self, f: impl FnOnce(&mut ChildBuilder)) -> &mut Self;
```

Okay... so this, like, nests an entity inside another entity? I guess? The documentation here isn't great

```rust
/// Struct for building children entities and adding them to a parent entity.
pub struct ChildBuilder<'w, 's, 'a> {
    commands: &'a mut Commands<'w, 's>,
    push_children: PushChildren,
}
```

```rust
impl<'w, 's, 'a> BuildChildren for EntityCommands<'w, 's, 'a> {
    fn with_children(&mut self, spawn_children: impl FnOnce(&mut ChildBuilder)) -> &mut Self {
        let parent = self.id();
        let mut builder = ChildBuilder {
            commands: self.commands(),
            push_children: PushChildren {
                children: SmallVec::default(),
                parent,
            },
        };

        spawn_children(&mut builder);
        let children = builder.push_children;
        if children.children.contains(&parent) {
            panic!("Entity cannot be a child of itself.");
        }
        self.commands().add(children);
        self
    }

    // -- snip --
}
```

I remember reading something about parents and children when we encountered `Transform` vs. `GlobalTransform`. I wonder if that's relevant?

What does it mean for an entity to have children? What does that imply? What are the ramifications? Lots of questions for another day, I suppose.

---

`SpriteBundle` is pretty straightforward, at least

```rust
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}
```

A `Sprite` is defined as

```rust
pub struct Sprite {
    /// The sprite's color tint
    pub color: Color,
    /// Flip the sprite along the `X` axis
    pub flip_x: bool,
    /// Flip the sprite along the `Y` axis
    pub flip_y: bool,
    /// An optional custom size for the sprite that will be used when rendering, instead of the size
    /// of the sprite's image
    pub custom_size: Option<Vec2>,
    /// An optional rectangle representing the region of the sprite's image to render, instead of
    /// rendering the full image. This is an easy one-off alternative to using a texture atlas.
    pub rect: Option<Rect>,
    /// [`Anchor`] point of the sprite in the world
    pub anchor: Anchor,
}
```

I suppose a "sprite" could be a simple rectangle, as it is in this kata, but I'd guess that `texture` would let us set an image, like a familiar "sprite"

```rust
pub struct Image {
    pub data: Vec<u8>,
    // TODO: this nesting makes accessing Image metadata verbose. Either flatten out descriptor or add accessors
    pub texture_descriptor: wgpu::TextureDescriptor<'static>,
    /// The [`ImageSampler`] to use during rendering.
    pub sampler: ImageSampler,
    pub texture_view_descriptor: Option<wgpu::TextureViewDescriptor<'static>>,
}
```

Ooh we're starting to hit `wgpu` bedrock. Maybe that's far enough for now.

---

`text_2d_bounds` is the last new bit from the text wrapping examples

```rust
/// The maximum width and height of text. The text will wrap according to the specified size.
/// Characters out of the bounds after wrapping will be truncated. Text is aligned according to the
/// specified [`TextAlignment`](crate::text::TextAlignment).
///
/// Note: only characters that are completely out of the bounds will be truncated, so this is not a
/// reliable limit if it is necessary to contain the text strictly in the bounds. Currently this
/// component is mainly useful for text wrapping only.
#[derive(Component, Copy, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct Text2dBounds {
    /// The maximum width and height of text in logical pixels.
    pub size: Vec2,
}
```

This is just the bounding box that we either overflow (where characters "will be truncated", according to the documentation above), or wrap within, as we saw earlier.

---

The final, final bit to cover is the text anchoring example (there are a lot of examples in this kata)

```rust
for (text_anchor, color) in [
    (Anchor::TopLeft, Color::RED),
    (Anchor::TopRight, Color::GREEN),
    (Anchor::BottomRight, Color::BLUE),
    (Anchor::BottomLeft, Color::YELLOW),
] {
    commands.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(
                format!(" Anchor::{text_anchor:?} "),
                TextStyle {
                    color,
                    ..slightly_smaller_text_style.clone()
                },
            )],
            ..Default::default()
        },
        transform: Transform::from_translation(250. * Vec3::Y),
        text_anchor,
        ..default()
    });
}
```

We create a vector of four (`Anchor`, `Color`) tuples and spawn them all in the same spot, but alter the anchor point of the `Text2dBundle`

```rust
pub struct Text2dBundle {
    /// Contains the text.
    pub text: Text,
    /// How the text is positioned relative to its transform.
    pub text_anchor: Anchor,
    // -- snip --
}
```

[The anchor point](https://matplotlib.org/stable/gallery/text_labels_and_annotations/text_alignment.html) defines how text is rendered relative to its "position". Because text doesn't exist at a single point -- it has an extent. Is the "position" the top-left corner of the area taken up by the text? Or the bottom-right corner? Or the center in the middle? That's what the anchor point defines: the point within the section of text which should be considered "its (x, y) position".

If two pieces of text have the same position, but one has an `Anchor::TopLeft` and the other has an `Anchor::BottomLeft`, one will be rendered above (with a greater y-position) than the other one.

Note that neither the text wrapping examples nor this anchor example use any systems. They are static and do not change with time.

Also note that these are not the only four kinds of `Anchor`s, there are a few more

```rust
/// How a sprite is positioned relative to its [`Transform`](bevy_transform::components::Transform).
/// It defaults to `Anchor::Center`.
#[derive(Component, Debug, Clone, Copy, Default, Reflect)]
#[doc(alias = "pivot")]
pub enum Anchor {
    #[default]
    Center,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    TopLeft,
    TopCenter,
    TopRight,
    /// Custom anchor point. Top left is `(-0.5, 0.5)`, center is `(0.0, 0.0)`. The value will
    /// be scaled with the sprite size.
    Custom(Vec2),
}
```

Notably, there are no baseline anchor points, as you might expect from the link above.

Phew! That was a marathon. I feel like I didn't go into as much detail as I usually would today because the Bevy example held so many things. It was really three (or six) examples in one.

But we got through it and at least touched on basically all the new stuff, so I'll consider that a win.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
