# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the nineteenth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## WASM Persistence

Today is the nineteenth day of Daily Bevy.

This kata uses [Bevy `v0.13.0`](https://github.com/bevyengine/bevy/tree/v0.13.0).

### Today's Kata

Today, we'll be exploring another `bonus` example (examples exclusive to Daily Bevy) -- we'll be looking at how to persist state for a Bevy browser app. This example will build on the [bonus WASM example](https://github.com/awwsmm/daily-bevy/tree/bonus/WASM).

#### The Code

Let's start with the code as-is from the [Daily Bevy WASM example](https://github.com/awwsmm/daily-bevy/tree/bonus/WASM). This is what `main.rs` looks like

```rust
// source: https://github.com/bevyengine/bevy/blob/v0.12.1/examples/ui/button.rs

//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]

use bevy::{prelude::*, winit::WinitSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
```

And here's the original `Cargo.toml` for this example

```toml
[dependencies]
bevy = "0.12.1"
```

We also need the `assets/fonts/` directory and the `example.html` file from the WASM example.

#### Discussion

The first thing we want to do is update this for Bevy `0.13.0`, which was [released in the interim](https://github.com/awwsmm/daily-bevy/tree/bonus/v0.13.0). Luckily, we only need a single change in `Cargo.toml`

```toml
[dependencies]
bevy = "0.13.0"
```

---

Next: how can we add persistence to a web app?

Well, there are [lots of different ways](https://blog.bitsrc.io/different-ways-to-store-data-in-browser-706a2afb4e58) to store data in a browser. [Cookies](https://en.wikipedia.org/wiki/HTTP_cookie) are probably the most popular example of this. But cookies do not support structured data (only key-value pairs) and are limited to a very small amount of storage space (usually 4KB).

[Web Storage](https://en.wikipedia.org/wiki/Web_storage) fulfills a different role, allowing larger amounts (5-10MB) of client-side, per-session ("session") or cross-session ("local") persistence. [IndexedDB](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API) is a new (c. 2015) alternative to Web Storage. IndexedDB is a [NoSQL](https://en.wikipedia.org/wiki/NoSQL), object-based database which runs "inside of the security sandbox of a browser".

> IndexedDB is a low-level API for client-side storage of significant amounts of structured data, including files/blobs. This API uses indexes to enable high-performance searches of this data. While Web Storage is useful for storing smaller amounts of data, it is less useful for storing larger amounts of structured data. IndexedDB provides a solution.

While I initially attempted to use [`rust-indexed-db`](https://github.com/Alorel/rust-indexed-db) to communicate with IndexedDB via a Rust wrapper, this solution was less than ideal. As this API is asynchronous, a messy mix of [`spawn_local`](https://github.com/rustwasm/wasm-bindgen/issues/2111#issuecomment-621225697) and `static mut` global variables (with `unsafe` blocks) was the only way I was able to get this to work with Bevy.

If anyone reading this has any ideas for getting IndexedDB to work nicely with Bevy in a WASM app, please let me know [at the Daily Bevy Discussion board](https://github.com/awwsmm/daily-bevy/discussions).

The fallback I settled on is using local Web Storage with [the `bevy_pkv` crate](https://github.com/johanhelsing/bevy_pkv).

---

In order to persist some state using Web Storage, we need to first have a state to persist.

So let's first change this example so that it counts the number of times the button is pressed.

At the top of the file, we add a new `Resource` called `State`, which counts the number of times the user `clicks` the button

```rust
#[derive(Resource, Default)]
struct State {
    clicks: u128
}
```

Next, we add the resource to the `App`

```rust
fn main() {
    App::new()
        // -- snip --
        .insert_resource(State::default()) // <- new
        .run();
}
```

The `button_system` needs a mutable reference to this new `Resource`, as well

```rust
fn button_system(
    // -- snip --
    mut state: ResMut<State> // <- new
) {
```

When the user presses the button, we need to make sure to update the count

```rust
Interaction::Pressed => {
    // -- snip --
    state.clicks += 1; // <- new
}
```

And finally, let's display the current count on the button when the user isn't interacting with it. Let's change this

```rust
Interaction::None => {
    text.sections[0].value = "Button".to_string();
    *color = NORMAL_BUTTON.into();
    border_color.0 = Color::BLACK;
}
```

to this

```rust
Interaction::None => {
    text.sections[0].value = format!("Clicked {} times!", state.clicks); // <- changed
    *color = NORMAL_BUTTON.into();
    border_color.0 = Color::BLACK;
}
```

...and increase the width of the button to accommodate this additional text

```rust
.spawn(ButtonBundle {
    style: Style {
        width: Val::Px(300.0), // <- changed from 150.0 to 300.0
        height: Val::Px(65.0),
```

Now, the `State` `Resource` tracks the number of button `clicks` and that number is displayed to the user on the button itself.

But when we refresh the page in the browser, we lose the count. Let's save our state in Web Storage, so it persists between refreshes.

---

First, add [the `bevy_pkv` crate](https://github.com/johanhelsing/bevy_pkv) and [the `serde` crate](https://github.com/serde-rs/serde) to `Cargo.toml`

```toml
[dependencies]
bevy = "0.13.0"
bevy_pkv = { git = "https://github.com/johanhelsing/bevy_pkv" } # <- new
serde = "1.0.196" # <- new
```

Then, we more or less just [follow the instructions](https://github.com/johanhelsing/bevy_pkv) on that project's GitHub repo.

First, we add a new `PkvStore` `Resource` to the `App`

```rust
use bevy_pkv::PkvStore; // <- new

fn main() {
    App::new()
        // -- snip --
        .insert_resource(PkvStore::new("Daily Bevy", "WASM Persistence")) // <- new
        // -- snip --
        .run();
    }
```

Then we add a `mut pkv: ResMut<PkvStore>` argument to the `button_system`

```rust
fn button_system(
    // -- snip --
    mut state: ResMut<State>,
    mut pkv: ResMut<PkvStore> // <- new
) {
```

When we aren't interacting with the button, we want it to display our current "score", so let's change the `Interaction::None` arm to this

```rust
Interaction::None => {
    let clicks = match pkv.get::<State>("state") { // <- read the current state
        Ok(state) => state.clicks, // <- if there is a state, use the existing number of clicks
        Err(_) => 0 // <- if not, start over from 0
    };

    text.sections[0].value = format!("Clicked {} times!", clicks);
    *color = NORMAL_BUTTON.into();
    border_color.0 = Color::BLACK;
}
```

Next, we have to increment the `clicks` count when the user presses the button, so we'll change the `Interaction::Pressed` match arm to look like this

```rust
Interaction::Pressed => {
    text.sections[0].value = "Press".to_string();
    *color = PRESSED_BUTTON.into();
    border_color.0 = Color::RED;

    let mut state = pkv.get::<State>("state").expect("could not read state"); // <- there should always be _some_ state by this point
    state.clicks += 1;
    let clone = state.clone(); // <- required to serialize the state
    pkv.set("state", &clone).expect("error saving state");
}
```

Finally, we need to `derive` the `Serialize`, `Deserialize`, and `Clone` traits on `State`

```rust
#[derive(Resource, Default, Serialize, Deserialize, Clone)]
struct State {
    clicks: u128
}
```

You might notice, as well, that we're no longer using the `mut state: ResMut<State>` argument anywhere in `button_system`. It has been completely replaced by the `pkv`. So we can remove

- the `mut state` argument from `button_system`
- the `Resource` `derive` from `State`, and
- `.insert_resource(State::default())` from `App`

All of this is taken care of by `pkv` now.

---

If you run this example as-is in the browser, you'll now see that the state is persisted across refreshes!

There are a few things that could be improved here, though...

- Maybe we don't need to save the state _every single time_ it changes. This could be really resource intensive. Maybe we could have a "save" button? Or save every 5 seconds?
- Similarly, we probably don't need to load the state from the `pkv` _every single time_ we need a value. If we load the state on startup, and then reintroduce our `State` `Resource`, we could use the `Resource` throughout the `App` and have an "in-memory" state, rather than reading "from disk" every single time we need a value.
- As this example stands, there is no way to _clear_ the state. What if you want to start over from zero? Introducing a way to clear the state, or to have different "save files" would be a huge improvement.

There's lots more to explore here, but I hope this example has given you a taste of what's possible with Bevy + WASM + Web Storage. This is really all you need to make a simple browser game. 

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
