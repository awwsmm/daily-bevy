# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the fourth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Clear Color

Today is the fourth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, I will be dissecting the [clear_color](https://github.com/bevyengine/bevy/blob/v0.12.1/examples/window/clear_color.rs) example found in the Bevy repo.

#### The Code

Here's the `main.rs` I started with

```rust
//! Shows how to set the solid color that is used to paint the window before the frame gets drawn.
//!
//! Acts as background color, since pixels that are not drawn in a frame remain unchanged.

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, change_clear_color)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn change_clear_color(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = Color::PURPLE;
    }
}
```

Here's the `Cargo.toml` I started with

```toml
[dependencies]
bevy = "0.12.1"
```

#### Discussion

This example ran "out of the box" for me, without having to change anything above. (Unlike [yesterday...](https://github.com/awwsmm/daily-bevy/tree/input/keyboard_input?tab=readme-ov-file#discussion))

If you run the example above, you'll see that it spawns a window with a [medium slate blue](https://www.htmlcsscolor.com/hex/8080E6) background.

If you press the space bar, the background changes to [purple](https://www.htmlcsscolor.com/hex/800080).

This example is not the first time we've spawned a window -- we did this in the [Drag and Drop kata](https://github.com/awwsmm/daily-bevy/tree/app/drag_and_drop) as well -- but it _feels_ like the first time we're "doing" anything with a window. We're showing some colors to the user! And changing those colours based on user input! Nice. This is like the most boring video game possible -- we always win! And the prize is the colour purple.

Let's have a look at the code.

---

We start with `.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))`.

We haven't seen `.insert_resource` before, though we have seen `.init_resource`. What's the difference?

The context in which we saw `.init_resource` before was when we were digging into the `InputPlugin`

```rust
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            // keyboard
            .add_event::<KeyboardInput>()
            .init_resource::<Input<KeyCode>>()
            .init_resource::<Input<ScanCode>>()
        // -- snip --
    }
}
```

There are a few `fn init_resource` defined throughout Bevy, but this one above is defined in the `bevy_app` crate

```rust
pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
    self.world.init_resource::<R>();
    self
}
```

...but it defers to a _different `fn init_resource` defined in the `bevy_ecs` crate. That one looks like

```rust
pub fn init_resource<R: Resource + FromWorld>(&mut self) -> ComponentId {
    let component_id = self.components.init_resource::<R>();
    if self
        .storages
        .resources
        .get(component_id)
        .map_or(true, |data| !data.is_present())
    {
        let value = R::from_world(self);
        OwningPtr::make(value, |ptr| {
            // SAFETY: component_id was just initialized and corresponds to resource of type R.
            unsafe {
                self.insert_resource_by_id(component_id, ptr);
            }
        });
    }
    component_id
}
```

This calls... another... `init_resource`, which is finally the end of the line

```rust
pub fn init_resource<T: Resource>(&mut self) -> ComponentId {
    // SAFETY: The [`ComponentDescriptor`] matches the [`TypeId`]
    unsafe {
        self.get_or_insert_resource_with(TypeId::of::<T>(), || {
            ComponentDescriptor::new_resource::<T>()
        })
    }
}
```

The fourth and final `fn init_resource` is a method on `struct Commands`, also in the `bevy_ecs` crate.

So what do these `init_resource` methods do? Ultimately, they boil down to the last one, which reads more or less like an English sentence

> "Get or insert a resource of type T."

`TypeId` is not a Bevy-specific thing. This comes from the Rust standard library

```rust
/// A `TypeId` represents a globally unique identifier for a type.
///
/// Each `TypeId` is an opaque object which does not allow inspection of what's
/// inside but does allow basic operations such as cloning, comparison,
/// printing, and showing.
///
/// A `TypeId` is currently only available for types which ascribe to `'static`,
/// but this limitation may be removed in the future.
///
/// While `TypeId` implements `Hash`, `PartialOrd`, and `Ord`, it is worth
/// noting that the hashes and ordering will vary between Rust releases. Beware
/// of relying on them inside of your code!
#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct TypeId {
    t: u128,
}
```

So a `TypeId` is literally just a wrapper around a `u128`. It's a unique numeric identifier attached to a type.

`TypeId::of::<T>` returns that `TypeId` (that `u128`) for the specified type `T`

```rust
pub const fn of<T: ?Sized + 'static>() -> TypeId {
    let t: u128 = intrinsics::type_id::<T>();
    TypeId { t }
}
```

`intrinsics` documentation starts with

```rust
//! Compiler intrinsics.
```

I'll stop there. I'll just assume the compiler knows what it's doing here.

Back in Bevyland, we're trying to `get_or_insert` a resource

```rust
/// # Safety
///
/// The [`ComponentDescriptor`] must match the [`TypeId`]
#[inline]
unsafe fn get_or_insert_resource_with(
    &mut self,
    type_id: TypeId,
    func: impl FnOnce() -> ComponentDescriptor,
) -> ComponentId {
    let components = &mut self.components;
    let index = self.resource_indices.entry(type_id).or_insert_with(|| {
        let descriptor = func();
        let index = components.len();
        components.push(ComponentInfo::new(ComponentId(index), descriptor));
        index
    });

    ComponentId(*index)
}
```

The `get` part of that is trying to pull a resource of the specified type with `self.resource_indices.entry(type_id)`. The `or_insert` part is creating a `new` `ComponentInfo` and `push`ing it to the list of `components`. In this case, `self` refers to a `World` in our `App` (remember we can have more than one, potentially).

The thing to note here is that this `get_or_insert_resource` method is _**not** creating an instance of type corresponding to the `TypeId`_. It is only creating a `ComponentInfo`. This is a bundle of information which _describes_ a `Component`.

Note that when we `or_insert_with`, we create this new `ComponentInfo` with a `ComponentId` and the `ComponentDescriptor` -- we don't actually instantiate a component.

This happens "one level up" in the "middle" `init_resource` function

```rust
pub fn init_resource<R: Resource + FromWorld>(&mut self) -> ComponentId {
    let component_id = self.components.init_resource::<R>();
    if self
        .storages
        .resources
        .get(component_id)
        .map_or(true, |data| !data.is_present())
    {
        let value = R::from_world(self);
        OwningPtr::make(value, |ptr| {
            // SAFETY: component_id was just initialized and corresponds to resource of type R.
            unsafe {
                self.insert_resource_by_id(component_id, ptr);
            }
        });
    }
    component_id
}
```

`R::from_world(self)` is the line that is _creating an instance of type `R`_. Note that it only does this if no instance of type `R` exists in `storages.resources` already.

`from_world` comes from the `FromWorld` trait

```rust
/// Creates an instance of the type this trait is implemented for
/// using data from the supplied [`World`].
///
/// This can be helpful for complex initialization or context-aware defaults.
pub trait FromWorld {
    /// Creates `Self` using data from the given [`World`].
    fn from_world(world: &mut World) -> Self;
}
```

We've mentioned this before in earlier katas, but haven't actually seen this trait yet. There's also a blanket implementation of `FromWorld` for any type which implements `Default`

```rust
impl<T: Default> FromWorld for T {
    fn from_world(_world: &mut World) -> Self {
        T::default()
    }
}
```

So `init_resource` retrieves a singleton instance of some type `R` from `self.world` if it exists, or instantiates a new `R` using `FromWorld` or `Default`.

---

`insert_resource` is simpler

```rust
pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
    self.world.insert_resource(resource);
    self
}
```

we again defer to a second identically-named method on `self.world`, but this time, there is only one more step in the process

```rust
pub fn insert_resource<R: Resource>(&mut self, value: R) {
    let component_id = self.components.init_resource::<R>();
    OwningPtr::make(value, |ptr| {
        // SAFETY: component_id was just initialized and corresponds to resource of type R.
        unsafe {
            self.insert_resource_by_id(component_id, ptr);
        }
    });
}
```

As with `init_resource`, we create the `ComponentId`, but this time there's no "get resource or...". We just `insert`.

But aren't `Resource`s supposed to always be singletons? They are. Here's the (abridged) documentation above `insert_resource`

```rust
/// Inserts a [`Resource`] to the current [`App`] and overwrites any [`Resource`] previously added of the same type.
```

So `init` spawns a new `Resource` of type `R` while `insert` takes an instance of type `R` and saves it, overwriting the existing instance, if there is one.

---

It might seem weird that a color should be a singleton `Resource`, but `ClearColor` is not just _any_ color

```rust
/// A [`Resource`] that stores the color that is used to clear the screen between frames.
///
/// This color appears as the "background" color for simple apps,
/// when there are portions of the screen with nothing rendered.
#[derive(Resource, Clone, Debug, Deref, DerefMut, ExtractResource, Reflect)]
#[reflect(Resource)]
pub struct ClearColor(pub Color);
```

`ClearColor` is the "background color" of the app. There's only one background, so there can only be one color. Makes sense.

As we can see in this example, that doesn't mean the background color is _immutable_. We can change it to whatever color we like, whenever we like. It's just that, at any given time, there is only a single color which is _the_ background color.

---

There are a few other things I want to touch on in this example; the first one is `input: Res<Input<KeyCode>>`.

We saw this exact argument in [the `keyboard_input` kata](https://github.com/awwsmm/daily-bevy/tree/input/keyboard_input), and we traced this back to an `EventReader<KeyboardInput>`. I wonder if we'll find an `EventReader<ClearColor>` somewhere?

...not quite. Remember `KeyboardInput` is an `Event`. `ClearColor` isn't. It's a `Resource`. So we can set it manually. It's retrieved using a pattern like

```rust
LoadOp::Clear(world.resource::<ClearColor>().0.into())
```

If you search Bevy for lines like this, you'll see that this is called when rendering a frame. `LoadOp::Clear` is actually [a `wgpu` instruction](https://github.com/gfx-rs/wgpu)

```rust
/// Operation to perform to the output attachment at the start of a render pass.
///
/// The render target must be cleared at least once before its content is loaded.
///
/// Corresponds to [WebGPU `GPULoadOp`](https://gpuweb.github.io/gpuweb/#enumdef-gpuloadop).
// -- snip --
pub enum LoadOp<V> {
    /// Clear with a specified value.
    Clear(V),
    /// Load from memory.
    Load,
}
```

...so that's probably deep enough into this for now.

We've got our two arguments: `input: Res<Input<KeyCode>>`, which we're quite familiar with by now, and `mut clear_color: ResMut<ClearColor>`. As you might imagine, `ResMut` is a mutable version of `Res`. It gives us a...

```rust
/// Unique mutable borrow of a [`Resource`].
```

Why do we need `mut` _and_ `ResMut`? Try removing either and see what errors you get.

---

The last thing to touch on today is the `setup` system.

Recall that `Startup` is the almost-but-not-quite-first sub-`Schedule` of the `Main` schedule

```rust
/// On the first run of the schedule (and only on the first run), it will run:
/// * [`PreStartup`]
/// * [`Startup`]
/// * [`PostStartup`]
```

So before `change_clear_color` is executed even once, `setup` will be executed.

And what does `setup` do?

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
```

It takes a `Commands` argument, which contains a queue of `Command`s. A `Command` is something that mutates the `World`

```rust
pub trait Command: Send + 'static {
    /// Applies this command, causing it to mutate the provided `world`.
    ///
    /// This method is used to define what a command "does" when it is ultimately applied.
    /// Because this method takes `self`, you can store data or settings on the type that implements this trait.
    /// This data is set by the system or other source of the command, and then ultimately read in this method.
    fn apply(self, world: &mut World);
}
```

and `spawn`?

```rust
/// Pushes a [`Command`] to the queue for creating a new entity with the given [`Bundle`]'s components,
/// and returns its corresponding [`EntityCommands`].
// -- snip --
pub fn spawn<'a, T: Bundle>(&'a mut self, bundle: T) -> EntityCommands<'w, 's, 'a> {
    let mut e = self.spawn_empty();
    e.insert(bundle);
    e
}
```

This was a bit difficult for me to read the first time through.

First, let's look at what a `Bundle` is

```rust
/// The `Bundle` trait enables insertion and removal of [`Component`]s from an entity.
///
/// Implementors of the `Bundle` trait are called 'bundles'.
///
/// Each bundle represents a static set of [`Component`] types.
// -- snip --
pub unsafe trait Bundle: DynamicBundle + Send + Sync + 'static {
    // -- snip --
}
```

Okay, so `Bundle`s let us easily add some predefined set of `Component`s to an entity. Sounds similar to `PluginGroup`: an easy way to collect a bunch of things in a reusable collection.

The `Camera2dBundle` specifically is a collection of all of these different `Component`s

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

There's only one bit of documentation in the file which defines the `Camera2dBundle`, and it calls it "an orthographic projection camera". Not a lot of information there.

But at least we can understand `spawn` a bit better now, with the help of some parentheses and some rearranged phrases

> "Pushes a `Command` (for creating a new entity with the given `Bundle`'s components) to the queue".

- `spawn` pushes a `Command` to the command queue
- that `Command` will mutate the `World`
- that `Command` will create a new entity
- that entity will have all of the `Component`s described by the `Bundle`
- in our case, that `Bundle` is a `Camera2dBundle`

Tomorrow, we're going to dig more into these cameras, what they are, how to position them, and how to use them.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).