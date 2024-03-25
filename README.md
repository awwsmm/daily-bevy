# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows entry #25 in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Virtual Time

Today is day #25 of Daily Bevy.

This kata uses [Bevy `v0.13.0`](https://github.com/bevyengine/bevy/tree/v0.13.0).

### Today's Kata

Today, we'll be checking out [the `virtual_time` example](https://github.com/bevyengine/bevy/blob/v0.13.0/examples/time/virtual_time.rs) from the Bevy repo.

#### The Code

The code for this example is a bit long, so I haven't reproduced it in full here, but you can find it at [`src/main.rs`](https://github.com/awwsmm/daily-bevy/blob/time/virtual_time/src/main.rs).

Here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.13.0"
```

We also need `icon.png` in `assets/branding`. 

#### Discussion

This example is really cool, I'm excited to dig into it. If you run the example locally, you'll see two Bevy logos moving back and forth across the window, but for one of the logos, you can control the speed at which time ticks past, even pausing it. The other logo continues to move at a steady, one-second-per-second pace. Let's figure out how to do this in Bevy!

This example starts off as most of the other ones do, with a `main` function, `DefaultPlugins`, and a `setup` system in the `Startup` schedule

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_virtual_time_sprites,
                move_real_time_sprites,
                // -- snip --
            ),
        )
        .run();
}
```

The next few systems added to the `Update` schedule are interesting, though. We haven't seen anything quite like these before. First, we've got

```rust
toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
```

This is a system with a `run_if`. We've seen this before, but usually, in the `run_if`, we would use `in_state` -- defined in `schedule/condition.rs` in the `bevy_ecs` crate -- and only run the system if we're in some particular state of an FSM.

In _this_ example, we're using a different _run condition_, `input_just_pressed`, which comes from `common_conditions.rs` in the `bevy_input` crate. Both of these run conditions -- `in_state` and `input_just_pressed` -- must `impl`ement the `Condition` `trait`, because that's what all four variants of `run_if` expect

```rust
// in schedule/config.rs in the bevy_ecs crate
fn run_if<M>(self, condition: impl Condition<M>) -> SystemConfigs { /* ... */ }
fn run_if<M>(mut self, condition: impl Condition<M>) -> SystemConfigs { /* ... */ }
fn run_if<M>(self, condition: impl Condition<M>) -> SystemSetConfigs { /* ... */ }
fn run_if<M>(mut self, condition: impl Condition<M>) -> SystemSetConfigs { /* ... */ }
```

`Condition` looks like this

```rust
pub trait Condition<Marker, In = ()>: sealed::Condition<Marker, In> {
    // -- snip --
}
```

where `sealed::Condition` looks like

```rust
pub trait Condition<Marker, In>:
    IntoSystem<In, bool, Marker, System = Self::ReadOnlySystem>
{
    // This associated type is necessary to let the compiler
    // know that `Self::System` is `ReadOnlySystem`.
    type ReadOnlySystem: ReadOnlySystem<In = In, Out = bool>;
}
```

and `IntoSystem` looks like

```rust
pub trait IntoSystem<In, Out, Marker>: Sized {
    /// The type of [`System`] that this instance converts into.
    type System: System<In = In, Out = Out>;
    // -- snip --
}
```

So a `Condition` is something which can be turned `Into` a `ReadOnlySystem` where `Out` is `bool`. That makes sense. It should evaluate to a boolean, since what we want to know is whether or not to run the system we're calling `run_if` on. And I'm not sure why this system must be `ReadOnly`, but I'd guess allowing systems which mutate the `World` here would complicate Bevy somewhat. If anyone reading knows why, specifically, `Condition`s must be `ReadOnly`, I would love to learn.

There are lots of other systems defined in the Bevy repo which implement `Condition`. Here are a few examples

```rust
/// A run `Condition` that always returns true
fn yes() -> bool {
    true
}

/// A run `Condition` that always returns false
fn no() -> bool {
    false
}
```

Pretty simple. The above two systems evaluate to `bool`.

```rust
/// Run Condition to only play audio if the audio output is available
pub(crate) fn audio_output_available(audio_output: Res<AudioOutput>) -> bool {
    audio_output.stream_handle.is_some()
}
```

The above system immutably accesses a `Res`ource and evaluates to `bool`.

```rust
pub fn resource_exists_and_equals<T>(value: T) -> impl FnMut(Option<Res<T>>) -> bool
where
    T: Resource + PartialEq,
{
    move |res: Option<Res<T>>| match res {
        Some(res) => *res == value,
        None => false,
    }
}
```

The above function _returns_ a system which takes a single argument and returns a `bool` 

```rust
fn every_other_time(mut has_ran: Local<bool>) -> bool {
    *has_ran = !*has_ran;
    *has_ran
}
```

The above system might be confusing. It clearly takes a `mut`able argument and... therefore isn't read-only? But that argument is a `Local`. What is a `Local`?

```rust
/// A system local [`SystemParam`].
///
/// A local may only be accessed by the system itself and is therefore not visible to other systems.
/// If two or more systems specify the same local type each will have their own unique local.
/// If multiple [`SystemParam`]s within the same system each specify the same local type
/// each will get their own distinct data storage.
```

So perhaps this is a way to get around the "systems must be read-only" rule. It's like [interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html), but for Bevy systems.

Note that `Condition`s also have `and_then` and `or_else` methods, which act like boolean operators to combine conditions

```rust
// .and_then, for example
my_system.run_if(resource_exists::<R>.and_then(resource_equals(R(0))))

// .or_else, for example
my_system.run_if(resource_exists::<A>.or_else(resource_exists::<B>))
```

...anyway, where were we? Got a bit off-track there.

```rust
toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
```

So this system, `toggle_pause`, will only run if the user just pressed the `Space` bar

```rust
pub fn input_just_pressed<T>(input: T) -> impl FnMut(Res<ButtonInput<T>>) -> bool + Clone
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    move |inputs: Res<ButtonInput<T>>| inputs.just_pressed(input)
}
```

Neat! That saves us having to write a system to listen for `ButtonInput`s, filter to `KeyCode::Space`, and run the logic we want, which is really simple, by the way

```rust
/// pause or resume `Relative` time
fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}
```

---

The next line of `main` is also fascinating, but for a different reason

```rust
change_time_speed::<1>.run_if(input_just_pressed(KeyCode::ArrowUp)),
```

Like the line above, we have a system which we `run_if(input_just_pressed(...))` for a particular key. In this case the `ArrowUp` key. But what's that `::<1>`? Is that the literal value `1` being used as a generic argument? This is what `change_time_speed` looks like

```rust
/// Update the speed of `Time<Virtual>.` by `DELTA`
fn change_time_speed<const DELTA: i8>(mut time: ResMut<Time<Virtual>>) {
    let time_speed = (time.relative_speed() + DELTA as f32)
        .round()
        .clamp(0.25, 5.);

    // set the speed of the virtual time to speed it up or slow it down
    time.set_relative_speed(time_speed);
}
```

The `<const DELTA: i8>` is a [const generic](https://www.awwsmm.com/blog/what-are-const-generics-and-how-are-they-used-in-rust)!

> "const generics are a limited form of dependent types, meaning types being generic not just over types but over values" [[source]](https://www.reddit.com/r/rust/comments/jy95xq/what_are_const_generics/)

[Const generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics) are what power array types like `[T; N]`. A function like

```rust
fn create_i32_array<const N: usize>() -> [i32; N] { /* ... */ }
```

is equivalent to [a _bunch_ of methods](https://manishearth.github.io/blog/2017/03/04/what-are-sum-product-and-pi-types/#what-in-the-name-of-sanity-is-a-pi-type), like

```rust
fn create_i32_0_array() -> [i32; 0] { /* ... */ }
fn create_i32_1_array() -> [i32; 1] { /* ... */ }
fn create_i32_2_array() -> [i32; 2] { /* ... */ }
fn create_i32_3_array() -> [i32; 3] { /* ... */ }
// ...
```

Where `[i32; 0]` and `[i32; 1]` are _distinct_ types, even though `0` and `1` are both of type `usize`. (If you're still not convinced of the power of const generics, check out [this matrix multiplication example](https://amacal.medium.com/learning-rust-const-generics-8c29fc26fad4).)

For `change_time_speed`, we can't easily pass an argument into the system without rearranging it into a function which _returns_ a system, like

```rust
/// Update the speed of `Time<Virtual>.` by `DELTA`
fn change_time_speed_2(delta: i8) -> impl FnMut(ResMut<Time<Virtual>>) {
    move |mut time| {
        let time_speed = (time.relative_speed() + delta as f32)
            .round()
            .clamp(0.25, 5.);

        // set the speed of the virtual time to speed it up or slow it down
        time.set_relative_speed(time_speed);
    }
}
```

We would then do something like this in `main`

```rust
change_time_speed_2(1).run_if(input_just_pressed(KeyCode::ArrowUp)),
```

So it's a matter of taste really. Both options work; you just have to decide whether you want to use const generics or rewrite your system to return a mutable closure. (If I had to guess, I'd say the const generic example is probably more performant, as well, because [I'd guess the compiler could monomorphize](https://www.reddit.com/r/rust/comments/u8x6g9/comment/i5o2ueq/?utm_source=share&utm_medium=web2x&context=3) it in the compiled code.)

---

The next line of `main` is similar

```rust
change_time_speed::<-1>.run_if(input_just_pressed(KeyCode::ArrowDown)),
```

We reduce the rate at which time passes if the user presses the down arrow.

Finally, we have the last two systems used in `main`

```rust
(update_virtual_time_info_text, update_real_time_info_text)
    // update the texts on a timer to make them more readable
    // `on_timer` run condition uses `Virtual` time meaning it's scaled
    // and would result in the UI updating at different intervals based
    // on `Time<Virtual>::relative_speed` and `Time<Virtual>::is_paused()`
    .run_if(on_real_timer(Duration::from_millis(250))),
```

These systems use a different `Condition`: they're run on a timer. Four times per second, the text will be updated. The comment explains that `on_real_timer` is used instead of `on_timer`, because `on_timer` uses the `Virtual` time, and could therefore update at unexpected intervals, based on the dilation of the virtual time.

---

After `main`, we have a pair of marker components to label the sprite and the text for the "real time" and "virtual time" halves of this example

```rust
/// `Real` time related marker
#[derive(Component)]
struct RealTime;

/// `Virtual` time related marker
#[derive(Component)]
struct VirtualTime;
```

...and then, we have the `setup` system

```rust
/// Setup the example
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut time: ResMut<Time<Virtual>>) {
    // -- snip --
}
```

We've seen `Commands`, `Res`, `ResMut`, and `AssetServer` before, but `Time<Virtual>` is new. Both `Time` and `Virtual` have a ton of great documentation, which I won't reproduce here in full, but here's the gist of it:

> `Time` is a "generic clock resource that tracks how much it has advanced since its previous update and since its creation."

`Time` looks like this (blank lines added for clarity)

```rust
pub struct Time<T: Default = ()> {
    context: T,
    wrap_period: Duration,

    delta: Duration,
    delta_seconds: f32,
    delta_seconds_f64: f64,

    elapsed: Duration,
    elapsed_seconds: f32,
    elapsed_seconds_f64: f64,

    elapsed_wrapped: Duration,
    elapsed_seconds_wrapped: f32,
    elapsed_seconds_wrapped_f64: f64,
}
```

> "The time elapsed since the previous time this clock was advanced is saved as `delta` and the total amount of time the clock has advanced is saved as `elapsed`."

The `wrapped` values at the end are used for "applications that require an `f32` value but suffer from gradual precision loss". I won't dig into that here.

`Time` takes a generic type parameter, `T`, which is set to `()` by default. In this example, we pass `Virtual` as the type parameter, which looks like this

```rust
pub struct Virtual {
    max_delta: Duration,
    paused: bool,
    relative_speed: f64,
    effective_speed: f64,
}
```

`Virtual` is the "virtual game clock representing game time." This can be compared to `Real`

```rust
pub struct Real {
    startup: Instant,
    first_update: Option<Instant>,
    last_update: Option<Instant>,
}
```

which is the "[r]eal time clock representing elapsed wall clock time."

A `Time<Virtual>` can
- "be paused by calling `pause()`"
- be "unpaused by calling `unpause()`"
- be sped up or slowed down, relative to a `Real` time clock, by calling `set_relative_speed()`

So, at the start of `setup`, we call `set_relative_speed()` to have the `Virtual` clock run faster than the `Real` clock at app startup

```rust
// start with double `Virtual` time resulting in one of the sprites moving at twice the speed
// of the other sprite which moves based on `Real` (unscaled) time
time.set_relative_speed(2.);
```

---

The rest of `setup` is fairly straightforward. We `spawn` a camera...

```rust
commands.spawn(Camera2dBundle::default());
```

...then we create some top-level variables in the body of `setup`: a color to distinguish the virtual-time systems from the real-time systems; a scale factor for the Bevy icon image; and a handle to that image, loaded by the asset server

```rust
let virtual_color = Color::GOLD;
let sprite_scale = Vec2::splat(0.5).extend(1.);
let texture_handle = asset_server.load("branding/icon.png");
```

...and finally, we spawn all the visible elements, like

```rust
// the sprite moving based on real time
commands.spawn((
    SpriteBundle {
        texture: texture_handle.clone(),
        transform: Transform::from_scale(sprite_scale),
        ..default()
    },
    RealTime,
));
```

and

```rust
// the sprite moving based on virtual time
commands.spawn((
    SpriteBundle {
        texture: texture_handle,
        sprite: Sprite {
            color: virtual_color,
            ..default()
        },
        transform: Transform {
            scale: sprite_scale,
            translation: Vec3::new(0., -160., 0.),
            ..default()
        },
        ..default()
    },
    VirtualTime,
));
```

Notice that the real-time sprite entity has a `RealTime` marker component, and the virtual-time one has a `VirtualTime` marker component.

Finally, we create the UI, which shows the real-time info (with a `RealTime` marker component), the controls, and the virtual-time info (with a `VirtualTime` marker component)

```rust
// info UI
let font_size = 40.;

commands
    .spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.),
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|builder| {
        // real time info
        builder.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size,
                    ..default()
                },
            ),
            RealTime,
        ));

        // keybindings
        builder.spawn(
            TextBundle::from_section(
                "CONTROLS\nUn/Pause: Space\nSpeed+: Up\nSpeed-: Down",
                TextStyle {
                    font_size,
                    color: Color::rgb(0.85, 0.85, 0.85),
                    ..default()
                },
            )
            .with_text_justify(JustifyText::Center),
        );

        // virtual time info
        builder.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size,
                    color: virtual_color,
                    ..default()
                },
            )
            .with_text_justify(JustifyText::Right),
            VirtualTime,
        ));
    });
```

We've covered things like this in previous katas, so I won't dig into details here. All of this should look pretty familiar if you've been following along the past few weeks.

And that's the end of the `setup` system.

---

Next, we've got the system that moves the real-time sprite back and forth

```rust
/// Move sprites using `Real` (unscaled) time
fn move_real_time_sprites(
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<RealTime>)>,
    // `Real` time which is not scaled or paused
    time: Res<Time<Real>>,
) {
    for mut transform in sprite_query.iter_mut() {
        // move roughly half the screen in a `Real` second
        // when the time is scaled the speed is going to change
        // and the sprite will stay still the the time is paused
        transform.translation.x = get_sprite_translation_x(time.elapsed_seconds());
    }
}
```

Above, we `Query` for all entities with a `Transform` component, a `Sprite` component, and a `RealTime` component, and take a mutable refrence to the `Transform` component.

We loop over all of those entities (there should only be a single one, because there's only one entity with both a `Sprite` component and a `RealTime` component), and adjust their x-positions using the `get_sprite_translation_x` method. How does that method work?

```rust
fn get_sprite_translation_x(elapsed: f32) -> f32 {
    elapsed.sin() * 500.
}
```

...it just takes the sine of `elapsed` and multiplies it by 500. So the sprite will move left and right across the screen, from x=-500 to x=500.

Notice that we use `time.elapsed_seconds()`, which gives the total number of seconds elapsed since `Startup`. (Rather than the seconds since the previous `Update`, which is what `.delta()` returns.)

---

Next, we have the system that moves the virtual-time sprite back and forth

```rust
/// Move sprites using `Virtual` (scaled) time
fn move_virtual_time_sprites(
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<VirtualTime>)>,
    // the default `Time` is either `Time<Virtual>` in regular systems
    // or `Time<Fixed>` in fixed timestep systems so `Time::delta()`,
    // `Time::elapsed()` will return the appropriate values either way
    time: Res<Time>,
) {
    for mut transform in sprite_query.iter_mut() {
        // move roughly half the screen in a `Virtual` second
        // when time is scaled using `Time<Virtual>::set_relative_speed` it's going
        // to move at a different pace and the sprite will stay still when time is
        // `Time<Virtual>::is_paused()`
        transform.translation.x = get_sprite_translation_x(time.elapsed_seconds());
    }
}
```

One thing that stood out to me here is the comment that the default `Time` is `Time<Virtual>`. This is confirmed in the docs above `struct Time`

```rust
/// - [`Time`] is a generic clock that corresponds to "current" or "default"
///   time for systems. It contains [`Time<Virtual>`](crate::virt::Virtual)
///   except inside the [`FixedMain`](bevy_app::FixedMain) schedule when it
///   contains [`Time<Fixed>`](crate::fixed::Fixed).
```

So `Time` is virtual by default, and can be paused, slowed down, or sped up. Good to know!

---

Next, we've got these two systems we discovered earlier in `main`: `change_time_speed`, which uses a const generic parameter

```rust
/// Update the speed of `Time<Virtual>.` by `DELTA`
fn change_time_speed<const DELTA: i8>(mut time: ResMut<Time<Virtual>>) {
    let time_speed = (time.relative_speed() + DELTA as f32)
        .round()
        .clamp(0.25, 5.);

    // set the speed of the virtual time to speed it up or slow it down
    time.set_relative_speed(time_speed);
}
```

and `toggle_pause`, which we `run_if` the user `just_pressed` the `Space` bar, as we saw in `main()`

```rust
/// pause or resume `Relative` time
fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}
```

These are both pretty straightforward, but now I'm wondering if we need the `<Virtual>` generic type parameter at all. If `Virtual` is the default, would these systems run the same way with `ResMut<Time>` as they do with `ResMut<Time<Virtual>>`?

After trying this locally, the answer appears to be "no". When we want a `Res`, we can use `Time` by itself, but when we want a `ResMut`, we have to use `Time<Virtual>`. Otherwise, we don't have access to methods defined in traits which are `impl`emented for `Time<Virtual>` (but not for `Time` itself), like the ones to set the relative speed.

---

Finally, we've got the two systems to update the time info text. These are again both pretty straightforward. There's nothing here that we haven't covered in previous katas

```rust
/// Update the `Real` time info text
fn update_real_time_info_text(
    time: Res<Time<Real>>,
    mut query: Query<&mut Text, With<RealTime>>
) {
    for mut text in &mut query {
        text.sections[0].value = format!(
            "REAL TIME\nElapsed: {:.1}\nDelta: {:.5}\n",
            time.elapsed_seconds(),
            time.delta_seconds(),
        );
    }
}

/// Update the `Virtual` time info text
fn update_virtual_time_info_text(
    time: Res<Time<Virtual>>,
    mut query: Query<&mut Text, With<VirtualTime>>,
) {
    for mut text in &mut query {
        text.sections[0].value = format!(
            "VIRTUAL TIME\nElapsed: {:.1}\nDelta: {:.5}\nSpeed: {:.2}",
            time.elapsed_seconds(),
            time.delta_seconds(),
            time.relative_speed()
        );
    }
}
```

Remember that these systems run on a timer -- they were set up that way when they were added to the `Update` systems in `main`

```rust
(update_virtual_time_info_text, update_real_time_info_text)
    // -- snip --
    .run_if(on_real_timer(Duration::from_millis(250))),
```

---

So there you have it! In this example we learned how to use `Virtual` `Time` in a Bevy app to speed up, slow down, and pause the time in a game. We also learned about const generics, and `Local` `SystemParam`s, and run `Condition`s like `input_just_pressed` and `in_state`, as well as the `and_then` and `or_else` combinators which can be used to combine `Condition`s. With all of this new knowledge, looking back over this example, this is a well-documented and well constructed introduction to virtual time in Bevy.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
