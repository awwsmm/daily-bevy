# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the second entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## File Drag and Drop

Today is the second day of Daily Bevy.

### Today's Kata

Today, I will be dissecting the [file drag and drop](https://github.com/bevyengine/bevy/blob/main/examples/app/drag_and_drop.rs) example found in the Bevy repo.

#### The Code

Here is the entirety of the `main.rs` file for this example, as of today

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, file_drag_and_drop_system)
        .run();
}

fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>) {
    for event in events.read() {
        info!("{:?}", event);
    }
}
```

This code requires only the base `bevy` crate in `Cargo.toml`

```toml
[dependencies]
bevy = "0.12.1"
```

#### Discussion

This example is -- on its face, at least -- quite similar to the [hello world](https://github.com/awwsmm/daily-bevy) example from yesterday.

We
- create an app with `App::new()`
- define and add a "system" to our app
- `run()` the app

But there are a few extra pieces
- we add some `DefaultPlugins` to our app
- and our "system" now takes an argument (it took no arguments yesterday)

---

Let's dive into `.add_plugins(DefaultPlugins)` first.

First of all, what is a `Plugin`? The `Plugin` `trait` is defined in the `bevy_app` crate

```rust
/// A collection of Bevy app logic and configuration.
///
/// Plugins configure an [`App`]. When an [`App`] registers a plugin,
/// the plugin's [`Plugin::build`] function is run. By default, a plugin
/// can only be added once to an [`App`].
///
/// If the plugin may need to be added twice or more, the function [`is_unique()`](Self::is_unique)
/// should be overridden to return `false`. Plugins are considered duplicate if they have the same
/// [`name()`](Self::name). The default `name()` implementation returns the type name, which means
/// generic plugins with different type parameters will not be considered duplicates.
///
/// ## Lifecycle of a plugin
///
/// When adding a plugin to an [`App`]:
/// * the app calls [`Plugin::build`] immediately, and register the plugin
/// * once the app started, it will wait for all registered [`Plugin::ready`] to return `true`
/// * it will then call all registered [`Plugin::finish`]
/// * and call all registered [`Plugin::cleanup`]
pub trait Plugin: Downcast + Any + Send + Sync {
    // -- snip --
}
```

Okay, so a `Plugin` is a "collection of Bevy app logic and configuration". What does that look like?

Here's a simple `Plugin` I found

```rust
/// Adds time functionality to Apps.
#[derive(Default)]
pub struct TimePlugin;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
/// Updates the elapsed time. Any system that interacts with [`Time`] component should run after
/// this.
pub struct TimeSystem;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Time>()
            .init_resource::<Time<Real>>()
            .init_resource::<Time<Virtual>>()
            .init_resource::<Time<Fixed>>()
            .init_resource::<TimeUpdateStrategy>()
            .register_type::<Time>()
            .register_type::<Time<Real>>()
            .register_type::<Time<Virtual>>()
            .register_type::<Time<Fixed>>()
            .register_type::<Timer>()
            .register_type::<Stopwatch>()
            .add_systems(
                First,
                (time_system, virtual_time_system.after(time_system)).in_set(TimeSystem),
            )
            .add_systems(RunFixedUpdateLoop, run_fixed_update_schedule);

        // ensure the events are not dropped until `FixedUpdate` systems can observe them
        app.init_resource::<EventUpdateSignal>()
            .add_systems(FixedUpdate, event_queue_update_system);

        // -- snip --
    }
}
```

This `Plugin` initializes some resources with `init_resource`, registers some types with `register_type`, and adds some systems with `add_systems`.

`init_resource` initializes a resource by adding it to the `World`

```rust
    /// Initialize a [`Resource`] with standard starting values by adding it to the [`World`].
    ///
    /// If the [`Resource`] already exists, nothing happens.
    ///
    /// The [`Resource`] must implement the [`FromWorld`] trait.
    /// If the [`Default`] trait is implemented, the [`FromWorld`] trait will use
    /// the [`Default::default`] method to initialize the [`Resource`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_app::prelude::*;
    /// # use bevy_ecs::prelude::*;
    /// #
    /// #[derive(Resource)]
    /// struct MyCounter {
    ///     counter: usize,
    /// }
    ///
    /// impl Default for MyCounter {
    ///     fn default() -> MyCounter {
    ///         MyCounter {
    ///             counter: 100
    ///         }
    ///     }
    /// }
    ///
    /// App::new()
    ///     .init_resource::<MyCounter>();
    /// ```
    pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
        self.world.init_resource::<R>();
        self
    }
```

`register_type` adds a new type to the type registry (we saw this yesterday) to enable some reflection in Bevy. This also mutates the `World`

```rust
    /// Registers the type `T` in the [`TypeRegistry`](bevy_reflect::TypeRegistry) resource,
    /// adding reflect data as specified in the [`Reflect`](bevy_reflect::Reflect) derive:
    /// ```rust,ignore
    /// #[derive(Reflect)]
    /// #[reflect(Component, Serialize, Deserialize)] // will register ReflectComponent, ReflectSerialize, ReflectDeserialize
    /// ```
    ///
    /// See [`bevy_reflect::TypeRegistry::register`].
    #[cfg(feature = "bevy_reflect")]
    pub fn register_type<T: bevy_reflect::GetTypeRegistration>(&mut self) -> &mut Self {
        let registry = self.world.resource_mut::<AppTypeRegistry>();
        registry.write().register::<T>();
        self
    }
```

We've used `add_systems` before as well, it adds one or more systems to an `App` to be run within the specified `Schedule`

```rust
    /// Adds a system to the given schedule in this app's [`Schedules`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_app::prelude::*;
    /// # use bevy_ecs::prelude::*;
    /// #
    /// # let mut app = App::new();
    /// # fn system_a() {}
    /// # fn system_b() {}
    /// # fn system_c() {}
    /// # fn should_run() -> bool { true }
    /// #
    /// app.add_systems(Update, (system_a, system_b, system_c));
    /// app.add_systems(Update, (system_a, system_b).run_if(should_run));
    /// ```
    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        let schedule = schedule.intern();
        let mut schedules = self.world.resource_mut::<Schedules>();

        if let Some(schedule) = schedules.get_mut(schedule) {
            schedule.add_systems(systems);
        } else {
            let mut new_schedule = Schedule::new(schedule);
            new_schedule.add_systems(systems);
            schedules.insert(new_schedule);
        }

        self
    }
```

...come to think of it, we actually saw another `Plugin` yesterday, the `MainSchedulePlugin`

```rust
/// Initializes the [`Main`] schedule, sub schedules,  and resources for a given [`App`].
pub struct MainSchedulePlugin;

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

Okay, so a `Plugin` is a way of adding `Schedule`s, types, resources, and other things to an `App`, in a modular way. Anything that `impl`ements `Plugin` is "a `Plugin`".

Let's check out `add_plugins` next

```rust
    /// Adds one or more [`Plugin`]s.
    ///
    /// One of Bevy's core principles is modularity. All Bevy engine features are implemented
    /// as [`Plugin`]s. This includes internal features like the renderer.
    ///
    /// [`Plugin`]s can be grouped into a set by using a [`PluginGroup`].
    ///
    /// There are built-in [`PluginGroup`]s that provide core engine functionality.
    /// The [`PluginGroup`]s available by default are `DefaultPlugins` and `MinimalPlugins`.
    ///
    /// To customize the plugins in the group (reorder, disable a plugin, add a new plugin
    /// before / after another plugin), call [`build()`](super::PluginGroup::build) on the group,
    /// which will convert it to a [`PluginGroupBuilder`](crate::PluginGroupBuilder).
    ///
    /// You can also specify a group of [`Plugin`]s by using a tuple over [`Plugin`]s and
    /// [`PluginGroup`]s. See [`Plugins`] for more details.
    ///
    /// ## Examples
    /// ```
    /// # use bevy_app::{prelude::*, PluginGroupBuilder, NoopPluginGroup as MinimalPlugins};
    /// #
    /// # // Dummies created to avoid using `bevy_log`,
    /// # // which pulls in too many dependencies and breaks rust-analyzer
    /// # pub struct LogPlugin;
    /// # impl Plugin for LogPlugin {
    /// #     fn build(&self, app: &mut App) {}
    /// # }
    /// App::new()
    ///     .add_plugins(MinimalPlugins);
    /// App::new()
    ///     .add_plugins((MinimalPlugins, LogPlugin));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if one of the plugins was already added to the application.
```

A few things to call out here
- "`Plugin`s can be grouped into a set by using a `PluginGroup`."
- "The `PluginGroup`s available by default are `DefaultPlugins` and `MinimalPlugins`."
- "You can also specify a group of `Plugin`s by using a tuple over `Plugin`s and `PluginGroup`s."

So we can group `Plugin`s in a tuple or a `PluginGroup`. Tuples seem easier for one-off groupings, but I'd guess that `PluginGroup`s are easier if you find yourself using the same set of `Plugin`s over and over.

`DefaultPlugins` is the `PluginGroup` that we're using in this example -- what's in there?

```rust
/// This plugin group will add all the default plugins for a *Bevy* application:
/// * [`LogPlugin`](crate::log::LogPlugin)
/// * [`TaskPoolPlugin`](crate::core::TaskPoolPlugin)
/// * [`TypeRegistrationPlugin`](crate::core::TypeRegistrationPlugin)
/// * [`FrameCountPlugin`](crate::core::FrameCountPlugin)
/// * [`TimePlugin`](crate::time::TimePlugin)
/// * [`TransformPlugin`](crate::transform::TransformPlugin)
/// * [`HierarchyPlugin`](crate::hierarchy::HierarchyPlugin)
/// * [`DiagnosticsPlugin`](crate::diagnostic::DiagnosticsPlugin)
/// * [`InputPlugin`](crate::input::InputPlugin)
/// * [`WindowPlugin`](crate::window::WindowPlugin)
/// * [`AssetPlugin`](crate::asset::AssetPlugin) - with feature `bevy_asset`
/// * [`ScenePlugin`](crate::scene::ScenePlugin) - with feature `bevy_scene`
/// * [`WinitPlugin`](crate::winit::WinitPlugin) - with feature `bevy_winit`
/// * [`RenderPlugin`](crate::render::RenderPlugin) - with feature `bevy_render`
/// * [`ImagePlugin`](crate::render::texture::ImagePlugin) - with feature `bevy_render`
/// * [`PipelinedRenderingPlugin`](crate::render::pipelined_rendering::PipelinedRenderingPlugin) - with feature `bevy_render` when not targeting `wasm32`
/// * [`CorePipelinePlugin`](crate::core_pipeline::CorePipelinePlugin) - with feature `bevy_core_pipeline`
/// * [`SpritePlugin`](crate::sprite::SpritePlugin) - with feature `bevy_sprite`
/// * [`TextPlugin`](crate::text::TextPlugin) - with feature `bevy_text`
/// * [`UiPlugin`](crate::ui::UiPlugin) - with feature `bevy_ui`
/// * [`PbrPlugin`](crate::pbr::PbrPlugin) - with feature `bevy_pbr`
/// * [`GltfPlugin`](crate::gltf::GltfPlugin) - with feature `bevy_gltf`
/// * [`AudioPlugin`](crate::audio::AudioPlugin) - with feature `bevy_audio`
/// * [`GilrsPlugin`](crate::gilrs::GilrsPlugin) - with feature `bevy_gilrs`
/// * [`AnimationPlugin`](crate::animation::AnimationPlugin) - with feature `bevy_animation`
///
/// [`DefaultPlugins`] obeys *Cargo* *feature* flags. Users may exert control over this plugin group
/// by disabling `default-features` in their `Cargo.toml` and enabling only those features
/// that they wish to use.
///
/// [`DefaultPlugins`] contains all the plugins typically required to build
/// a *Bevy* application which includes a *window* and presentation components.
/// For *headless* cases – without a *window* or presentation, see [`MinimalPlugins`].
```

...a _ton_ of stuff. `FileDragAndDrop` is in the `bevy_window` crate, so I guess we are using this for the `WindowPlugin`?

I wonder if we could get away with this instead

```rust
fn main() {
    App::new()
        .add_plugins(WindowPlugin::default())
        .add_systems(Update, file_drag_and_drop_system)
        .run();
}
```

Hmm, no. We're missing something else...

After some trial and error, it looks like this is the minimal set of `Plugin`s we need for this example

```rust
fn main() {
    App::new()
        .add_plugins(WindowPlugin::default())
        .add_plugins(log::LogPlugin::default())
        .add_plugins(input::InputPlugin)
        .add_plugins(winit::WinitPlugin::default())
        .add_plugins(a11y::AccessibilityPlugin)
        .add_systems(Update, file_drag_and_drop_system)
        .run();
}
```

...so, yeah, okay, fair enough that `DefaultPlugins` is less cluttered and more ergonomic.

Right, so we've brought all of these `Plugin`s into our app... what's next?

---

The next line of `main` is `.add_systems(Update, file_drag_and_drop_system)`.

We've added systems to the `Update` schedule before.

The `MainSchedule` is documented as follows

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
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Main;
```

...so `Update` is run "each tick of `App::update()`". `update()` is called in lots of places in Bevy. Digging into where (and why, and when) `update` is called is probably a topic for another kata.

The thing that really sets this example apart from yesterday's, though, is the system itself.

---

The system we add to our `App` in the `Update` schedule is called `file_drag_and_drop_system`, and it's defined as follows

```rust
fn file_drag_and_drop_system(mut events: EventReader<FileDragAndDrop>) {
    for event in events.read() {
        info!("{:?}", event);
    }
}
```

This function takes a single argument `events` which is an `EventReader<FileDragAndDrop>`. We then iterate over the `events` and print them to the console.

As the name of this example (and the type `FileDragAndDrop`) suggests, these are events related to dragging and dropping files. If you drag a file from your desktop onto the rendered window, and drop it, you should see log lines in the console similar to the ones I see on my machine

```
2024-01-29T01:30:47.393568Z  INFO bevy_winit::system: Creating new window "App" (0v0)
2024-01-29T01:30:49.686856Z  INFO daily_bevy: HoveredFile { window: 0v0, path_buf: "/Users/andrew/Desktop/2024-01-10.md" }
2024-01-29T01:30:50.277097Z  INFO daily_bevy: HoveredFileCanceled { window: 0v0 }
2024-01-29T01:30:51.424967Z  INFO daily_bevy: HoveredFile { window: 0v0, path_buf: "/Users/andrew/Desktop/2024-01-10.md" }
2024-01-29T01:30:52.154017Z  INFO daily_bevy: DroppedFile { window: 0v0, path_buf: "/Users/andrew/Desktop/2024-01-10.md" }
2024-01-29T01:30:54.661990Z  INFO bevy_window::system: No windows are open, exiting
2024-01-29T01:30:54.662062Z  INFO bevy_winit::system: Closing window 0v0
```

`HoveredFileCanceled` events fire when a file is dragged into the window, then back out, without dropping it.

There are currently only these three kinds of `FileDragAndDrop` events, defined in the `bevy_window` crate

```rust
pub enum FileDragAndDrop {
    /// File is being dropped into a window.
    DroppedFile {
        /// Window the file was dropped into.
        window: Entity,
        /// Path to the file that was dropped in.
        path_buf: PathBuf,
    },

    /// File is currently being hovered over a window.
    HoveredFile {
        /// Window a file is possibly going to be dropped into.
        window: Entity,
        /// Path to the file that might be dropped in.
        path_buf: PathBuf,
    },

    /// File hovering was canceled.
    HoveredFileCanceled {
        /// Window that had a canceled file drop.
        window: Entity,
    },
}
```

But how does that `EventReader` work? Here is its documentation and `struct` definition

```rust
/// Reads events of type `T` in order and tracks which events have already been read.
#[derive(SystemParam, Debug)]
pub struct EventReader<'w, 's, E: Event> {
    reader: Local<'s, ManualEventReader<E>>,
    events: Res<'w, Events<E>>,
}
```

We call `.read()` on the `EventReader` passed into `file_drag_and_drop_system`, how does that work?

```rust
impl<'w, 's, E: Event> EventReader<'w, 's, E> {
    /// Iterates over the events this [`EventReader`] has not seen yet. This updates the
    /// [`EventReader`]'s event counter, which means subsequent event reads will not include events
    /// that happened before now.
    pub fn read(&mut self) -> EventIterator<'_, E> {
        self.reader.read(&self.events)
    }

    // -- snip --
}
```

Okay... so it calls `.read` on `self.reader`, which is the `Local<'s, ManualEventReader<E>>` stored in the `EventReader`. We pass `self.events` into `self.reader.read()` as an argument. But `events` is just another field in `EventReader`. It's not clear to me how this field gets populated.

[This resource](https://taintedcoders.com/bevy/events/) that I found online says

> "Events are created by an `EventWriter` and read by an `EventReader` which we access in our systems as parameters."

So somewhere there must be an `EventWriter<FileDragAndDrop>` populating some `Event` buffer?

Sure enough, in the `bevy_winit` crate, there is an `EventWriter<FileDragAndDrop>`

```rust
#[derive(SystemParam)]
struct WindowAndInputEventWriters<'w> {
    // `winit` `WindowEvent`s
    window_resized: EventWriter<'w, WindowResized>,
    // -- snip --
    file_drag_and_drop: EventWriter<'w, FileDragAndDrop>,
    cursor_moved: EventWriter<'w, CursorMoved>,
    cursor_entered: EventWriter<'w, CursorEntered>,
    cursor_left: EventWriter<'w, CursorLeft>,
    // `winit` `DeviceEvent`s
    mouse_motion: EventWriter<'w, MouseMotion>,
}
```

Here's the path through all of this...

First, we have the `WinitPlugin`

```rust
impl Plugin for WinitPlugin {
    fn build(&self, app: &mut App) {
        // -- snip --

        app.init_non_send_resource::<WinitWindows>()
            .init_resource::<WinitSettings>()
            .set_runner(winit_runner)
            .add_systems(
                Last,
                (
                    // `exit_on_all_closed` only checks if windows exist but doesn't access data,
                    // so we don't need to care about its ordering relative to `changed_windows`
                    changed_windows.ambiguous_with(exit_on_all_closed),
                    despawn_windows,
                )
                    .chain(),
            );
        
        // -- snip --
    }
}
```

which sets its runner to the `winit_runner`.

```rust
/// The default [`App::runner`] for the [`WinitPlugin`] plugin.
///
/// Overriding the app's [runner](bevy_app::App::runner) while using `WinitPlugin` will bypass the
/// `EventLoop`.
pub fn winit_runner(mut app: App) {
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }

    let mut event_loop = app
        .world
        .remove_non_send_resource::<EventLoop<()>>()
        .unwrap();

    // -- snip --

    // setup up the event loop
    let event_handler = move |event: Event<()>,
                              event_loop: &EventLoopWindowTarget<()>,
                              control_flow: &mut ControlFlow| {
        // -- snip --
    }
    
    // -- snip --

    if return_from_run {
        run_return(&mut event_loop, event_handler);
    } else {
        run(event_loop, event_handler);
    }
}
```

As seen above, the `winit_runner` defines some `event_loop`, which runs some `event_handler`. The `event_handler` handles `Events` of all kinds...

```rust
let event_handler = move |event: Event<()>,
                              event_loop: &EventLoopWindowTarget<()>,
                              control_flow: &mut ControlFlow| {
        // -- snip --
        match event {
            event::Event::NewEvents(start_cause) => match start_cause {
                // -- snip --
            },
            event::Event::WindowEvent {
                event, window_id, ..
            } => {
                // -- snip --
                match event {
                    // -- snip --
                    WindowEvent::DroppedFile(path_buf) => {
                        event_writers
                            .file_drag_and_drop
                            .send(FileDragAndDrop::DroppedFile {
                                window: window_entity,
                                path_buf,
                            });
                    }
                    WindowEvent::HoveredFile(path_buf) => {
                        event_writers
                            .file_drag_and_drop
                            .send(FileDragAndDrop::HoveredFile {
                                window: window_entity,
                                path_buf,
                            });
                    }
                    WindowEvent::HoveredFileCancelled => {
                        event_writers.file_drag_and_drop.send(
                            FileDragAndDrop::HoveredFileCanceled {
                                window: window_entity,
                            },
                        );
                    }
                    // -- snip --
                    _ => {}
                }
                // -- snip --
            }
            event::Event::DeviceEvent {
                // -- snip --
            }
            // -- snip --
            _ => (),
        }
    };
```

...including `WindowEvent`s, which it handles by `send`ing those events to the `file_drag_and_drop` `EventWriter`.

So, `bevy_winit` seems to keep track of these `FileDragAndDrop` events for us, all we need to do is handle them with an `EventReader<FileDragAndDrop>`.

---

I thought this would be an easy kata relative to yesterday, but there was just as much complexity hidden behind handling this one specific group of events as there was yesterday in setting up our basic "hello world" app.

I feel as though I've only just scratched the surface of `EventReader`s and `EventWriter`s and `Plugin`s and `PluginGroup`s, but I am certain I know more now than I knew yesterday. Therefore, I consider Day 2 a success.

See you tomorrow!

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).