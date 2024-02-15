# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the fifteenth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Game Menu

Today is the fifteenth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today we're looking at the [`game_menu`](https://github.com/bevyengine/bevy/blob/release-0.12.1/examples/games/game_menu.rs) example from the Bevy repo.

#### The Code

The `main.rs` for this example is very long, so I won't reproduce it here. You can see it [here](src/main.rs).

Here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.12.1"
```

We also need a few `.png` assets
- `assets/branding/icon.png`
- `assets/textures/Game Icons/right.png`
- `assets/textures/Game Icons/wrench.png`
- `assets/textures/Game Icons/exitRight.png`

#### Discussion

This is the longest example we've explored so far, at over 800 lines of code. So we won't be able to dig into too many details, but let's at least understand what this example is doing and how it's doing it. We can explore the details in later katas.

Running this example, you can see that there...
- is a splash screen
- is a menu where we can play a game, change settings or quit
- is a game which just shows the current settings for a few seconds, then quits
- is a settings menu where we can change the volume and display quality
- is navigation between the different menus

As the comment at the top of the example explains

```rust
//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.
```

Let's explore all the items in the outermost scope first.

---

```rust
// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]
```

We've seen this `allow` before, in [the Button example](https://github.com/awwsmm/daily-bevy/tree/ui/button). What breaks if we get rid of this?

...nothing. It looks like this was a copy-and-paste from another example, or maybe these types have been rearranged since this example was first written.

---

Above `main`, we've got a few more pieces which we more or less fully understand at this point

```rust
use bevy::prelude::*;
```

We bring in everything from the `prelude`, rather than importing things one at a time.

```rust
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
```

We define an [RGB text color](https://en.wikipedia.org/wiki/RGB_color_model), which we'll use throughout the app.

```rust
// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);
```

And we have two `Resource`s (which are also `Component`s). One of these (`DisplayQuality`) is an `enum`, while the other (`Volume`) is a tuple `struct`.

The last item above `main`, `GameState`, needs a bit more explanation.

---

```rust
// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}
```

What is that `States` trait?

```rust
/// Types that can define world-wide states in a finite-state machine.
///
/// The [`Default`] trait defines the starting state.
/// Multiple states can be defined for the same world,
/// allowing you to classify the state of the world across orthogonal dimensions.
/// You can access the current state of type `T` with the [`State<T>`] resource,
/// and the queued state with the [`NextState<T>`] resource.
///
/// State transitions typically occur in the [`OnEnter<T::Variant>`] and [`OnExit<T:Variant>`] schedules,
/// which can be run via the [`apply_state_transition::<T>`] system.
///
/// # Example
///
/// ```rust
/// use bevy_ecs::prelude::States;
///
/// #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
/// enum GameState {
///  #[default]
///   MainMenu,
///   SettingsMenu,
///   InGame,
/// }
///
/// ```
pub trait States: 'static + Send + Sync + Clone + PartialEq + Eq + Hash + Debug + Default {}
```

`States` is just a marker trait, but the additional functionality in Bevy _around_ `States` lets us build [finite state machines (FSMs)](https://en.wikipedia.org/wiki/Finite-state_machine).

The _current_ `State` is a `Resource`, and therefore a singleton, which makes sense

```rust
/// A finite-state machine whose transitions have associated schedules
/// ([`OnEnter(state)`] and [`OnExit(state)`]).
///
/// The current state value can be accessed through this resource. To *change* the state,
/// queue a transition in the [`NextState<S>`] resource, and it will be applied by the next
/// [`apply_state_transition::<S>`] system.
///
/// The starting state is defined via the [`Default`] implementation for `S`.
#[derive(Resource, Default, Debug)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Resource, Default)
)]
pub struct State<S: States>(S);
```

If we want to transition to a _new_ `State`, we set `NextState` (also a `Resource`) to a non-`None` value

```rust
/// The next state of [`State<S>`].
///
/// To queue a transition, just set the contained value to `Some(next_state)`.
/// Note that these transitions can be overridden by other systems:
/// only the actual value of this resource at the time of [`apply_state_transition`] matters.
#[derive(Resource, Default, Debug)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Resource, Default)
)]
pub struct NextState<S: States>(pub Option<S>);
```

There are also `Schedule`s for state transitions

- the `OnEnter` schedule can be used to run systems when a particular state is entered
- the `OnExit` schedule does the same, but when a particular state is exited
- the `OnTransition` schedule is a struct with `from` and `to` fields and only runs when the FSM moves directly from the `from` state to the `to` state

How do we run a system _while we are in_ some particular state? We'll see how to do this in a bit.

In this example, `enum GameState` contains all possible game states: `Splash` (the splash screen, the default state), `Menu` (the menu screen), and `Game` (the actual game).

If you execute this example, you'll see that there is a "main" menu and also "sub" menus for the volume and display quality settings. How is that handled? Let's continue and find out.

---

Next up: `main`

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
        .run();
}
```

`DisplayQuality` and `Volume` are `Resource`s, so we must insert them into the `World` with some initial values. If we forget to do this, the app will run initially, but panic when we try to access these resources

```
thread 'Compute Task Pool (3)' panicked at /Users/andrew/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.12.1/src/system/system_param.rs:451:17:
Resource requested by daily_bevy::menu::sound_settings_menu_setup does not exist: daily_bevy::Volume
```

---

Next, we call `add_state`, which we haven't yet done in any kata. Here is `add_state` in its entirety

```rust
/// Adds [`State<S>`] and [`NextState<S>`] resources, [`OnEnter`] and [`OnExit`] schedules
/// for each state variant (if they don't already exist), an instance of [`apply_state_transition::<S>`] in
/// [`StateTransition`] so that transitions happen before [`Update`](crate::Update) and
/// a instance of [`run_enter_schedule::<S>`] in [`StateTransition`] with a
/// [`run_once`](`run_once_condition`) condition to run the on enter schedule of the
/// initial state.
///
/// If you would like to control how other systems run based on the current state,
/// you can emulate this behavior using the [`in_state`] [`Condition`].
///
/// Note that you can also apply state transitions at other points in the schedule
/// by adding the [`apply_state_transition`] system manually.
pub fn add_state<S: States>(&mut self) -> &mut Self {
    self.init_resource::<State<S>>()
        .init_resource::<NextState<S>>()
        .add_systems(
            StateTransition,
            (
                run_enter_schedule::<S>.run_if(run_once_condition()),
                apply_state_transition::<S>,
            )
                .chain(),
        );

    // The OnEnter, OnExit, and OnTransition schedules are lazily initialized
    // (i.e. when the first system is added to them), and World::try_run_schedule is used to fail
    // gracefully if they aren't present.

    self
}
```

So we

- `init`ialize the `Default` `State<S>`
- `init`ialize the `NextState<S>` (as `None`)
- add all the state transition logic

The last point is a bit opaque. Remember that even though `S` implements a trait called "`States`", `s: S` is just a _particular_ state, and not some collection of _all_ states. But `add_state` is initializing `State<S>` and `NextState<S>` which are singletons for all `S: States`. The singular / plural language could maybe use a bit of tweaking here.

We then add the `run_enter_schedule::<S>` and `apply_state_transition::<S>` systems to the `StateTransition` `Schedule`. Recall the steps of the default `Main` schedule

```rust
/// The schedule that contains the app logic that is evaluated each tick of [`App::update()`].
///
/// By default, it will run the following schedules in the given order:
///
/// On the first run of the schedule (and only on the first run), it will run:
/// * [`PreStartup`]
/// * [`Startup`]
/// * [`PostStartup`]
///
/// Then it will run:
/// * [`First`]
/// * [`PreUpdate`]
/// * [`StateTransition`]
/// * [`RunFixedUpdateLoop`]
///     * This will run [`FixedUpdate`] zero to many times, based on how much time has elapsed.
/// * [`Update`]
/// * [`PostUpdate`]
/// * [`Last`]
```

`StateTransition` runs after `PreUpdate` but before `RunFixedUpdateLoop` and `Update`.

So in the `StateTransition` `Schedule`, we run these two systems, `run_enter_schedule::<S>` and `apply_state_transition::<S>`.

`run_enter_schedule` will run the `OnEnter` `Schedule` for the current state, `S`, if that schedule is defined

```rust
/// Run the enter schedule (if it exists) for the current state.
pub fn run_enter_schedule<S: States>(world: &mut World) {
    world
        .try_run_schedule(OnEnter(world.resource::<State<S>>().0.clone()))
        .ok();
}
```

...but only if it hasn't already been run. That's what `.run_if(run_once_condition())` takes care of

```rust
pub fn run_once() -> impl FnMut() -> bool + Clone {
    let mut has_run = false;
    move || {
        if !has_run {
            has_run = true;
            true
        } else {
            false
        }
    }
}
```

> "But", you may be saying, "this is called `run_once()` and the code above calls `run_once_condition()`".
> 
> Yep, that's because this function is aliased where it is imported
> 
> ```rust
> use bevy_ecs::{
>     prelude::*,
>     schedule::{
>         apply_state_transition, common_conditions::run_once as run_once_condition,
>         // -- snip --
>     },
> };
> ```

Finally, `apply_state_transition` will execute, but only if `NextState` is not `None`

```rust
/// If a new state is queued in [`NextState<S>`], this system:
/// - Takes the new state value from [`NextState<S>`] and updates [`State<S>`].
/// - Runs the [`OnExit(exited_state)`] schedule, if it exists.
/// - Runs the [`OnTransition { from: exited_state, to: entered_state }`](OnTransition), if it exists.
/// - Runs the [`OnEnter(entered_state)`] schedule, if it exists.
pub fn apply_state_transition<S: States>(world: &mut World) {
    // We want to take the `NextState` resource,
    // but only mark it as changed if it wasn't empty.
    let mut next_state_resource = world.resource_mut::<NextState<S>>();
    if let Some(entered) = next_state_resource.bypass_change_detection().0.take() {
        next_state_resource.set_changed();

        let mut state_resource = world.resource_mut::<State<S>>();
        if *state_resource != entered {
            let exited = mem::replace(&mut state_resource.0, entered.clone());
            // Try to run the schedules if they exist.
            world.try_run_schedule(OnExit(exited.clone())).ok();
            world
                .try_run_schedule(OnTransition {
                    from: exited,
                    to: entered.clone(),
                })
                .ok();
            world.try_run_schedule(OnEnter(entered)).ok();
        }
    }
}
```

Note also the `.chain()` in `add_state()`. `run_enter_schedule` and `apply_state_transition` are both run in the `StateTransition` `Schedule`, and `.chain` ensures that they will always be run _in that order_: `run_enter_schedule` first, `apply_state_transition` second. This is accomplished by setting `chained` to `true` in `SystemSetConfigs`, which is just a particular kind of `NodeConfigs`

```rust
/// A collection of [`SystemSetConfig`].
pub type SystemSetConfigs = NodeConfigs<InternedSystemSet>;
```

```rust
/// A collections of generic [`NodeConfig`]s.
pub enum NodeConfigs<T> {
    /// Configuration for a single node.
    NodeConfig(NodeConfig<T>),
    /// Configuration for a tuple of nested `Configs` instances.
    Configs {
        /// Configuration for each element of the tuple.
        configs: Vec<NodeConfigs<T>>,
        /// Run conditions applied to everything in the tuple.
        collective_conditions: Vec<BoxedCondition>,
        /// If `true`, adds `before -> after` ordering constraints between the successive elements.
        chained: bool,
    },
}
```

---

The only bits we haven't yet discussed from `main` are

- `.add_systems(Startup, setup)` -- we've seen this before, we're just adding a system to a `Schedule`
- `.add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))`
- and `.run()` -- we run the `App`

The second point above is a bit unusual: the plugins have module paths (e.g. `splash::SplashPlugin` instead of just `SplashPlugin`). Why is that?

It's just because that's how they're defined in `main.rs`. After the `main()` method, we've got a simple `setup` system...

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
```

...a system to despawn all entities in between state transitions (which is pretty self-explanatory, especially with the helpful comment above it)

```rust
// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
```

and three modules in between

```rust
mod splash {
    // -- snip --
}

mod game {
    // -- snip --
}

mod menu {
    // -- snip --
}
```

These three modules correspond to the three `Plugin`s added to the `App`. In a "real" Bevy application, you'd probably split these modules over multiple files, but (almost) all the examples in the Bevy repo are written as single-file examples, so we've got `mod`ule definitions in `main.rs`.

---

The first module, `splash`, shows a [splash screen](https://en.wikipedia.org/wiki/Splash_screen) when the `App` starts up

`SplashPlugin` is a unit struct with a `Plugin` `impl`ementation which explains its FSM state transitions

```rust
// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}
```

- when the `App` enters the `Splash` `GameState`, the `splash_setup` system is run
- while the `App` is in the `Splash` `GameState`, the `countdown` system is run over and over
- when the `App` exits the `Splash` `GameState`, all entities with an `OnSplashScreen` `Component` are despawned, using the `despawn_screen` system from the outer scope

The above snippet answers our question from earlier around running a system over and over when we are in a certain state: use the `system.run_if(in_state(state))` pattern. `in_state` is a `Condition` which can be used to filter when a system is run. There are many more predefined `Condition`s, and you can write your own custom ones, as well.

`OnSplashScreen` is a marker `Component` used only within the `splash` module...

```rust
// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;
```

...and `SplashTimer` is a `Resource` and a tuple struct which wraps a `bevy_time::timer::Timer`.

```rust
// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);
```

The `Timer` is used to ensure that the splash screen is visible for one second.

It seems like there's a lot going on in the `splash_setup`, but we've actually seen most of this already

```rust
fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/icon.png");
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}
```

We

- use an immutable reference to the `AssetServer` `Res`ource to `load` an asset. In this case, a `.png` icon.
- `spawn` a `NodeBundle` (which is just a UI node) with 100% width, 100% height, and all content centered vertically and horizontally
- tag the `NodeBundle` with the `OnSplashScreen` marker `Component`, so it can be despawned when this state is exited
- `spawn` an `ImageBundle` as a child of the UI node
- set the `width` of the `ImageBundle`s image to 200px
- set the `image` of the `ImageBundle` to the `icon` asset we loaded earlier

`UiImage` is a type we haven't seen before. It's very simple, though

```rust
/// The 2D texture displayed for this UI node
#[derive(Component, Clone, Debug, Reflect, Default)]
#[reflect(Component, Default)]
pub struct UiImage {
    /// Handle to the texture
    pub texture: Handle<Image>,
    /// Whether the image should be flipped along its x-axis
    pub flip_x: bool,
    /// Whether the image should be flipped along its y-axis
    pub flip_y: bool,
}
```

Finally, we add the `SplashTimer` `Resource` with a 1 second `Once` `Timer`. `Timer` looks like this

```rust
/// Tracks elapsed time. Enters the finished state once `duration` is reached.
///
/// Non repeating timers will stop tracking and stay in the finished state until reset.
/// Repeating timers will only be in the finished state on each tick `duration` is reached or
/// exceeded, and can still be reset at any given point.
///
/// Paused timers will not have elapsed time increased.
#[derive(Clone, Debug, Default, PartialEq, Eq, Reflect)]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize, serde::Serialize))]
#[reflect(Default)]
pub struct Timer {
    stopwatch: Stopwatch,
    duration: Duration,
    mode: TimerMode,
    finished: bool,
    times_finished_this_tick: u32,
}
```

Creating a `Timer` `from_seconds` means we set the `duration` and `TimerMode`, but leave all other fields at their `Default` values

```rust
/// Creates a new timer with a given duration in seconds.
///
/// # Example
/// ```
/// # use bevy_time::*;
/// let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
/// ```
pub fn from_seconds(duration: f32, mode: TimerMode) -> Self {
    Self {
        duration: Duration::from_secs_f32(duration),
        mode,
        ..Default::default()
    }
}
```

There are only two `TimerMode`s

```rust
/// Specifies [`Timer`] behavior.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, Reflect)]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize, serde::Serialize))]
#[reflect(Default)]
pub enum TimerMode {
    /// Run once and stop.
    #[default]
    Once,
    /// Reset when finished.
    Repeating,
}
```

The documentation above `Timer` explains the difference between these two modes.

How does the `Timer` get updated, though? This we have to do manually, in the `countdown` system

```rust
// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
```

- `time: Res<Time>` keeps track of in-`App` time
- `time.delta()` gives the amount of time elapsed since the last time the `Update` schedule was run
- `timer.tick(N)` advances the `timer` by `N` seconds
- `timer.finished()` returns `true` if the `timer` has reached or surpassed its `duration`

So the effect of all of the above is that `NextState` will be set to `GameState::Menu` after 1 second of in-`App` time has elapsed, and the FSM will move from the `Splash` state to the `Menu` state.

---

I think it's probably too much for one kata to dig into all of these `States` in one go. Now that we've got a good idea for how things work here, we should be able to get through the next two states tomorrow, but at ~130 and ~580 lines respectively, I think it's just a bit too much code to explore in one sitting.

Tomorrow, let's finish this example by digging into the `Menu` and `Game` states.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
