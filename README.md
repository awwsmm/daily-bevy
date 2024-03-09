# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows entry #21 in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## 2D Viewport to World

Today is day #21 of Daily Bevy.

This kata uses [Bevy `v0.13.0`](https://github.com/bevyengine/bevy/tree/v0.13.0).

### Today's Kata

Today, we're exploring [the `2d_viewport_to_world` example](https://github.com/bevyengine/bevy/blob/release-0.13.0/examples/2d/2d_viewport_to_world.rs) in the Bevy repo.

#### The Code

Here's the `main.rs` for this example

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.13.0"
```

#### Discussion

I've taken about two weeks away from Daily Bevy because I've been building [a small game](https://github.com/awwsmm/tic-tac-toe) using what I've learned about Bevy so far.

While implementing this little tic-tac-toe game, one of the things I did in a bit of a clunky way was to convert window coordinates (measured from the top-left of the window, with `y` increasing toward the bottom of the window) to world coordinates (centered in the vertical and horizontal center of the window, with `y` increasing toward the top of the window).

I posted this game to Reddit and someone in the comments [helpfully explained](https://www.reddit.com/r/bevy/comments/1b325n3/comment/kspyad2/?utm_source=share&utm_medium=web2x&context=3) that I should use `viewport_to_world_2d` to do this instead. So that's what we'll be learning today!

We start with the `main` function

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}
```

As usual, we have an `App::new()`, we add the `DefaultPlugins`, and then we add a few systems. In this case, there is only a single `Startup` system (`setup`) and a single `Update` system (`draw_cursor`).

---

`setup` is very simple, it just spawns a camera

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
```

---

`draw_cursor` is where the interesting stuff happens

```rust
fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    // -- snip --
}
```

`draw_cursor` takes three arguments

- a `camera_query`, which queries for any entity with both a `Camera` and `GlobalTransform` component
- a `windows` `Query`, which queries for any entity with a `Window` component, and
- a `gizmos`, which gives the `Gizmos` `SystemParam` that we learned about in [the previous kata](https://github.com/awwsmm/daily-bevy/tree/2d/2d_gizmos)

`camera_query` will return only the `Camera2dBundle` which we `spawn`ed in the `setup` system. `windows` will return only the single window that's spawned by the `DefaultPlugins` (we first learned about this in [the `Camera2dBundle` `bonus` kata](https://github.com/awwsmm/daily-bevy/tree/bonus/Camera2dBundle_3)).

---

The body of `draw_cursor` is just a few lines. First, we get the single entity returned by the `camera_query`, and immediately destructure the resulting tuple into the `Camera` and `GlobalTransform` components

```rust
let (camera, camera_transform) = camera_query.single();
```

Calling `.single()` on a `Query` will panic if there is not exactly one entity returned from the query. A non-panicking version (which returns an `Option` instead) is available as `.get_single()`. Under the hood, `.single()` just calls `.get_single()` and then `.unwrap()`

```rust
pub fn single(&self) -> ROQueryItem<'_, D> {
    self.get_single().unwrap()
}
```

---

Next, we do the same for the entity containing the `Window` component -- we call `.single()` on it...

```rust
let Some(cursor_position) = windows.single().cursor_position() else {
    return;
};
```

...but then we immediately use that `Window` by calling `.cursor_position()` on it to get the pixel (window) coordinates of the cursor within that window.

This `let`-`else` block will silently `return` if there is a problem getting the `cursor_position()` (like, if the cursor is outside of window)

```rust
/// The cursor position in this window in logical pixels.
///
/// Returns `None` if the cursor is outside the window area.
///
/// See [`WindowResolution`] for an explanation about logical/physical sizes.
#[inline]
pub fn cursor_position(&self) -> Option<Vec2> {
    self.physical_cursor_position()
        .map(|position| (position.as_dvec2() / self.scale_factor() as f64).as_vec2())
}
```

---

Then, we take the `cursor_position` and the `GlobalTransform` component of the `Camera2dBundle`, and use both of those pieces of information to calculate "world" coordinates.

```rust
// Calculate a world position based on the cursor's position.
let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
    return;
};
```

By default, the `(0,0)` origin of world coordinates is centered in the horizontal and vertical center of the screen. So in a 600x600 window, the world origin is 300 pixels down and 300 pixels to the right of the top-left corner of the window. `viewport_to_world_2d` takes care of this calculation for us.

---

Finally, this example uses those world coordinates to draw a little circle at the cursor position

```rust
gizmos.circle_2d(point, 10., Color::WHITE);
```

Try running this example yourself!

And try it _without_ the coordinate transformation. What happens?

That's it for today. A short one to ease back into these daily katas.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
