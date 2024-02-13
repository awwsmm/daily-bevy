# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the thirteenth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Scene

Today is the thirteenth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today we'll be looking at the [`scene` example](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/scene/scene.rs) from the Bevy repo.

#### The Code

Here's the `main.rs` for this example

```rust
//! This example illustrates loading scenes from files.
use bevy::{prelude::*, tasks::IoTaskPool, utils::Duration};
use std::{fs::File, io::Write};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .register_type::<ResourceA>()
        .add_systems(
            Startup,
            (save_scene_system, load_scene_system, infotext_system),
        )
        .add_systems(Update, log_system)
        .run();
}

// Registered components must implement the `Reflect` and `FromWorld` traits.
// The `Reflect` trait enables serialization, deserialization, and dynamic property access.
// `Reflect` enable a bunch of cool behaviors, so its worth checking out the dedicated `reflect.rs`
// example. The `FromWorld` trait determines how your component is constructed when it loads.
// For simple use cases you can just implement the `Default` trait (which automatically implements
// `FromWorld`). The simplest registered component just needs these three derives:
#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

// Some components have fields that cannot (or should not) be written to scene files. These can be
// ignored with the #[reflect(skip_serializing)] attribute. This is also generally where the `FromWorld`
// trait comes into play. `FromWorld` gives you access to your App's current ECS `Resources`
// when you construct your component.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        ComponentB {
            _time_since_startup: time.elapsed(),
            value: "Default Value".to_string(),
        }
    }
}

// Resources can be serialized in scenes as well, with the same requirements `Component`s have.
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct ResourceA {
    pub score: u32,
}

// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

// The new, updated scene data will be saved here so that you can see the changes
const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Spawning" a scene bundle creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    commands.spawn(DynamicSceneBundle {
        // Scenes are loaded just like any other asset.
        scene: asset_server.load(SCENE_FILE_PATH),
        ..default()
    });
}

// This system logs all ComponentA components in our world. Try making a change to a ComponentA in
// load_scene_example.scn. If you enable the `file_watcher` cargo feature you should immediately see
// the changes appear in the console whenever you make a change.
fn log_system(
    query: Query<(Entity, &ComponentA), Changed<ComponentA>>,
    res: Option<Res<ResourceA>>,
) {
    for (entity, component_a) in &query {
        info!("  Entity({})", entity.index());
        info!(
            "    ComponentA: {{ x: {} y: {} }}\n",
            component_a.x, component_a.y
        );
    }
    if let Some(res) = res {
        if res.is_added() {
            info!("  New ResourceA: {{ score: {} }}\n", res.score);
        }
    }
}

fn save_scene_system(world: &mut World) {
    // Scenes can be created from any ECS World.
    // You can either create a new one for the scene or use the current World.
    // For demonstration purposes, we'll create a new one.
    let mut scene_world = World::new();

    // The `TypeRegistry` resource contains information about all registered types (including components).
    // This is used to construct scenes, so we'll want to ensure that our previous type registrations
    // exist in this new scene world as well.
    // To do this, we can simply clone the `AppTypeRegistry` resource.
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);

    let mut component_b = ComponentB::from_world(world);
    component_b.value = "hello".to_string();
    scene_world.spawn((
        component_b,
        ComponentA { x: 1.0, y: 2.0 },
        Transform::IDENTITY,
    ));
    scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
    scene_world.insert_resource(ResourceA { score: 1 });

    // With our sample world ready to go, we can now create our scene:
    let scene = DynamicScene::from_world(&scene_world);

    // Scenes can be serialized like this:
    let type_registry = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();

    // Showing the scene in the console
    info!("{}", serialized_scene);

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in WASM as there is no filesystem access
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

// This is only necessary for the info message in the UI. See examples/ui/text.rs for a standalone
// text example.
fn infotext_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        TextBundle::from_section(
            "Nothing to see in this window! Check the console output!",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
    );
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.12.1"
```

#### Discussion

With just the `main.rs` and `Cargo.toml` files in my new Cargo project for this example, I tried to run `main` and got the following error

```rust
thread 'IO Task Pool (0)' panicked at src/main.rs:142:18:
Error while writing scene to file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

...so it looks like this example is starting with some debugging.

Line `142` is the last line of this snippet

```rust
File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
    .expect("Error while writing scene to file");
```

`NEW_SCENE_FILE_PATH` is defined earlier as

```rust
const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";
```

...so it looks like I'm missing an `assets/scenes` directory. Let's add that and run `main` again

```
2024-02-12T23:17:04.446814Z  INFO bevy_winit::system: Creating new window "App" (0v0)
...
2024-02-12T23:17:04.503118Z ERROR bevy_asset::server: path not found: /Users/andrew/Git/daily-bevy/assets/scenes/load_scene_example.scn.ron
```

We only ever read this file (`SCENE_FILE_PATH`), we don't write to it. So let's copy that over from the Bevy repo, as well. We get a different error this time

```
2024-02-12T23:24:09.374914Z ERROR bevy_asset::server: Failed to load asset 'scenes/load_scene_example.scn.ron' with asset loader 'bevy_scene::scene_loader::SceneLoader': Could not parse RON: 3:23: No registration found for `scene::ResourceA`
```

Note that we do _not_ get this error if we run this example directly in the Bevy repo. My hunch is that `scene::ResourceA` needs to be changed to `daily_bevy::ResourceA`. `scene` is the name of the `example` in the Bevy repo, but here, the name of the `project` is `daily_bevy`.

So let's try that -- let's find-and-replace `scene::` in `load_scene_example.scn.ron` with `daily_bevy::`

```
2024-02-12T23:27:37.835790Z ERROR bevy_asset::server: Failed to load asset 'scenes/load_scene_example.scn.ron' with asset loader 'bevy_scene::scene_loader::SceneLoader': Could not parse RON: 16:22: Found invalid std identifier `0.0`, try the raw identifier `r#0.0` instead
```

Great! Now we're at least getting the same error here as we are in the Bevy repo directly. So what's the problem? The error message above says `Could not parse RON: 16:22`. Line 16 is the `rotation` line in this snippet

```json
components: {
  "bevy_transform::components::transform::Transform": (
    translation: (
      x: 0.0,
      y: 0.0,
      z: 0.0
    ),
    rotation: (0.0, 0.0, 0.0, 1.0),
    scale: (
      x: 1.0,
      y: 1.0,
      z: 1.0
    ),
  ),
```

`scale` and `translation` both use named arguments... I wonder if that's what's missing from `rotation`? In fact, if you look at the `-new` file `main` creates as output, it strongly seems to suggest that that is the case. Here's a snippet from that one

```json
"bevy_transform::components::transform::Transform": (
  translation: (
    x: 0.0,
    y: 0.0,
    z: 0.0,
  ),
  rotation: (
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 1.0,
  ),
  scale: (
    x: 1.0,
    y: 1.0,
    z: 1.0,
  ),
),
```

So let's again edit `load_scene_example.scn.ron`, this time adding argument labels to `rotation`

```
2024-02-12T23:31:50.146764Z  INFO daily_bevy:   Entity(4)
2024-02-12T23:31:50.146787Z  INFO daily_bevy:     ComponentA: { x: 1 y: 2 }

2024-02-12T23:31:50.146792Z  INFO daily_bevy:   Entity(5)
2024-02-12T23:31:50.146795Z  INFO daily_bevy:     ComponentA: { x: 3 y: 4 }

2024-02-12T23:31:50.146799Z  INFO daily_bevy:   New ResourceA: { score: 2 }
```

No errors this time! Just the expected output of the app. Now that it's working correctly, let's dig into what's actually happening here.

---

This example shows how to save a `Scene` to a file, and read a `Scene` from a file. What is a `Scene`?

```rust
pub struct Scene {
    /// The world of the scene, containing its entities and resources.
    pub world: World,
}
```

...so a `Scene` appears to be "one step higher" than even a `World`. A `Scene` contains a `World` within it.

The `save_scene_system` in this example has lots of great explanatory comments around `Scene`s, so let's start there

```rust
fn save_scene_system(world: &mut World) {
    // Scenes can be created from any ECS World.
    // You can either create a new one for the scene or use the current World.
    // For demonstration purposes, we'll create a new one.
    let mut scene_world = World::new();
```

To create a `Scene`, we can just construct one directly by passing in a `World` argument. The text above says "the current World"... is there already a `World` in this `App`?

There is, recall that `App::empty()` creates a default `World`

```rust
    pub fn empty() -> App {
        let mut world = World::new();
        world.init_resource::<Schedules>();
        Self {
            world,
            // -- snip --
        }
    }
```

...so whenever we have an `App`, we have a "current" `World`. This example wants to make a `World` from scratch and use _that_ `World` to create the `Scene`.

```rust
    // The `TypeRegistry` resource contains information about all registered types (including components).
    // This is used to construct scenes, so we'll want to ensure that our previous type registrations
    // exist in this new scene world as well.
    // To do this, we can simply clone the `AppTypeRegistry` resource.
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);
```

The above might seem a bit silly. We're creating a new `World`, but copying a seemingly quite large chunk of it from the old `World`. What is the benefit of this?

The benefit is that creating the `Scene` doesn't consume the "current" `World`. `Scene` contains a `World`. If we move the current `World` into the `Scene`, we cannot use the current `World` for anything else. So by copying the bits of it we need, we can create a _new_ `World` to pass to the `Scene`, which contains all the important stuff we need from the current world.

In this case, that "important stuff" is the `AppTypeRegistry`, which contains the types registered to the `App` at the beginning of `main.rs`

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .register_type::<ResourceA>()
        // -- snip --
        .run();
}
```

These types are quite simple, and there is a _lot_ of documentation in the example. Let's start with `ComponentA`.

---

```rust
// Registered components must implement the `Reflect` and `FromWorld` traits.
// The `Reflect` trait enables serialization, deserialization, and dynamic property access.
// `Reflect` enable a bunch of cool behaviors, so its worth checking out the dedicated `reflect.rs`
// example. The `FromWorld` trait determines how your component is constructed when it loads.
// For simple use cases you can just implement the `Default` trait (which automatically implements
// `FromWorld`). The simplest registered component just needs these three derives:
#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct ComponentA {
    pub x: f32,
    pub y: f32,
}
```

We've already seen `FromWorld`, but we haven't dug too much into `Reflect`. There is no `reflect.rs` example in the Bevy repo, though there is a `reflection.rs` example. Maybe this comment is out of date? Let's check out `reflection.rs` tomorrow.

The comment above explains why we need to `derive` the three traits listed, but do we need `#[reflect(Component)]`? What happens when we comment out that line?

```
thread 'main' panicked at /Users/andrew/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_scene-0.12.1/src/scene_spawner.rs:430:35:
scene contains the unregistered component `daily_bevy::ComponentA`. consider adding `#[reflect(Component)]` to your type
```

This error message explains _what_ to do to fix the problem (uncomment the commented-out line) and kind of explains why ("unregistered component"), but this `panic` is spawned here, in `scene_spawner`

```rust
scene_spawner
    .spawn_queued_scenes(world)
    .unwrap_or_else(|err| panic!("{}", err));
```

...and this doesn't really explain what's going wrong. If we dig a few levels deep, there's a bit of code in `dynamic_scene.rs` which looks like

```rust
let reflect_component =
    registration.data::<ReflectComponent>().ok_or_else(|| {
        SceneSpawnError::UnregisteredComponent {
            type_path: type_info.type_path().to_string(),
        }
    })?;
```

`UnregisteredComponent` is a `SceneSpawnError`. There are lots of things that can go wrong when spawning a `Scene`

```rust
/// Errors that can occur when spawning a scene.
#[derive(Error, Debug)]
pub enum SceneSpawnError {
    /// Scene contains an unregistered component type.
    #[error("scene contains the unregistered component `{type_path}`. consider adding `#[reflect(Component)]` to your type")]
    UnregisteredComponent {
        /// Type of the unregistered component.
        type_path: String,
    },
    /// Scene contains an unregistered resource type.
    #[error("scene contains the unregistered resource `{type_path}`. consider adding `#[reflect(Resource)]` to your type")]
    UnregisteredResource {
        /// Type of the unregistered resource.
        type_path: String,
    },
    /// Scene contains an unregistered type.
    #[error(
        "scene contains the unregistered type `{std_type_name}`. \
        consider reflecting it with `#[derive(Reflect)]` \
        and registering the type using `app.register_type::<T>()`"
    )]
    UnregisteredType {
        /// The [type name] for the unregistered type.
        /// [type name]: std::any::type_name
        std_type_name: String,
    },
    /// Scene contains an unregistered type which has a `TypePath`.
    #[error(
        "scene contains the reflected type `{type_path}` but it was not found in the type registry. \
        consider registering the type using `app.register_type::<T>()``"
    )]
    UnregisteredButReflectedType {
        /// The unregistered type.
        type_path: String,
    },
    /// Scene contains a proxy without a represented type.
    #[error("scene contains dynamic type `{type_path}` without a represented type. consider changing this using `set_represented_type`.")]
    NoRepresentedType {
        /// The dynamic instance type.
        type_path: String,
    },
    /// Dynamic scene with the given id does not exist.
    #[error("scene does not exist")]
    NonExistentScene {
        /// Id of the non-existent dynamic scene.
        id: AssetId<DynamicScene>,
    },
    /// Scene with the given id does not exist.
    #[error("scene does not exist")]
    NonExistentRealScene {
        /// Id of the non-existent scene.
        id: AssetId<Scene>,
    },
}
```

---

`ComponentB` shows another reflection feature -- skipping the serialization of some fields

```rust
// Some components have fields that cannot (or should not) be written to scene files. These can be
// ignored with the #[reflect(skip_serializing)] attribute. This is also generally where the `FromWorld`
// trait comes into play. `FromWorld` gives you access to your App's current ECS `Resources`
// when you construct your component.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        ComponentB {
            _time_since_startup: time.elapsed(),
            value: "Default Value".to_string(),
        }
    }
}
```

In the above case, we don't want to save the `_time_since_startup` field in the serialized output (the "save file"). Instead, we want to start over from the start time of the _currently-running instance_ of the `App` when we create a new `ComponentB` from the `World`.

As noted in the comment in the snippet above, this is also when we could use `FromWorld` rather than `Default` -- because `from_world` takes a `World` argument, and we can access things like `Time`, which we wouldn't be able to access in a `Default` implementation.

---

Finally, we've also got an example `Resource` to serialize

```rust
// Resources can be serialized in scenes as well, with the same requirements `Component`s have.
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct ResourceA {
    pub score: u32,
}
```

Nice and easy.

With our three types registered in the `App`, we add some systems to different schedules, and `run` the `App`

```rust
fn main() {
    App::new()
        // -- snip --
        .add_systems(
            Startup,
            (save_scene_system, load_scene_system, infotext_system),
        )
        .add_systems(Update, log_system)
        .run();
}
```

---

So, back to the system we were looking at before we diverted into these type definitions... what's next?

First, we create an instance of `ComponentB` using its `FromWorld` implementation, then set its `value` field.

```rust
let mut component_b = ComponentB::from_world(world);
component_b.value = "hello".to_string();
```

Then, we `spawn` an entity with several `Component`s by passing a tuple to the `spawn` function.

```rust
scene_world.spawn((
    component_b,
    ComponentA { x: 1.0, y: 2.0 },
    Transform::IDENTITY,
));
```

The entity contains

- the instance of `ComponentB` we just created
- an instance of `ComponentA`, and
- ...`Transform::IDENTITY`? What's that?

Remember that `Transform` is also a `Component`

```rust
#[derive(Component, Debug, PartialEq, Clone, Copy, Reflect)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[reflect(Component, Default, PartialEq)]
pub struct Transform {
    // -- snip --
}
```

I think we're adding a `Transform` `Component` to our `World` as just another example of serializing components -- in this case, serializing a `Component` that we did not define ourselves.

Next we spawn a second entity, containing only a single component: another instance of `ComponentA`.

```rust
scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
```

Finally, we insert `ResourceA` into the `World`. Remember that `Resource`s are singletons -- there can only be a single `ResourceA` in the `World`.

```rust
scene_world.insert_resource(ResourceA { score: 1 });
```

---

```rust
// With our sample world ready to go, we can now create our scene:
let scene = DynamicScene::from_world(&scene_world);
```

What is a `DynamicScene`? And how does `from_world` create one from a `World`?

```rust
/// A collection of serializable resources and dynamic entities.
///
/// Each dynamic entity in the collection contains its own run-time defined set of components.
// -- snip --
#[derive(Asset, TypePath, Default)]
pub struct DynamicScene {
    /// Resources stored in the dynamic scene.
    pub resources: Vec<Box<dyn Reflect>>,
    /// Entities contained in the dynamic scene.
    pub entities: Vec<DynamicEntity>,
}
```

A `DynamicScene` is just a collection of `resources` and `entities`. `resources` and `entities` are both `Vec`tors of things. `entities` is a vector of `DynamicEntity`s...

```rust
/// A reflection-powered serializable representation of an entity and its components.
pub struct DynamicEntity {
    /// The identifier of the entity, unique within a scene (and the world it may have been generated from).
    ///
    /// Components that reference this entity must consistently use this identifier.
    pub entity: Entity,
    /// A vector of boxed components that belong to the given entity and
    /// implement the [`Reflect`] trait.
    pub components: Vec<Box<dyn Reflect>>,
}
```

...and `resources` is a vector of `Box<dyn Reflect>`s -- that is, pointers to unknown types which implement `Reflect`. There's a lot of reflection going on here.

`DynamicScene::from_world` looks like

```rust
/// Create a new dynamic scene from a given world.
pub fn from_world(world: &World) -> Self {
    DynamicSceneBuilder::from_world(world)
        .extract_entities(world.iter_entities().map(|entity| entity.id()))
        .extract_resources()
        .build()
}
```

We extract entities and resources from the `World`, and then presumably use those to populate the `entities` and `resources` fields of a `DynamicScene`.

This extraction is pretty flexible in both cases: we can allow or deny certain resources and entities to be extracted, or filter based on a query.

For now, let's pause here, though. We can come back to this in a later kata.

---

Next, we serialize the `Scene` using [Rusty Object Notation (RON)](https://github.com/ron-rs/ron), and log it using `info!`

```rust
// Scenes can be serialized like this:
let type_registry = world.resource::<AppTypeRegistry>();
let serialized_scene = scene.serialize_ron(type_registry).unwrap();

// Showing the scene in the console
info!("{}", serialized_scene);
```

That output can be found in `load_scene_example-new.scn.ron`, but it is also reproduced here

```rust
(
  resources: {
    "daily_bevy::ResourceA": (
      score: 1,
    ),
  },
  entities: {
    0: (
      components: {
        "daily_bevy::ComponentB": (
          value: "hello",
        ),
        "daily_bevy::ComponentA": (
          x: 1.0,
          y: 2.0,
        ),
        "bevy_transform::components::transform::Transform": (
          translation: (
            x: 0.0,
            y: 0.0,
            z: 0.0,
          ),
          rotation: (
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
          ),
          scale: (
            x: 1.0,
            y: 1.0,
            z: 1.0,
          ),
        ),
      },
    ),
    1: (
      components: {
        "daily_bevy::ComponentA": (
          x: 3.0,
          y: 4.0,
        ),
      },
    ),
  },
)
```

Above, you can see that there are two entities (`0` and `1`), where the first entity has the three components we defined, and the second has only one component. The `Resource` we defined is also serialized.

Printing this to the console is easy, writing it to a file is a bit more difficult

```rust
// Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
// as they are blocking
// This can't work in WASM as there is no filesystem access
#[cfg(not(target_arch = "wasm32"))]
IoTaskPool::get()
    .spawn(async move {
        // Write the scene RON data to file
        File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
            .and_then(|mut file| file.write(serialized_scene.as_bytes()))
            .expect("Error while writing scene to file");
    })
    .detach();
```

First, we only run this code when we are _not_ compiling to WASM. WASM doesn't have access to the filesystem. We could probably add a WASM-specific serialization which would write this output to a cookie, or download a save file, though.

Next, we spawn a thread from [an IO-bound task pool](https://en.wikipedia.org/wiki/I/O_bound). In this thread we `create` the `File` and write the serialized data to it.

None of this really has anything to do with Bevy directly, but you may encounter similar issues when creating your own Bevy games, so it's good to have some exposure to this sort of thing.

---

Everything above is the bulk of the actual content of this example, but there are a few more supporting systems which showcase some other features of scenes.

First, we've already written a serialized scene _to_ a file, but we can also read a serialized scene _from_ a file, using code similar to what's found in the `load_scene_system`

```rust
fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Spawning" a scene bundle creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    commands.spawn(DynamicSceneBundle {
        // Scenes are loaded just like any other asset.
        scene: asset_server.load(SCENE_FILE_PATH),
        ..default()
    });
}
```

We `load` the scene using the `asset_server`, "just like any other asset".

Maybe now is a good time to dig into `Commands` as well. The documentation comments do a lot of heavy lifting here

```rust
/// A [`Command`] queue to perform impactful changes to the [`World`].
///
/// Since each command requires exclusive access to the `World`,
/// all queued commands are automatically applied in sequence
/// when the [`apply_deferred`] system runs.
///
/// The command queue of an individual system can also be manually applied
/// by calling [`System::apply_deferred`].
/// Similarly, the command queue of a schedule can be manually applied via [`Schedule::apply_deferred`].
///
/// Each command can be used to modify the [`World`] in arbitrary ways:
/// * spawning or despawning entities
/// * inserting components on new or existing entities
/// * inserting resources
/// * etc.
///
// -- snip --
///
/// Add `mut commands: Commands` as a function argument to your system to get a copy of this struct that will be applied the next time a copy of [`apply_deferred`] runs.
/// Commands are almost always used as a [`SystemParam`](crate::system::SystemParam).
///
// -- snip --
///
/// Each built-in command is implemented as a separate method, e.g. [`Commands::spawn`].
/// In addition to the pre-defined command methods, you can add commands with any arbitrary
/// behavior using [`Commands::add`], which accepts any type implementing [`Command`].
///
/// Since closures and other functions implement this trait automatically, this allows one-shot,
/// anonymous custom commands.
///
/// ```
/// # use bevy_ecs::prelude::*;
/// # fn foo(mut commands: Commands) {
/// // NOTE: type inference fails here, so annotations are required on the closure.
/// commands.add(|w: &mut World| {
///     // Mutate the world however you want...
///     # todo!();
/// });
/// # }
/// ```
///
/// [`System::apply_deferred`]: crate::system::System::apply_deferred
/// [`apply_deferred`]: crate::schedule::apply_deferred
/// [`Schedule::apply_deferred`]: crate::schedule::Schedule::apply_deferred
#[derive(SystemParam)]
pub struct Commands<'w, 's> {
    queue: Deferred<'s, CommandQueue>,
    entities: &'w Entities,
}
```

So `Commands` "perform impactful changes to the `World`", and by adding "`mut commands: Commands` as a function argument" to a system, we can add one or more `Command`s, which "will be applied the next time a copy of `apply_deferred` runs".

"Deferred application" means that all actions which could potentially mutate the `World` are applied as close to each other in time as possible, so that more code can do more read-only access of the `World` for a longer period of time. Since read-only access can safely happen across multiple threads simultaneously, it's better to try to collect all the read-write accesses and try to execute them all in a relatively short period of time.

So, essentially, a `Command` is anything which takes a mutable reference to the `World`

```rust
/// A [`World`] mutation.
///
/// Should be used with [`Commands::add`].
///
/// # Usage
///
/// ```
/// # use bevy_ecs::prelude::*;
/// # use bevy_ecs::system::Command;
/// // Our world resource
/// #[derive(Resource, Default)]
/// struct Counter(u64);
///
/// // Our custom command
/// struct AddToCounter(u64);
///
/// impl Command for AddToCounter {
///     fn apply(self, world: &mut World) {
///         let mut counter = world.get_resource_or_insert_with(Counter::default);
///         counter.0 += self.0;
///     }
/// }
///
/// fn some_system(mut commands: Commands) {
///     commands.add(AddToCounter(42));
/// }
/// ```
pub trait Command: Send + 'static {
    /// Applies this command, causing it to mutate the provided `world`.
    ///
    /// This method is used to define what a command "does" when it is ultimately applied.
    /// Because this method takes `self`, you can store data or settings on the type that implements this trait.
    /// This data is set by the system or other source of the command, and then ultimately read in this method.
    fn apply(self, world: &mut World);
}
```

...this includes "closures and other functions", which "implement this trait automatically"

```rust
impl<F> Command for F
where
    F: FnOnce(&mut World) + Send + 'static,
{
    fn apply(self, world: &mut World) {
        self(world);
    }
}
```

---

We've only got two more systems to cover. First is the `infotext_system`, which is largely unnecessary

```rust
// This is only necessary for the info message in the UI. See examples/ui/text.rs for a standalone
// text example.
fn infotext_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        TextBundle::from_section(
            "Nothing to see in this window! Check the console output!",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
    );
}
```

This just spawns a camera and writes text to the `Window` which is spawned automatically by `App::default`. We could probably get rid of this entirely by picking the plugins we need in our `App` à la carte, instead of using the `DefaultPlugins` convenience group.

The only thing new here is `AlignSelf::FlexEnd`, which just causes the text to be bottom-aligned in the spawned `Window`.

---

Finally, we've got the `log_system`

```rust
// This system logs all ComponentA components in our world. Try making a change to a ComponentA in
// load_scene_example.scn. If you enable the `file_watcher` cargo feature you should immediately see
// the changes appear in the console whenever you make a change.
fn log_system(
    query: Query<(Entity, &ComponentA), Changed<ComponentA>>,
    res: Option<Res<ResourceA>>,
) {
    for (entity, component_a) in &query {
        info!("  Entity({})", entity.index());
        info!(
            "    ComponentA: {{ x: {} y: {} }}\n",
            component_a.x, component_a.y
        );
    }
    if let Some(res) = res {
        if res.is_added() {
            info!("  New ResourceA: {{ score: {} }}\n", res.score);
        }
    }
}
```

This system is the reason why the two `ComponentA` entities and `ResourceA` are printed to the console

```
2024-02-13T03:35:19.799618Z  INFO daily_bevy:   Entity(5)
2024-02-13T03:35:19.799645Z  INFO daily_bevy:     ComponentA: { x: 3 y: 4 }

2024-02-13T03:35:19.799651Z  INFO daily_bevy:   Entity(4)
2024-02-13T03:35:19.799654Z  INFO daily_bevy:     ComponentA: { x: 1 y: 2 }

2024-02-13T03:35:19.799658Z  INFO daily_bevy:   New ResourceA: { score: 2 }
```

As mentioned in the comment above this system, the Bevy Cargo [`file_watcher` option](https://github.com/bevyengine/bevy/blob/main/docs/cargo_features.md#optional-features) allows for hot reloading of assets.

You can see this in action by

1. changing the `bevy` dependency in `Cargo.toml` to `bevy = { version = "0.12.1", features = ["file_watcher"]}`
2. running `main.rs`
3. editing `load_scene_example.scn.ron` and saving the file

You should see output similar to

```
2024-02-13T03:42:05.249961Z  INFO daily_bevy:   Entity(5)
2024-02-13T03:42:05.249988Z  INFO daily_bevy:     ComponentA: { x: 3 y: 4 }

2024-02-13T03:42:05.249994Z  INFO daily_bevy:   Entity(4)
2024-02-13T03:42:05.249997Z  INFO daily_bevy:     ComponentA: { x: 1 y: 2 }

2024-02-13T03:42:05.250001Z  INFO daily_bevy:   New ResourceA: { score: 2 }

2024-02-13T03:42:15.320347Z  INFO bevy_asset::server: Reloading scenes/load_scene_example.scn.ron because it has changed
2024-02-13T03:42:15.337447Z  INFO daily_bevy:   Entity(5)
2024-02-13T03:42:15.337478Z  INFO daily_bevy:     ComponentA: { x: 5 y: 4 }

2024-02-13T03:42:15.337484Z  INFO daily_bevy:   Entity(4)
2024-02-13T03:42:15.337487Z  INFO daily_bevy:     ComponentA: { x: 1 y: 2 }

2024-02-13T03:42:21.803562Z  INFO bevy_asset::server: Reloading scenes/load_scene_example.scn.ron because it has changed
2024-02-13T03:42:21.829308Z  INFO daily_bevy:   Entity(5)
2024-02-13T03:42:21.829333Z  INFO daily_bevy:     ComponentA: { x: 3 y: 4 }

2024-02-13T03:42:21.829339Z  INFO daily_bevy:   Entity(4)
2024-02-13T03:42:21.829342Z  INFO daily_bevy:     ComponentA: { x: 1 y: 2 }
```

Hot reloading makes development much easier, it's nice to see that Bevy has this feature!

Everything else in this system -- `Query`, `Entity`, `Changed`, `Res` -- we've seen already, and explored. We're making great progress on getting familiar with lots of what Bevy has to offer.

Tomorrow, let's take a look at `reflection.rs` to get some more exposure to where, and how, and why reflection is used in Bevy.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
