# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the twelfth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Asset Loading

Today is the twelfth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, we'll be exploring the [`asset_loading` example](https://github.com/bevyengine/bevy/blob/release-0.12.1/examples/asset/asset_loading.rs) from the Bevy repo.

#### The Code

Here's the `main.rs` for this example

```rust
//! This example illustrates various ways to load assets.

use bevy::{asset::LoadedFolder, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // By default AssetServer will load assets from inside the "assets" folder.
    // For example, the next line will load "ROOT/assets/models/cube/cube.gltf#Mesh0/Primitive0",
    // where "ROOT" is the directory of the Application.
    //
    // This can be overridden by setting the "CARGO_MANIFEST_DIR" environment variable (see
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html)
    // to another directory. When the Application is run through Cargo, "CARGO_MANIFEST_DIR" is
    // automatically set to your crate (workspace) root directory.
    let cube_handle = asset_server.load("models/cube/cube.gltf#Mesh0/Primitive0");
    let sphere_handle = asset_server.load("models/sphere/sphere.gltf#Mesh0/Primitive0");

    // All assets end up in their Assets<T> collection once they are done loading:
    if let Some(sphere) = meshes.get(&sphere_handle) {
        // You might notice that this doesn't run! This is because assets load in parallel without
        // blocking. When an asset has loaded, it will appear in relevant Assets<T>
        // collection.
        info!("{:?}", sphere.primitive_topology());
    } else {
        info!("sphere hasn't loaded yet");
    }

    // You can load all assets in a folder like this. They will be loaded in parallel without
    // blocking. The LoadedFolder asset holds handles to each asset in the folder. These are all
    // dependencies of the LoadedFolder asset, meaning you can wait for the LoadedFolder asset to
    // fire AssetEvent::LoadedWithDependencies if you want to wait for all assets in the folder
    // to load.
    // If you want to keep the assets in the folder alive, make sure you store the returned handle
    // somewhere.
    let _loaded_folder: Handle<LoadedFolder> = asset_server.load_folder("models/torus");

    // If you want a handle to a specific asset in a loaded folder, the easiest way to get one is to call load.
    // It will _not_ be loaded a second time.
    // The LoadedFolder asset will ultimately also hold handles to the assets, but waiting for it to load
    // and finding the right handle is more work!
    let torus_handle = asset_server.load("models/torus/torus.gltf#Mesh0/Primitive0");

    // You can also add assets directly to their Assets<T> storage:
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });

    // torus
    commands.spawn(PbrBundle {
        mesh: torus_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(-3.0, 0.0, 0.0),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: cube_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    // sphere
    commands.spawn(PbrBundle {
        mesh: sphere_handle,
        material: material_handle,
        transform: Transform::from_xyz(3.0, 0.0, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.12.1"
```

As you might expect, we also need some assets for this example. In the `assets/models` directory are three `.gltf` files: `cube/cube.gltf`, `torus/torus.gltf`, and `sphere/sphere.gltf`. There are also two `.bin` files: `cube/cube.bin` and `sphere/sphere.bin`.

#### Discussion

First -- what's a `.gltf` file?

[According to Wikipedia](https://en.wikipedia.org/wiki/GlTF), it's "a standard file format for three-dimensional scenes and models".

What does a `.gltf` file look like?

Well, we're in luck, we've got three in this example. `.gltf` files are human-readable. Here's a snippet from the `torus.gltf` file in this example

```json
{
    "asset":{
        "generator":"Khronos glTF Blender I/O v3.6.27",
        "version":"2.0"
    },
    "scene":0,
    "scenes":[
        {
            "name":"Scene",
            "nodes":[
                0
            ]
        }
    ],
    "nodes":[
        {
            "mesh":0,
            "name":"Torus.001",
            "rotation":[
                0.7071068286895752,
                0,
                0,
                0.7071068286895752
            ]
        }
    ],
    "meshes":[
        {
            "name":"Torus.005",
            "primitives":[
                {
                    "attributes":{
                        "POSITION":0,
                        "NORMAL":1,
                        "TEXCOORD_0":2
                    },
                    "indices":3
                }
            ]
        }
    ],
    ...
```

That looks a lot like JSON, eh? That's because it is

> "The glTF format stores data primarily in JSON." [[wikipedia]](https://en.wikipedia.org/wiki/GlTF)

"Primarily" is maybe a bit of a stretch, there's also a data buffer, which holds tens of thousands of bytes of information

```json
    ...
    "buffers":[
        {
            "byteLength":71744,
            "uri":"data:application/octet-stream;base64,AACgPwAAAA... 95640 more characters ...OBBkEDcQY="
        }
    ]
    ...
```

This data is "[embedded into the JSON](https://github.com/KhronosGroup/glTF-Tutorials/blob/main/gltfTutorial/gltfTutorial_002_BasicGltfStructure.md), in binary format, by using a [data URI](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URLs)."

The only reason I could think that this simple torus model would be so large (97-98KB) is that the mesh contains many, many points. Looking at the rendered model on my laptop, I don't see any "jaggedness" at all, so I could believe this.

Note that the `torus` is the only model without [a corresponding `.bin` file](https://stackoverflow.com/a/59205686/2925434). This is because, in the case of the other two models (the `cube` and the `sphere`), the binary data is stored in a separate file, rather than embedded in the `.gltf` file. For example, `cube.gltf` looks like this

```json
    ...
    "buffers" : [
        {
            "byteLength" : 840,
            "uri" : "cube.bin"
        }
    ]
    ...
```

Note that the cube uses many, many fewer bytes than the torus, as well. You need many fewer points in a mesh to accurately model a cube, compared to a torus.

---

Our `App` today is pretty simple, we only have one system

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
```

The `setup` system runs in the `Startup` schedule and its signature looks like this

```rust
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // -- snip --
}
```

We need
- mutable `Commands` (we should really dig into this soon)
- an immutable reference to the `AssetServer` `Resource`
- an immutable reference to the `Mesh` `Asset`, and
- a mutable reference to the `StandardMaterial` `Asset`

We've dug a bit into `Resource`s already.

Remember that `Resource`s are singletons; there can only be one instance of a particular `Resource` in the `World` at any given time. We access `Resource`s using the `Res` and `ResMut` system parameters, based on whether we want to just _read_ the resource or if we want to _mutate_ the resource, respectively.

We also had a look at `Asset`, recall...

```rust
pub trait Asset: VisitAssetDependencies + TypePath + Send + Sync + 'static {}
```

...that `Asset` is really just a marker trait. Anything that implements Bevy's `VisitAssetDependencies` `trait`, and a few other built-in Rust `trait`s, is considered an `Asset`.

`VisitAssetDependencies` is very simple

```rust
pub trait VisitAssetDependencies {
    fn visit_dependencies(&self, visit: &mut impl FnMut(UntypedAssetId));
}
```

This trait can be `derive`d for types which don't have any dependencies, I guess? Like `Font`

```rust
#[derive(Asset, TypePath, Debug, Clone)]
pub struct Font {
    pub font: FontArc,
}
```

The expanded `#[derive(Asset)]` macro for `Font` looks like

```rust
impl bevy_asset::Asset for Font {}
impl bevy_asset::VisitAssetDependencies for Font {
    fn visit_dependencies(&self, visit: &mut impl FnMut(bevy_asset::UntypedAssetId)) {}
}
```

...and so `visit_dependencies` genuinely does nothing here.

`Mesh` and `StandardMaterial` also `derive` `Asset`, so there's not much for us to explore here around the `Assets` type, specifically.

---

However, we can look at how we load and render assets from `.gltf` files.

The first two lines of `setup` have this big block comment above them

```rust
// By default AssetServer will load assets from inside the "assets" folder.
// For example, the next line will load "ROOT/assets/models/cube/cube.gltf#Mesh0/Primitive0",
// where "ROOT" is the directory of the Application.
//
// This can be overridden by setting the "CARGO_MANIFEST_DIR" environment variable (see
// https://doc.rust-lang.org/cargo/reference/environment-variables.html)
// to another directory. When the Application is run through Cargo, "CARGO_MANIFEST_DIR" is
// automatically set to your crate (workspace) root directory.
let cube_handle = asset_server.load("models/cube/cube.gltf#Mesh0/Primitive0");
let sphere_handle = asset_server.load("models/sphere/sphere.gltf#Mesh0/Primitive0");
```

This comment just explains the default path where assets are expected to be defined. But it also refers to `#Mesh0/Primitive0` in each of these files. I suppose, with respect to the `cube` model, that refers to this chunk of the `.gltf` file

```json
    ...
    "meshes" : [
        {
            "name" : "Cube",
            "primitives" : [
                {
                    "attributes" : {
                        "POSITION" : 0,
                        "NORMAL" : 1,
                        "TEXCOORD_0" : 2
                    },
                    "indices" : 3,
                    "material" : 0
                }
            ]
        }
    ],
    ...
```

`meshes` is an array of objects, and each object contains a `primitives` field, which is also an array of objects. So, presumably, `#Mesh0/Primitive0` is a way of accessing that first primitive in that first mesh.

Note that it may also seem a bit redundant to put a `cube.gltf` file in a `cube` directory in `models` and to have to refer to it as `models/cube/cube.gltf`, but remember that there is also a `cube.bin`. Keeping all sorts of `.gltf` and `.bin` files for all sorts of models in one directory may get messy, so perhaps it's best to put everything required for a model in its own directory.

---

Next in `setup`, we've got this block of code

```rust
    // All assets end up in their Assets<T> collection once they are done loading:
    if let Some(sphere) = meshes.get(&sphere_handle) {
        // You might notice that this doesn't run! This is because assets load in parallel without
        // blocking. When an asset has loaded, it will appear in relevant Assets<T>
        // collection.
        info!("{:?}", sphere.primitive_topology());
    } else {
        info!("sphere hasn't loaded yet");
    }
```

This comment is worded a bit confusingly, maybe. When I run this app, I see the following log line

```
2024-02-10T13:52:36.026153Z  INFO daily_bevy: sphere hasn't loaded yet
```

What the comments in the above code block are saying is that we _expect_ the "sphere hasn't loaded yet" message because the `asset_server.load()` call we made earlier is non-blocking. So we register the fact that we need to load this asset, but that happens "in the background". (How, specifically, this happens might be system-dependent. I would guess that maybe we use a separate thread in desktop environments, but that is not possible in a web browser, for example.)

So the sphere _will_ load, eventually, but it hasn't yet at this point in the program.

---

We've got another big comment above the next line

```rust
    // You can load all assets in a folder like this. They will be loaded in parallel without
    // blocking. The LoadedFolder asset holds handles to each asset in the folder. These are all
    // dependencies of the LoadedFolder asset, meaning you can wait for the LoadedFolder asset to
    // fire AssetEvent::LoadedWithDependencies if you want to wait for all assets in the folder
    // to load.
    // If you want to keep the assets in the folder alive, make sure you store the returned handle
    // somewhere.
    let _loaded_folder: Handle<LoadedFolder> = asset_server.load_folder("models/torus");
```

Here we go: some asset dependencies! The comment above explains that you can load an entire directory of assets, and then the `LoadedFolder` has dependencies on all the assets in that directory

Somehow, `LoadedFolder` also just `derive`s `Asset`, though...

```rust
#[derive(Asset, TypePath)]
pub struct LoadedFolder {
    #[dependency]
    pub handles: Vec<UntypedHandle>,
}
```

It seems like the `VisitAssetDependencies` `trait` is only ever a [non-noop](https://en.wikipedia.org/wiki/NOP_(code)) for `Handle`-like types

```rust
impl<A: Asset> VisitAssetDependencies for Handle<A> {
    fn visit_dependencies(&self, visit: &mut impl FnMut(UntypedAssetId)) {
        visit(self.id().untyped());
    }
}
```

The `load_folder` example also explicitly specifies the type returned -- it's a `Handle`. We discussed these a bit in a previous kata, but here's a refresher

```rust
/// A strong or weak handle to a specific [`Asset`]. If a [`Handle`] is [`Handle::Strong`], the [`Asset`] will be kept
/// alive until the [`Handle`] is dropped. If a [`Handle`] is [`Handle::Weak`], it does not necessarily reference a live [`Asset`],
/// nor will it keep assets alive.
///
/// [`Handle`] can be cloned. If a [`Handle::Strong`] is cloned, the referenced [`Asset`] will not be freed until _all_ instances
/// of the [`Handle`] are dropped.
///
/// [`Handle::Strong`] also provides access to useful [`Asset`] metadata, such as the [`AssetPath`] (if it exists).
#[derive(Component, Reflect)]
#[reflect(Component)]
pub enum Handle<A: Asset> {
    /// A "strong" reference to a live (or loading) [`Asset`]. If a [`Handle`] is [`Handle::Strong`], the [`Asset`] will be kept
    /// alive until the [`Handle`] is dropped. Strong handles also provide access to additional asset metadata.  
    Strong(Arc<StrongHandle>),
    /// A "weak" reference to an [`Asset`]. If a [`Handle`] is [`Handle::Weak`], it does not necessarily reference a live [`Asset`],
    /// nor will it keep assets alive.
    Weak(AssetId<A>),
}
```

The single-asset `asset_server.load()` calls above also return `Handle`s to those assets.

The next line and comment explains how to pull a specific asset out of a folder of assets

```rust
    // If you want a handle to a specific asset in a loaded folder, the easiest way to get one is to call load.
    // It will _not_ be loaded a second time.
    // The LoadedFolder asset will ultimately also hold handles to the assets, but waiting for it to load
    // and finding the right handle is more work!
    let torus_handle = asset_server.load("models/torus/torus.gltf#Mesh0/Primitive0");
```

...basically, just use `.load()` as normal!

---

Finally...

```rust
    // You can also add assets directly to their Assets<T> storage:
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });
```

...we directly add an `Asset` to the `mut materials: ResMut<Assets<StandardMaterial>>` that `setup` takes as an argument.

---

With all of that in place, we simply `spawn` all of our models, as well as our light source and our camera

```rust
    // torus
    commands.spawn(PbrBundle {
        mesh: torus_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(-3.0, 0.0, 0.0),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: cube_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    // sphere
    commands.spawn(PbrBundle {
        mesh: sphere_handle,
        material: material_handle,
        transform: Transform::from_xyz(3.0, 0.0, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
```

Notice that we use the shape `handle`s here in the `mesh` fields of the `PbrBundle`s.

---

This example was great! Probably the best-commented `example` we've seen so far from the Bevy repo. It makes it much easier to work through these when the relevant bits are commented.

Hopefully you understand a bit more about how models are loaded, where they're loaded from, what glTF files are, and how to explore them. This definitely cleared a lot up for me, and I hope it has for you, as well. See you tomorrow!

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
