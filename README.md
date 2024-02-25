# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows entry #20 in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## 2D Gizmos

Today is day #20 of Daily Bevy.

This kata uses [Bevy `v0.13.0`](https://github.com/bevyengine/bevy/tree/v0.13.0).

### Today's Kata

Today, we'll be digging into [the `2d_gizmos` example](https://github.com/bevyengine/bevy/blob/v0.13.0/examples/2d/2d_gizmos.rs) from the Bevy repo.

#### The Code

Here's the `main.rs` for this example

```rust
//! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.

use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_gizmo_group::<MyRoundGizmos>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_example_collection, update_config))
        .run();
}

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // text
    commands.spawn(TextBundle::from_section(
        "Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
        Hold 'Up' or 'Down' to change the line width of round gizmos\n\
        Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos",
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 24.,
            color: Color::WHITE,
        },
    ));
}

fn draw_example_collection(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    time: Res<Time>,
) {
    let sin = time.elapsed_seconds().sin() * 50.;
    gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);

    // Triangle
    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., Color::BLUE),
        (Vec2::new(-255., -155.), Color::RED),
        (Vec2::new(255., -155.), Color::GREEN),
        (Vec2::Y * 300., Color::BLUE),
    ]);

    gizmos.rect_2d(
        Vec2::ZERO,
        time.elapsed_seconds() / 3.,
        Vec2::splat(300.),
        Color::BLACK,
    );

    // The circles have 32 line-segments by default.
    my_gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
    my_gizmos.ellipse_2d(
        Vec2::ZERO,
        time.elapsed_seconds() % TAU,
        Vec2::new(100., 200.),
        Color::YELLOW_GREEN,
    );
    // You may want to increase this for larger circles.
    my_gizmos
        .circle_2d(Vec2::ZERO, 300., Color::NAVY)
        .segments(64);

    // Arcs default amount of segments is linearly interpolated between
    // 1 and 32, using the arc length as scalar.
    my_gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);

    gizmos.arrow_2d(
        Vec2::ZERO,
        Vec2::from_angle(sin / -10. + PI / 2.) * 50.,
        Color::YELLOW,
    );
}

fn update_config(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    if keyboard.pressed(KeyCode::ArrowRight) {
        config.line_width += 5. * time.delta_seconds();
        config.line_width = config.line_width.clamp(0., 50.);
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        config.line_width -= 5. * time.delta_seconds();
        config.line_width = config.line_width.clamp(0., 50.);
    }
    if keyboard.just_pressed(KeyCode::Digit1) {
        config.enabled ^= true;
    }

    let (my_config, _) = config_store.config_mut::<MyRoundGizmos>();
    if keyboard.pressed(KeyCode::ArrowUp) {
        my_config.line_width += 5. * time.delta_seconds();
        my_config.line_width = my_config.line_width.clamp(0., 50.);
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        my_config.line_width -= 5. * time.delta_seconds();
        my_config.line_width = my_config.line_width.clamp(0., 50.);
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        my_config.enabled ^= true;
    }
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.13.0"
```

We also need the usual `FiraMono` font in `assets/fonts`.

#### Discussion

Running this example, we see that there are a bunch of primitive shapes (some round, some straight) moving in regular patterns. We can interact with the example by following the instructions written to the window. Let's dig into how this works.

For the first time in a while, we have a totally new method call in `App`

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_gizmo_group::<MyRoundGizmos>()
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_example_collection, update_config))
        .run();
}
```

By this point, we know what `add_systems` does, we know about the `Startup` and `Update` schedules, and we've seen adding tuples of systems (e.g. `(sys1, sys2)`) to a schedule before. We've also seen `add_plugins(DefaultPlugins)` more than a few times.

But `init_gizmo_group` is brand new.

```rust
impl AppGizmoBuilder for App {
    fn init_gizmo_group<T: GizmoConfigGroup + Default>(&mut self) -> &mut Self {
        // -- snip --
    }

    // -- snip --
}
```

`init_gizmo_group` is a method on the `AppGizmoBuilder` trait...

```rust
/// A trait adding `init_gizmo_group<T>()` to the app
pub trait AppGizmoBuilder {
    /// Registers [`GizmoConfigGroup`] `T` in the app enabling the use of [Gizmos&lt;T&gt;](crate::gizmos::Gizmos).
    ///
    /// Configurations can be set using the [`GizmoConfigStore`] [`Resource`].
    fn init_gizmo_group<T: GizmoConfigGroup + Default>(&mut self) -> &mut Self;

    /// Insert the [`GizmoConfigGroup`] in the app with the given value and [`GizmoConfig`].
    ///
    /// This method should be preferred over [`AppGizmoBuilder::init_gizmo_group`] if and only if you need to configure fields upon initialization.
    fn insert_gizmo_group<T: GizmoConfigGroup>(
        &mut self,
        group: T,
        config: GizmoConfig,
    ) -> &mut Self;
}
```

...which seems to only let us add `GizmoConfigGroup`s to the `App`

```rust
/// A trait used to create gizmo configs groups.
///
/// Here you can store additional configuration for you gizmo group not covered by [`GizmoConfig`]
///
/// Make sure to derive [`Default`] + [`Reflect`] and register in the app using `app.init_gizmo_group::<T>()`
pub trait GizmoConfigGroup: Reflect + TypePath + Default {}
```

A `GizmoConfigGroup` seems to be some kind of superset of a `GizmoConfig`

```rust
/// A struct that stores configuration for gizmos.
#[derive(Clone, Reflect)]
pub struct GizmoConfig {
    // -- snip --
    pub enabled: bool,
    // -- snip --
    pub line_width: f32,
    // -- snip --
    pub line_perspective: bool,
    // -- snip --
    pub depth_bias: f32,
    // -- snip --
    pub render_layers: RenderLayers,
}
```

...but here the trail goes cold. What is a gizmo anyway?

> "The term is kinda fuzzy and means different things in different engines, but in Bevy it refers to lightweight 3D wireframe overlays that you can use for visual debugging." - [pcwalton](https://news.ycombinator.com/item?id=39413585), [Bevy contributor](https://github.com/pcwalton)

[`gizmos.rs`]() defines all of the different gizmos which can be drawn to the window

```rust
impl<'w, 's, T: GizmoConfigGroup> Gizmos<'w, 's, T> {
    // -- snip --
    pub fn line(&mut self, start: Vec3, end: Vec3, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn line_gradient(&mut self, start: Vec3, end: Vec3, start_color: Color, end_color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn ray(&mut self, start: Vec3, vector: Vec3, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn ray_gradient(
        &mut self,
        start: Vec3,
        vector: Vec3,
        start_color: Color,
        end_color: Color,
    ) {
        // -- snip --
    }

    // -- snip --
    pub fn linestrip(&mut self, positions: impl IntoIterator<Item = Vec3>, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn linestrip_gradient(&mut self, points: impl IntoIterator<Item = (Vec3, Color)>) {
        // -- snip --
    }

    // -- snip --
    pub fn sphere(
        &mut self,
        position: Vec3,
        rotation: Quat,
        radius: f32,
        color: Color,
    ) -> SphereBuilder<'_, 'w, 's, T> {
        // -- snip --
    }

    // -- snip --
    pub fn rect(&mut self, position: Vec3, rotation: Quat, size: Vec2, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn cuboid(&mut self, transform: impl TransformPoint, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn line_2d(&mut self, start: Vec2, end: Vec2, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn line_gradient_2d(
        &mut self,
        start: Vec2,
        end: Vec2,
        start_color: Color,
        end_color: Color,
    ) {
        // -- snip --
    }

    // -- snip --
    pub fn linestrip_2d(&mut self, positions: impl IntoIterator<Item = Vec2>, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn linestrip_gradient_2d(&mut self, positions: impl IntoIterator<Item = (Vec2, Color)>) {
        // -- snip --
    }

    // -- snip --
    pub fn ray_2d(&mut self, start: Vec2, vector: Vec2, color: Color) {
        // -- snip --
    }

    // -- snip --
    pub fn ray_gradient_2d(
        &mut self,
        start: Vec2,
        vector: Vec2,
        start_color: Color,
        end_color: Color,
    ) {
        // -- snip --
    }

    // -- snip --
    pub fn rect_2d(&mut self, position: Vec2, rotation: f32, size: Vec2, color: Color) {
        // -- snip --
    }

    // -- snip --
}
```

We can draw `line`s and `ray`s and `rect`angles and all sorts of 3D shapes as well, like `sphere`s and `cuboid`s.

Note that the above is an `impl` on the type `Gizmos`, `Gizmos` itself looks like this

```rust
/// A [`SystemParam`] for drawing gizmos.
///
/// They are drawn in immediate mode, which means they will be rendered only for
/// the frames in which they are spawned.
/// Gizmos should be spawned before the [`Last`](bevy_app::Last) schedule to ensure they are drawn.
pub struct Gizmos<'w, 's, T: GizmoConfigGroup = DefaultGizmoConfigGroup> {
    buffer: Deferred<'s, GizmoBuffer<T>>,
    pub(crate) enabled: bool,
    /// The currently used [`GizmoConfig`]
    pub config: &'w GizmoConfig,
    /// The currently used [`GizmoConfigGroup`]
    pub config_ext: &'w T,
}
```

This bit of documentation in particular...

> "They are drawn in immediate mode, which means they will be rendered only for the frames in which they are spawned."

...is probably something to keep in mind, as I can foresee us trying to draw gizmos, and them not being rendered because we drew them in the wrong schedule, and being confused as to what's happening.

Also, what's a `SystemParam`?

```rust
/// A parameter that can be used in a [`System`](super::System).
///
// -- snip --
///
/// Derived `SystemParam` structs may have two lifetimes: `'w` for data stored in the [`World`],
/// and `'s` for data stored in the parameter's state.
///
/// The following list shows the most common [`SystemParam`]s and which lifetime they require
///
/// ```
/// # use bevy_ecs::prelude::*;
/// # #[derive(Resource)]
/// # struct SomeResource;
/// # #[derive(Event)]
/// # struct SomeEvent;
/// # #[derive(Resource)]
/// # struct SomeOtherResource;
/// # use bevy_ecs::system::SystemParam;
/// # #[derive(SystemParam)]
/// # struct ParamsExample<'w, 's> {
/// #    query:
/// Query<'w, 's, Entity>,
/// #    res:
/// Res<'w, SomeResource>,
/// #    res_mut:
/// ResMut<'w, SomeOtherResource>,
/// #    local:
/// Local<'s, u8>,
/// #    commands:
/// Commands<'w, 's>,
/// #    eventreader:
/// EventReader<'w, 's, SomeEvent>,
/// #    eventwriter:
/// EventWriter<'w, SomeEvent>
/// # }
///```
// -- snip --
///
/// # Example
///
/// ```
/// # use bevy_ecs::prelude::*;
/// # #[derive(Resource)]
/// # struct SomeResource;
/// use std::marker::PhantomData;
/// use bevy_ecs::system::SystemParam;
///
/// #[derive(SystemParam)]
/// struct MyParam<'w, Marker: 'static> {
///     foo: Res<'w, SomeResource>,
///     marker: PhantomData<Marker>,
/// }
///
/// fn my_system<T: 'static>(param: MyParam<T>) {
///     // Access the resource through `param.foo`
/// }
///
/// # bevy_ecs::system::assert_is_system(my_system::<()>);
/// ```
// -- snip --
pub unsafe trait SystemParam: Sized {
    // -- snip --
}
```

This is why `Gizmos` can be passed directly in systems, e.g.

```rust
fn draw_example_collection(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    time: Res<Time>,
) {
```

...without needing to be wrapped in a `Res` or a `Query` or similar.

---

So, back to `init_gizmo_group`

```rust
impl AppGizmoBuilder for App {
    fn init_gizmo_group<T: GizmoConfigGroup + Default>(&mut self) -> &mut Self {
        if self.world.contains_resource::<GizmoStorage<T>>() {
            return self;
        }

        self.init_resource::<GizmoStorage<T>>()
            .add_systems(Last, update_gizmo_meshes::<T>);

        self.world
            .get_resource_or_insert_with::<GizmoConfigStore>(Default::default)
            .register::<T>();

        let Ok(render_app) = self.get_sub_app_mut(RenderApp) else {
            return self;
        };

        render_app.add_systems(ExtractSchedule, extract_gizmo_data::<T>);

        self
    }

    // -- snip --
}
```

When this is called in `App`, we pass in an empty `struct` for `T` called `MyRoundGizmos`

```rust
// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}
```

Recall from above that `GizmoConfigGroup` requires both `Default` and `Reflect` to be implemented as well

```rust
pub trait GizmoConfigGroup: Reflect + TypePath + Default {}
```

The first thing we do in `init_gizmo_group` is check if a `GizmoStorage<T>` `Resource` exists in the `World`. If it does, we return `self` (the existing `AppGizmoBuilder`); if it doesn't, we `init_resource` and add a new system to the `Last` schedule

```rust
if self.world.contains_resource::<GizmoStorage<T>>() {
    return self;
}

self.init_resource::<GizmoStorage<T>>()
    .add_systems(Last, update_gizmo_meshes::<T>);
```

I bet this is related to the line in the `Gizmos` documentation above, which reads "Gizmos should be spawned before the `Last` schedule to ensure they are drawn."

`update_gizmo_meshes` looks like

```rust
fn update_gizmo_meshes<T: GizmoConfigGroup>(
    mut line_gizmos: ResMut<Assets<LineGizmo>>,
    mut handles: ResMut<LineGizmoHandles>,
    mut storage: ResMut<GizmoStorage<T>>,
) {
    if storage.list_positions.is_empty() {
        handles.list.remove(&TypeId::of::<T>());
    } else if let Some(handle) = handles.list.get(&TypeId::of::<T>()) {
        let list = line_gizmos.get_mut(handle).unwrap();

        list.positions = mem::take(&mut storage.list_positions);
        list.colors = mem::take(&mut storage.list_colors);
    } else {
        let mut list = LineGizmo {
            strip: false,
            ..Default::default()
        };

        list.positions = mem::take(&mut storage.list_positions);
        list.colors = mem::take(&mut storage.list_colors);

        handles
            .list
            .insert(TypeId::of::<T>(), line_gizmos.add(list));
    }

    if storage.strip_positions.is_empty() {
        handles.strip.remove(&TypeId::of::<T>());
    } else if let Some(handle) = handles.strip.get(&TypeId::of::<T>()) {
        let strip = line_gizmos.get_mut(handle).unwrap();

        strip.positions = mem::take(&mut storage.strip_positions);
        strip.colors = mem::take(&mut storage.strip_colors);
    } else {
        let mut strip = LineGizmo {
            strip: true,
            ..Default::default()
        };

        strip.positions = mem::take(&mut storage.strip_positions);
        strip.colors = mem::take(&mut storage.strip_colors);

        handles
            .strip
            .insert(TypeId::of::<T>(), line_gizmos.add(strip));
    }
}
```

Notice that there are two `if` statements in the above body which are _identical_ except

1. the first uses `list` everywhere the second uses `strip`
2. the first creates a `LineGizmo` where `strip` is `false`, the second creates one where `strip` is `true`

And what's a `LineGizmo`?

```rust
#[derive(Asset, Debug, Default, Clone, TypePath)]
struct LineGizmo {
    positions: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    /// Whether this gizmo's topology is a line-strip or line-list
    strip: bool,
}
```

What's the difference between `list` and `strip`?

Well, there's no documentation around that in this crate, so we have to infer from context.

```rust
/// Draw a line in 3D from `start` to `end`.
///
/// This should be called for each frame the line needs to be rendered.
///
/// # Example
/// ```
/// # use bevy_gizmos::prelude::*;
/// # use bevy_render::prelude::*;
/// # use bevy_math::prelude::*;
/// fn system(mut gizmos: Gizmos) {
///     gizmos.line(Vec3::ZERO, Vec3::X, Color::GREEN);
/// }
/// # bevy_ecs::system::assert_is_system(system);
/// ```
#[inline]
pub fn line(&mut self, start: Vec3, end: Vec3, color: Color) {
    if !self.enabled {
        return;
    }
    self.extend_list_positions([start, end]);
    self.add_list_color(color, 2);
}
```

```rust
/// Draw a line in 3D made of straight segments between the points.
///
/// This should be called for each frame the line needs to be rendered.
///
/// # Example
/// ```
/// # use bevy_gizmos::prelude::*;
/// # use bevy_render::prelude::*;
/// # use bevy_math::prelude::*;
/// fn system(mut gizmos: Gizmos) {
///     gizmos.linestrip([Vec3::ZERO, Vec3::X, Vec3::Y], Color::GREEN);
/// }
/// # bevy_ecs::system::assert_is_system(system);
/// ```
#[inline]
pub fn linestrip(&mut self, positions: impl IntoIterator<Item = Vec3>, color: Color) {
    if !self.enabled {
        return;
    }
    self.extend_strip_positions(positions);
    let len = self.buffer.strip_positions.len();
    self.buffer
        .strip_colors
        .resize(len - 1, color.as_linear_rgba_f32());
    self.buffer.strip_colors.push([f32::NAN; 4]);
}
```

It looks like a `line` is just a particular kind of `linestrip`, where we only have two points, a `start` and an `end`. It seems like a `line` is just a single line segment, but a `linestrip` is a series of connected line segments.  

To test this theory, here's the original "triangle" example in this kata

```rust
// Triangle
gizmos.linestrip_gradient_2d([
    (Vec2::Y * 300., Color::BLUE),
    (Vec2::new(-255., -155.), Color::RED),
    (Vec2::new(255., -155.), Color::GREEN),
    (Vec2::Y * 300., Color::BLUE),
]);
```

![](https://raw.githubusercontent.com/awwsmm/daily-bevy/2d/2d_gizmos/assets/linestrip_gradient_2d.png)

...and here's that same triangle, but made using `line_gradient_2d`s

```rust
gizmos.line_gradient_2d(Vec2::Y * 300., Vec2::new(-255., -155.), Color::BLUE, Color::RED);
gizmos.line_gradient_2d(Vec2::new(-255., -155.), Vec2::new(255., -155.), Color::RED, Color::GREEN);
gizmos.line_gradient_2d(Vec2::new(255., -155.), Vec2::Y * 300., Color::GREEN, Color::BLUE);
```

![](https://raw.githubusercontent.com/awwsmm/daily-bevy/2d/2d_gizmos/assets/line_gradient_2d.png)

The triangle in the middle of the example looks exactly the same. As `line`s seem to be a special case of `linestrip`s, it seems like `line`s could probably be removed and replaced everywhere with `linestrip`s in this crate.

---

So anyway, the first thing that `update_gizmo_meshes` checks for, in either the "`list`" or the "`strip`" case, is whether that corresponding `Vec` of positions is empty in the `mut storage: ResMut<GizmoStorage<T>>`

```rust
if storage.list_positions.is_empty() { // also: if storage.strip_positions.is_empty() {
```

```rust
type PositionItem = [f32; 3];
type ColorItem = [f32; 4];

#[derive(Resource, Default)]
pub(crate) struct GizmoStorage<T: GizmoConfigGroup> {
  pub list_positions: Vec<PositionItem>,
  pub list_colors: Vec<ColorItem>,
  pub strip_positions: Vec<PositionItem>,
  pub strip_colors: Vec<ColorItem>,
  marker: PhantomData<T>,
}
```

(Presumably, `PositionItem` is a 3D position and `ColorItem` is an RGBA value.)

If there are no (list/strip) `positions` in `GizmoStorage<T>`, we remove the `TypeId` of type `T` from the `mut handles: ResMut<LineGizmoHandles>` struct...

```rust
handles.list.remove(&TypeId::of::<T>());
```

```rust
#[derive(Resource, Default)]
struct LineGizmoHandles {
    list: TypeIdMap<Handle<LineGizmo>>,
    strip: TypeIdMap<Handle<LineGizmo>>,
}
```

... which contains (key, value) pairs of type (`TypeId`, `LineGizmo`) for both the `list` and `strip` cases.

But why should we do this? The only other place the `LineGizmoHandles` is used is in `extract_gizmo_data`...

```rust
fn extract_gizmo_data<T: GizmoConfigGroup>(
    mut commands: Commands,
    handles: Extract<Res<LineGizmoHandles>>,
    config: Extract<Res<GizmoConfigStore>>,
) {
    let (config, _) = config.config::<T>();

    if !config.enabled {
        return;
    }

    for map in [&handles.list, &handles.strip].into_iter() {
        let Some(handle) = map.get(&TypeId::of::<T>()) else {
            continue;
        };
        commands.spawn((
            LineGizmoUniform {
                line_width: config.line_width,
                depth_bias: config.depth_bias,
                #[cfg(feature = "webgl")]
                _padding: Default::default(),
            },
            (*handle).clone_weak(),
            GizmoMeshConfig::from(config),
        ));
    }
}
```

...where we iterate over the `handles` and -- if there is a `handle` for a particular `TypeId` -- `spawn` an entity using that `handle`. So, it seems like, if there are no `position`s, we `remove` that `TypeId` from the `handles` so that we don't unnecessarily `spawn` a new entity with that `handle` for this particular `T: GizmoConfigGroup`.

---

But if there _are_ `positions`...

```rust
} else if let Some(handle) = handles.list.get(&TypeId::of::<T>()) {
    let list = line_gizmos.get_mut(handle).unwrap();

    list.positions = mem::take(&mut storage.list_positions);
    list.colors = mem::take(&mut storage.list_colors);
```

...and `handles` already contains a `handle` with the key of this type `T: GizmoConfigGroup`, then we extract the `LineGizmo` (`list`), and `take` the `positions` and `colors` from the `storage`, assigning them to the appropriate fields in the `LineGizmo`. `take` then assigns the default value (in this case, an empty vector) to `list_positions` and `list_colors`.

The next time this method is called, if we haven't repopulated these values, we will hit the first case, where `storage.list_positions.is_empty()`.

---

The final path is followed when there _are_ `positions` but _no_ `handle`

```rust
    } else {
        let mut list = LineGizmo {
            strip: false,
            ..Default::default()
        };

        list.positions = mem::take(&mut storage.list_positions);
        list.colors = mem::take(&mut storage.list_colors);

        handles
            .list
            .insert(TypeId::of::<T>(), line_gizmos.add(list));
    }
```

This will create a new `LineGizmo` using the `positions` and `colors` in `storage`, and then `insert` a new handle for this particular `T: GizmoConfigGroup` into the list of `handles`.

All of the above is run in every game loop, in the `Last` schedule. It seems like we need to add new positions _every single loop_ in order to render a gizmo, otherwise its `positions` in `storage` will be replaced with an empty vector, and the `handle` will be dropped.

---

Back in `init_gizmo_group`, after we add the above system to the `Last` schedule, we add a `GizmoConfigStore` `Resource` to the `World`

```rust
self.world
    .get_resource_or_insert_with::<GizmoConfigStore>(Default::default)
    .register::<T>();
```

We then call `register` on this `GizmoConfigStore`, which causes it to `insert`... itself... into... itself?

```rust
/// Inserts [`GizmoConfig`] and [`GizmoConfigGroup`] replacing old values
pub fn insert<T: GizmoConfigGroup>(&mut self, config: GizmoConfig, ext_config: T) {
    // INVARIANT: hash map must correctly map TypeId::of::<T>() to &dyn Reflect of type T
    self.store
        .insert(TypeId::of::<T>(), (config, Box::new(ext_config)));
}

pub(crate) fn register<T: GizmoConfigGroup>(&mut self) {
    self.insert(GizmoConfig::default(), T::default());
}
```

```rust
/// A [`Resource`] storing [`GizmoConfig`] and [`GizmoConfigGroup`] structs
///
/// Use `app.init_gizmo_group::<T>()` to register a custom config group.
#[derive(Resource, Default)]
pub struct GizmoConfigStore {
    // INVARIANT: must map TypeId::of::<T>() to correct type T
    store: TypeIdMap<(GizmoConfig, Box<dyn Reflect>)>,
}
```

At this point, we have a `GizmoConfigStore` `Resource` in the `World` which has a `store` field of type `TypeIdMap`, which contains (key, value) pairs of type (`TypeId`, `(GizmoConfig, Box<dyn Reflect>)`), and we have added one (key, value) pair

```rust
(TypeId::of::<T>(), (GizmoConfig::default(), Box::new(T::default())))
```

Maybe this will make sense a bit later.

---

Next in `init_gizmo_group`, we get a sub app, which is another thing we haven't done yet in these katas

```rust
let Ok(render_app) = self.get_sub_app_mut(RenderApp) else {
    return self;
};
```

```rust
/// A [`SubApp`] contains its own [`Schedule`] and [`World`] separate from the main [`App`].
/// This is useful for situations where data and data processing should be kept completely separate
/// from the main application. The primary use of this feature in bevy is to enable pipelined rendering.
///
// -- snip --
pub struct SubApp {
    /// The [`SubApp`]'s instance of [`App`]
    pub app: App,

    /// A function that allows access to both the main [`App`] [`World`] and the [`SubApp`]. This is
    /// useful for moving data between the sub app and the main app.
    extract: Box<dyn Fn(&mut World, &mut App) + Send>,
}
```

We have discussed this in previous katas, but this is the first time we're actually seeing multiple `World`s in one example. The `RenderApp` appears to be a `SubApp` with its own `World` and `Schedule`.

```rust
/// A Label for the rendering sub-app.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
pub struct RenderApp;
```

`RenderApp` gets added to Bevy `App`s via `DefaultPlugins`

```rust
#[cfg(feature = "bevy_render")]
{
    group = group
        .add(bevy_render::RenderPlugin::default())
        // NOTE: Load this after renderer initialization so that it knows about the supported
        // compressed texture formats
        .add(bevy_render::texture::ImagePlugin::default());

    #[cfg(all(not(target_arch = "wasm32"), feature = "multi-threaded"))]
    {
        group = group.add(bevy_render::pipelined_rendering::PipelinedRenderingPlugin);
    }
}
```

```rust
/// Contains the default Bevy rendering backend based on wgpu.
///
/// Rendering is done in a [`SubApp`], which exchanges data with the main app
/// between main schedule iterations.
///
/// Rendering can be executed between iterations of the main schedule,
/// or it can be executed in parallel with main schedule when
/// [`PipelinedRenderingPlugin`](pipelined_rendering::PipelinedRenderingPlugin) is enabled.
#[derive(Default)]
pub struct RenderPlugin {
    pub render_creation: RenderCreation,
    /// If `true`, disables asynchronous pipeline compilation.
    /// This has no effect on macOS, Wasm, or without the `multi-threaded` feature.
    pub synchronous_pipeline_compilation: bool,
}
```

```rust
impl Plugin for RenderPlugin {
    /// Initializes the renderer, sets up the [`RenderSet`] and creates the rendering sub-app.
    fn build(&self, app: &mut App) {
        // -- snip --
        match &self.render_creation {
            RenderCreation::Manual(device, queue, adapter_info, adapter, instance) => {
                // -- snip --
                unsafe { initialize_render_app(app) };
            }
            // -- snip --
        }
        // -- snip --
    }
    // -- snip --
}
```

```rust
unsafe fn initialize_render_app(app: &mut App) {
    app.init_resource::<ScratchMainWorld>();

    let mut render_app = App::empty();
    // -- snip --
    app.insert_sub_app(RenderApp, SubApp::new(render_app, move |main_world, render_app| {
        // -- snip --

        // run extract schedule
        extract(main_world, render_app);
    }));
// -- snip --
}
```

`SubApp`s are `run` in the `Update` `Schedule` of the main `App`

```rust
/// Advances the execution of the [`Schedule`] by one cycle.
///
/// This method also updates sub apps.
/// See [`insert_sub_app`](Self::insert_sub_app) for more details.
///
/// The schedule run by this method is determined by the [`main_schedule_label`](App) field.
/// By default this is [`Main`].
///
/// # Panics
///
/// The active schedule of the app must be set before this method is called.
pub fn update(&mut self) {
    #[cfg(feature = "trace")]
    let _bevy_update_span = info_span!("update").entered();
    {
        #[cfg(feature = "trace")]
        let _bevy_main_update_span = info_span!("main app").entered();
        self.world.run_schedule(self.main_schedule_label);
    }
    for (_label, sub_app) in &mut self.sub_apps {
        #[cfg(feature = "trace")]
        let _sub_app_span = info_span!("sub app", name = ?_label).entered();
        sub_app.extract(&mut self.world);
        sub_app.run();
    }

    self.world.clear_trackers();
}
```

If we _don't_ have this `SubApp`, we `return self` in `init_gizmo_group`. If we _do_ (which we _should_), we continue to the last line in `init_gizmo_group`

```rust
render_app.add_systems(ExtractSchedule, extract_gizmo_data::<T>);
```

We `extract_gizmo_data` in the `ExtractSchedule`, which, as seen above, is a `Schedule` in the `RenderApp` `SubApp`.

---

So now, we finally understand what `init_gizmo_group` is doing

1. we `init` a `GizmoStorage<T>` `Resource` for the specified `T: GizmoConfigGroup`
    - `GizmoStorage<T>` contains `list_positions` / `list_colors` and `strip_positions` / `strip_colors`
    - these positions and colors are used to render the `Gizmos` in the group
2. we add an `update_gizmo_meshes::<T>` system to the `Last` `Schedule` of the main `App`
    - this moves the latest `positions` and `colors` out of `GizmoStorage<T>` and into a new or existing `LineGizmo` `Handle`
    - this is called on every `Update`, so we need to be constantly repopulating the `storage` in order for our `Gizmos` to rerender
3. we add a `GizmoConfigStore` `Resource` to the `World`
    - and we `register` the type `T` in the `GizmoConfigStore`'s `store` field
    - multiple `GizmoConfigGroup`s can `register` with the same `GizmoConfigStore`
4. we get a mutable reference to the `RenderApp` `SubApp`
5. we add the `extract_gizmo_data::<T>` system to the `ExtractSchedule` in the `RenderApp`
    - `ExtractSchedule` for a `SubApp` is called within the `Update` `Schedule` of the main `App`

Phew. What's next?

---

After defining the `main()` entrypoint, we "create our own gizmo config group"...

```rust
// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}
```

...and then we `spawn` a `Camera2dBundle` and a simple `TextBundle` in the `setup` system, in the `Startup` schedule of the main `App`

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // text
    commands.spawn(TextBundle::from_section(
        "Hold 'Left' or 'Right' to change the line width of straight gizmos\n\
        Hold 'Up' or 'Down' to change the line width of round gizmos\n\
        Press '1' or '2' to toggle the visibility of straight gizmos or round gizmos",
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 24.,
            color: Color::WHITE,
        },
    ));
}
```

The only other systems in this kata, `draw_example_collection` and `update_config`, are both run in the `Update` `Schedule`.

---

Let's start with `update_config`. This system listens for user `keyboard` input and updates the `GizmoConfigStore` based on that input

```rust
fn update_config(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
   // -- snip --
}
```

...and so we need read-only access to the `ButtonInput<KeyCode>` `Resource`, but mutable access to the `GizmoConfigStore`.

Note that we also access the `Time` `Resource` in this system, in a read-only manner. This is unnecessary. Where we write e.g.

```rust
config.line_width += 5. * time.delta_seconds();
```

...we could just as easily write...

```rust
config.line_width += 5. / 120.0;
```

...assuming 120 FPS. We often scale by time in situations where the framerate may vary, and we want to ensure smooth movement from the perspective of the user, in spite of this variable framerate, but this is unnecessary in such a simple example.

---

In `update_config`, we get the `config` for the `DefaultGizmoConfigGroup`, which is where the straight-line gizmos live in this example

```rust
let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
```

For each of the straight-line gizmos, if the user presses the right arrow key on their keyboard, we increase the line width of the gizmos, clamped to a minimum width of `0.` and a maximum width of `50.`

```rust
 if keyboard.pressed(KeyCode::ArrowRight) {
     config.line_width += 5. * time.delta_seconds();
     config.line_width = config.line_width.clamp(0., 50.);
 }
```

When the user presses the left arrow, we decrease the width of the straight-line gizmos

```rust
 if keyboard.pressed(KeyCode::ArrowLeft) {
     config.line_width -= 5. * time.delta_seconds();
     config.line_width = config.line_width.clamp(0., 50.);
 }
```

And when the user presses the `1` key, we toggle the visibility of the straight-line gizmos, using the XOR assignment operator

```rust
 if keyboard.just_pressed(KeyCode::Digit1) {
     config.enabled ^= true;
 }
```

Above, if `config.enabled` is `true`, XOR sets it to `false`, and if `config.enabled` is `false`, XOR sets it to `true`.

We then do all of the above again, but for the `MyRoundGizmos` `GizmoConfigGroup`, which contains the round gizmos.

---

In the final system, `draw_example_collection`, we take `Gizmos` as a `SystemParam` and `my_gizmos`, which refers to the custom `GizmoConfigGroup` we added to the `App` with `init_gizmo_group`

```rust
fn draw_example_collection(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    time: Res<Time>,
) {
   // -- snip --
}
```

We also take an immutable reference to the `Time` `Resource`, which we use to transform the gizmos in a repeating pattern

```rust
let sin = time.elapsed_seconds().sin() * 50.;
```

---

We draw all the straight-line gizmos into the default gizmos group

```rust
 gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
 gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);

 // Triangle
 gizmos.linestrip_gradient_2d([
     (Vec2::Y * 300., Color::BLUE),
     (Vec2::new(-255., -155.), Color::RED),
     (Vec2::new(255., -155.), Color::GREEN),
     (Vec2::Y * 300., Color::BLUE),
 ]);

 gizmos.rect_2d(
     Vec2::ZERO,
     time.elapsed_seconds() / 3.,
     Vec2::splat(300.),
     Color::BLACK,
 );

// ...

gizmos.arrow_2d(
     Vec2::ZERO,
     Vec2::from_angle(sin / -10. + PI / 2.) * 50.,
     Color::YELLOW,
 );
```

`Vec2::Y` is a unit vector pointing along the y-axis

```rust
pub const Y: Self = Self::new(0.0, 1.0);
```

`Vec2::splat()` is well-explained by its doc comment

```rust
 /// Creates a vector with all elements set to `v`.
 #[inline]
 #[must_use]
 pub const fn splat(v: f32) -> Self {
     Self { x: v, y: v }
 }
```

And `Vec2::from_angle()` creates a unit vector pointing in the direction of the specified angle

```rust
 /// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
 /// conjunction with the [`rotate()`][Self::rotate()] method, e.g.
 /// `Vec2::from_angle(PI).rotate(Vec2::Y)` will create the vector `[-1, 0]`
 /// and rotate [`Vec2::Y`] around it returning `-Vec2::Y`.
 #[inline]
 #[must_use]
 pub fn from_angle(angle: f32) -> Self {
     let (sin, cos) = math::sin_cos(angle);
     Self { x: cos, y: sin }
 }
```

As for the gizmos themselves...

...`line_2d()` creates a 2D line segment from `start` to `end` with the specified `color`

```rust
 /// Draw a line in 2D from `start` to `end`.
 // -- snip --
 #[inline]
 pub fn line_2d(&mut self, start: Vec2, end: Vec2, color: Color) {
     if !self.enabled {
         return;
     }
     self.line(start.extend(0.), end.extend(0.), color);
 }
```

...`ray_2d()` does the same, but the second `Vec2` argument is relative to the first `Vec2` argument, rather than being relative to the origin

```rust
 /// Draw a line in 2D from `start` to `start + vector`.
 // -- snip --
 #[inline]
 pub fn ray_2d(&mut self, start: Vec2, vector: Vec2, color: Color) {
     if !self.enabled {
         return;
     }
     self.line_2d(start, start + vector, color);
 }
```

...as we saw earlier, `linestrip_gradient_2d` draws several line segments, connected to each other end-to-end

```rust
    /// Draw a line in 2D made of straight segments between the points, with a color gradient.
    // -- snip --
    #[inline]
    pub fn linestrip_gradient_2d(&mut self, positions: impl IntoIterator<Item = (Vec2, Color)>) {
        if !self.enabled {
            return;
        }
        self.linestrip_gradient(
            positions
                .into_iter()
                .map(|(vec2, color)| (vec2.extend(0.), color)),
        );
    }
```

...`rect_2d()` draws a rectangle from a center point (`position`), width-and-height dimensions (`size`), a `rotation` angle, and a `color`

```rust
 /// Draw a wireframe rectangle in 2D.
 // -- snip --
 #[inline]
 pub fn rect_2d(&mut self, position: Vec2, rotation: f32, size: Vec2, color: Color) {
     if !self.enabled {
         return;
     }
     let rotation = Mat2::from_angle(rotation);
     let [tl, tr, br, bl] = rect_inner(size).map(|vec2| position + rotation * vec2);
     self.linestrip_2d([tl, tr, br, bl, tl], color);
 }

// -- snip --

fn rect_inner(size: Vec2) -> [Vec2; 4] {
   let half_size = size / 2.;
   let tl = Vec2::new(-half_size.x, half_size.y);
   let tr = Vec2::new(half_size.x, half_size.y);
   let bl = Vec2::new(-half_size.x, -half_size.y);
   let br = Vec2::new(half_size.x, -half_size.y);
   [tl, tr, br, bl]
}
```

...and `arrow_2d()` draws an arrow from the `start` position to the `end` position with the specified `color`

```rust
 /// Draw an arrow in 3D, from `start` to `end`. Has four tips for convenient viewing from any direction.
 // -- snip --
 pub fn arrow(&mut self, start: Vec3, end: Vec3, color: Color) -> ArrowBuilder<'_, 'w, 's, T> {
     let length = (end - start).length();
     ArrowBuilder {
         gizmos: self,
         start,
         end,
         color,
         tip_length: length / 10.,
     }
 }

 /// Draw an arrow in 2D (on the xy plane), from `start` to `end`.
 // -- snip --
 pub fn arrow_2d(
     &mut self,
     start: Vec2,
     end: Vec2,
     color: Color,
 ) -> ArrowBuilder<'_, 'w, 's, T> {
     self.arrow(start.extend(0.), end.extend(0.), color)
 }
```

The arrow is _actually_ drawn here, by drawing a `line` for the body of the arrow, and then separate `line`s for the tips

```rust
impl<T: GizmoConfigGroup> Drop for ArrowBuilder<'_, '_, '_, T> {
    /// Draws the arrow, by drawing lines with the stored [`Gizmos`]
    fn drop(&mut self) {
        if !self.gizmos.enabled {
            return;
        }
        // first, draw the body of the arrow
        self.gizmos.line(self.start, self.end, self.color);
        // now the hard part is to draw the head in a sensible way
        // put us in a coordinate system where the arrow is pointing towards +x and ends at the origin
        let pointing = (self.end - self.start).normalize();
        let rotation = Quat::from_rotation_arc(Vec3::X, pointing);
        let tips = [
            Vec3::new(-1., 1., 0.),
            Vec3::new(-1., 0., 1.),
            Vec3::new(-1., -1., 0.),
            Vec3::new(-1., 0., -1.),
        ];
        // - extend the vectors so their length is `tip_length`
        // - rotate the world so +x is facing in the same direction as the arrow
        // - translate over to the tip of the arrow
        let tips = tips.map(|v| rotation * (v.normalize() * self.tip_length) + self.end);
        for v in tips {
            // then actually draw the tips
            self.gizmos.line(self.end, v, self.color);
        }
    }
}
```

As our calls to `line_2d()`, `ray_2d()`, and `arrow_2d()` all use the `sin` variable, they all change position over time. Similarly, our call to `rect_2d()` uses `time` directly, and so the rectangle also moves with respect to time. The triangle created using `linestrip_gradient_2d()`, however, does not move. It is re-rendered over and over in the same place.

---

We draw the round gizmos in the `mut my_gizmos: Gizmos<MyRoundGizmos>` group

```rust
    // The circles have 32 line-segments by default.
    my_gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
    my_gizmos.ellipse_2d(
        Vec2::ZERO,
        time.elapsed_seconds() % TAU,
        Vec2::new(100., 200.),
        Color::YELLOW_GREEN,
    );
    // You may want to increase this for larger circles.
    my_gizmos
        .circle_2d(Vec2::ZERO, 300., Color::NAVY)
        .segments(64);

    // Arcs default amount of segments is linearly interpolated between
    // 1 and 32, using the arc length as scalar.
    my_gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);
```

The two circles are rendered using `circle_2d()` and are static

```rust
 /// Draw a circle in 2D.
 // -- snip --
 ///     // Circles have 32 line-segments by default.
 ///     // You may want to increase this for larger circles.
 // -- snip --
 #[inline]
 pub fn circle_2d(
     &mut self,
     position: Vec2,
     radius: f32,
     color: Color,
 ) -> Ellipse2dBuilder<'_, 'w, 's, T> {
     Ellipse2dBuilder {
         gizmos: self,
         position,
         rotation: Mat2::IDENTITY,
         half_size: Vec2::splat(radius),
         color,
         segments: DEFAULT_CIRCLE_SEGMENTS,
     }
 }
```

Similar to `arrow_2d()`, circles are _actually_ rendered in the `Drop` implementation, where they consist of 32 individual line segments by default, created using `linestrip_2d()`

```rust
impl<T: GizmoConfigGroup> Drop for Ellipse2dBuilder<'_, '_, '_, T> {
    fn drop(&mut self) {
        if !self.gizmos.enabled {
            return;
        };

        let positions = ellipse_inner(self.half_size, self.segments)
            .map(|vec2| self.rotation * vec2)
            .map(|vec2| vec2 + self.position);
        self.gizmos.linestrip_2d(positions, self.color);
    }
}
```

This `Drop`-implementation-rendering is not specific to arrows and circles. This is how all gizmos are drawn.

A circle is just a special kind of ellipse, so the rendering for ellipses is similar to circles

```rust
 /// Draw an ellipse in 2D.
 // -- snip --
 ///     // Ellipses have 32 line-segments by default.
 ///     // You may want to increase this for larger ellipses.
 // -- snip --
 #[inline]
 pub fn ellipse_2d(
     &mut self,
     position: Vec2,
     angle: f32,
     half_size: Vec2,
     color: Color,
 ) -> Ellipse2dBuilder<'_, 'w, 's, T> {
     Ellipse2dBuilder {
         gizmos: self,
         position,
         rotation: Mat2::from_angle(angle),
         half_size,
         color,
         segments: DEFAULT_CIRCLE_SEGMENTS,
     }
 }
```

Finally, `arc_2d()` uses the `Arc2dBuilder`. Arcs are just partially-drawn circles.

```rust
 /// Draw an arc, which is a part of the circumference of a circle, in 2D.
 ///
 /// This should be called for each frame the arc needs to be rendered.
 ///
 /// # Arguments
 /// - `position` sets the center of this circle.
 /// - `radius` controls the distance from `position` to this arc, and thus its curvature.
 /// - `direction_angle` sets the clockwise  angle in radians between `Vec2::Y` and
 /// the vector from `position` to the midpoint of the arc.
 /// - `arc_angle` sets the length of this arc, in radians.
 ///
 // -- snip --
 ///     // Arcs have 32 line-segments by default.
 ///     // You may want to increase this for larger arcs.
 // -- snip --
 #[inline]
 pub fn arc_2d(
     &mut self,
     position: Vec2,
     direction_angle: f32,
     arc_angle: f32,
     radius: f32,
     color: Color,
 ) -> Arc2dBuilder<'_, 'w, 's, T> {
     Arc2dBuilder {
         gizmos: self,
         position,
         direction_angle,
         arc_angle,
         radius,
         color,
         segments: None,
     }
 }
```

```rust
impl<T: GizmoConfigGroup> Drop for Arc2dBuilder<'_, '_, '_, T> {
    fn drop(&mut self) {
        if !self.gizmos.enabled {
            return;
        }

        let segments = self
            .segments
            .unwrap_or_else(|| segments_from_angle(self.arc_angle));

        let positions = arc_2d_inner(self.direction_angle, self.arc_angle, self.radius, segments)
            .map(|vec2| (vec2 + self.position));
        self.gizmos.linestrip_2d(positions, self.color);
    }
}

fn arc_2d_inner(
    direction_angle: f32,
    arc_angle: f32,
    radius: f32,
    segments: usize,
) -> impl Iterator<Item = Vec2> {
    (0..segments + 1).map(move |i| {
        let start = direction_angle - arc_angle / 2.;

        let angle = start + (i as f32 * (arc_angle / segments as f32));
        Vec2::from(angle.sin_cos()) * radius
    })
}
```

---

The longest kata to date! There's definitely more we could dig into here around gizmos, but that is a pretty comprehensive introduction.

See you in the next one.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
