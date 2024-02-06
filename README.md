# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the ninth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## 3D Shapes

Today is the ninth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, I'll be digging into the [`3d_shapes` example](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/3d/3d_shapes.rs) found in the Bevy repo. 

#### The Code

Here's the `main.rs` for this example

```rust
//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        meshes.add(shape::Cube::default().into()),
        meshes.add(shape::Box::default().into()),
        meshes.add(shape::Capsule::default().into()),
        meshes.add(shape::Torus::default().into()),
        meshes.add(shape::Cylinder::default().into()),
        meshes.add(shape::Icosphere::default().try_into().unwrap()),
        meshes.add(shape::UVSphere::default().into()),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                    .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.12.1"
```

#### Discussion

We could (and some people might prefer if we) move "linearly" through the examples, slowly building up lots of knowledge in one area before moving to the next thing, but it's easy to get bored if you're just looking at terminal output and rendered text all day. So today let's do something a bit more exciting -- let's do something 3D!

It's just as simple to render 3D graphics in Bevy as it is to render 2D ones.

Starting at the top, we've got...

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}
```

...an `App` with a `setup` system in the `Startup` schedule, a system called `rotate` in the `Update` schedule, and the usual `DefaultPlugins`, but with something `set`? What does calling `.set` on `DefaultPlugins` do?

```rust
/// Combines multiple [`Plugin`]s into a single unit.
pub trait PluginGroup: Sized {
    /// Configures the [`Plugin`]s that are to be added.
    fn build(self) -> PluginGroupBuilder;

    // -- snip --

    /// Sets the value of the given [`Plugin`], if it exists
    fn set<T: Plugin>(self, plugin: T) -> PluginGroupBuilder {
        self.build().set(plugin)
    }
}
```

So `set` `build`s the `PluginGroupBuilder` and then calls `set` on _that_

```rust
/// Sets the value of the given [`Plugin`], if it exists.
// -- snip --
pub fn set<T: Plugin>(mut self, plugin: T) -> Self {
    let entry = self.plugins.get_mut(&TypeId::of::<T>()).unwrap_or_else(|| {
        panic!(
            "{} does not exist in this PluginGroup",
            std::any::type_name::<T>(),
        )
    });
    entry.plugin = Box::new(plugin);
    self
}
```

`PluginGroupBuilder`'s `set` method `panic`s if this `PluginGroup` _doesn't_ contain a `Plugin` of the provided type. If it _does_, though, it overwrites the existing `Plugin` with the one provided.

So `DefaultPlugins.set(ImagePlugin::default_nearest())` is overriding the default `ImagePlugin` in the `DefaultPlugins` `PluginGroup` with `ImagePlugin::default_nearest()`. The default `ImagePlugin` is `linear`

```rust
impl Default for ImagePlugin {
    fn default() -> Self {
        ImagePlugin::default_linear()
    }
}
```

These are the only two predefined `ImagePlugin`s, the default `linear()` one, and the one we're overriding to: `nearest()`

```rust
impl ImagePlugin {
    /// Creates image settings with linear sampling by default.
    pub fn default_linear() -> ImagePlugin {
        ImagePlugin {
            default_sampler: ImageSamplerDescriptor::linear(),
        }
    }

    /// Creates image settings with nearest sampling by default.
    pub fn default_nearest() -> ImagePlugin {
        ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }
    }
}
```

Similarly, these are the only two predefined `ImageSamplerDescriptor`s

```rust
/// Returns a sampler descriptor with [`Linear`](crate::render_resource::FilterMode::Linear) min and mag filters
#[inline]
pub fn linear() -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..Default::default()
    }
}

/// Returns a sampler descriptor with [`Nearest`](crate::render_resource::FilterMode::Nearest) min and mag filters
#[inline]
pub fn nearest() -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        mag_filter: ImageFilterMode::Nearest,
        min_filter: ImageFilterMode::Nearest,
        mipmap_filter: ImageFilterMode::Nearest,
        ..Default::default()
    }
}
```

...and these are the only two `ImageFilterMode`s

```rust
/// Texel mixing mode when sampling between texels.
///
/// This type mirrors [`wgpu::FilterMode`].
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum ImageFilterMode {
    /// Nearest neighbor sampling.
    ///
    /// This creates a pixelated effect when used as a mag filter.
    #[default]
    Nearest,
    /// Linear Interpolation.
    ///
    /// This makes textures smooth but blurry when used as a mag filter.
    Linear,
}
```

What's the difference between "nearest neighbor sampling" and "linear interpolation" when it comes to images? This [Quora answer](https://qr.ae/pKdluw) gives a pretty succinct explanation

> "In nearest neighbor interpolation, the value of each pixel in the output image is determined by taking the value of the nearest pixel in the input image. This method is fast and simple but can result in jagged edges and a loss of detail.
> ...
> Linear interpolation uses a linear function to interpolate the values of pixels between two neighboring pixels in the input image. It produces smoother results than nearest neighbor interpolation and preserves more detail."

So we're swapping out the `default_linear` `ImagePlugin` for the `default_nearest` `ImagePlugin`. How does this affect the resulting textures? Try swapping one out for the other and see!

---

Next we've got a marker component and a constant length of some kind

```rust
/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;
```

followed by the `setup` system

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // -- snip --
}
```

The signature here has several things we haven't seen before
- `Assets`
- `Mesh`
- `Image`, and
- `StandardMaterial`

Let's start with `Assets`.

---

`Assets` is a `struct` and a `Resource`. It looks like this

```rust
/// Stores [`Asset`] values identified by their [`AssetId`].
///
/// Assets identified by [`AssetId::Index`] will be stored in a "dense" vec-like storage. This is more efficient, but it means that
/// the assets can only be identified at runtime. This is the default behavior.
///
/// Assets identified by [`AssetId::Uuid`] will be stored in a hashmap. This is less efficient, but it means that the assets can be referenced
/// at compile time.
///
/// This tracks (and queues) [`AssetEvent`] events whenever changes to the collection occur.
#[derive(Resource)]
pub struct Assets<A: Asset> {
    dense_storage: DenseAssetStorage<A>,
    hash_map: HashMap<Uuid, A>,
    handle_provider: AssetHandleProvider,
    queued_events: Vec<AssetEvent<A>>,
}
```

Okay, so what's an `Asset`?

```rust
pub trait Asset: VisitAssetDependencies + TypePath + Send + Sync + 'static {}
```

Oh, that's not very helpful. It's just a trait which must implement all these other traits. Maybe `VisitAssetDependencies` explains some of it?

```rust
pub trait VisitAssetDependencies {
    fn visit_dependencies(&self, visit: &mut impl FnMut(UntypedAssetId));
}
```

Hmm... nope. Maybe looking at _an implementation_ of `Asset` will be helpful?

```rust
#[derive(Asset, TypePath, Debug, Clone)]
pub struct Font {
    pub font: FontArc,
}
```

Still no, not really. `AssetPlugin` has a short explanation of what an `Asset` is, though

> "An `Asset` is a 'runtime value' that is loaded from an `AssetSource`, which can be something like a filesystem, a network, etc."

Okay... so it's just some kind of marker trait that Bevy understands, I guess, but there are basically no restrictions on what can _be_ an `Asset`, including

```rust
impl Asset for () {}
```

¯\_(ツ)_/¯

There's a lot more to explore around `Asset`s, but let's save that for future katas.

---

The next new thing is `Mesh`, which is _a kind of_ `Asset`, and has a ton of good documentation above it

```rust
/// A 3D object made out of vertices representing triangles, lines, or points,
/// with "attribute" values for each vertex.
///
/// Meshes can be automatically generated by a bevy `AssetLoader` (generally by loading a `Gltf` file),
/// or by converting a primitive [`shape`](crate::mesh::shape) using [`into`](std::convert::Into).
/// It is also possible to create one manually.
/// They can be edited after creation.
///
/// Meshes can be rendered with a `Material`, like `StandardMaterial` in `PbrBundle`
/// or `ColorMaterial` in `ColorMesh2dBundle`.
///
/// A [`Mesh`] in Bevy is equivalent to a "primitive" in the glTF format, for a
/// glTF Mesh representation, see `GltfMesh`.
// -- snip --
#[derive(Asset, Debug, Clone, Reflect)]
pub struct Mesh {
    #[reflect(ignore)]
    primitive_topology: PrimitiveTopology,
    // -- snip --
    #[reflect(ignore)]
    attributes: BTreeMap<MeshVertexAttributeId, MeshAttributeData>,
    indices: Option<Indices>,
    morph_targets: Option<Handle<Image>>,
    morph_target_names: Option<Vec<String>>,
}
```

A mesh is the ["shape" of a 3D model](https://gamedev.stackexchange.com/a/38414). It doesn't include information like texture, reflectivity, etc., but it does describe the general 3D space that the model takes up.

The docs above say we can load a mesh using an `AssetLoader`, from a `Gltf` file, or we can create a primitive `mesh::shape`. We will see this last method later in this kata.

I guess the `primitive_topology` and `indices` describe the "shape" of the model and the `attributes` give additional data at each vertex, but I'll have to dig into this more at a later date.

For now, if you're interested in learning more, check out the documentation above `Mesh`.

---

Next up: `Image`

```rust
#[derive(Asset, Reflect, Debug, Clone)]
#[reflect_value]
pub struct Image {
    pub data: Vec<u8>,
    // TODO: this nesting makes accessing Image metadata verbose. Either flatten out descriptor or add accessors
    pub texture_descriptor: wgpu::TextureDescriptor<'static>,
    /// The [`ImageSampler`] to use during rendering.
    pub sampler: ImageSampler,
    pub texture_view_descriptor: Option<wgpu::TextureViewDescriptor<'static>>,
}
```

Okay, an image is another `Asset`. It has `data` which is a vector of bytes, and a few other fields. Presumably the `data` is just the serialized bitmap, so why do we need the other fields?

```rust
/// Describes a [`Texture`].
///
/// For use with [`Device::create_texture`].
///
/// Corresponds to [WebGPU `GPUTextureDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gputexturedescriptor).
pub type TextureDescriptor<'a> = wgt::TextureDescriptor<Label<'a>, &'a [TextureFormat]>;
static_assertions::assert_impl_all!(TextureDescriptor: Send, Sync);
```

We're digging down into [WebGPU](https://gpuweb.github.io/gpuweb/#dictdef-gputexturedescriptor) docs here. I'm sure the people at Bevy know what they're doing, but this is a bit opaque to me.

All of this is a bit over my head. Maybe we'll dig into this in a later kata. For now, let's move onto `StandardMaterial`

---

`StandardMaterial` looks like this (with most comments and attributes removed)

```rust
/// A material with "standard" properties used in PBR lighting
/// Standard property values with pictures here
/// <https://google.github.io/filament/Material%20Properties.pdf>.
///
/// May be created directly from a [`Color`] or an [`Image`].
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct StandardMaterial {
    pub base_color: Color,
    pub base_color_texture: Option<Handle<Image>>,
    pub emissive: Color,
    pub emissive_texture: Option<Handle<Image>>,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub metallic_roughness_texture: Option<Handle<Image>>,
    pub reflectance: f32,
    pub diffuse_transmission: f32,
    pub diffuse_transmission_texture: Option<Handle<Image>>,
    pub specular_transmission: f32,
    pub specular_transmission_texture: Option<Handle<Image>>,
    pub thickness: f32,
    pub thickness_texture: Option<Handle<Image>>,
    pub ior: f32,
    pub attenuation_distance: f32,
    pub attenuation_color: Color,
    pub normal_map_texture: Option<Handle<Image>>,
    pub flip_normal_map_y: bool,
    pub occlusion_texture: Option<Handle<Image>>,
    pub double_sided: bool,
    pub cull_mode: Option<Face>,
    pub unlit: bool,
    pub fog_enabled: bool,
    pub alpha_mode: AlphaMode,
    pub depth_bias: f32,
    pub depth_map: Option<Handle<Image>>,
    pub parallax_depth_scale: f32,
    pub parallax_mapping_method: ParallaxMappingMethod,
    pub max_parallax_layer_count: f32,
    pub opaque_render_method: OpaqueRendererMethod,
    pub deferred_lighting_pass_id: u8,
}
```

There is a _ton_ of stuff here. And a ton of documentation. ([The linked PDF](https://google.github.io/filament/Material%20Properties.pdf) is very informative, as well.) There are about 35 lines of code above, but more than 400 lines of comments have been removed. So if you really want to dig into this one, there's a ton of learning material. I'll just pick out a few things, though.

First, `StandardMaterial` is another `Asset`. It has a `base_color` and an optional `base_color_texture`, which I guess correspond to the documentation line which says "may be created directly from a `Color` or an `Image`".

But there are also tons of physical material properties here: roughness, reflectance, diffuse and specular transmission, etc. Clearly, there are a lot of ways to define a lot of different materials in Bevy.

We will use it in this kata in only those two simplest-possible ways: creating a material from just a color, and also from an image.

---

Right, so, `setup`. In the body of this system, we first create a `StandardMaterial` and add it to the list of `materials`, which is a `ResMut<Assets<StandardMaterial>>` (a mutable reference to a `Resource` of type `Assets<StandardMaterial>`).

> Aside: my Java brain says that, since `Res` refers to a `Resource`, and `Resource`s are singletons, then a `Resource` of `Assets<StandardMaterial>` and a `Resource` of `Assets<SomeOtherType>` must be the same `Assets`, because of type erasure. I have to remind myself that [this is not a thing in Rust, that these generic types are monomorphized](https://stackoverflow.com/a/32546093/2925434) upon compilation, and that they are indeed two _different_ singletons.

```rust
let debug_material = materials.add(StandardMaterial {
    base_color_texture: Some(images.add(uv_debug_texture())),
    ..default()
});
```

This material is constructed from the `Image` returned from `uv_debug_texture()`, defined at the bottom of this example

```rust
/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
```

In `uv_debug_texture`, we define a bunch of integers in different arrangements and manipulate them, then... rotate? them... and then somehow turn that into an `Image`. How does this work?

Well, `Image::new_fill()` looks like this

```rust
/// Creates a new image from raw binary data and the corresponding metadata, by filling
/// the image data with the `pixel` data repeated multiple times.
///
/// # Panics
/// Panics if the size of the `format` is not a multiple of the length of the `pixel` data.
pub fn new_fill(
    size: Extent3d,
    dimension: TextureDimension,
    pixel: &[u8],
    format: TextureFormat,
) -> Self {
    // -- snip --
}
```

So we pass in `texture_data` for `pixel`, which is just a slice of bytes, say that the `TextureDimension` is `D2` (aka `2D`), define the size of the image fill which is 3D for some reason, and finally also provide a `TextureFormat`.

`TextureFormat` is one of _many_ different values which map to WebGPU texture formats

```rust
/// Underlying texture data format.
///
/// If there is a conversion in the format (such as srgb -> linear), the conversion listed here is for
/// loading from texture in a shader. When writing to the texture, the opposite conversion takes place.
///
/// Corresponds to [WebGPU `GPUTextureFormat`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gputextureformat).
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum TextureFormat {
    // -- snip --

    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    Rgba8UnormSrgb,

    // -- snip --
}
```

...but the one we care about says it needs 8-bit [red, green, blue, and alpha channels](https://en.wikipedia.org/wiki/RGBA_color_model) for each color. That's what our  `texture_data` is

If you add a line like

```rust
println!("{:?}", texture_data);
```

above `Image::new_fill()`, you'll see that `texture_data` is just a slightly longer array of bytes, where every fourth byte is 255. Every four subsequent bytes define a single pixel, and the alpha value is always 100% (255/255).

The original `palette` defined eight colors, and the `texture_data` just repeats each of those eight colors eight times.

We have eight colors, in an 8x8 grid. We draw the eight colors left-to-right in the first row, and then the `rotate_right` _moves_ the _first_ color in the array to the _end_ of the array. This gives a nice rainbow checkerboard effect. Try removing the `rotate_right` line and seeing what the resulting texture looks like.

Try `rotate_right(3)` or `rotate_right(5)`. What happens? Can you explain why?

---

After we define the `debug_material` in `setup`, we create a bunch of shapes

```rust
let shapes = [
    meshes.add(shape::Cube::default().into()),
    meshes.add(shape::Box::default().into()),
    meshes.add(shape::Capsule::default().into()),
    meshes.add(shape::Torus::default().into()),
    meshes.add(shape::Cylinder::default().into()),
    meshes.add(shape::Icosphere::default().try_into().unwrap()),
    meshes.add(shape::UVSphere::default().into()),
];

let num_shapes = shapes.len();
```

These are the "primitive" shapes described earlier. Each of these `shape`s `impl`ements `Mesh`.

If you're wondering "what's an `Icosphere`?" So was I. The short answer is that, with a middling number of polygons, [a UV sphere kind of looks like a disco ball, but an icosphere kind of looks like a golf ball](https://blender.stackexchange.com/a/73).

Note that "UV" does not stand for "ultraviolet" (as I thought, with my physics brain), but for [the coordinate axes of the 2D texture](https://en.wikipedia.org/wiki/UV_mapping), which are `u` and `v` instead of `x` and `y`.

---

Next, for each shape, we define a `PbrBundle` using that "primitive mesh" to define the structure of the object and our colorful `debug_material` to paint it

```rust
for (i, shape) in shapes.into_iter().enumerate() {
    commands.spawn((
        PbrBundle {
            mesh: shape,
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                2.0,
                0.0,
            )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        },
        Shape,
    ));
}
```

Each shape is translated along the `x` axis, offset a bit from the `y` axis, and rotated a bit. We are familiar with all of these `Transform`s by now.

The thing that's new here is the `PbrBundle`. What's that?

```rust
/// A component bundle for PBR entities with a [`Mesh`] and a [`StandardMaterial`].
pub type PbrBundle = MaterialMeshBundle<StandardMaterial>;

/// A component bundle for entities with a [`Mesh`] and a [`Material`].
#[derive(Bundle, Clone)]
pub struct MaterialMeshBundle<M: Material> {
    pub mesh: Handle<Mesh>,
    pub material: Handle<M>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}
```

What does PBR stand for (because my head goes to [this PBR](https://en.wikipedia.org/wiki/Pabst_Blue_Ribbon))?

["Physically-based rendering."](https://en.wikipedia.org/wiki/Physically_based_rendering)

Okay, so a `PbrBundle` is a `MaterialMeshBundle` where the `Material` must be a `StandardMaterial`. What other kinds of `Material`s are there?

There are also `ExtendedMaterial`s

```rust
impl<B: Material, E: MaterialExtension> Material for ExtendedMaterial<B, E> {
    // -- snip --
}
```

and `WireframeMaterial`s

```rust
impl Material for WireframeMaterial {
    // -- snip --
}
```

...apparently we can't use these (yet).

But a `MaterialMeshBundle` contains a lot of `Component`s we have seen before: a `Mesh`, a `Material`, a `Transform`, etc.

There's also this wrapper type `Handle` here

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

I suppose this has to do with the fact that `Asset` is a super generic thing -- we could have a network connection or an image file or whatever else stored in an `Asset`. So we have different guarantees about the "liveness" of an `Asset` based on the kind of `Handle` we have to it.

We'll see `PbrBundle`s in _a lot_ of future katas. We can dig into it a bit more later.

---

Once we've `spawn`ed all of our `shape`s, we have three more entities to spawn into our world

1. a light source
2. the ground to see shadows cast by our `shape`s, and
3. a `Camera3dBundle`, to view the scene

Technically, we do not _need_ (1) or (2). But it makes the example look nicer. Try deleting them to see what the scene looks like without them.

---

The light source is a `PointLightBundle`

```rust
commands.spawn(PointLightBundle {
    point_light: PointLight {
        intensity: 9000.0,
        range: 100.,
        shadows_enabled: true,
        ..default()
    },
    transform: Transform::from_xyz(8.0, 16.0, 8.0),
    ..default()
});
```

A `PointLightBundle` looks like

```rust
/// A component bundle for [`PointLight`] entities.
#[derive(Debug, Bundle, Default)]
pub struct PointLightBundle {
    pub point_light: PointLight,
    pub cubemap_visible_entities: CubemapVisibleEntities,
    pub cubemap_frusta: CubemapFrusta,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// Enables or disables the light
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub view_visibility: ViewVisibility,
}
```

We have the typical `Visibility` and `Transform` `Component`s, but then we also have

- `PointLight`
- `CubemapVisibleEntities`, and
- `CubemapFrusta`

...what on earth are these things?

`PointLight` is straightforward enough

```rust
/// A light that emits light in all directions from a central point.
// -- snip --
#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component, Default)]
pub struct PointLight {
    pub color: Color,
    /// Luminous power in lumens
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub shadows_enabled: bool,
    pub shadow_depth_bias: f32,
    // -- snip --
    pub shadow_normal_bias: f32,
}
```

Color, intensity, range, radius... okay, that all makes sense.

But `CubemapVisibleEntities` is a bit more mysterious... it has no documentation

```rust
#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CubemapVisibleEntities {
    #[reflect(ignore)]
    data: [VisibleEntities; 6],
}
```

Given that it contains a length-6 `data` array of `VisibleEntities`...

```rust
/// Collection of entities visible from the current view.
///
/// This component contains all entities which are visible from the currently
/// rendered view. The collection is updated automatically by the [`VisibilitySystems::CheckVisibility`]
/// system set, and renderers can use it to optimize rendering of a particular view, to
/// prevent drawing items not visible from that view.
///
/// This component is intended to be attached to the same entity as the [`Camera`] and
/// the [`Frustum`] defining the view.
#[derive(Clone, Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct VisibleEntities {
    #[reflect(ignore)]
    pub entities: Vec<Entity>,
}
```

...I'd guess this is just a way to identify which faces of some cube are visible? (You can only ever see at most three sides of a cube at once in 3D space without a mirror -- try it with a pair of dice.)

But why do we have a cube in here? I thought this was a point source of light?

Similarly, the `CubemapFrusta` contains a length-6 array of `Frustum`s

```rust
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CubemapFrusta {
    #[reflect(ignore)]
    pub frusta: [Frustum; 6],
}
```

where a `Frustum` is defined as

```rust
pub struct Frustum {
    #[reflect(ignore)]
    pub half_spaces: [HalfSpace; 6],
}
```

`Frustum` and `HalfSpace` both have good documentation, but I won't reproduce all of it here. We're already getting into the weeds a bit.

So that's as far as I'll dig into the `PointLightBundle` for now.

---

Next up is the `PbrBundle`, which we've seen already

```rust
// ground plane
commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(50.0).into()),
    material: materials.add(Color::SILVER.into()),
    ..default()
});
```

This `mesh` is created from a square `Plane` of side length `50.0`, centered at the origin

```rust
impl Plane {
    /// Creates a new plane centered at the origin with the supplied side length and zero subdivisions.
    pub fn from_size(size: f32) -> Self {
        Self {
            size,
            subdivisions: 0,
        }
    }
}
```

---

Finally, we have the 3D analog of the `Camera2dBundle` (which we spent a few days diving into) -- the `Camera3dBundle`

```rust
commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ..default()
});
```

How do `Camera2dBundle` and `Camera3dBundle` differ? Well, surely their `impl`ementations differ quite a bit, but from a `struct`ural perspective

- `Camera2dBundle` has a `pub projection: OrthographicProjection` while `Camera3dBundle` has a `pub projection: Projection`
- both have a `DebandDither` field, but it is called `deband_dither` in `Camera2dBundle` and `dither` in `Camera3dBundle`
- `Camera3dBundle` has a `pub color_grading: ColorGrading` field, while `Camera2dBundle` does not

Surely we'll dig into the reasons behind these differences in future katas.

---

So, bringing it all together, in the `Update` `Schedule`, we run the `rotate` system, which is defined as follows

```rust
fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}
```

Every `Transform`-able `Shape` gets rotated at the same rate, forever.

---

So that's it! We've got a 3D scene, with a camera, some ground, a light source, and a bunch of colorful, rotating shapes which cast shadows.

And all in only about 100 lines of Rust code, with no dependencies beyond Bevy.

And less than 10 days into Daily Bevy! I think we're doing pretty well so far.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
