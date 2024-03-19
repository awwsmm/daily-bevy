# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows entry #24 in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Bounding Box 2D

Today is day #24 of Daily Bevy.

This kata uses [Bevy `v0.13.0`](https://github.com/bevyengine/bevy/tree/v0.13.0).

### Today's Kata

Today, we're looking at [the `bounding_2d` example](https://github.com/bevyengine/bevy/blob/v0.13.0/examples/2d/bounding_2d.rs) from the Bevy repo.

#### The Code

The code for this example is a bit too long to reproduce here, but can be found at `main.rs` on this branch.

The `Cargo.toml` for this example is the usual

```toml
[dependencies]
bevy = "0.13.0"
```

We also need `FiraMono-Medium.ttf` in `assets/fonts`.

#### Discussion

Today's example explores [bounding boxes](https://en.wikipedia.org/wiki/Minimum_bounding_box). In 2D, a bounding box is the smallest rectangle which can fully enclose a shape. In 3D, it's the smallest rectangular prism. This example uses bounding boxes to detect collisions between 2D shapes. Let's dig in!

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<Test>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_text, spin, update_volumes, update_test_state),
        )
        .add_systems(
            PostUpdate,
            (
                render_shapes,
                (
                    aabb_intersection_system.run_if(in_state(Test::AabbSweep)),
                    circle_intersection_system.run_if(in_state(Test::CircleSweep)),
                    ray_cast_system.run_if(in_state(Test::RayCast)),
                    aabb_cast_system.run_if(in_state(Test::AabbCast)),
                    bounding_circle_cast_system.run_if(in_state(Test::CircleCast)),
                ),
                render_volumes,
            )
                .chain(),
        )
        .run();
}
```

The example begins with `main()`. We add the usual `DefaultPlugins` and then initialize an [FSM](https://en.wikipedia.org/wiki/Finite-state_machine) called `Test`. `Test` is defined just below `main()` and is an `enum` of five `States`

```rust
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Test {
    AabbSweep,
    CircleSweep,
    #[default]
    RayCast,
    AabbCast,
    CircleCast,
}
```

We've seen `States` before in [examples like `game_menu`](https://github.com/awwsmm/daily-bevy/tree/games/game_menu).

After we set the initial `Test` state to the `default` (`RayCast`), we add a `setup` system to the `Startup` schedule (as we've done many times before), and add a few other systems to the `Update` schedule. The next bit is the most interesting part of `main()`, though.

We also add some systems to the `PostUpdate` schedule. Recall that, by default, the event loop runs all of these schedules in this order each loop:

```rust
/// * [`First`]
/// * [`PreUpdate`]
/// * [`StateTransition`]
/// * [`RunFixedMainLoop`]
///     * This will run [`FixedMain`] zero to many times, based on how much time has elapsed.
/// * [`Update`]
/// * [`PostUpdate`]
/// * [`Last`]
```

How is `PostUpdate` different from `Update`? 

```rust
/// The schedule that contains logic that must run after [`Update`]. For example, synchronizing "local transforms" in a hierarchy
/// to "global" absolute transforms. This enables the [`PostUpdate`] transform-sync system to react to "local transform" changes in
/// [`Update`] without the [`Update`] systems needing to know about (or add scheduler dependencies for) the "global transform sync system".
///
/// [`PostUpdate`] exists to do "engine/plugin response work" to things that happened in [`Update`].
/// [`PostUpdate`] abstracts out "implementation details" from users defining systems in [`Update`].
///
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostUpdate;
```

It's helpful here to look at `Update` and `PreUpdate`, as well

```rust
/// The schedule that contains app logic. Ideally containing anything that must run once per
/// render frame, such as UI.
///
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Update;
```

```rust
/// The schedule that contains logic that must run before [`Update`]. For example, a system that reads raw keyboard
/// input OS events into an `Events` resource. This enables systems in [`Update`] to consume the events from the `Events`
/// resource without actually knowing about (or taking a direct scheduler dependency on) the "os-level keyboard event system".
///
/// [`PreUpdate`] exists to do "engine/plugin preparation work" that ensures the APIs consumed in [`Update`] are "ready".
/// [`PreUpdate`] abstracts out "pre work implementation details".
///
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreUpdate;
```

So, from my very limited understanding, in a nutshell, these three schedules are meant to
- (`PreUpdate`) do "every-loop" setup before app logic, like capturing any user input
- (`Update`) run the main app logic
- (`PostUpdate`) "cleanup" logic, or, aggregate logic that must be run after _all_ `Update` systems are done

In this example, we run four systems (`update_text, spin, update_volumes, update_test_state`) in the `Update` schedule, and then up to five additional systems in the `PostUpdate` schedule

```rust
// -- snip --
    PostUpdate,
    (
        render_shapes,
        (
            aabb_intersection_system.run_if(in_state(Test::AabbSweep)),
            circle_intersection_system.run_if(in_state(Test::CircleSweep)),
            ray_cast_system.run_if(in_state(Test::RayCast)),
            aabb_cast_system.run_if(in_state(Test::AabbCast)),
            bounding_circle_cast_system.run_if(in_state(Test::CircleCast)),
        ),
        render_volumes,
    )
        .chain(),
// -- snip --
```

The `PostUpdate` systems are `.chain()`ed, as well. `.chain()` forces the systems to run in order

```rust
    /// Treat this collection as a sequence of systems.
    ///
    /// Ordering constraints will be applied between the successive elements.
    ///
    /// If the preceeding node on a edge has deferred parameters, a [`apply_deferred`](crate::schedule::apply_deferred)
    /// will be inserted on the edge. If this behavior is not desired consider using
    /// [`chain_ignore_deferred`](Self::chain_ignore_deferred) instead.
    fn chain(self) -> SystemConfigs {
        self.into_configs().chain()
    }
```

You might be asking yourself "so, couldn't we just use `.chain()` for all of these systems? Instead of using `PostUpdate`?" And the answer would be yes, we totally could do something like

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<Test>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_text,
                spin,
                update_volumes,
                update_test_state,
                render_shapes,
                (
                    aabb_intersection_system.run_if(in_state(Test::AabbSweep)),
                    circle_intersection_system.run_if(in_state(Test::CircleSweep)),
                    ray_cast_system.run_if(in_state(Test::RayCast)),
                    aabb_cast_system.run_if(in_state(Test::AabbCast)),
                    bounding_circle_cast_system.run_if(in_state(Test::CircleCast)),
                ),
                render_volumes,
            )
                .chain(),
        )
        .run();
}
```

But `.chain()`ing systems forces an ordering which (I can only assume) reduces performance. If we don't _need_ systems to run in a specific order _within_ a schedule, then we don't need to `.chain()` them, and Bevy can run them in whatever order is optimal.

This is also why (again, I can only assume) not _all_ systems in the `PostUpdate` schedule have this enforced ordering. Note that all the `.run_if(in_state(...))` systems are in a tuple. These systems, individually, can be run in any order. But we are telling Bevy that, as a group, they must all be run after `render_shapes` and before `render_volumes`.

Deciding when to use `PreUpdate` vs. `Update` vs. `PostUpdate` and when to `.chain()` vs. when not to `.chain()` are things that I suppose will become easier and more familiar over time.

---

After `main()`, we define a `Spin` marker `Component` and a `spin` system

```rust
#[derive(Component)]
struct Spin;

fn spin(time: Res<Time>, mut query: Query<&mut Transform, With<Spin>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() / 5.);
    }
}
```

`spin` runs in the `Update` schedule and will mutate the `transform` of any entity containing a `Spin` marker component by rotating it at a constant speed. All of these entities are `spawn`ed in the `setup` system, as we'll see later.

One thing I want to touch on here is the `Query`. A `Query` takes two arguments, `QueryData` and an optional `QueryFilter`

```rust
pub struct Query<'world, 'state, D: QueryData, F: QueryFilter = ()> {
    // -- snip --
}
```

The `QueryFilter` is made optional through the use of [default generic type parameters](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading).

Also, whatever components are specified in the `QueryFilter` are _not_ returned in the query result. Note how, in this system, we are looking for entities with both `Transform` and `Spin` components, but we are only _using_ the `Transform` component, so it's part of the `QueryData`, rather than the `QueryFilter`. We don't need access to the `Spin` component itself for anything, we just want to know that the entity has one, so it goes in the `QueryFilter` only. When we iterate over `query` above, we get only `Transform` components.

---

Next we have the `Test` `States`, mentioned above

```rust
#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Test {
    AabbSweep,
    CircleSweep,
    #[default]
    RayCast,
    AabbCast,
    CircleCast,
}
```

If this is your first time coming across `Aabb`, know that it stands for [axis-aligned bounding box](https://stackoverflow.com/q/22512319/2925434). It is a bounding box (as mentioned earlier) where the sides of the rectangle are parallel to the coordinate axes of the scene.

Only two systems in this example use these `Test` `States`: `update_test_state` and `update_text`.

```rust
fn update_test_state(
    keycode: Res<ButtonInput<KeyCode>>,
    cur_state: Res<State<Test>>,
    mut state: ResMut<NextState<Test>>,
) {
    if !keycode.just_pressed(KeyCode::Space) {
        return;
    }

    use Test::*;
    let next = match **cur_state {
        AabbSweep => CircleSweep,
        CircleSweep => RayCast,
        RayCast => AabbCast,
        AabbCast => CircleCast,
        CircleCast => AabbSweep,
    };
    state.set(next);
}
```

`update_test_state` just listens for the user to press the `Space` bar and moves to the next `Test` when they do. Note that we need to double-dereference `cur_state` (`**cur_state`) to unwrap the `State<Test>` value from the `Res<State<Test>>` and then to unwrap the `Test` value from `State<Test>`.

`update_text` checks if the `Test` state has changed since the last time it was run (`.is_changed()`), and if so, updates the text in the window

```rust
fn update_text(mut text: Query<&mut Text>, cur_state: Res<State<Test>>) {
    if !cur_state.is_changed() {
        return;
    }

    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;
    text.clear();

    text.push_str("Intersection test:\n");
    use Test::*;
    for &test in &[AabbSweep, CircleSweep, RayCast, AabbCast, CircleCast] {
        let s = if **cur_state == test { "*" } else { " " };
        text.push_str(&format!(" {s} {test:?} {s}\n"));
    }
    text.push_str("\npress Space to cycle");
}
```

By calling `single_mut()` on the `text` `Query`, we are asserting that there is only a single entity with a `Text` component in this world (because our query is just `Query<&mut Text>`), which is true -- we spawn a `TextBundle` in the `setup` system, which we'll get to in a bit.

Getting the raw text, or text styling, out of a `Text` component is kind of clunky. `Text` contains a `Vec<TextSection>` and each `TextSection` contains a raw `value: String` and a `TextStyle`. In almost every case I've encountered so far, `Text` has only a single "section", so we have to jump through this indexing (`[0]`) hoop each time. (In fact, if you search for "`.sections[`" in the Bevy repo, more than half of all occurrences _only_ reference `[0]`.)

Finally, writing

```rust
format!(" {s} {test:?} {s}\n")
```

...looks a bit unusual to me. I would write this as (and in fact, this is identical to)

```rust
format!(" {} {:?} {}\n", s, test, s)
```

We add an asterisk `*` next to the currently-selected option, so the user knows what state they're in.

And that's it. So if `Test` isn't used anywhere else, how does the behavior of the app change with the changing state? It all has to do with the `PostUpdate` systems defined earlier

```rust
aabb_intersection_system.run_if(in_state(Test::AabbSweep)),
circle_intersection_system.run_if(in_state(Test::CircleSweep)),
ray_cast_system.run_if(in_state(Test::RayCast)),
aabb_cast_system.run_if(in_state(Test::AabbCast)),
bounding_circle_cast_system.run_if(in_state(Test::CircleCast)),
```

Each of these systems only runs when the specified state is active. We do not need to query for the state at any point, Bevy takes care of running these systems only when appropriate (only when we are in the specified state).

---

Next, we define a `Shape` `enum`

```rust
#[derive(Component)]
enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle2d),
    Line(Segment2d),
    Capsule(Capsule2d),
    Polygon(RegularPolygon),
}
```

We wrap these `bevy_math::primitives` shapes like `Triangle2d` and `Segment2d` in newtypes like `Triangle` and `Line` so that we can put the newtypes into a closed `enum` hierarchy. We don't care about _all_ shapes, only the ones we put into this list. So later, when we `match` over a _particular_ shape, we can process only the ones we care about.

Like in `render_shapes`, for example

```rust
fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
    let color = Color::GRAY;
    for (shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        match shape {
            Shape::Rectangle(r) => {
                gizmos.primitive_2d(*r, translation, rotation, color);
            }
            Shape::Circle(c) => {
                gizmos.primitive_2d(*c, translation, rotation, color);
            }
            Shape::Triangle(t) => {
                gizmos.primitive_2d(*t, translation, rotation, color);
            }
            Shape::Line(l) => {
                gizmos.primitive_2d(*l, translation, rotation, color);
            }
            Shape::Capsule(c) => {
                gizmos.primitive_2d(*c, translation, rotation, color);
            }
            Shape::Polygon(p) => {
                gizmos.primitive_2d(*p, translation, rotation, color);
            }
        }
    }
}
```

This example uses `Gizmos` (which we've [seen in a previous kata](https://github.com/awwsmm/daily-bevy/tree/2d/2d_gizmos)) to render simple shapes to the screen. Remember from previous katas that `Gizmos` use an "immediate drawing mode" and must be redrawn each game loop, because they're cleared each game loop. This is perfect for us, because each loop, we need to re-render each shape anyway, as they're all rotating.

We `Query` for all `Shape`s (our `enum`) and extract their `Transform` components. Remember we rotate all `Shape`s in the `spin` system we saw earlier.

Getting the `translation` is straightforward enough, we just get the `.xy` components. But the `rotation` confuses me a bit here -- why `.to_euler()`? Why `YXZ` rather than `XYZ`? `.to_euler()` returns a `Vec3d`, and calling `.2` on that returns the z component, so is it irrelevant whether we use `XYZ` or `YXZ`? There are lots of other `.to_something()` methods, so why `.to_euler()`? Some comments in this example would be very helpful.

---

Next we have two more `enum` `Component`s

```rust
#[derive(Component)]
enum DesiredVolume {
    Aabb,
    Circle,
}

#[derive(Component, Debug)]
enum CurrentVolume {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}
```

"Desired" makes it sound like this is something the user is changing, but that's not the case. This is more like configuration for the example. (Although it would be interesting to extend this example to be able to change the `DesiredVolume` associate with a given shape.) Some of the rotating shapes have Aabb bounding boxes, some have circular bounding boxes; that's what `DesiredVolume` is. As the shape rotates, we need to recalculate the bounding box / bounding circle; that's what `CurrentVolume` is.

`update_volumes` recalculates those bounding boxes / circles

```rust
fn update_volumes(
    mut commands: Commands,
    query: Query<
        (Entity, &DesiredVolume, &Shape, &Transform),
        Or<(Changed<DesiredVolume>, Changed<Shape>, Changed<Transform>)>,
    >,
) {
    for (entity, desired_volume, shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        match desired_volume {
            DesiredVolume::Aabb => {
                let aabb = match shape {
                    Shape::Rectangle(r) => r.aabb_2d(translation, rotation),
                    Shape::Circle(c) => c.aabb_2d(translation, rotation),
                    Shape::Triangle(t) => t.aabb_2d(translation, rotation),
                    Shape::Line(l) => l.aabb_2d(translation, rotation),
                    Shape::Capsule(c) => c.aabb_2d(translation, rotation),
                    Shape::Polygon(p) => p.aabb_2d(translation, rotation),
                };
                commands.entity(entity).insert(CurrentVolume::Aabb(aabb));
            }
            DesiredVolume::Circle => {
                let circle = match shape {
                    Shape::Rectangle(r) => r.bounding_circle(translation, rotation),
                    Shape::Circle(c) => c.bounding_circle(translation, rotation),
                    Shape::Triangle(t) => t.bounding_circle(translation, rotation),
                    Shape::Line(l) => l.bounding_circle(translation, rotation),
                    Shape::Capsule(c) => c.bounding_circle(translation, rotation),
                    Shape::Polygon(p) => p.bounding_circle(translation, rotation),
                };
                commands
                    .entity(entity)
                    .insert(CurrentVolume::Circle(circle));
            }
        }
    }
}
```

`Or` in a `QueryFilter` is something I haven't seen before.

```rust
/// A filter that tests if any of the given filters apply.
///
/// This is useful for example if a system with multiple components in a query only wants to run
/// when one or more of the components have changed.
///
/// The `And` equivalent to this filter is a [`prim@tuple`] testing that all the contained filters
/// apply instead.
// -- snip --
pub struct Or<T>(PhantomData<T>);
```

Seems pretty self-explanatory. And there's an `And` filter as well!

So, in our example, `query` returns all entities with `DesiredVolume`, `Shape`, and `Transform` components, provided that at least one of those components has changed since the system was last run. We know `DesiredVolume` won't change -- at least as the example is currently written -- so `Changed<DesiredVolume>` will never cause the `Or` to be satisfied.

Again, we calculate the `translation` and `rotation` of each entity, based on its transform (updated in `spin`) and then we inspect the `DesiredVolume` to draw the appropriate bounding box / circle around the shape. `aabb_2d` and `bounding_circle` are two methods on `Bounded2d`, `impl`emented on all `RegularPolygon`s

```rust
impl Bounded2d for RegularPolygon {
    fn aabb_2d(&self, translation: Vec2, rotation: f32) -> Aabb2d {
        // -- snip --
    }

    fn bounding_circle(&self, translation: Vec2, _rotation: f32) -> BoundingCircle {
        // -- snip --
    }
}
```

Once we've calculated the bounding `aabb` or `circle`, we `insert` that `Component` into the original `entity` returned by the `query`, overwriting the `CurrencVolume` which previously existed on the entity.

---

After we update the bounding boxes / circles, we render them

```rust
fn render_volumes(mut gizmos: Gizmos, query: Query<(&CurrentVolume, &Intersects)>) {
    for (volume, intersects) in query.iter() {
        let color = if **intersects {
            Color::CYAN
        } else {
            Color::ORANGE_RED
        };
        match volume {
            CurrentVolume::Aabb(a) => {
                gizmos.rect_2d(a.center(), 0., a.half_size() * 2., color);
            }
            CurrentVolume::Circle(c) => {
                gizmos.circle_2d(c.center(), c.radius(), color);
            }
        }
    }
}
```

`Intersects` is a simple `Component` defined just below this system, and a newtype around a `bool`

```rust
#[derive(Component, Deref, DerefMut, Default)]
struct Intersects(bool);
```

...we will explore how this boolean flag is set later. For now, just know that, for each entity returned by the `query` above, if it is currently being intersected, we draw it in cyan, otherwise, we draw it in red.

---

Finally, we're at the `setup` system.

First, we define two global `const`s which will be used to offset the rotating shapes in a grid

```rust
const OFFSET_X: f32 = 125.;
const OFFSET_Y: f32 = 75.;
```

The first and last things we do in the `setup` system are: `spawn` the camera, and write the text to the screen

```rust
fn setup(mut commands: Commands, loader: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // -- snip --
    commands.spawn(
        TextBundle::from_section(
            "",
            TextStyle {
                font: loader.load("fonts/FiraMono-Medium.ttf"),
                font_size: 26.0,
                ..default()
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
    );
}
```

Working with styling in Bevy can be a bit clunky. There is a `Style` component, but also `TextStyle`, and also things like `ButtonBundle` and `NodeBundle` have separate styling for background color, border color, and more.

In the `-- snip --` in the middle, we spawn all the rotating shapes. They all look something like this

```rust
    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.),
            ..default()
        },
        Shape::Circle(Circle::new(45.)),
        DesiredVolume::Aabb,
        Intersects::default(),
    ));
```

Each entity has
- a `SpatialBundle` with a `transform`, positioning the shape in the window
- a `Shape`
- a `DesiredVolume`
- an `Intersects` component, which holds a `false` value by default

At the end of this example, we've got a bunch of systems. There are
- three `cast` systems
- two `intersection` systems
- and a few helper methods

The five systems correspond to the five "modes" the user can run the example in, and can switch between, using the space bar.

Let's start by understanding the `cast` systems.

---

The three `cast` systems are `ray_cast_system`, `aabb_cast_system`, and `bounding_circle_cast_system`. Each of these uses a helper method, `get_and_draw_ray`, which itself uses a helper method called `draw_ray`. `draw_ray` is _only_ used within `get_and_draw_ray`.

```rust
fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
    gizmos.line_2d(
        ray.ray.origin,
        ray.ray.origin + *ray.ray.direction * ray.max,
        Color::WHITE,
    );
    for r in [1., 2., 3.] {
        gizmos.circle_2d(ray.ray.origin, r, Color::FUCHSIA);
    }
}
```

`draw_ray` draws a `WHITE` `line_2d` from the `ray.ray.origin` to the end point of the ray, and it also draws three `circle_2d`s at the same point, but with different radii. This simulates drawing a "filled-in" circle (`circle_2d`s only have an outline, with no fill color).

`get_and_draw_ray` uses `draw_ray` to do the actual rendering, but also adjusts the position and rotation of the ray, using the `Time` resource

```rust
fn get_and_draw_ray(gizmos: &mut Gizmos, time: &Time) -> RayCast2d {
    let ray = Vec2::new(time.elapsed_seconds().cos(), time.elapsed_seconds().sin());
    let dist = 150. + (0.5 * time.elapsed_seconds()).sin().abs() * 500.;

    let aabb_ray = Ray2d {
        origin: ray * 250.,
        direction: Direction2d::new_unchecked(-ray),
    };
    let ray_cast = RayCast2d::from_ray(aabb_ray, dist - 20.);

    draw_ray(gizmos, &ray_cast);
    ray_cast
}
```

- `ray` is a unit-length 2D vector, which traces a circle around the origin as `time` increases
- `dist` is the length of the ray, which grows and shrinks over time (with a reasonable min and max length)
- `aabb_ray` is a ray with an origin, but an infinite length
- `ray_cast` cuts that infinite-length ray down to a finite-length ray
- `draw_ray` draws the finite-length ray on the screen
- and then the finite-length ray is returned from the function

With these helper methods in place (plus one more we'll get to later), we're down to only the five systems which control the five ways of running this example (which can be toggled by the user). The first one up is the `ray_cast_system`.

---

In the `ray_cast_system`, we draw a ray, and then draw a small, filled-in circle at each point where the ray intersects with the bounding boxes of the rotating shapes in the scene.

```rust
fn ray_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);

    for (volume, mut intersects) in volumes.iter_mut() {
        // -- snip --
    }
}
```

We need
- `gizmos` to draw the filled-in circle at the intersection points
- `time`, because the ray's length, origin, and orientation change with respect to time in `get_and_draw_ray`
- the `volumes` (bounding boxes) so that we can find the intersections (if any) between them and the ray

To find the intersection point, we use methods on `ray_cast`, which is of type `RayCast2d`. There are methods on this type for finding the distance from the origin of the ray to a bounding box (`aabb_intersection_at`) or a bounding circle (`circle_intersection_at`)

```rust
let toi = match volume {
    CurrentVolume::Aabb(a) => ray_cast.aabb_intersection_at(a),
    CurrentVolume::Circle(c) => ray_cast.circle_intersection_at(c),
};
```

`toi` is the distance from the origin of the `ray_cast` to the `Aabb` or the bounding `Circle`. It is `Option`al, because a ray might not intersect with a given bounding box / circle. (`toi`, I think, is meant to stand for "to intersection".)

If there is an intersection point, we set the `Intersects` component of this entity to `true`...

```rust
**intersects = toi.is_some();
```

...and draw the filled-in circle at the intersection point

```rust
if let Some(toi) = toi {
    for r in [1., 2., 3.] {
        gizmos.circle_2d(
            ray_cast.ray.origin + *ray_cast.ray.direction * toi,
            r,
            Color::GREEN,
        );
    }
}
```

---

The next two systems are similar, but instead of drawing a small, filled-in circle at the intersection point, we draw a small aabb or circle. In `aabb_cast_system`, we have the same signature and draw the same ray as in `ray_cast_system`...

```rust
fn aabb_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
    
    // -- snip --
}
```

...but then we construct this very specific kind of object, an `AabbCast2d`

```rust
let aabb_cast = AabbCast2d {
    aabb: Aabb2d::new(Vec2::ZERO, Vec2::splat(15.)),
    ray: ray_cast,
};
```

What is an `AabbCast2d`?

```rust
/// An intersection test that casts an [`Aabb2d`] along a ray.
#[derive(Clone, Debug)]
pub struct AabbCast2d {
    /// The ray along which to cast the bounding volume
    pub ray: RayCast2d,
    /// The aabb that is being cast
    pub aabb: Aabb2d,
}
```

It seems like a purpose-built `struct` for doing this exact thing.

Again, we loop over the entities in the query, and find the distance from the ray origin to the intersection (toi) point, if there is one, but only for shapes which have an aabb (and not a bounding circle)

```rust
for (volume, mut intersects) in volumes.iter_mut() {
    let toi = match *volume {
        CurrentVolume::Aabb(a) => aabb_cast.aabb_collision_at(a),
        CurrentVolume::Circle(_) => None,
    };
```

...and then, if there is an intersection point, we draw a small aabb instead of a small, filled-in circle

```rust
    **intersects = toi.is_some();
    if let Some(toi) = toi {
        gizmos.rect_2d(
            aabb_cast.ray.ray.origin
                + *aabb_cast.ray.ray.direction * toi
                + aabb_cast.aabb.center(),
            0.,
            aabb_cast.aabb.half_size() * 2.,
            Color::GREEN,
        );
    }
}
```

The last three arguments to `rect_2d()` are easy to understand
- `0.` is the `rotation`; we want this aabb to be _axis-aligned_, so it should not be rotated at all
- `aabb_case.aabb.half_size() * 2.` is the 2D (width x height) size of the `rect`angle to draw; we defined this to be a `15.` by `15.` px square earlier, with `Vec2::splat()`
- `Color::GREEN` is the color of the `rect`angle to draw

The most complex bit here is the first argument to `rect_2d()`, which is the position of the small aabb we want to draw. How do we find where we should draw the little aabb at the intersection? Well, we start with the origin of the ray (`aabb_cast.ray.ray.origin`), and we get the direction the ray is pointing in (`*aabb_cast.ray.ray.direction`), and we move some length along that direction until we intersect with the rotating shape (`toi`).

And what is the purpose of adding `aabb_cast.aabb.center()`? Well... nothing. This is always `Vec2(0.0, 0.0)`, so it can be removed and the app will behave in the exact same way.

---

`bounding_circle_cast_system` is exactly the same as `aabb_cast_system` except we
- replace `AabbCast2d` with `BoundingCircleCast`
- replace `aabb: Aabb2d` with `circle: BoundingCircle`
- replace `aabb_collision_at` on the `Aabb` match arm of `toi` with `circle_collision_at` on the `Circle` match arm
- draw a `circle_2d` at the intersection instead of a `rect_2d`

---

The much more exciting systems in this example, if you're interested in 2d collision detection (for making a game maybe?) are the two `intersection` systems -- `aabb_intersection_system` and `circle_intersection_system` -- at the end of this example.

Both of these systems use the `get_intersection_position` helper method

```rust
fn get_intersection_position(time: &Time) -> Vec2 {
    let x = (0.8 * time.elapsed_seconds()).cos() * 250.;
    let y = (0.4 * time.elapsed_seconds()).sin() * 100.;
    Vec2::new(x, y)
}
```

...which I think maybe doesn't have the greatest name, as it sounds like it's _finding_ an intersection point, when it's actually just defining the center of a shape which will move around the screen over time.

These two systems follow a similar pattern to the `cast` systems. First, we bring in the `gizmos` to draw shapes, the `time` to allow us to move the intersecting aabb / circle around the screen, and the `volumes` to find which shape(s) the aabb / circle is currently intersecting...

```rust
fn aabb_intersection_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>,
) {
```

...then, we move the aabb / circle around the screen, in a repeating pattern...

```rust
    let center = get_intersection_position(&time);
```

...then, we create the overlaid `aabb` and draw it to the screen...

```rust
    let aabb = Aabb2d::new(center, Vec2::splat(50.));
    gizmos.rect_2d(center, 0., aabb.half_size() * 2., Color::YELLOW);
```

...and finally, we call the `intersects()` method on the `aabb`, which is an extremely easy way to tell if an aabb intersects with another aabb / bounding circle.

```rust
    for (volume, mut intersects) in volumes.iter_mut() {
        let hit = match volume {
            CurrentVolume::Aabb(a) => aabb.intersects(a),
            CurrentVolume::Circle(c) => aabb.intersects(c),
        };

        **intersects = hit;
    }
}
```

After all of this buildup, it's hard to believe that it's that easy to find the intersection between two 2D shapes, but that's it! You can create a sprite, define a bounding box around it, and then just all `.intersects()` to see if it's collided with anything else on the screen. Easy!

---

I don't know about you, but I'm excited to use what I learned in this example to build a little 2D game with collisions. I hope you learned something from this kata! See you in the next one!

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
