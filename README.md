# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the fifth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Bonus: Camera2dBundle

Today is the fifth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, I will be digging into `Camera2dBundle` by reviewing a few articles

The articles
- https://bevy-cheatbook.github.io/graphics/camera.html
- https://bevy-cheatbook.github.io/2d/camera.html
- https://taintedcoders.com/bevy/cameras/

#### Discussion

Today, we're forgoing the usual structure of these katas. Instead of presenting some code and dissecting it, we'll be dissecting a few _articles_ to learn how cameras in Bevy work.

Let's start here: https://taintedcoders.com/bevy/cameras/

> Bevy’s cameras by default use an orthogonal projection with a symmetric [frustum].

We "look through" the camera at a scene and "further away" objects appear "behind" "nearer" objects.

![frustum](https://raw.githubusercontent.com/awwsmm/daily-bevy/bonus/Camera2dBundle/assets/camera.png)

[[source]](https://www.researchgate.net/figure/Standard-virtual-camera-parameters_fig1_270888488)

[The _frustum_](https://relativity.net.au/gaming/java/Frustum.html) is the "viewable space", from the leftmost point to the rightmost point, the topmost point to the bottommost point, and the nearest point to the furthest point. The frustum is the shaded-blue volume in the above image.

A "symmetric frustum" just means the top and bottom are parallel, the left and right sides are parallel, and that those four sides meet at right angles, defining a rectangle.

An "orthogonal projection" means that if we render the same object twice, but one is further away, they will appear to be the same size. This is in contrast to a "perspective projection", which is what is shown in the above image, where "further away" objects are smaller than "nearer" objects.

In an orthonal projection, the frustum will look more like a cube or a rectangular prism, rather than (as it does in a perspective projection) a pyramid with its top cut off.

Remember I said that we "look through" a camera

> You must have at least one camera entity, in order for anything to be displayed at all! If you forget to spawn a camera, you will get an empty black screen. [[source]](https://bevy-cheatbook.github.io/graphics/camera.html)

Without a camera, nothing will be rendered. Bevy needs to know _from what perspective_ we are viewing a scene, before it can calculate what pixels to color, and in what way.

Bevy offers two default camera implementations, `Camera2dBundle` and `Camera3dBundle`. For this kata, we will only be discussing the 2D camera. Here's what that looks like

```rust
#[derive(Bundle)]
pub struct Camera2dBundle {
    pub camera: Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection: OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera_2d: Camera2d,
    pub tonemapping: Tonemapping,
    pub deband_dither: DebandDither,
}
```

---

Remember a `Bundle` is a collection of `Component`s. The first `Component` is the `Camera`

```rust
/// The defining [`Component`] for camera entities,
/// storing information about how and what to render through this camera.
///
/// The [`Camera`] component is added to an entity to define the properties of the viewpoint from
/// which rendering occurs. It defines the position of the view to render, the projection method
/// to transform the 3D objects into a 2D image, as well as the render target into which that image
/// is produced.
// -- snip --
pub struct Camera {
    /// If set, this camera will render to the given [`Viewport`] rectangle within the configured [`RenderTarget`].
    pub viewport: Option<Viewport>,
    /// Cameras with a higher order are rendered later, and thus on top of lower order cameras.
    pub order: isize,
    /// If this is set to `true`, this camera will be rendered to its specified [`RenderTarget`]. If `false`, this
    /// camera will not be rendered.
    pub is_active: bool,
    /// Computed values for this camera, such as the projection matrix and the render target size.
    #[reflect(ignore)]
    pub computed: ComputedCameraValues,
    /// The "target" that this camera will render to.
    #[reflect(ignore)]
    pub target: RenderTarget,
    /// If this is set to `true`, the camera will use an intermediate "high dynamic range" render texture.
    /// This allows rendering with a wider range of lighting values.
    pub hdr: bool,
    // -- snip --
}
```

Note that we can (optionally) render a `Camera` to a specific `Viewport`, and we can `order` `Camera`s, placing one camera's "view" on top of another one's view. Why would we want to do that?

```rust
/// Render viewport configuration for the [`Camera`] component.
///
/// The viewport defines the area on the render target to which the camera renders its image.
/// You can overlay multiple cameras in a single window using viewports to create effects like
/// split screen, minimaps, and character viewers.
// -- snip --
pub struct Viewport {
    /// The physical position to render this viewport to within the [`RenderTarget`] of this [`Camera`].
    /// (0,0) corresponds to the top-left corner
    pub physical_position: UVec2,
    /// The physical size of the viewport rectangle to render to within the [`RenderTarget`] of this [`Camera`].
    /// The origin of the rectangle is in the top-left corner.
    pub physical_size: UVec2,
    /// The minimum and maximum depth to render (on a scale from 0.0 to 1.0).
    pub depth: Range<f32>,
}
```

Note, in particular

> You can overlay multiple cameras in a single window using viewports to create effects like split screen, minimaps, and character viewers.

---

We've also got an `OrthographicProjection` `Component` in the `Camera2dBundle` (I purposefully skipped the not-very-exciting-looking `CameraRenderGraph` `Component`).

There is a ton of fantastic documentation here

```rust
/// Project a 3D space onto a 2D surface using parallel lines, i.e., unlike [`PerspectiveProjection`],
/// the size of objects remains the same regardless of their distance to the camera.
///
/// The volume contained in the projection is called the *view frustum*. Since the viewport is rectangular
/// and projection lines are parallel, the view frustum takes the shape of a cuboid.
///
/// Note that the scale of the projection and the apparent size of objects are inversely proportional.
/// As the size of the projection increases, the size of objects decreases.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct OrthographicProjection {
    /// The distance of the near clipping plane in world units.
    ///
    /// Objects closer than this will not be rendered.
    ///
    /// Defaults to `0.0`
    pub near: f32,
    /// The distance of the far clipping plane in world units.
    ///
    /// Objects further than this will not be rendered.
    ///
    /// Defaults to `1000.0`
    pub far: f32,
    /// Specifies the origin of the viewport as a normalized position from 0 to 1, where (0, 0) is the bottom left
    /// and (1, 1) is the top right. This determines where the camera's position sits inside the viewport.
    ///
    /// When the projection scales due to viewport resizing, the position of the camera, and thereby `viewport_origin`,
    /// remains at the same relative point.
    ///
    /// Consequently, this is pivot point when scaling. With a bottom left pivot, the projection will expand
    /// upwards and to the right. With a top right pivot, the projection will expand downwards and to the left.
    /// Values in between will caused the projection to scale proportionally on each axis.
    ///
    /// Defaults to `(0.5, 0.5)`, which makes scaling affect opposite sides equally, keeping the center
    /// point of the viewport centered.
    pub viewport_origin: Vec2,
    /// How the projection will scale when the viewport is resized.
    ///
    /// Defaults to `ScalingMode::WindowSize(1.0)`
    pub scaling_mode: ScalingMode,
    /// Scales the projection in world units.
    ///
    /// As scale increases, the apparent size of objects decreases, and vice versa.
    ///
    /// Defaults to `1.0`
    pub scale: f32,
    /// The area that the projection covers relative to `viewport_origin`.
    ///
    /// Bevy's [`camera_system`](crate::camera::camera_system) automatically
    /// updates this value when the viewport is resized depending on `OrthographicProjection`'s other fields.
    /// In this case, `area` should not be manually modified.
    ///
    /// It may be necessary to set this manually for shadow projections and such.
    pub area: Rect,
}
```

`ScalingMode` is the only thing that stands out to me here. It's an enum -- there is a set number of `ScalingMode`s

```rust
pub enum ScalingMode {
    /// Manually specify the projection's size, ignoring window resizing. The image will stretch.
    /// Arguments are in world units.
    Fixed { width: f32, height: f32 },
    /// Match the viewport size.
    /// The argument is the number of pixels that equals one world unit.
    WindowSize(f32),
    /// Keeping the aspect ratio while the axes can't be smaller than given minimum.
    /// Arguments are in world units.
    AutoMin { min_width: f32, min_height: f32 },
    /// Keeping the aspect ratio while the axes can't be bigger than given maximum.
    /// Arguments are in world units.
    AutoMax { max_width: f32, max_height: f32 },
    /// Keep the projection's height constant; width will be adjusted to match aspect ratio.
    /// The argument is the desired height of the projection in world units.
    FixedVertical(f32),
    /// Keep the projection's width constant; height will be adjusted to match aspect ratio.
    /// The argument is the desired width of the projection in world units.
    FixedHorizontal(f32),
}
```

---

Next in the `Camera2dBundle`, we've got `VisibleEntities`... do we need to keep track of these ourselves?

```rust
/// This component contains all entities which are visible from the currently
/// rendered view. The collection is updated automatically by the [`VisibilitySystems::CheckVisibility`]
/// system set, and renderers can use it to optimize rendering of a particular view, to
/// prevent drawing items not visible from that view.
```

Nope! Good, that's one less thing to worry about for now.

---

Next we've got our friend the `Frustum`, followed by a `Transform` component

```rust
/// Describe the position of an entity.
// -- snip --
pub struct Transform {
    /// Position of the entity. In 2d, the last value of the `Vec3` is used for z-ordering.
    ///
    /// See the [`translations`] example for usage.
    ///
    /// [`translations`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/translation.rs
    pub translation: Vec3,
    /// Rotation of the entity.
    ///
    /// See the [`3d_rotation`] example for usage.
    ///
    /// [`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs
    pub rotation: Quat,
    /// Scale of the entity.
    ///
    /// See the [`scale`] example for usage.
    ///
    /// [`scale`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/scale.rs
    pub scale: Vec3,
}
```

Here's a quick [explanation of "z-ordering"](https://en.wikipedia.org/wiki/Z-order), if you're unfamiliar.

Presumably this `Transform` transforms... this camera itself? If we want to translate or rotate the camera? Not entirely clear to me what scaling a camera should do, though...

Note that `Quat` is a [_quaternion_](https://allenchou.net/2014/04/game-math-quaternion-basics/) type. This is a 4D vector which is used to describe rotations

```rust
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
```

---

Next up we have the `GlobalTransform` `Component`. Here's some abridged documentation for that

```rust
/// * To place or move an entity, you should set its [`Transform`].
/// * [`GlobalTransform`] is fully managed by bevy, you cannot mutate it, use
///   [`Transform`] instead.
/// * To get the global transform of an entity, you should get its [`GlobalTransform`].
/// * For transform hierarchies to work correctly, you must have both a [`Transform`] and a [`GlobalTransform`].
///   * You may use the [`TransformBundle`](crate::TransformBundle) to guarantee this.
///
/// ## [`Transform`] and [`GlobalTransform`]
///
/// [`Transform`] is the position of an entity relative to its parent position, or the reference
/// frame if it doesn't have a [`Parent`](bevy_hierarchy::Parent).
///
/// [`GlobalTransform`] is the position of an entity relative to the reference frame.
///
/// [`GlobalTransform`] is updated from [`Transform`] by systems in the system set
/// [`TransformPropagate`](crate::TransformSystem::TransformPropagate).
///
/// This system runs during [`PostUpdate`](bevy_app::PostUpdate). If you
/// update the [`Transform`] of an entity in this schedule or after, you will notice a 1 frame lag
/// before the [`GlobalTransform`] is updated.
```

So we need a `GlobalTransform` and a `Transform` `Component` on any entity in order to correctly place it into the world.

---

Only a few `Component`s left. First we've got the `Camera2d`, which contains the `ClearColor` we saw yesterday

```rust
pub struct Camera2d {
    pub clear_color: ClearColorConfig,
}
```

the `DebandDither`, which is just a toggle

```rust
/// Enables a debanding shader that applies dithering to mitigate color banding in the final image for a given [`Camera`] entity.
```

and the `Tonemapping`, which has a few pretty fantastic enum variant names

```rust
pub enum Tonemapping {
    None,
    Reinhard,
    ReinhardLuminance,
    AcesFitted,
    AgX,
    SomewhatBoringDisplayTransform,
    #[default]
    TonyMcMapface,
    BlenderFilmic,
}
```

---

So, with that all dissected... what do we _do_ with a `Camera2dBundle`?

Well, we can

- get our mouse coordinates in screen pixels or world coordinates [[source]](https://taintedcoders.com/bevy/cameras/)
- scale / zoom the screen so that 1 pixel is less than or greater than 1 world coordinate unit [[source]](https://bevy-cheatbook.github.io/2d/camera.html)
- render to a specific viewport, for split-screen games [[source]](https://bevy-cheatbook.github.io/graphics/camera.html#viewport)
- overlay a [HUD](https://en.wikipedia.org/wiki/Head-up_display) [[source]](https://bevy-cheatbook.github.io/graphics/camera.html#overlays)
- render multiple windows, for example a minimap [[source]](https://bevy-cheatbook.github.io/graphics/camera.html#multiple-windows)

Each of these could be a separate kata (and indeed, many of these are explored in different Bevy examples).

Originally, I wanted to build _a few_ of these examples for today's kata, but I think it will be better if we spread this out a bit.

Repetition is our friend. Let's play more with cameras over the next few days.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).