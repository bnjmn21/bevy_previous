![Crates.io Size](https://img.shields.io/crates/size/bevy_previous?label=size)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/bnjmn21/bevy_previous)
![MIT License](https://img.shields.io/crates/l/bevy_previous)
![Bevy 0.14](https://img.shields.io/badge/bevy-0.14-green)

# `bevy_previous`

A simple library for [bevy](https://docs.rs/bevy) to access previous values of components.

## Example

Suppose a game that consists of enemies with a `struct Health(pub u32)` component.
Whenever an enemy is hit, a text should appear showing the amount of health they lost.

For this we can use bevy's change detection, however to show the difference in health we also need
the previous health value. This is where `bevy_previous` comes in.

```rust
struct Health(pub u32);

fn main() {
    App::new()
        .add_plugins(PreviousPlugin::<Health>::default())
        .add_systems(Update, print_differences)
        .run();
}

fn print_differences(query: Query<(&Health, &Previous<Health>), Changed<Health>>) {
    for (health, previous_health) in &query {
        println!("Health reduced by {}", previous_health - health);
    }
}
```

## In-depth

Register the `PreviousPlugin::<T>` to activate previous values for a component `T`.
Note that by default, changes are updated in the `Last` schedule.
You also do not need to manually add the `Previous<T>` component,
that is done automatically in the `Last` schedule.
This also means that `Query<(&T, &Previous<T>)>`
will not match `T` entities that were just created.

If you want to customize the schedule where `Previous<T>` gets set back to `T`,
you can pass that in as a second type parameter in `PreviousPlugin` as well as `Previous` like this.

```rust
struct Health(pub u32);

fn main() {
    App::new()
        .add_plugins(PreviousPlugin::<Health, FixedLast>::default())
        .add_systems(FixedUpdate, print_differences)
        .run();
}

fn print_differences(query: Query<(&Health, &Previous<Health, FixedLast>), Changed<Health>>) {
    for (health, previous_health) in &query {
        println!("Health reduced by {}", previous_health - health);
    }
}
```

The only important thing is that `Previous` gets updated *after* your game logic.

Since this specific use case with `FixedMain` is quite common, there are type aliases for it:
`FixedPreviousPlugin<T>` and `FixedPrevious<T>`.
If you have custom schedules, consider adding type aliases for those too.

## Compatability

`bevy` | `bevy_previous`
-------|----------------
`0.14` | `1.0.0`

Additionally, the main branch of `bevy_previous` is
up-to-date with bevy's main branch.

## A note on `PreviousPlugin::default()`

This only works for schedules that implement the `DefaultSchedule` trait.
This is automatically implemented by the standard bevy schedules, but
for your own, you must either manually implement `DefaultSchedule` using
`derive(DefaultSchedule)` helper macro or by using `PreviousPlugin::new(schedule)`.

## Feature Flags

Flag     | Description
---------|-------------
`derive` | *default:* Enable the derive macro for `DefaultSchedule`
`serde`  | Enable `serde` implementations for `Previous`