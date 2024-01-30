# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the third entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Keyboard Input

Today is the third day of Daily Bevy.

### Today's Kata

Today, I will be dissecting the [keyboard input](https://github.com/bevyengine/bevy/blob/main/examples/input/keyboard_input.rs) example found in the Bevy repo.

#### The Code

Here's the `main.rs` I started with, which is the `keyboard_input` example on the Bevy `master` branch, as of today

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_input_system)
        .run();
}

/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        info!("'A' just pressed");
    }
    if keyboard_input.just_released(KeyCode::KeyA) {
        info!("'A' just released");
    }
}
```

Here's the `Cargo.toml` I started with

```toml
[dependencies]
bevy = "0.12.1"
```

#### Discussion

Today is the first day where I copied-and-pasted the example from the Bevy repo and it didn't work with my minimal `Cargo.toml` -- `ButtonInput` and `KeyA` couldn't be found. What gives?

I tried bringing them in from `bevy::input`, in the `bevy` crate, then I tried adding `bevy_input` to my `Cargo.toml`, but couldn't get either of these approaches to work.

[`ButtonInput` doesn't exist in the `bevy_input` crate?](https://docs.rs/bevy_input/0.12.1/bevy_input/?search=ButtonInput) It does, though, it's [right here](https://github.com/bevyengine/bevy/blob/d7c65e40ee633d1feee1ce36df92ccea5e161807/crates/bevy_input/src/button_input.rs#L48).

Cloning the Bevy repo and running this example _does_ work, so I just need to figure out where that type is getting exported. Eventually I got tired of opening documentation and searching for `ButtonInput` in crate after crate after crate...

I brought every Bevy crate into my dependencies

```rust
[dependencies]
bevy = "0.12.1"
bevy_a11y = "0.12.1"
bevy_animation = "0.12.1"
bevy_app = "0.12.1"
bevy_asset = "0.12.1"
bevy_audio = "0.12.1"
bevy_core = "0.12.1"
bevy_core_pipeline = "0.12.1"
bevy_derive = "0.12.1"
bevy_diagnostic = "0.12.1"
bevy_dylib = "0.12.1"
bevy_dynamic_plugin = "0.12.1"
bevy_ecs = "0.12.1"
bevy_encase_derive = "0.12.1"
bevy_gilrs = "0.12.1"
bevy_gizmos = "0.12.1"
bevy_gltf = "0.12.1"
bevy_hierarchy = "0.12.1"
bevy_input = "0.12.1"
bevy_internal = "0.12.1"
bevy_log = "0.12.1"
bevy_macro_utils = "0.12.1"
bevy_math = "0.12.1"
bevy_mikktspace = "0.12.1"
bevy_pbr = "0.12.1"
bevy_ptr = "0.12.1"
bevy_reflect = "0.12.1"
bevy_render = "0.12.1"
bevy_scene = "0.12.1"
bevy_sprite = "0.12.1"
bevy_tasks = "0.12.1"
bevy_text = "0.12.1"
bevy_time = "0.12.1"
bevy_transform = "0.12.1"
bevy_ui = "0.12.1"
bevy_utils = "0.12.1"
bevy_window = "0.12.1"
bevy_winit = "0.12.1"
```

My IDE _still_ couldn't find where these types were defined. I [searched the Bevy cheatbook](https://bevy-cheatbook.github.io/2d/camera.html?search=buttoninput), still nothing.

I began to wonder if the example in the repo is _newer_ than the code in release `v0.12.1`... what does the example look like at that tag in the repo? Sure enough, [yep, it's different](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/input/keyboard_input.rs).

So **an important point for newbies to Bevy**: don't use the examples in `master` on the repo. Instead, use the examples _at the specific tag of the version you're bringing into your project_.

At `v0.12.1`, the `main.rs` looks like

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_input_system)
        .run();
}

/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}
```

This worked immediately, even with just a minimal `Cargo.toml`

```toml
[dependencies]
bevy = "0.12.1"
```

Going forward, I'll try to make sure that all of my links to the Bevy repo point to the specific version of Bevy I'm using, and not `master`.

---

So where were we?

Right, learning Bevy. Let's go line-by-line through `main.rs`

```rust
App::new()
```

We've seen this before, creating a new `App`. Let's dig into this just a little bit more today, though.

`App::new()` calls `App::default()` which calls `App::empty()`, as we've seen before. But `App::empty()` also creates a `World`, which we've seen in passing before, but haven't really dug into.

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

A `World` looks like this

```rust
/// Stores and exposes operations on [entities](Entity), [components](Component), resources,
/// and their associated metadata.
///
/// Each [`Entity`] has a set of components. Each component can have up to one instance of each
/// component type. Entity components can be created, updated, removed, and queried using a given
/// [`World`].
///
/// For complex access patterns involving [`SystemParam`](crate::system::SystemParam),
/// consider using [`SystemState`](crate::system::SystemState).
///
/// To mutate different parts of the world simultaneously,
/// use [`World::resource_scope`] or [`SystemState`](crate::system::SystemState).
///
/// ## Resources
///
/// Worlds can also store [`Resource`]s,
/// which are unique instances of a given type that don't belong to a specific Entity.
/// There are also *non send resources*, which can only be accessed on the main thread.
/// See [`Resource`] for usage.
pub struct World {
    id: WorldId,
    pub(crate) entities: Entities,
    pub(crate) components: Components,
    pub(crate) archetypes: Archetypes,
    pub(crate) storages: Storages,
    pub(crate) bundles: Bundles,
    pub(crate) removed_components: RemovedComponentEvents,
    /// Access cache used by [`WorldCell`]. Is only accessed in the `Drop` impl of `WorldCell`.
    pub(crate) archetype_component_access: ArchetypeComponentAccess,
    pub(crate) change_tick: AtomicU32,
    pub(crate) last_change_tick: Tick,
    pub(crate) last_check_tick: Tick,
}
```

A `World` contains a `WorldId` -- so it's not a singleton; we could have _multiple_ `World`s in a single `App`. Maybe we'll use that in a later kata.

A `World` also contains `Entities` and `Components` (two thirds of ["ECS"](https://en.wikipedia.org/wiki/Entity_component_system)) and a bunch of other stuff we haven't seen yet. (`removed_components`? That sounds interesting...) One thing I'll call out (because I've looked ahead at where this example is going) are the `tick`s at the end.

So we create a `World` in `App::empty()` with `World::new()`, and add a default `Schedules` resource (`init_resource` uses the `Default` implementation of a type, if no `FromWorld` implementation exists for that same type)

```rust
/// Resource that stores [`Schedule`]s mapped to [`ScheduleLabel`]s.
#[derive(Default, Resource)]
pub struct Schedules {
    inner: HashMap<InternedScheduleLabel, Schedule>,
    /// List of [`ComponentId`]s to ignore when reporting system order ambiguity conflicts
    pub ignored_scheduling_ambiguities: BTreeSet<ComponentId>,
}

#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
struct DefaultSchedule;

impl Default for Schedule {
    /// Creates a schedule with a default label. Only use in situations where
    /// you don't care about the [`ScheduleLabel`]. Inserting a default schedule
    /// into the world risks overwriting another schedule. For most situations
    /// you should use [`Schedule::new`].
    fn default() -> Self {
        Self::new(DefaultSchedule)
    }
}
```

Ignoring the `ignored_scheduling_ambiguities` for now, a `Schedules` is just a map of labels to `Schedule`s. Pretty simple.

---

After `App::new()`, we `.add_plugins(DefaultPlugins)`. We've also seen this before.

One of the `DefaultPlugins` which we will probably use in this kata is the `bevy_input::InputPlugin`. We will also use the `bevy_window::WindowPlugin`. Anything else you think we might need?

---

Finally, we get to the meat of this example: `.add_systems(Update, keyboard_input_system)`.

`keyboard_input_system` is defined in `main.rs` as follows

```rust
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}
```

Yesterday, we had an `EventReader` that let us handle `FileDragAndDrop` events. Recall yesterday's system looked like

```rust
fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>) {
    for event in events.read() {
        info!("{:?}", event);
    }
}
```

Today, however, we've got a totally different signature. Our single argument (`keyboard_input`) is _not_ mutable, and it's of quite a different type -- `Res<Input<KeyCode>>`, rather than something like `EventReader<Keyboard>`. So what is all this stuff?

`Res` is "a shared borrow of a resource"...

```rust
/// Shared borrow of a [`Resource`].
///
/// See the [`Resource`] documentation for usage.
///
/// If you need a unique mutable borrow, use [`ResMut`] instead.
///
/// # Panics
///
/// Panics when used as a [`SystemParameter`](crate::system::SystemParam) if the resource does not exist.
///
/// Use `Option<Res<T>>` instead if the resource might not always exist.
pub struct Res<'w, T: ?Sized + Resource> {
    pub(crate) value: &'w T,
    pub(crate) ticks: Ticks<'w>,
}
```

...so something like an `Rc`. Note that we've got a `ticks: Ticks` field here. `Ticks` is defined in a file called `change_detection` in the `bevy_ecs` crate

```rust
#[derive(Clone)]
pub(crate) struct Ticks<'a> {
    pub(crate) added: &'a Tick,
    pub(crate) changed: &'a Tick,
    pub(crate) last_run: Tick,
    pub(crate) this_run: Tick,
}
```

...it holds a bunch of `Tick`s. What's a `Tick`?

```rust
/// A value that tracks when a system ran relative to other systems.
/// This is used to power change detection.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tick {
    tick: u32,
}
```

There's not a lot of documentation in the crate around this. It's a pretty fundamental concept in gamedev. Here's what [the Unofficial Bevy Cheatbook](https://bevy-cheatbook.github.io/fundamentals/time.html) has to say about all of this.

So anyway, `Res` wraps a value of type `T` (itself a `Resource`) with some lifetime `'w`. `Resource` is an implementationless trait defined as follows

```rust
pub trait Resource: Send + Sync + 'static {}
```

`Resource`s
- can be `#[derive(...)]`d
- are singletons
- are `Send` and `Sync` (except when they're not)

Here's the documentation above `Resource`

```rust
/// A type that can be inserted into a [`World`] as a singleton.
///
/// You can access resource data in systems using the [`Res`] and [`ResMut`] system parameters
///
/// Only one resource of each type can be stored in a [`World`] at any given time.
///
/// # Examples
///
/// ```
/// # let mut world = World::default();
/// # let mut schedule = Schedule::default();
/// # use bevy_ecs::prelude::*;
/// #[derive(Resource)]
/// struct MyResource { value: u32 }
///
/// world.insert_resource(MyResource { value: 42 });
///
/// fn read_resource_system(resource: Res<MyResource>) {
///     assert_eq!(resource.value, 42);
/// }
///
/// fn write_resource_system(mut resource: ResMut<MyResource>) {
///     assert_eq!(resource.value, 42);
///     resource.value = 0;
///     assert_eq!(resource.value, 0);
/// }
/// # schedule.add_systems((read_resource_system, write_resource_system).chain());
/// # schedule.run(&mut world);
/// ```
///
/// # `!Sync` Resources
/// A `!Sync` type cannot implement `Resource`. However, it is possible to wrap a `Send` but not `Sync`
/// type in [`SyncCell`] or the currently unstable [`Exclusive`] to make it `Sync`. This forces only
/// having mutable access (`&mut T` only, never `&T`), but makes it safe to reference across multiple
/// threads.
///
/// This will fail to compile since `RefCell` is `!Sync`.
/// ```compile_fail
/// # use std::cell::RefCell;
/// # use bevy_ecs::system::Resource;
///
/// #[derive(Resource)]
/// struct NotSync {
///    counter: RefCell<usize>,
/// }
/// ```
///
/// This will compile since the `RefCell` is wrapped with `SyncCell`.
/// ```
/// # use std::cell::RefCell;
/// # use bevy_ecs::system::Resource;
/// use bevy_utils::synccell::SyncCell;
///
/// #[derive(Resource)]
/// struct ActuallySync {
///    counter: SyncCell<RefCell<usize>>,
/// }
/// ```
///
/// [`SyncCell`]: bevy_utils::synccell::SyncCell
/// [`Exclusive`]: https://doc.rust-lang.org/nightly/std/sync/struct.Exclusive.html
```

So `Res<T>` is "access[ing] resource data in systems using the [`Res`]... system parameter".

`Input` is a `Resource` and it looks like this

```rust
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Default)]
pub struct Input<T: Copy + Eq + Hash + Send + Sync + 'static> {
    /// A collection of every button that is currently being pressed.
    pressed: HashSet<T>,
    /// A collection of every button that has just been pressed.
    just_pressed: HashSet<T>,
    /// A collection of every button that has just been released.
    just_released: HashSet<T>,
}
```

The name "input" seems a bit too general here, which is maybe why this has been renamed to `ButtonInput` on `master`.

Every button is in one of four states
1. unpressed (no input information)
2. `just_pressed`: transitioning from unpressed to `pressed`
3. `pressed` (currently held down)
4. `just_released`: transitioning from `pressed` to unpressed

You can see this when you run this example. If you press the `A` key, you will see `just_pressed` fire just one time, then `pressed` will fire continually as long as you are holding down the key, then `just_relased` will fire when you release the key.

`Input` is generic and so theoretically there is no limit to the number of kinds of `Input` that Bevy can process. Practically, there are four. The ones defined in the `InputPlugin`: `KeyCode`s, `ScanCode`s, `MouseButton`s, and `GamepadButton`s

```rust
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            // keyboard
            .add_event::<KeyboardInput>()
            .init_resource::<Input<KeyCode>>()
            .init_resource::<Input<ScanCode>>()
            .add_systems(PreUpdate, keyboard_input_system.in_set(InputSystem))
            // mouse
            .add_event::<MouseButtonInput>()
            .add_event::<MouseMotion>()
            .add_event::<MouseWheel>()
            .init_resource::<Input<MouseButton>>()
            // -- snip --
            .add_event::<TouchpadMagnify>()
            .add_event::<TouchpadRotate>()
            // gamepad
            .add_event::<GamepadConnectionEvent>()
            .add_event::<GamepadButtonChangedEvent>()
            .add_event::<GamepadButtonInput>()
            .add_event::<GamepadAxisChangedEvent>()
            .add_event::<GamepadEvent>()
            .add_event::<GamepadRumbleRequest>()
            // -- snip --
            .init_resource::<Input<GamepadButton>>()
            // -- snip --
            // touch
            .add_event::<TouchInput>()
        // -- snip --
    }
}
```

Note that there are also a bunch of `Event`s as well as `Input`s. We'll cover `Event`s in later katas.

`KeyCode`s are the `Input` we're interested in. These are human-readable keyboard key identifiers.

[`ScanCode`s are like machine-specific `KeyCode`s](https://en.wikipedia.org/wiki/Scancode). They map key presses on your keyboard to some series of bytes.

`MouseButton`s are pretty simple; `GamepadButton`s are gamepad-specific. I'm sure we'll cover these in later katas. For now, let's focus on `KeyCode`s. Here's an abridged list of them

```rust
pub enum KeyCode {
    /// The `1` key over the letters.
    Key1,
    // -- snip --
    Key9,
    /// The `0` key over the letters.
    Key0,

    /// The `A` key.
    A,
    // -- snip --
    /// The `Z` key.
    Z,

    /// The `Escape` / `ESC` key, next to the `F1` key.
    Escape,

    /// The `F1` key.
    F1,
    // -- snip --
    /// The `F24` key.
    F24,

    /// The `Snapshot` / `Print Screen` key.
    Snapshot,
    /// The `Scroll` / `Scroll Lock` key.
    Scroll,
    /// The `Pause` / `Break` key, next to the `Scroll` key.
    Pause,

    /// The `Insert` key, next to the `Backspace` key.
    Insert,
    /// The `Home` key.
    Home,
    /// The `Delete` key.
    Delete,
    /// The `End` key.
    End,
    /// The `PageDown` key.
    PageDown,
    /// The `PageUp` key.
    PageUp,

    /// The `Left` / `Left Arrow` key.
    Left,
    /// The `Up` / `Up Arrow` key.
    Up,
    /// The `Right` / `Right Arrow` key.
    Right,
    /// The `Down` / `Down Arrow` key.
    Down,

    /// The `Back` / `Backspace` key.
    Back,
    /// The `Return` / `Enter` key.
    Return,
    /// The `Space` / `Spacebar` / ` ` key.
    Space,

    /// The `Compose` key on Linux.
    Compose,
    /// The `Caret` / `^` key.
    Caret,

    /// The `Numlock` key.
    Numlock,
    /// The `Numpad0` / `0` key.
    Numpad0,
    // -- snip --
    Numpad9,

    /// The `AbntC1` key.
    AbntC1,
    /// The `AbntC2` key.
    AbntC2,

    /// The `NumpadAdd` / `+` key.
    NumpadAdd,
    /// The `Apostrophe` / `'` key.
    Apostrophe,
    /// The `Apps` key.
    Apps,
    /// The `Asterisk` / `*` key.
    Asterisk,
    /// The `Plus` / `+` key.
    Plus,
    /// The `At` / `@` key.
    At,
    /// The `Ax` key.
    Ax,
    /// The `Backslash` / `\` key.
    Backslash,
    /// The `Calculator` key.
    Calculator,
    /// The `Capital` key.
    Capital,
    /// The `Colon` / `:` key.
    Colon,
    /// The `Comma` / `,` key.
    Comma,
    /// The `Convert` key.
    Convert,
    /// The `NumpadDecimal` / `.` key.
    NumpadDecimal,
    /// The `NumpadDivide` / `/` key.
    NumpadDivide,
    /// The `Equals` / `=` key.
    Equals,
    /// The `Grave` / `Backtick` / `` ` `` key.
    Grave,
    /// The `Kana` key.
    Kana,
    /// The `Kanji` key.
    Kanji,

    /// The `Left Alt` key. Maps to `Left Option` on Mac.
    AltLeft,
    /// The `Left Bracket` / `[` key.
    BracketLeft,
    /// The `Left Control` key.
    ControlLeft,
    /// The `Left Shift` key.
    ShiftLeft,
    /// The `Left Super` key.
    /// Generic keyboards usually display this key with the *Microsoft Windows* logo.
    /// Apple keyboards call this key the *Command Key* and display it using the ⌘ character.
    #[doc(alias("LWin", "LMeta", "LLogo"))]
    SuperLeft,

    /// The `Mail` key.
    Mail,
    /// The `MediaSelect` key.
    MediaSelect,
    /// The `MediaStop` key.
    MediaStop,
    /// The `Minus` / `-` key.
    Minus,
    /// The `NumpadMultiply` / `*` key.
    NumpadMultiply,
    /// The `Mute` key.
    Mute,
    /// The `MyComputer` key.
    MyComputer,
    /// The `NavigateForward` / `Prior` key.
    NavigateForward,
    /// The `NavigateBackward` / `Next` key.
    NavigateBackward,
    /// The `NextTrack` key.
    NextTrack,
    /// The `NoConvert` key.
    NoConvert,
    /// The `NumpadComma` / `,` key.
    NumpadComma,
    /// The `NumpadEnter` key.
    NumpadEnter,
    /// The `NumpadEquals` / `=` key.
    NumpadEquals,
    /// The `Oem102` key.
    Oem102,
    /// The `Period` / `.` key.
    Period,
    /// The `PlayPause` key.
    PlayPause,
    /// The `Power` key.
    Power,
    /// The `PrevTrack` key.
    PrevTrack,

    /// The `Right Alt` key. Maps to `Right Option` on Mac.
    AltRight,
    /// The `Right Bracket` / `]` key.
    BracketRight,
    /// The `Right Control` key.
    ControlRight,
    /// The `Right Shift` key.
    ShiftRight,
    /// The `Right Super` key.
    /// Generic keyboards usually display this key with the *Microsoft Windows* logo.
    /// Apple keyboards call this key the *Command Key* and display it using the ⌘ character.
    #[doc(alias("RWin", "RMeta", "RLogo"))]
    SuperRight,

    /// The `Semicolon` / `;` key.
    Semicolon,
    /// The `Slash` / `/` key.
    Slash,
    /// The `Sleep` key.
    Sleep,
    /// The `Stop` key.
    Stop,
    /// The `NumpadSubtract` / `-` key.
    NumpadSubtract,
    /// The `Sysrq` key.
    Sysrq,
    /// The `Tab` / `   ` key.
    Tab,
    /// The `Underline` / `_` key.
    Underline,
    /// The `Unlabeled` key.
    Unlabeled,

    /// The `VolumeDown` key.
    VolumeDown,
    /// The `VolumeUp` key.
    VolumeUp,

    /// The `Wake` key.
    Wake,

    /// The `WebBack` key.
    WebBack,
    /// The `WebFavorites` key.
    WebFavorites,
    /// The `WebForward` key.
    WebForward,
    /// The `WebHome` key.
    WebHome,
    /// The `WebRefresh` key.
    WebRefresh,
    /// The `WebSearch` key.
    WebSearch,
    /// The `WebStop` key.
    WebStop,

    /// The `Yen` key.
    Yen,

    /// The `Copy` key.
    Copy,
    /// The `Paste` key.
    Paste,
    /// The `Cut` key.
    Cut,
}
```

That's a lotta keys. Note that OS-specific, language-specific, and hardware-specific keys are included, like the `⌘` MacBook key, the [Linux Compose key `⎄`](https://en.wikipedia.org/wiki/Compose_key), the [Kana and Kanji keys](https://en.wikipedia.org/wiki/Language_input_keys) and others.

Remember that some of these have been renamed between Bevy `v0.12.1` and `master`, so they may be different in the later releases.

---

The only thing left to discuss is how the information gets "into" the `Input` argument that we're reading from. As always in Bevy, it's a system

```rust
/// Updates the [`Input<KeyCode>`] resource with the latest [`KeyboardInput`] events.
///
/// ## Differences
///
/// The main difference between the [`KeyboardInput`] event and the [`Input<KeyCode>`] or [`Input<ScanCode>`] resources is that
/// the latter have convenient functions such as [`Input::pressed`], [`Input::just_pressed`] and [`Input::just_released`].
pub fn keyboard_input_system(
    mut scan_input: ResMut<Input<ScanCode>>,
    mut key_input: ResMut<Input<KeyCode>>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    // Avoid clearing if it's not empty to ensure change detection is not triggered.
    scan_input.bypass_change_detection().clear();
    key_input.bypass_change_detection().clear();
    for event in keyboard_input_events.read() {
        let KeyboardInput {
            scan_code, state, ..
        } = event;
        if let Some(key_code) = event.key_code {
            match state {
                ButtonState::Pressed => key_input.press(key_code),
                ButtonState::Released => key_input.release(key_code),
            }
        }
        match state {
            ButtonState::Pressed => scan_input.press(ScanCode(*scan_code)),
            ButtonState::Released => scan_input.release(ScanCode(*scan_code)),
        }
    }
}
```

The `keyboard_input_system` is added to the `InputPlugin`, which you can see if you scroll up a bit here.

And how does `keyboard_input_system` work? Another `EventReader` (`keyboard_input_events`)! Don't you love when things come full-circle like that?

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).