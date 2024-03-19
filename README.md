# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the first entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Hello, Bevy!

Today is the first day of Daily Bevy.

### An Introduction to Daily Bevy

Daily Bevy is a kind of programming [kata](https://www.oreilly.com/library/view/skill-up-a/9781787287037/ch57s05.html) journal, where I will dissect a small, self-contained Bevy example (nearly) every day. The official Bevy docs say that exploring the examples in the repo "is currently the best way to learn Bevy's features and how to use them." So that's what I'll do!

The goal is to break this example down and understand the constituent parts, working toward a complete understanding of the Bevy game engine. It is _not_ to write examples from scratch, explaining how to write Rust / Bevy code.

Each day, I will explore one of the [examples](https://github.com/bevyengine/bevy/tree/main/examples) from the `bevy` repo, an example I find on the Internet, or an example I write myself.

Each day will start _fresh_ from the initial empty commit to this repo (rather than accumulating code from every example), so that readers don't need to inspect a diff, or understand what is relevant (or not) to a particular example.

Each day will exist on a new branch of this repository so that earlier examples can be updated* without affecting later examples.

> &ast; Updates may be required because ["Bevy is still in the early stages of development"](https://bevyengine.org/learn/book/introduction/) and may introduce breaking changes in new releases. 

### Today's Kata

Okay, so let's get into it.

Today, I will be dissecting the standard [hello world](https://github.com/bevyengine/bevy/blob/main/examples/hello_world.rs) example found in the Bevy repo.

#### The Code

Here is the entirety of the `main.rs` file for this example, as of today

```rust
use bevy::prelude::*;

fn main() {
    App::new().add_systems(Update, hello_world_system).run();
}

fn hello_world_system() {
    println!("hello world");
}
```

This code requires only the base `bevy` crate in `Cargo.toml`

```toml
[dependencies]
bevy = "0.12.1"
```

#### Discussion

I have done a bit of Bevy exploration already, and I've found that importing `bevy::prelude::*` at the top of `main.rs` is usually a good idea. Chances are, you're going to want to use a good number of items from this module anyway, and I have already found a few copy-and-paste examples where name conflicts can lead to code not working quite right when importing items one-by-one using IDE code hints. So I'll always `use bevy::prelude::*;` at the top of every Bevy source file, until further notice.

```rust
App::new()
```

This instantiates a new `App` by delegating to `App::default()`. `App::default()` looks like this

```rust
impl Default for App {
    fn default() -> Self {
        let mut app = App::empty();
        #[cfg(feature = "bevy_reflect")]
        app.init_resource::<AppTypeRegistry>();

        app.add_plugins(MainSchedulePlugin);

        app.add_event::<AppExit>();

        #[cfg(feature = "bevy_ci_testing")]
        {
            crate::ci_testing::setup_app(&mut app);
        }

        app
    }
}
```

The `bevy_ci_testing` block looks like something related to the Bevy team's [GitHub CI pipeline testing](https://github.com/bevyengine/bevy/blob/bfb8e9978acebd4c137f277fe45ead3c0b5bf463/docs/cargo_features.md?plain=1#L49), so I'm happy to ignore that for now

```rust
impl Default for App {
    fn default() -> Self {
        let mut app = App::empty();
        #[cfg(feature = "bevy_reflect")]
        app.init_resource::<AppTypeRegistry>();

        app.add_plugins(MainSchedulePlugin);

        app.add_event::<AppExit>();

        app
    }
}
```

There's another [attribute](https://doc.rust-lang.org/rust-by-example/attribute.html) here, though: `bevy_reflect`. What does this do?

Well, `bevy_reflect` is a [whole separate crate](https://github.com/bevyengine/bevy/tree/bfb8e9978acebd4c137f277fe45ead3c0b5bf463/crates/bevy_reflect) in [the Bevy `workspace`](https://github.com/bevyengine/bevy/blob/bfb8e9978acebd4c137f277fe45ead3c0b5bf463/Cargo.toml#L16). My IDE ([RustRover](https://www.jetbrains.com/rust/nextversion/)) helpfully shows that this feature is enabled by default (unlike `bevy_ci_testing`). But where? Well, as `App` is defined within [the `bevy_app` crate](https://github.com/bevyengine/bevy/tree/bfb8e9978acebd4c137f277fe45ead3c0b5bf463/crates/bevy_app), this feature is [enabled in the `Cargo.toml` file of that crate](https://github.com/bevyengine/bevy/blob/bfb8e9978acebd4c137f277fe45ead3c0b5bf463/crates/bevy_app/Cargo.toml#L14). So

```rust
fn main() {
    App::new().add_systems(Update, hello_world_system).run();
}
```

...is equivalent to...

```rust
fn main() {
    let mut app = App::empty();

    app.init_resource::<AppTypeRegistry>();
    app.add_plugins(MainSchedulePlugin);
    app.add_event::<AppExit>();
    app.add_systems(Update, hello_world_system);

    app.run();
}
```

We create an `App` and `run` it, but the interesting stuff happens in the middle.

---

We first initialize an `AppTypeRegistry`...

```rust
#[derive(Resource, Clone, Default)]
pub struct AppTypeRegistry(pub TypeRegistryArc);
```

...which is a `Resource` and a tuple `struct`, and contains only a single field, a `TypeRegistryArc`

```rust
// TODO:  remove this wrapper once we migrate to Atelier Assets and the Scene AssetLoader doesn't
// need a TypeRegistry ref
/// A synchronized wrapper around a [`TypeRegistry`].
#[derive(Clone, Default)]
pub struct TypeRegistryArc {
    pub internal: Arc<RwLock<TypeRegistry>>,
}
```

It looks like this might be removed in the near future. (But Atelier Assets? [This one](https://github.com/Moxinilian/atelier-assets)? With 1 GitHub star?) Anyway, finally, we get to [the actual `TypeRegistry`](https://github.com/bevyengine/bevy/blob/bfb8e9978acebd4c137f277fe45ead3c0b5bf463/crates/bevy_reflect/src/type_registry.rs#L24), which is where I'm happy to stop digging for now. This seems to hold lots of type information to enable some [reflection in Bevy](https://taintedcoders.com/bevy/reflection/).

---

The next line is `app.add_plugins(MainSchedulePlugin)`

```rust
/// Initializes the [`Main`] schedule, sub schedules,  and resources for a given [`App`].
pub struct MainSchedulePlugin;
```

I was surprised to learn that the `Main` schedule could so easily be disabled (by using `App::empty()` instead of `App::new()`). Anyway, here's where I start to get a bit confused...

First, we instantiate a `Schedule` with the `Main` label, and set it to use a `SingleThreaded` executor

```rust
// simple "facilitator" schedules benefit from simpler single threaded scheduling
let mut main_schedule = Schedule::new(Main);
main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
```

The comment explains why we use a single-threaded executor for the `Main` schedule: [multithreading has overhead](https://jenkov.com/tutorials/java-concurrency/costs.html) and sometimes the increased complexity is not worth it. Nowhere in the Bevy codebase is it explained what a "facilitator schedule" is, though. Maybe it will become clear in later katas.

The `Main` schedule is very simple...

```rust
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Main;
```

...it derives a bunch of standard attributes, plus the Bevy-specific `ScheduleLabel` attribute. I won't reproduce the implementation of that attribute here; it seems to just register the label within the app's configuration. Maybe I'll dig into this more later, as well.

As for the `ExecutorKind`s...

```rust
/// Specifies how a [`Schedule`](super::Schedule) will be run.
///
/// The default depends on the target platform:
///  - [`SingleThreaded`](ExecutorKind::SingleThreaded) on WASM.
///  - [`MultiThreaded`](ExecutorKind::MultiThreaded) everywhere else.
#[derive(PartialEq, Eq, Default, Debug, Copy, Clone)]
pub enum ExecutorKind {
    /// Runs the schedule using a single thread.
    ///
    /// Useful if you're dealing with a single-threaded environment, saving your threads for
    /// other things, or just trying minimize overhead.
    #[cfg_attr(any(target_arch = "wasm32", not(feature = "multi-threaded")), default)]
    SingleThreaded,
    /// Like [`SingleThreaded`](ExecutorKind::SingleThreaded) but calls [`apply_deferred`](crate::system::System::apply_deferred)
    /// immediately after running each system.
    Simple,
    /// Runs the schedule using a thread pool. Non-conflicting systems can run in parallel.
    #[cfg_attr(all(not(target_arch = "wasm32"), feature = "multi-threaded"), default)]
    MultiThreaded,
}
```

...there are three: `SingleThreaded`, `Simple`, and `MultiThreaded`. Interestingly, `MultiThreaded` is not available for WASM targets. [Rust / WASM multithreading is possible, but still experimental](https://users.rust-lang.org/t/multithreading-in-wasm-how-did-that-come-out/87518), so perhaps Bevy will support this in a few years' time.

Another thing to note above is that `Simple` executors are just `SingleThreaded` executors which "[call] `apply_deferred` immediately after running each system." [`Deferred` refers to operations which mutate the `World`](https://github.com/bevyengine/bevy/blob/bfb8e9978acebd4c137f277fe45ead3c0b5bf463/crates/bevy_ecs/src/system/system_param.rs#L905) of your Bevy application. By waiting until a specified period to do all of your mutation, you leave the World in a read-only state for a longer period of time, which means more can be done in parallel.

As I said, this is where I begin to get confused, because `app.add_plugins(MainSchedulePlugin)` creates a `Main` `Schedule`, assigns it an `ExecutorKind`, adds it to the `App`, and adds a `System` (`Main::run_main`) for that `Schedule`...

```rust
impl Plugin for MainSchedulePlugin {
    fn build(&self, app: &mut App) {
        // simple "facilitator" schedules benefit from simpler single threaded scheduling
        let mut main_schedule = Schedule::new(Main);
        main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        let mut fixed_update_loop_schedule = Schedule::new(RunFixedUpdateLoop);
        fixed_update_loop_schedule.set_executor_kind(ExecutorKind::SingleThreaded);

        app.add_schedule(main_schedule)
            .add_schedule(fixed_update_loop_schedule)
            .init_resource::<MainScheduleOrder>()
            .add_systems(Main, Main::run_main);
    }
}
```

...that all seems to make sense to me, but then what is `RunFixedUpdateLoop` doing? There is no `System` associated with that `Schedule`. Eventually, though, I found this in the `bevy_time` crate

```rust
impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Time>()
            // -- snip --
            .add_systems(RunFixedUpdateLoop, run_fixed_update_schedule);

        // -- snip --
    }
}
```

I can look into this later, this is already getting a bit long.

---

After we add the `MainSchedulePlugin`, we add the `AppExit` event: `app.add_event::<AppExit>();`.

The documentation above `add_event` says

```rust
    /// Setup the application to manage events of type `T`.
    ///
    /// This is done by adding a [`Resource`] of type [`Events::<T>`],
    /// and inserting an [`event_update_system`] into [`First`].
```

which I read as a list of things that _I_ needed to do, but this is just describing the implementation of the `add_event` method itself

```rust
    pub fn add_event<T>(&mut self) -> &mut Self
    where
        T: Event,
    {
        if !self.world.contains_resource::<Events<T>>() {
            self.init_resource::<Events<T>>().add_systems(
                First,
                bevy_ecs::event::event_update_system::<T>
                    .run_if(bevy_ecs::event::event_update_condition::<T>),
            );
        }
        self
    }
```

`First`, used above, is run by the `MainSchedule`

```rust
impl Default for MainScheduleOrder {
    fn default() -> Self {
        Self {
            labels: vec![
                First.intern(),
                PreUpdate.intern(),
                StateTransition.intern(),
                RunFixedUpdateLoop.intern(),
                Update.intern(),
                SpawnScene.intern(),
                PostUpdate.intern(),
                Last.intern(),
            ],
        }
    }
}
```

so it's good that we added that `Plugin` to our `App`.

So from my understanding, at the start of the `MainSchedule` loop, systems tagged `First` will be executed, well, first. A system which listens for `AppExit` events is now in that list of systems, and so if the `App` has been closed since the previous iteration of the `MainSchedule`, at the start of the next iteration, Bevy will begin the process of unwinding things and closing the `App`.

Note that there doesn't seem to be an obvious way to _remove_ events, if we no longer want to listen for them.

The documentation above `AppExit` is interesting, as well

```rust
/// An event that indicates the [`App`] should exit. This will fully exit the app process at the
/// start of the next tick of the schedule.
///
/// You can also use this event to detect that an exit was requested. In order to receive it, systems
/// subscribing to this event should run after it was emitted and before the schedule of the same
/// frame is over. This is important since [`App::run()`] might never return.
///
/// If you don't require access to other components or resources, consider implementing the [`Drop`]
/// trait on components/resources for code that runs on exit. That saves you from worrying about
/// system schedule ordering, and is idiomatic Rust.
#[derive(Event, Debug, Clone, Default)]
pub struct AppExit;
```

"You can also use this event to detect that an exit was requested" implies that you might want to do some cleanup before the app is closed, but the "idiomatic Rust" way of doing this is to just implement `Drop` on any complex resources. Good to know.

---

Finally, we add _our_ system with `app.add_systems(Update, hello_world_system);`.

What _is_ a "system" anyway?

```rust
    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        // -- snip --
    }
```

A system is anything that implements `IntoSystemConfigs`. So how does

```rust
fn hello_world_system() {
    println!("hello world");
}
```

implement this trait? `IntoSystemConfigs` is implemented by this macro

```rust
macro_rules! impl_system_collection {
    ($(($param: ident, $sys: ident)),*) => {
        impl<$($param, $sys),*> IntoSystemConfigs<(SystemConfigTupleMarker, $($param,)*)> for ($($sys,)*)
        where
            $($sys: IntoSystemConfigs<$param>),*
        {
            #[allow(non_snake_case)]
            fn into_configs(self) -> SystemConfigs {
                let ($($sys,)*) = self;
                SystemConfigs::Configs {
                    configs: vec![$($sys.into_configs(),)*],
                    collective_conditions: Vec::new(),
                    chained: false,
                }
            }
        }
    }
}

all_tuples!(impl_system_collection, 1, 20, P, S);
```

and then also in three other places without using macros directly

```rust
impl IntoSystemConfigs<()> for BoxedSystem<(), ()>
```

```rust
impl<Marker, F> IntoSystemConfigs<Marker> for F
where
    F: IntoSystem<(), (), Marker>
```

```rust
impl IntoSystemConfigs<()> for SystemConfigs
```

The `IntoSystem` implementation is interesting, because its documentation says

> "Use this to get a system from a function."

Replacing `add_systems(Update, hello_world_system)` with `add_systems(Update, IntoSystem::into_system(hello_world_system))` _does_ work, but is that what's actually being done implicitly?

After some more poking around, I found that `IntoSystem` is implemented for any type `F` such that `F` implements `SystemParamFunction` 

```rust
impl<Marker, F> IntoSystem<F::In, F::Out, (IsFunctionSystem, Marker)> for F
where
    Marker: 'static,
    F: SystemParamFunction<Marker>,
{
```

and `SystemParamFunction` is implemented by _another_ macro, explicitly for function pointers

```rust
macro_rules! impl_system_function

// -- snip --

all_tuples!(impl_system_function, 0, 16, F);
```

`SystemParamFunction`'s documentation describes it as

> "A trait implemented for all functions that can be used as [`System`]s."

So I think this is probably the path by which an arbitrary function actually gets translated into a `System`. But I can dig into this more in later katas.

---

Finally, we are near the end with `.run()`.

> "Starts the application by calling the app's [runner function](Self::set_runner)."

But we haven't set a runner, at least not explicitly. As it turns out, this was hidden in `App::empty`, which is not _really_ empty

```rust
    /// Creates a new empty [`App`] with minimal default configuration.
    ///
    /// This constructor should be used if you wish to provide custom scheduling, exit handling, cleanup, etc.
    pub fn empty() -> App {
        let mut world = World::new();
        world.init_resource::<Schedules>();
        Self {
            world,
            runner: Box::new(run_once),
            sub_apps: HashMap::default(),
            plugin_registry: Vec::default(),
            plugin_name_added: Default::default(),
            main_schedule_label: Main.intern(),
            building_plugin_depth: 0,
            plugins_state: PluginsState::Adding,
        }
    }
```

The default runner here is `run_once`, though it's possible to set a custom runner with

```rust
    pub fn set_runner(&mut self, run_fn: impl FnOnce(App) + 'static + Send) -> &mut Self {
        self.runner = Box::new(run_fn);
        self
    }
```

But the implementation of `.run()` itself is quite confusing to me

```rust
    pub fn run(&mut self) {
        #[cfg(feature = "trace")]
        let _bevy_app_run_span = info_span!("bevy_app").entered();

        let mut app = std::mem::replace(self, App::empty());
        if app.building_plugin_depth > 0 {
            panic!("App::run() was called from within Plugin::build(), which is not allowed.");
        }

        let runner = std::mem::replace(&mut app.runner, Box::new(run_once));
        (runner)(app);
    }
```

This line

```rust
let mut app = std::mem::replace(self, App::empty());
```

appears to replace `self` with an `empty` `App`...? `std::mem::replace` returns the _original_ `self`, which we then use, but why are we swapping a new `App::empty` into memory where the old one was stored?

And I have the same question for

```rust
let runner = std::mem::replace(&mut app.runner, Box::new(run_once));
```

`runner` contains the _original_ runner, which was maybe set by the user. Why are we rewriting the memory in that location with the default `run_once` runner?

Maybe these are questions for another time.

---

So this original example

```rust
use bevy::prelude::*;

fn main() {
    App::new().add_systems(Update, hello_world_system).run();
}

fn hello_world_system() {
    println!("hello world");
}
```

is much more complex than it seems. We know that `App::new` actually initializes an `App::default`, which is a specialization of `App::empty`. The `App` contains an `AppTypeRegistry` resource for reflection within Bevy. We add the `SingleThreaded` `MainSchedule` as well as the `RunFixedUpdateLoop` schedule, listen for `AppExit` events in the `First` sub-schedule of `Main`, and add our `hello_world_system`, which is converted from a plain Rust function into the appropriate type through `Into` traits and macros. We `run` the `App` using the default `run_once` runner, and the app exits when an `AppExit` event is seen, cleaning up by calling `Drop` implementations in an idiomatic way.

So much complexity is hidden behind this simple "hello world" example. I hope to dig into more of this in the coming weeks, and understand all of this a bit better.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).

## All Katas

1. [Hello, Bevy!](https://github.com/awwsmm/daily-bevy)
2. [File Drag and Drop](https://github.com/awwsmm/daily-bevy/tree/app/drag_and_drop)
3. [Keyboard Input](https://github.com/awwsmm/daily-bevy/tree/input/keyboard_input)
4. [Clear Color](https://github.com/awwsmm/daily-bevy/tree/window/clear_color)
5. [Camera2dBundle](https://github.com/awwsmm/daily-bevy/tree/bonus/Camera2dBundle) (bonus!)
6. [Camera2dBundle 2](https://github.com/awwsmm/daily-bevy/tree/bonus/Camera2dBundle_2) (bonus!)
7. [Camera2dBundle 3](https://github.com/awwsmm/daily-bevy/tree/bonus/Camera2dBundle_3) (bonus!)
8. [Text 2D](https://github.com/awwsmm/daily-bevy/tree/2d/text2d)
9. [3D Shapes](https://github.com/awwsmm/daily-bevy/tree/3d/3d_shapes)
10. [Button](https://github.com/awwsmm/daily-bevy/tree/ui/button)
11. [WASM](https://github.com/awwsmm/daily-bevy/tree/bonus/WASM) (bonus!)
12. [Asset Loading](https://github.com/awwsmm/daily-bevy/tree/asset/asset_loading)
13. [Scene](https://github.com/awwsmm/daily-bevy/tree/scene/scene)
14. [Reflection](https://github.com/awwsmm/daily-bevy/tree/reflection/reflection)
15. [Game Menu Part 1](https://github.com/awwsmm/daily-bevy/tree/games/game_menu)
16. [Game Menu Part 2](https://github.com/awwsmm/daily-bevy/tree/games/game_menu_2)
17. [Game Menu Part 3](https://github.com/awwsmm/daily-bevy/tree/games/game_menu_3)
18. [v0.13.0](https://github.com/awwsmm/daily-bevy/tree/bonus/v0.13.0) (bonus!)
19. [WASM Persistence](https://github.com/awwsmm/daily-bevy/tree/bonus/WASM_Persistence) (bonus!)
20. [2D Gizmos](https://github.com/awwsmm/daily-bevy/tree/2d/2d_gizmos)
21. [2D Viewport to World](https://github.com/awwsmm/daily-bevy/tree/2d/2d_viewport_to_world)
22. [Low-Power Windows](https://github.com/awwsmm/daily-bevy/tree/window/low_power)
23. [Sprite Sheet](https://github.com/awwsmm/daily-bevy/tree/2d/sprite_sheet)
24. [Bounding Box 2D](https://github.com/awwsmm/daily-bevy/tree/2d/bounding_2d)

_...more coming soon!_

### putting the above into practice

After katas 1-20, above, I was able to build [this Tic-Tac-Toe game](https://github.com/awwsmm/tic-tac-toe).