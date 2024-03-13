# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows entry #23 in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Sprite Sheet

Today is day #23 of Daily Bevy.

This kata uses [Bevy `v0.13.0`](https://github.com/bevyengine/bevy/tree/v0.13.0).

### Today's Kata

Today, we're exploring [the `sprite_sheet` example](https://github.com/bevyengine/bevy/blob/v0.13.0/examples/2d/sprite_sheet.rs) from the Bevy repo.

#### The Code

Here's the `main.rs` for this example

```rust
//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.13.0"
```

We also need the `gabe-idle-run.png` at `assets/textures/rpg/chars/gabe`. That can be downloaded from [here](https://github.com/bevyengine/bevy/blob/v0.13.0/assets/textures/rpg/chars/gabe/gabe-idle-run.png).

#### Discussion

Running this example shows a pixel art character ("Gabe") running in place in the middle of the window. But the only asset we have is a static `*.png`, so how is this animated? Let's dig into how we make that happen.

Looking at `main` first, only one thing stands out...

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}
```

...we use `DefaultPlugins`, but we `set` the value of a particular `Plugin` within this `PluginGroup`. We use `ImagePlugin::default_nearest()` as we did in [the `3d_shapes` example](https://github.com/awwsmm/daily-bevy/tree/3d/3d_shapes). Here, as there, this is used to scale up the small pixel art without blurring it. Try removing `.set(ImagePlugin::default_nearest())` to see what this scaled-up sprite looks like without it.

---

Next, we've got a simple `Component` (we've seen this pattern before)

```rust
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}
```

...and another `Component` which `derive`s `Deref` and `DerefMut` (we _haven't_ seen this before)

```rust
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
```

`Deref` and `DerefMut` let us write

```rust
timer.tick(time.delta());
if timer.just_finished() {
```

rather than

```rust
timer.0.tick(time.delta());
if timer.0.just_finished() {
```

in `animate_sprite()`.

---

Speaking of `animate_sprite()`, here's that whole system

```rust
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
```

`query` returns any entities with `AnimationIndices`, `AnimationTimer`, and `TextureAtlas` `Component`s. There's only one entity like this in our game, which we'll get to in a second. And actually, it's one of only two entites we `spawn` -- the only other entity is a camera.

But we need the `indices`, `timer`, and `atlas` to walk through the sprite sheet, and we need an immutable reference to the `Time` `Res`ource to step through the sheet at regular intervals.

Every time this system runs (every `Update` schedule), we get the time since the last execution (`time.delta()`) and advance our `timer` by that amount `timer.tick()`. Remember this `AnimationTimer` is a type we defined, and it comes from the `Query`. We'll see where this is set later on.

Anyway, if the timer has `just_finished` (if incrementing the `timer` by `time.delte()` caused us to exceed the `duration` of the `timer`), we advance to the next sprite in the sprite sheet (the "`atlas`"). If we're at the `last` sprite, we go back to the `first` one, otherwise, we increment the `atlas.index` to move to the next sprite in the sheet.

Since the `timer` uses `TimerMode::Repeating` (we'll see this later), it automaticall resets to zero when it exceeds its `duration`.

That's it! Pretty simple to animate. But where does all of this stuff come from?

---

The only other system we have in this example is the `setup` system, run in the `Startup` schedule

```rust
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // -- snip --
}
```

`setup` takes a `Commands` argument (we've seen this before), an `AssetServer` argument (we've seen this before), and a `TextureAtlasLayout` asset (we've not seen this before). What is a `TextureAtlasLayout`?

```rust
/// Stores a map used to lookup the position of a texture in a [`TextureAtlas`].
/// This can be used to either use and look up a specific section of a texture, or animate frame-by-frame as a sprite sheet.
///
/// Optionally it can store a mapping from sub texture handles to the related area index (see
/// [`TextureAtlasBuilder`]).
///
/// [Example usage animating sprite.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)
/// [Example usage loading sprite sheet.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)
///
/// [`TextureAtlasBuilder`]: crate::TextureAtlasBuilder
#[derive(Asset, Reflect, Debug, Clone)]
#[reflect(Debug)]
pub struct TextureAtlasLayout {
    // TODO: add support to Uniforms derive to write dimensions and sprites to the same buffer
    pub size: Vec2,
    /// The specific areas of the atlas where each texture can be found
    pub textures: Vec<Rect>,
    /// Maps from a specific image handle to the index in `textures` where they can be found.
    ///
    /// This field is set by [`TextureAtlasBuilder`].
    ///
    /// [`TextureAtlasBuilder`]: crate::TextureAtlasBuilder
    pub(crate) texture_handles: Option<HashMap<AssetId<Image>, usize>>,
}
```

Oh, that is... extremely nice documentation. It even references this very example.

It's not obvious to me what the `size` field is supposed to represent, but maybe we can figure it out from context.

---

```rust
let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 7, 1, None, None);
```

In the first line here, we use the `asset_server` to `load` a resource -- we've done this before. But we haven't created a `TextureAtlasLayout` before. What does `from_grid()` do?

```rust
/// Generate a [`TextureAtlasLayout`] as a grid where each
/// `tile_size` by `tile_size` grid-cell is one of the *section* in the
/// atlas. Grid cells are separated by some `padding`, and the grid starts
/// at `offset` pixels from the top left corner. Resulting layout is
/// indexed left to right, top to bottom.
///
/// # Arguments
///
/// * `tile_size` - Each layout grid cell size
/// * `columns` - Grid column count
/// * `rows` - Grid row count
/// * `padding` - Optional padding between cells
/// * `offset` - Optional global grid offset
pub fn from_grid(
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    padding: Option<Vec2>,
    offset: Option<Vec2>,
) -> Self {
    // -- snip --
}
```

Okay, so `tile_size` defines the width and height of each individual sprite in the sprite sheet. Sprites are separated by some number of `padding` pixels in each direction, and are `offset` from the top-left corner of the sprite sheet by some number of pixels in each direction. The sprite sheet is composed of some number of `columns` and `rows` of sprites. That all makes sense.

In our case, we have `7` columns and `1` row of 24px-by-24px sprites, with no padding and no offset. `from_grid()` returns a `TextureAtlasLayout` that looks like this

```rust
Self {
    size: ((tile_size + current_padding) * grid_size) - current_padding,
    textures: sprites,
    texture_handles: None,
}
```

If I'm reading the implementation of `from_grid()` correctly, I think `size` is the width x height of the whole grid of sprites, and `textures` is a `Vec` of `Rect`angles within that grid.

Finally, we add this new `layout` to the `texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>`

```rust
let texture_atlas_layout = texture_atlas_layouts.add(layout);
```

---

With this sprite atlas constructed, we can finally build the entities that will be used to animate the sprite in the other system

```rust
// Use only the subset of sprites in the sheet that make up the run animation
let animation_indices = AnimationIndices { first: 1, last: 6 };
```

The sprite at index `0` shows Gabe standing still. If you change `first` to `0`, this frame will be included, as well, but the animation will look a bit weird. (It almost looks like Gabe is sliding through a few frames.) Remember, `AnimationIndices` is a type we defined earlier -- it only holds these two numbers.

---

The second-to-last thing we do is spawn one of our two entities, a camera

```rust
commands.spawn(Camera2dBundle::default());
```

And then the last thing we do is spawn the other entity, which holds all the other `Component`s we care about

```rust
commands.spawn((
    SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    },
    animation_indices,
    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
));
```

We saw earlier what we do with the `animation_indices` and `AnimationTimer` -- we mutate the timer using the `Time` `Res`ource, and we use the `indices` just to track where we are in the sprite sheet, and jump back to the beginning if we're at the end.

The `SpriteSheetBundle` is a bit more involved, though

```rust
/// A [`Bundle`] of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `TextureAtlas`) or for animated sprites.
///
/// Note:
/// This bundle is identical to [`SpriteBundle`] with an additional [`TextureAtlas`] component.
///
/// Check the following examples for usage:
/// - [`animated sprite sheet example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)
/// - [`texture atlas example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)
#[derive(Bundle, Clone, Default)]
pub struct SpriteSheetBundle {
    /// Specifies the rendering properties of the sprite, such as color tint and flip.
    pub sprite: Sprite,
    /// The local transform of the sprite, relative to its parent.
    pub transform: Transform,
    /// The absolute transform of the sprite. This should generally not be written to directly.
    pub global_transform: GlobalTransform,
    /// The sprite sheet base texture
    pub texture: Handle<Image>,
    /// The sprite sheet texture atlas, allowing to draw a custom section of `texture`.
    pub atlas: TextureAtlas,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}
```

Again, lots of great documentation. We set the `texture`, `atlas`, and `transform` fields and leave everything else at their `..default()` values.

- the `texture` is just the raw `gabe-idle-run.png` image file we `load`ed using the `asset_server`

- the `transform` we create scales the resulting sprites 6x, but doesn't otherwise mutate them by translating them or rotating them

- and finally, the `atlas` -- we create this `TextureAtlas` using the `texture_atlas_layout` and `animation_indices` we defined earlier

We now understand where `animate_sprite` is getting all this information from. The `SpriteSheetBundle` uses the `atlas` to pull the appropriate sprite from the `texture`.

---

Well, that was pretty straightforward! Not too much to dig into here. I think that's because of the very clear and helpful documentation we encountered in this example. It's easier to understand how something works when you don't have to go digging through multiple files to figure things out from context.

This example was also very well-written, and didn't include anything extraneous. It was straight to the point! It was very easy to learn how to create pixel animations using a sprite sheet.

See you in the next kata!

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
