# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the tenth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Button

Today is the tenth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, we'll have a look at the [`button` example](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/ui/button.rs) in the Bevy repo.

#### The Code

Here's the `main.rs` for this example

```rust
//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]

use bevy::{prelude::*, winit::WinitSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.12.1"
```

We've also got our now-familiar `assets/fonts` directory, as well.

#### Discussion

Today's example looks into some basic UI capabilities provided by Bevy.

The very first thing I want to look at is this attribute and the comment above it

```rust
// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]
```

Removing this attribute and running the linter provided by `clippy` gives a warning on this section of code

```rust
mut interaction_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
    ),
    (Changed<Interaction>, With<Button>),
>
```

Clippy says

> "Very complex type used. Consider factoring parts into `type` definitions"

...as the comment above the `allow` attribute says, this would not make our code any clearer, and would in fact _increase_ indirection, making it _less_ clear. We know that `Query`s can get quite long, but putting this on a separate line just moves that complexity from one place to another. Most of the time, `clippy`'s suggestions are spot on, but here, we'll use our judgement and ignore this warning.

---

Next, we've got the usual `App::new()`, but with an _unusual_ `Resource` added

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}
```

...what is `WinitSettings`? and what does `desktop_app()` do?

```rust
pub struct WinitSettings {
    /// Controls how the [`EventLoop`](winit::event_loop::EventLoop) is deployed.
    ///
    /// - If this value is set to `false` (default), [`run`] is called, and exiting the loop will
    /// terminate the program.
    /// - If this value is set to `true`, [`run_return`] is called, and exiting the loop will
    /// return control to the caller.
    // -- snip --
    pub return_from_run: bool,
    /// Determines how frequently the application can update when it has focus.
    pub focused_mode: UpdateMode,
    /// Determines how frequently the application can update when it's out of focus.
    pub unfocused_mode: UpdateMode,
}
```

```rust
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
        ..Default::default()
    }
}
```

It sounds like this allows the app to be idle in between user interactions. From what I understand, the app will update when there is input of some kind, or after the `wait` period, which is 5 seconds in `Reactive` mode and 60 seconds in `ReactiveLowPower` mode. That all generally makes sense.

```rust
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
        /// The minimum time from the start of one update to the next.
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
        /// The minimum time from the start of one update to the next.
        ///
        /// **Note:** This has no upper limit.
        /// The [`App`](bevy_app::App) will wait indefinitely if you set this to [`Duration::MAX`].
        wait: Duration,
    },
}
```

So, since we're not making a game, which needs to be constantly updating as quickly as possible, we want to save CPU / GPU cycles by only updating when we _need_ to update.

The default is just to update as quickly as possible

```rust
impl Default for WinitSettings {
    fn default() -> Self {
        WinitSettings {
            return_from_run: false,
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        }
    }
}
```

---

Next, we've got a few marker traits (we've seen this pattern before)

```rust
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
```

followed by a `button_system` which we run in the `Update` schedule. The signature of `button_system` looks like

```rust
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    // -- snip --
}
```

We saw some of this earlier, when we were fighting with `clippy`. The second argument is simple

```rust
mut text_query: Query<&mut Text>
```

This just queries for any `Text` components in the app, and returns a mutable reference to that text. (Later in the app, we `spawn` a `TextBundle`, which contains a `Text` component.)

The first argument is more complex, though. Let's shorten the type names to make it a bit simpler

```rust
mut interaction_query: Query<(&A, &mut B, &mut C, &D), (Changed<Interaction>, With<Button>)>
```

Okay, so this is a `Query` which is querying for any entities which contain _all_ of the components listed: of types `A`, `B`, `C`, _and_ `D`. If an entity doesn't have all four of these components, it won't be returned from the `Query`.

And as we've seen before, the second argument to `Query` is the _filter_. Here, we're filtering down to only entities which have a `Button` component and... what does `Changed<Interaction>` mean?

```rust
/// A filter on a component that only retains results added or mutably dereferenced after the system last ran.
///
/// A common use for this filter is avoiding redundant work when values have not changed.
// -- snip --
/// # Examples
///
/// ```
/// # use bevy_ecs::component::Component;
/// # use bevy_ecs::query::Changed;
/// # use bevy_ecs::system::IntoSystem;
/// # use bevy_ecs::system::Query;
/// #
/// # #[derive(Component, Debug)]
/// # struct Name {};
/// # #[derive(Component)]
/// # struct Transform {};
///
/// fn print_moving_objects_system(query: Query<&Name, Changed<Transform>>) {
///     for name in &query {
///         println!("Entity Moved: {:?}", name);
///     }
/// }
///
/// # bevy_ecs::system::assert_is_system(print_moving_objects_system);
/// ```
pub struct Changed<T>(PhantomData<T>);
```

Okay, fair enough, we're again trying to make this app a bit more performant by _only_ returning items from the `Query` where the `Interaction` has `Changed`. Since we're looking at a button, this means we will only return an `Interaction` from the `Query` when the user _first_ hovers over the button, and not over and over _as_ the user _is hovering_ over the button. Nice.

`A`, `B`, `C`, and `D` are actually `Interaction`, `BackgroundColor`, `BorderColor`, and `Children`. So we're `Query`ing for entities which have all these components. The reason why we need these `Component`s is the next thing we'll look into.

---

Those four `Component`s in the `interaction_query` are destructured into four fields

```rust
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
```

Based on the _kind_ of `Interaction` -- `Pressed`, `Hovered`, or `None` (when we move from an "interacting" state back to a "non-interacting" state) -- we change the value of the text, the border color, and the background color.

The least obvious bit here is probably how the text is extracted from the `text_query`. Remember that the entity here which has a `Text` component is the `TextBundle`. But the entity which has a `BackgroundColor` and a `BorderColor` is the `ButtonBundle`. So the `interaction_query` will return the `ButtonBundle`.

The `TextBundle` is `spawn`ed as a "child" of the `ButtonBundle`. So we need to access the first (at index `0`) child of the `ButtonBundle` to get to the `TextBundle`. _We know_ that the `TextBundle` is a child of the `ButtonBundle`, but the `Query` doesn't "know" this, so we have to add the `Children` component into the `Query` so that we can then _inspect_ the children of the `ButtonBundle` returned from the `Query`. Does that make sense?

Then, we execute the `text_query` on the `children` of the `ButtonBundle` -- we sort of find the intersection of the venn diagram of those two queries. Here's the explanation of `get_mut` from the docs, it's still a bit unclear to me, though the example in the docs helps a little

```rust
    /// Returns the query item for the given [`Entity`].
    ///
    /// In case of a nonexisting entity or mismatched component, a [`QueryEntityError`] is returned instead.
    ///
    /// # Example
    ///
    /// Here, `get_mut` is used to retrieve the exact query item of the entity specified by the `PoisonedCharacter` resource.
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// #
    /// # #[derive(Resource)]
    /// # struct PoisonedCharacter { character_id: Entity }
    /// # #[derive(Component)]
    /// # struct Health(u32);
    /// #
    /// fn poison_system(mut query: Query<&mut Health>, poisoned: Res<PoisonedCharacter>) {
    ///     if let Ok(mut health) = query.get_mut(poisoned.character_id) {
    ///         health.0 -= 1;
    ///     }
    /// }
    /// # bevy_ecs::system::assert_is_system(poison_system);
    /// ```
    ///
    /// # See also
    ///
    /// - [`get`](Self::get) to get a read-only query item.
    #[inline]
    pub fn get_mut(&mut self, entity: Entity) -> Result<Q::Item<'_>, QueryEntityError> {
        // SAFETY: system runs without conflicts with other systems.
        // same-system queries have runtime borrow checks when they conflict
        unsafe {
            self.state
                .get_unchecked_manual(self.world, entity, self.last_run, self.this_run)
        }
    }
```

---

So that's the `button_system`. We probably should have done this in the opposite order, but the `setup` is next

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());

    // -- snip --
}
```

We have a `commands: Commands`, which we've seen before, but still haven't really dug into (maybe soon), and an `asset_server`, which we will _definitely_ get to in an upcoming kata.

We also `spawn` our usual `Camera2dBundle`.

---

Next, we spawn the "root" entity, a `NodeBundle`

```rust
commands
    .spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // -- snip --
    }
```

What is a `NodeBundle`?

```rust
/// The basic UI node
///
/// Useful as a container for a variety of child nodes.
#[derive(Bundle, Clone, Debug)]
pub struct NodeBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Styles which control the layout (size and position) of the node and it's children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// The background color, which serves as a "fill" for this node
    pub background_color: BackgroundColor,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}
```

It's just "a UI node". I suppose this is something similar to an HTML `<div>`, but in gamedev land.

Our node takes up 100% of the width and height of the window and aligns everything in the center vertically and horizontally. [Much easier than with CSS.](https://www.reddit.com/r/webdev/comments/pqbxst/why_is_there_so_many_memes_about_how_hard/)

---

Inside our node, we spawn a child. What does that mean, though?

```rust
/// Trait for removing, adding and replacing children and parents of an entity.
pub trait BuildChildren {
    /// Takes a closure which builds children for this entity using [`ChildBuilder`].
    fn with_children(&mut self, f: impl FnOnce(&mut ChildBuilder)) -> &mut Self;

    // -- snip --
}
```

There's not a ton of _explanatory_ documentation around parents and children. It's just sort of assumed that you know what this means. (It's actually difficult to even google this. If anyone has a good source explaining parent-child relationships in UI / UX, please send it along.)

I rememebr `Transform` had some docs about itself vs. `GlobalTransform` and how that was related to one being relative to a parent and one being relative to the world...

```rust
/// [`Transform`] is the position of an entity relative to its parent position, or the reference
/// frame if it doesn't have a [`Parent`](bevy_hierarchy::Parent).
///
/// [`GlobalTransform`] is the position of an entity relative to the reference frame.
```

...and I'm sure there are other things which are relative to a parent vs. the world, but it's not explicitly called out here, from what I can see.

---

So we spawn a child, and that child is a `ButtonBundle`

```rust
.with_children(|parent| {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            // -- snip --
        });
```

`ButtonBundle` has tons of familiar `Component`s

```rust
/// A UI node that is a button
#[derive(Bundle, Clone, Debug)]
pub struct ButtonBundle {
    /// Describes the logical size of the node
    pub node: Node,
    /// Marker component that signals this node is a button
    pub button: Button,
    /// Styles which control the layout (size and position) of the node and it's children
    /// In some cases these styles also affect how the node drawn/painted.
    pub style: Style,
    /// Describes whether and how the button has been interacted with by the input
    pub interaction: Interaction,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The background color, which serves as a "fill" for this node
    ///
    /// When combined with `UiImage`, tints the provided image.
    pub background_color: BackgroundColor,
    /// The color of the Node's border
    pub border_color: BorderColor,
    /// The image of the node
    pub image: UiImage,
    /// The transform of the node
    ///
    /// This component is automatically managed by the UI layout system.
    /// To alter the position of the `ButtonBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This component is automatically updated by the [`TransformPropagate`](`bevy_transform::TransformSystem::TransformPropagate`) systems.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}
```

...but the ones we care about in this example are width, height, border, vertical and horizontal text alignment, and the background and border colors.

---

Finally, within our button (a child of the button) is the `TextBundle`

```rust
.with_children(|parent| {
    parent.spawn(TextBundle::from_section(
        "Button",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));
});
```

We've seen this before: we've got a `font`, a `font_size`, and a `color`. The text is `"Button"` by default, but most of these properties change with user `Interaction`.

---

The end result is a button, centered vertically and horizontally in the window, with a background color and a border color. The button contains some text, as well. When the user hovers over the button, the border color, background color, and text change. When the user presses the button, all of these attributes change again.

All of this is done very efficiently, as well, as the scene is not re-rendered unless there is user interaction.

So that's your introduction to buttons and UI in Bevy! I hope it was informative -- I know I learned quite a lot. Tomorrow, let's see if we can't compile this example to WASM and get it running in a web browser!

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
