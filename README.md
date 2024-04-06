# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows entry #26 in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Events

Today is day #26 of Daily Bevy.

This kata uses [Bevy `v0.13.2`](https://github.com/bevyengine/bevy/tree/v0.13.2).

### Today's Kata

Today, we'll be looking at [the `event` example](https://github.com/bevyengine/bevy/blob/v0.13.2/examples/ecs/event.rs) from the Bevy repo.

#### The Code

Here's the `main.rs` for this example

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MyEvent>()
        .add_event::<PlaySound>()
        .init_resource::<EventTriggerState>()
        .add_systems(Update, (event_trigger, event_listener, sound_player))
        .run();
}

#[derive(Event)]
struct MyEvent {
    pub message: String,
}

#[derive(Event, Default)]
struct PlaySound;

#[derive(Resource)]
struct EventTriggerState {
    event_timer: Timer,
}

impl Default for EventTriggerState {
    fn default() -> Self {
        EventTriggerState {
            event_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

// sends MyEvent and PlaySound every second
fn event_trigger(
    time: Res<Time>,
    mut state: ResMut<EventTriggerState>,
    mut my_events: EventWriter<MyEvent>,
    mut play_sound_events: EventWriter<PlaySound>,
) {
    if state.event_timer.tick(time.delta()).finished() {
        my_events.send(MyEvent {
            message: "MyEvent just happened!".to_string(),
        });
        play_sound_events.send_default();
    }
}

// prints events as they come in
fn event_listener(mut events: EventReader<MyEvent>) {
    for my_event in events.read() {
        info!("{}", my_event.message);
    }
}

fn sound_player(mut play_sound_events: EventReader<PlaySound>) {
    for _ in play_sound_events.read() {
        info!("Playing a sound");
    }
}
```

And here's the `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.13.2"
```

#### Discussion

This example is a very short introduction to _events_ in Bevy. It starts in the usual way, with a `main` function

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MyEvent>()
        .add_event::<PlaySound>()
        .init_resource::<EventTriggerState>()
        .add_systems(Update, (event_trigger, event_listener, sound_player))
        .run();
}
```

The only bit here that's new to us is the `add_event` method

```rust
/// Setup the application to manage events of type `T`.
///
/// This is done by adding a [`Resource`] of type [`Events::<T>`],
/// and inserting an [`event_update_system`] into [`First`].
// -- snip --
pub fn add_event<T>(&mut self) -> &mut Self
where
    T: Event,
{
    if !self.world.contains_resource::<Events<T>>() {
        self.init_resource::<Events<T>>().add_systems(
            First,
            bevy_ecs::event::event_update_system::<T>
                .in_set(bevy_ecs::event::EventUpdates)
                .run_if(bevy_ecs::event::event_update_condition::<T>),
        );
    }
    self
}
```

Recall that there can only be a single instance of any given `Resource` in a `World`. As noted in the doc comments above, `Events<T>` is a `Resource`. `add_event` initializes (`init_resource`) the `Events<T>` queue for a particular `T`, and also sets an `event_update_system` to run in the `First` schedule.

Let's have a closer look at the documentation for `Events`...

```rust
/// An event collection that represents the events that occurred within the last two
/// [`Events::update`] calls.
```

"[W]ithin that last two `Events::update` calls" sounds a bit unusual. Normally, we care about `x` that occurred since the _last_ `y`, not since the last _two_ `y`.

```rust
/// Events can be written to using an [`EventWriter`]
/// and are typically cheaply read using an [`EventReader`].
```

We've seen `EventWriter`s and `EventReader`s before, way back in the [second Daily Bevy kata](https://github.com/awwsmm/daily-bevy/tree/app/drag_and_drop), when we looked at file drag and drop events. There, we were only _reading_ events, but in today's kata, we'll be _writing_ some events, as well.

```rust
/// Each event can be consumed by multiple systems, in parallel,
/// with consumption tracked by the [`EventReader`] on a per-system basis.
```

This is very useful for when you want to handle the _same_ event, but in multiple ways. You might have one system to despawn entities, and another to increase a player's experience points. If an enemy in a game is defeated, you could write an `EnemyDefeated` event, and this same event could be handled in different ways by a `despawn_entity` system and an `increase_xp` system.

```rust
/// If no [ordering](https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs)
/// is applied between writing and reading systems, there is a risk of a race condition.
/// This means that whether the events arrive before or after the next [`Events::update`] is unpredictable.
///
/// This collection is meant to be paired with a system that calls
/// [`Events::update`] exactly once per update/frame.
///
/// [`event_update_system`] is a system that does this, typically initialized automatically using
/// [`add_event`](https://docs.rs/bevy/*/bevy/app/struct.App.html#method.add_event).
/// [`EventReader`]s are expected to read events from this collection at least once per loop/frame.
/// Events will persist across a single frame boundary and so ordering of event producers and
/// consumers is not critical (although poorly-planned ordering may cause accumulating lag).
/// If events are not handled by the end of the frame after they are updated, they will be
/// dropped silently.
```

This, I suppose, is the reason for the "last two `Events::update` calls" documentation. It's possible for an `EventReader<T>` and an `EventWriter<T>` to run out of order, so Bevy is a bit lenient and holds onto events for _two_ `Update` cycles, rather than just one.

Back to the `Events` documentation...

```rust
/// # Details
///
/// [`Events`] is implemented using a variation of a double buffer strategy.
```

[Double-buffering](https://www.pcmag.com/encyclopedia/term/double-buffering) lets us write events into one buffer while processing the events in a second buffer.

```rust
/// Each call to [`update`](Events::update) swaps buffers and clears out the oldest one.
/// - [`EventReader`]s will read events from both buffers.
/// - [`EventReader`]s that read at least once per update will never drop events.
/// - [`EventReader`]s that read once within two updates might still receive some events
/// - [`EventReader`]s that read after two updates are guaranteed to drop all events that occurred
/// before those updates.
///
/// The buffers in [`Events`] will grow indefinitely if [`update`](Events::update) is never called.
///
/// An alternative call pattern would be to call [`update`](Events::update)
/// manually across frames to control when events are cleared.
/// This complicates consumption and risks ever-expanding memory usage if not cleaned up,
/// but can be done by adding your event as a resource instead of using
/// [`add_event`](https://docs.rs/bevy/*/bevy/app/struct.App.html#method.add_event).
```

This pattern is more advanced than what we're covering here.

Finally, we've got the `Events` `struct` itself

```rust
#[derive(Debug, Resource)]
pub struct Events<E: Event> {
    /// Holds the oldest still active events.
    /// Note that a.start_event_count + a.len() should always === events_b.start_event_count.
    events_a: EventSequence<E>,
    /// Holds the newer events.
    events_b: EventSequence<E>,
    event_count: usize,
}
```

So `events_a` and `events_b` are the two buffers; they are both `EventSequence`s

```rust
struct EventSequence<E: Event> {
    events: Vec<EventInstance<E>>,
    start_event_count: usize,
}
```

where `EventInstance` looks like

```rust
struct EventInstance<E: Event> {
    pub event_id: EventId<E>,
    pub event: E,
}
```

and `EventId` looks like

```rust
/// An `EventId` uniquely identifies an event stored in a specific [`World`].
///
/// An `EventId` can among other things be used to trace the flow of an event from the point it was
/// sent to the point it was processed.
///
/// [`World`]: crate::world::World
pub struct EventId<E: Event> {
    /// Uniquely identifies the event associated with this ID.
    // This value corresponds to the order in which each event was added to the world.
    pub id: usize,
    _marker: PhantomData<E>,
}
```

As `id` here is `usize`, it means that a Bevy app can only ever have a maximum of `4294967296` (~4.29 billion) events of the same type. That might sound like a lot, but if you have 1000 entities creating new `Event`s of the same type every millisecond, you run out of unique IDs after 71.5 minutes. I wonder if / how Bevy solves this problem? (If it _is_ a problem.) Can we just wrap around and assume that the 4294967297th event will only come into existence well after we no longer care about the 0th event? (I assume so, since events are only kept in the `Events` struct for two `Update` schedules.)

Anyway, `EventId` holds the count of a particular kind of `Event`, `E`

```rust
/// A type that can be stored in an [`Events<E>`] resource
/// You can conveniently access events using the [`EventReader`] and [`EventWriter`] system parameter.
///
/// Events must be thread-safe.
pub trait Event: Send + Sync + 'static {}
```

`Event` is minimal. And `EventId` doesn't actually _hold_ events, but it does have a `PhantomData<E>`. If you've never seen `PhantomData` before, it's documentation explains what it's used for

```rust
/// Zero-sized type used to mark things that "act like" they own a `T`.
///
/// Adding a `PhantomData<T>` field to your type tells the compiler that your
/// type acts as though it stores a value of type `T`, even though it doesn't
/// really. This information is used when computing certain safety properties.
///
/// For a more in-depth explanation of how to use `PhantomData<T>`, please see
/// [the Nomicon](../../nomicon/phantom-data.html).
///
/// # A ghastly note 👻👻👻
///
/// Though they both have scary names, `PhantomData` and 'phantom types' are
/// related, but not identical. A phantom type parameter is simply a type
/// parameter which is never used. In Rust, this often causes the compiler to
/// complain, and the solution is to add a "dummy" use by way of `PhantomData`.
```

So now that we fully understand `EventId`, let's pop that off the stack and go back to `EventInstance`

```rust
struct EventInstance<E: Event> {
    pub event_id: EventId<E>,
    pub event: E,
}
```

This is _actually_ an event. `EventInstance` holds the `Event` itself, plus its `EventId`. And `EventSequence`...

```rust
struct EventSequence<E: Event> {
    events: Vec<EventInstance<E>>,
    start_event_count: usize,
}
```

...holds a `Vec` of these `EventInstance`s. But what is `start_event_count` here? That's actually explained pretty well back up in `Events`

```rust
#[derive(Debug, Resource)]
pub struct Events<E: Event> {
    /// Holds the oldest still active events.
    /// Note that a.start_event_count + a.len() should always === events_b.start_event_count.
    events_a: EventSequence<E>,
    /// Holds the newer events.
    events_b: EventSequence<E>,
    event_count: usize,
}
```

Specifically, `a.start_event_count + a.len()` should always equal `events_b.start_event_count`. Remember that, with the double buffer, we are reading from `a` while we are writing to `b`. What this doc comment is saying is that the `id` / event index of the oldest event in `b` should be exactly `1` greater than the latest event in `a`. Both will be "slices" of all events of type `E` received throughout the lifetime of the app, but they will never overlap, or have a gap of missing events between them.

---

Let's go back to `add_event`

```rust
pub fn add_event<T>(&mut self) -> &mut Self
where
    T: Event,
{
    if !self.world.contains_resource::<Events<T>>() {
        self.init_resource::<Events<T>>().add_systems(
            First,
            bevy_ecs::event::event_update_system::<T>
                .in_set(bevy_ecs::event::EventUpdates)
                .run_if(bevy_ecs::event::event_update_condition::<T>),
        );
    }
    self
}
```

We now have a much better understanding of `Events<T>` does, so let's look at the `event_update_system`

```rust
/// A system that calls [`Events::update`].
pub fn event_update_system<T: Event>(
    update_signal: Option<Res<EventUpdateSignal>>,
    mut events: ResMut<Events<T>>,
) {
    if let Some(signal) = update_signal {
        // If we haven't got a signal to update the events, but we *could* get such a signal
        // return early and update the events later.
        if !signal.0 {
            return;
        }
    }

    events.update();
}
```

`event_update_system` listens for an `EventUpdateSignal`, which is just a boolean wrapped in a `Resource`

```rust
#[derive(Resource, Default)]
pub struct EventUpdateSignal(bool);
```

This is set to `true` in the `signal_event_update_system`

```rust
/// Signals the [`event_update_system`] to run after `FixedUpdate` systems.
pub fn signal_event_update_system(signal: Option<ResMut<EventUpdateSignal>>) {
    if let Some(mut s) = signal {
        s.0 = true;
    }
}
```

and back to `false` in the `reset_event_update_signal_system`

```rust
/// Resets the `EventUpdateSignal`
pub fn reset_event_update_signal_system(signal: Option<ResMut<EventUpdateSignal>>) {
    if let Some(mut s) = signal {
        s.0 = false;
    }
}
```

In the `TimePlugin` in the `bevy_time` crate, these systems are added to the world

```rust
impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        // -- snip --

        // ensure the events are not dropped until `FixedMain` systems can observe them
        app.init_resource::<EventUpdateSignal>()
            .add_systems(
                First,
                bevy_ecs::event::reset_event_update_signal_system.after(EventUpdates),
            )
            .add_systems(FixedPostUpdate, signal_event_update_system);

        // -- snip --
    }
}
```

`reset_event_update_signal_system` is set to run `after` `EventUpdates`, which is a group of systems (a `SystemSet`)

```rust
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventUpdates;
```

The only system added to `EventUpdates` is the one from `add_event` -- `event_update_system`. So, the order of events is

- in the `First` schedule, the `event_update_system` runs if and only if the `event_update_condition` is met
- then, in the `First` schedule, the `reset_event_update_signal_system` runs, setting the `EventUpdateSignal` to `false`
- later, in the `FixedPostUpdate` schedule, the `signal_event_update_system` runs, setting the `EventUpdateSignal` to `true`

`event_update_condition` just checks whether there are any events in either `Events` buffer

```rust
/// A run condition that checks if the event's [`event_update_system`]
/// needs to run or not.
pub fn event_update_condition<T: Event>(events: Res<Events<T>>) -> bool {
    !events.events_a.is_empty() || !events.events_b.is_empty()
}
```

So if there are any events in either `Events` buffer, and if the `EventUpdateSignal` is `true`, `event_update_system` will call `events.update()`, which...

```rust
/// Swaps the event buffers and clears the oldest event buffer. In general, this should be
/// called once per frame/update.
///
/// If you need access to the events that were removed, consider using [`Events::update_drain`].
pub fn update(&mut self) {
    let _ = self.update_drain();
}
```

It might seem weird that there's no logic to "handle" / "process" events here, but remember, it's up to the author of the program to do that, with an `EventReader`.

The only remaining question I have is... why do we (apparently) set `EventUpdateSignal` to `false` and then to `true` on every game loop?

And I think the answer to this is that the `signal_event_update_system`, which sets `EventUpdateSignal` to `true`, does not run exactly once on every game loop. Since it is in the `FixedPostUpdate` schedule, it runs as part of the `FixedMain` loop

```rust
/// The schedule that contains systems which only run after a fixed period of time has elapsed.
///
/// The exclusive `run_fixed_main_schedule` system runs this schedule.
/// This is run by the [`RunFixedMainLoop`] schedule.
///
/// Frequency of execution is configured by inserting `Time<Fixed>` resource, 64 Hz by default.
/// See [this example](https://github.com/bevyengine/bevy/blob/latest/examples/time/time.rs).
///
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FixedMain;
```

The documentation above `Main` gives the order of the schedules

```rust
/// On the first run of the schedule (and only on the first run), it will run:
/// * [`PreStartup`]
/// * [`Startup`]
/// * [`PostStartup`]
///
/// Then it will run:
/// * [`First`]
/// * [`PreUpdate`]
/// * [`StateTransition`]
/// * [`RunFixedMainLoop`]
///     * This will run [`FixedMain`] zero to many times, based on how much time has elapsed.
/// * [`Update`]
/// * [`PostUpdate`]
/// * [`Last`]
```

So if `RunFixedMainLoop` runs zero times, which the documentation above says is possible, then `EventUpdateSignal` will not be set to `true`, and the `event_update_system` will not call `events.update()`, i.e. it will not drain the events, because the `FixedMain` loop has not had a chance to run yet. This is what this comment means

```rust
// ensure the events are not dropped until `FixedMain` systems can observe them
```

So, if a developer adds a system to a `FixedMain` system which has an `EventReader`, we want to make sure that that system does not miss out on any events. So we do not clear those events until the `FixedMain` loop runs and sets the `EventUpdateSignal` to `true`, _signalling_ that we can call `events.update()`. (The name of `EventUpdateSignal` should make sense now.)

We now understand everything that happens in `add_event`, and therefore, in the `main()` of this example.

So what's next?

---

Next we define a pair of `Event`s...

```rust
#[derive(Event)]
struct MyEvent {
    pub message: String,
}

#[derive(Event, Default)]
struct PlaySound;
```

...and a `Resource` called `EventTriggerState` with an `event_timer` which repeats on 1-second intervals 

```rust
#[derive(Resource)]
struct EventTriggerState {
    event_timer: Timer,
}

impl Default for EventTriggerState {
    fn default() -> Self {
        EventTriggerState {
            event_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}
```

---

We have only one system which uses `EventWriter`s, the `event_trigger` system

```rust
// sends MyEvent and PlaySound every second
fn event_trigger(
    time: Res<Time>,
    mut state: ResMut<EventTriggerState>,
    mut my_events: EventWriter<MyEvent>,
    mut play_sound_events: EventWriter<PlaySound>,
) {
    if state.event_timer.tick(time.delta()).finished() {
        my_events.send(MyEvent {
            message: "MyEvent just happened!".to_string(),
        });
        play_sound_events.send_default();
    }
}
```

This system increments the `event_timer` from the `EventTriggerState` with the amount of time that has passed since the last `Update` (`time.delta()`)

```rust
state.event_timer.tick(time.delta())
```

...and then checks if it's finished (if its current `duration` is 1 second or greater)

```rust
state.event_timer.tick(time.delta()).finished()
```

...and if so, it `send`s events to the `EventWriter`s

```rust
my_events.send(MyEvent {
    message: "MyEvent just happened!".to_string(),
});
play_sound_events.send_default();
```

Note that, in addition to `send()` and `send_default()`, there's also a `send_batch()` method on `EventWriter`, which lets you send multiple events at once. But that's it! Writing to an `EventWriter` is pretty easy.

---

Reading these events is pretty easy, too. We do this in the final two systems of this example

```rust
// prints events as they come in
fn event_listener(mut events: EventReader<MyEvent>) {
    for my_event in events.read() {
        info!("{}", my_event.message);
    }
}

fn sound_player(mut play_sound_events: EventReader<PlaySound>) {
    for _ in play_sound_events.read() {
        info!("Playing a sound");
    }
}
```

Super simple!

---

Well, I learned a ton about `Event`s with this example. They are an easy way to communicate across systems, rather than using `Resource`s. I will definitely try to work them into my next project!

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
