# daily-bevy

Learn [Bevy](https://bevyengine.org/) by exploring a small example (almost) every day.

[Bevy](https://github.com/bevyengine/bevy/) is a free, open-source, cross-platform (Windows, macOS, Linux, Web, iOS, Android) game engine written in [Rust](https://www.rust-lang.org/).

This README shows the sixteenth entry in this series. All other entries can be found at [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches).

## Game Menu, Part 2

Today is the sixteenth day of Daily Bevy.

This kata uses [Bevy `v0.12.1`](https://github.com/bevyengine/bevy/tree/v0.12.1).

### Today's Kata

Today, we will be looking at more of the [`game_menu` example](https://github.com/bevyengine/bevy/blob/release-0.12.1/examples/games/game_menu.rs) from the Bevy repo, which we started yesterday.

#### The Code

We are using the same assets and code [as in the previous kata](https://github.com/awwsmm/daily-bevy/tree/games/game_menu).

#### Discussion

We left off yesterday with two more systems to dig into, `game` and `menu`.

Let's start with `menu`, because that's always the next state after `Splash` in the [FSM](https://en.wikipedia.org/wiki/Finite-state_machine).

---

In the previous kata, we discussed how we move out of the `Splash` state and into the `Menu` state. If you need a refresher of how that works, please refer to [the README from yesterday](https://github.com/awwsmm/daily-bevy/tree/games/game_menu).

`menu` is ~580 lines long, so this is going to be a bit of a marathon. Let's get started.

Right away, this might be confusing

```rust
// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub struct MenuPlugin;
```

...because I only count four screens there, not five. And there are only four marker `Component`s to tag the entities on each screen

```rust
// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;
```

But there's also a "`Disabled`" state, which is the default state...

```rust
// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}
```

...and the state that the menu enters when we hit the "New Game" button to start the game

```rust
MenuButtonAction::Play => {
    game_state.set(GameState::Game);
    menu_state.set(MenuState::Disabled);
}
```

---

`impl Plugin for MenuPlugin` describes all of the state transitions

```rust
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::Menu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            // Systems to handle the main menu screen
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            // Systems to handle the settings menu screen
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_screen::<OnSettingsMenuScreen>,
            )
            // Systems to handle the display settings screen
            .add_systems(
                OnEnter(MenuState::SettingsDisplay),
                display_settings_menu_setup,
            )
            .add_systems(
                Update,
                (
                    setting_button::<DisplayQuality>
                        .run_if(in_state(MenuState::SettingsDisplay)),
                ),
            )
            .add_systems(
                OnExit(MenuState::SettingsDisplay),
                despawn_screen::<OnDisplaySettingsMenuScreen>,
            )
            // Systems to handle the sound settings screen
            .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
            .add_systems(
                Update,
                setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
            )
            .add_systems(
                OnExit(MenuState::SettingsSound),
                despawn_screen::<OnSoundSettingsMenuScreen>,
            )
            // Common systems to all screens that handles buttons behavior
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            );
    }
}
```

There's a lot here. Let's break it down line-by-line.

First, we register `MenuState`, which -- remember -- sets the current `State` to the `Default` (`Disabled`) state and the `NextState` to `None`.

```rust
.add_state::<MenuState>()
```

But, in addition to the `MenuState`, we have the `GameState` (we have two Finite State Machines, one inside the other). Recall that the `SplashPlugin` handles entering, exiting, and existing within the `Splash` `GameState`. If you look ahead to the `GamePlugin`, you'll see that that `Plugin` also handles `OnEnter`, `OnExit`, and has a `run_if(in_state(...))` for the `Game` `GameState`.

But in the `MenuPlugin`, we only have an `OnEnter` and a `run_if(in_state(...))` for the `Menu` `GameState`. There is no `OnExit` for the `Menu` `GameState` anywhere. Why? Well, whenever we exit the `Menu` `GameState`, there are only two things that can happen

1. we are quitting the app
2. we are moving into the `Game` `GameState`

We will see this aaaaallllll the way at the end of this module, in the `menu_action` system, but whenever we enter the `Game` `GameState`, we set the `MenuState` to `Disabled`, exiting the `Main` `MenuState`. In other words, the situations in which `OnExit(GameState::Menu)` would fire is a strict subset of the situations in which `OnExit(MenuState::Main)` would fire. So, as long as we don't want specific logic for when we are quitting the app, `OnExit(MenuState::Main)` handles all situations in which `OnExit(GameState::Menu)` would fire.

Here's a diagram of the states in this module, which may make the states and transitions clearer

![](assets/diagram.png)

When we exit the `Main` `MenuState`, we just want to despawn everything tagged to the main menu

```rust
.add_systems(OnEnter(GameState::Menu), menu_setup)
// Systems to handle the main menu screen
.add_systems(OnEnter(MenuState::Main), main_menu_setup)
.add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
```

So, above, we've got an `OnEnter` and an `OnExit` for the `Main` `MenuState`. Below, we've got the same handlers for the `Settings` `MenuState` (the main settings menu)

```rust
// Systems to handle the settings menu screen
.add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
.add_systems(
    OnExit(MenuState::Settings),
    despawn_screen::<OnSettingsMenuScreen>,
)
```

...and for the `SettingsDisplay` `MenuState`, which also has a `run_if(in_state(...))` system which is run during the `Update` schedule

```rust
// Systems to handle the display settings screen
.add_systems(
    OnEnter(MenuState::SettingsDisplay),
    display_settings_menu_setup,
)
.add_systems(
    Update,
    (
        setting_button::<DisplayQuality>
            .run_if(in_state(MenuState::SettingsDisplay)),
    ),
)
.add_systems(
    OnExit(MenuState::SettingsDisplay),
    despawn_screen::<OnDisplaySettingsMenuScreen>,
)
```

...and similar for the `SettingsSound` `MenuState`

```rust
// Systems to handle the sound settings screen
.add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
.add_systems(
    Update,
    setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
)
.add_systems(
    OnExit(MenuState::SettingsSound),
    despawn_screen::<OnSoundSettingsMenuScreen>,
)
```

Finally, we've got our `run_if(in_state(...))` handler for the `Menu` `GameState`, mentioned earlier

```rust
// Common systems to all screens that handles buttons behavior
.add_systems(
    Update,
    (menu_action, button_system).run_if(in_state(GameState::Menu)),
);
```

---

After the `App` setup, we define the five `MenuState`s, which we've seen the FSM transitions for, above

```rust
// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}
```

...and the marker `Component`s which are used to tag entities in each of these `MenuState`s

```rust
// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component)]
struct OnSoundSettingsMenuScreen;
```

---

Then, we define the colors of the buttons, which can be in four states: normal, hovered, pressed, and pressed-and-hovered.

```rust
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15); // <- this is a dark grey
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25); // <- this is a bright grey
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25); // this is a dark green
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35); // <- this is a bright green
```

...and we define a marker `Component` which is applied to the menu setting which is currently selected

```rust
// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;
```

Throughout all the different submenus, there are lots of different buttons with lots of different actions

```rust
// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play, // <- play the game, visible in the `Main` menu
    Settings, // <- open the settings menu, visible in the `Main` menu
    SettingsDisplay, // <- open the `SettingsDisplay` menu, visible in the `Settings` menu
    SettingsSound, // <- open the `SettingsSound` menu, visible in the `Settings` menu
    BackToMainMenu, // <- open the `Main` menu, visible in the `Settings` menu
    BackToSettings, // <- open the `Settings` menu, visible in the `SettingsDisplay` and `SettingsSound` menus
    Quit, // <- quit the app, visible in the `Main` `MenuState`
}
```

- `Play` moves the `GameState` from `Menu` to `Game` and the `MenuState` from `Main` to `Disabled`
- `Settings` moves the `MenuState` from `Main` to `Settings`
- `SettingsDisplay` moves the `MenuState` from `Settings` to `SettingsDisplay`
- `SettingsSound` moves the `MenuState` from `Settings` to `SettingsSound`
- `BackToMainMenu` moves the `MenuState` from `Settings` to `Main`
- `BackToSettings` moves the `MenuState` from  `SettingsDisplay` or `SettingsSound` to `Settings`

All of this is shown graphically in the diagram above.

---

Next, we've got the `button_system`

```rust
// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}
```

This `Query` is quite complex, but we have seen most of these elements before. Let's break it down

```rust
(&Interaction, &mut BackgroundColor, Option<&SelectedOption>)
```

We want all entities which have an `Interaction` `Component` and a `BackgroundColor` `Component`. We haven't seen `Option` in the context of a `Query` before, though. The syntax `Option<&SelectedOption>` in a `Query` does what you might think it would do: it gives `None` for entities which _do not_ have a `SelectedOption` component, and `Some(selectedOption)` for entities which do.

The filter on this `Query`...

```rust
(Changed<Interaction>, With<Button>)
```

...means that the query will only return entities whose `Interaction` `Component` has `Changed` since the last `Update`, and only entities which contain `Button` `Component`s.

Why does `Button` appear in the filter and not in the required components? Great question! I think it's because we don't actually want to _do_ anything with the `Button` `Component`, we just want to know that this entity has one. (In fact, we _can't_ do anything with `Button`, since it's just a marker `Component`.) And actually, this filter is redundant anyway. As we'll see later, we only ever apply the `SelectedOption` `Component` to `With<Button>` entities, and so checking for it here as well is unnecessary. 

So `button_system` gets all entities with `Interaction`s, `BackgroundColor`s, and `SelectedOption`s, and adjusts the background color of each button based on the interaction and whether the button is currently selected.

Note that, once again, we have a Finite State Machine. A `Button` can move from the `None` `Interaction` state to the `Hovered` `Interaction` state, and from `Hovered` to `Pressed`, but not directly from `None` to `Pressed`. Similarly, it can move from `Pressed` back to `Hovered`, and from `Hovered` to `None`, but it cannot move directly from `Pressed` to `None`. So...

```rust
(Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into()
```

is handling the cases where

- the button was just pressed
- the mouse just hovered off of the button (moved from `Hovered` to `None`) and the button _was_ previously selected

In both of these cases, we have a "selected" button (`PRESSED_BUTTON`, the variable names are confusing) which is not currently being hovered over: it is currently being pressed, or the mouse is somewhere else.

The rest of the `match` arms are pretty self-explanatory

```rust
(Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into()
```

Above, we are hovering over the button, and it is the selected option (`HOVERED_PRESSED_BUTTON` is, again, not the clearest name here).

```rust
(Interaction::Hovered, None) => HOVERED_BUTTON.into()
```

Here, we are hovering over the button, and it is _not_ the selected option.

```rust
(Interaction::None, None) => NORMAL_BUTTON.into()
```

And finally, above, we are not interacting with the button in any way, and it is not the selected option.

All of these arms use `.into()` to turn these `Color`s into `BackgroundColors`, utilizing this `From<Color>` implementation

```rust
impl From<Color> for BackgroundColor {
    fn from(color: Color) -> Self {
        Self(color)
    }
}
```

---

After the `button_system`, we have the `setting_button` system

```rust
// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}
```

Again, this is quite complex, so let's take it step by step

```rust
T: Resource + Component + PartialEq + Copy
```

`T` must implement all four of these traits

- `Resource` is required to satisfy `ResMut<T>`
- `Component` is required for the `&T` in `interaction_query: Query<...>`
- `PartialEq` is required for the `!=` in `*setting != *button_setting`, and
- `Copy` is required for `*setting = *button_setting`

and `T` must be used in both `interaction_query` and `setting` because we are comparing `setting` to `button_setting` (which comes from `interaction_query`), and then assigning one to the other.

```rust
interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>
```

`interaction_query` queries for all entities with an `Interaction` `Component`, whatever this `T` `Component` is and... an `Entity` `Component`? Why do we need this?

The `Entity` `Component` "can be used to refer to a specific entity". Usually, we want to query for entities with specific components becasue we want to do things with those components. But in this case, we want to query for entities with specific components and then _mutate those entities_, so we need a reference to the entities themselves. Enter `Entity`.

And again, we have `(Changed<Interaction>, With<Button>)`. Earlier, when we realized that we didn't actually need `With<Button>` because it was redundant -- we were filtering on that component on both the "read" and "write" sides of this contract -- this is the line that was referring to. (Actually, we can get rid of _both_ of these `With<Button>` filters, because the only entities in this app with `Interaction` `Component`s are the buttons.)

Anyway, the next argument to `setting_button` is `selected_query`

```rust
mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>
```

Here, we are collecting all entities with `SelectedOption` and `BackgroundColor` components: i.e. selected buttons. This argument is `mut`, so we'll probably be changing some things here.

Finally, we've got our usual `mut commands: Commands`, followed by

```rust
mut setting: ResMut<T>
```

...a `setting`. So we finally need to look into what type `T` is. How is `setting_button` used? Well, it's called in two places. We've seen both of these already, when we walked through the `App` setup for this `Plugin`

```rust
    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                // -- snip --
                .add_systems(
                    Update,
                    (
                        setting_button::<DisplayQuality>
                            .run_if(in_state(MenuState::SettingsDisplay)),
                    ),
                )
                // -- snip --
                .add_systems(
                    Update,
                    setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
                )
            // -- snip --
        }
    }
```

So `T` is either `DisplayQuality` or `Volume` -- the two settings we allow the user to change in the settings menu.

Now, to the body of the method

```rust
for (interaction, button_setting, entity) in &interaction_query {
```

We get all entities with the particular `button_setting` component that we care about (`DisplayQuality` or `Volume`), and which have an `Interaction` component. As discussed before, this will _only_ be buttons in this app. Again we have a `Changed<Interaction>` in the filter for this query, so `interaction_query` will only return buttons which have just moved from one interaction state to another.

```rust
    if *interaction == Interaction::Pressed && *setting != *button_setting {
```

We filter to only the buttons which have just entered the `Pressed` state, and where the current configuration of this setting is _not_ equal to the previous configuration of this setting. In other words, we filter only to buttons which weren't previously selected, but are now.

```rust
let (previous_button, mut previous_color) = selected_query.single_mut();
*previous_color = NORMAL_BUTTON.into();
commands.entity(previous_button).remove::<SelectedOption>();
```

Next, we destructure the `selected_query`, which, remember, contains an entity with a `SelectedOption` `Component` and a `BackgroundColor` `Component`. According to the docs, `single_mut()` "panics if the number of query item is **not** exactly one", which confuses me a bit. Note that `selected_query` is not filtered to `T` at all -- it is returning _all_ entities containing `BackgroundColor` and `SelectedOption` components. But _both_ settings submenus have entities like this. My guess is that `single_mut()` only returns the entity we want because the entities on the other submenu are despawned when we make the appropriate `MenuState` FSM transition.

So anyway, the `selected_query` contains the _previously selected_ configuration of this menu setting. Since this button is being deselected (as a different button is being selected), we reset the color of this button to "normal" and remove the `SelectedOption` `Component` from this button. 

```rust
commands.entity(entity).insert(SelectedOption);
*setting = *button_setting;
```

Finally, we mutate the newly-selected button by _adding_ the `SelectedOption` component, and we update the `Resource` for this menu setting.

That's everything for `setting_button`.

---

Oh man, that was a lot, and we're still working through this. Looks like this is going to be a three-parter, at least.

See you tomorrow for the continuing story of `games/game_menu`.

## Learn More

If you found this first kata interesting, head over to [daily-bevy/branches](https://github.com/awwsmm/daily-bevy/branches) to see the rest of them.

If you have questions, comments, or corrections, please head over to [daily-bevy/discussions](https://github.com/awwsmm/daily-bevy/discussions) to join the conversation.

If you like what you've read above, you can [follow me on Bluesky](https://bsky.app/profile/awwsmm.bsky.social) or [Mastodon](https://mas.to/@awwsmm).
