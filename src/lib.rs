use std::marker::PhantomData;

#[allow(unused_imports)] // reason: used in docs
use bevy::{app::FixedMain, ecs::schedule::ScheduleLabel, prelude::*};

#[cfg(feature = "derive")]
pub use bevy_previous_derive::DefaultSchedule;

/// A component that represents the previous value of another component `T`.
/// To enable previous-value-tracking for a component use [`PreviousPlugin`].
/// The parameter `S` must be the same as the one specified in [`PreviousPlugin`],
/// or be ommited, like with [`PreviousPlugin`].
///
/// You don't have to manually add [`Previous`] to your entity.
/// This is done automatically in the specified schedule `S`.
///
/// Also note that queries like `Query<(&T, &Previous<T>)>` won't match entities
/// that were just created, as the may not have [`Previous`] yet.
///
/// Like with [`PreviousPlugin`], there is a [`FixedMain`] type alias for it: [`FixedUpdate`].
///
/// # Examples
///
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_previous::*;
///
/// #[derive(Component, Clone)]
/// struct Health(pub u32);
///
/// fn main() {
///     App::new()
///         .add_plugins(PreviousPlugin::<Health>::default())
///         .add_systems(Update, print_differences)
///         .run();
/// }
///
/// fn print_differences(query: Query<(&Health, &Previous<Health>), Changed<Health>>) {
///     for (health, previous_health) in &query {
///         println!("Health reduced by {}", previous_health.0.0 - health.0);
///     }
/// }
/// ```
///
/// With custom schedule:
///
/// ```rust
/// # use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
/// # use bevy_previous::*;
/// #
/// # #[derive(DefaultSchedule, ScheduleLabel, Debug, Clone, Hash, PartialEq, Eq)]
/// # struct GameLogic;
///
/// #[derive(Component, Clone)]
/// struct Health(pub u32);
///
/// #[derive(DefaultSchedule, ScheduleLabel, Debug, Clone, Hash, PartialEq, Eq)]
/// struct AfterGameLogic;
///
///
/// // create a type alias to reduce boilerplate
/// type Previous<T> = bevy_previous::Previous<T, AfterGameLogic>;
///
/// fn main() {
///     App::new()
///         .add_plugins(PreviousPlugin::<Health, AfterGameLogic>::default())
///         .add_systems(GameLogic, print_differences)
///         .run();
/// }
///
/// fn print_differences(query: Query<(&Health, &Previous<Health>), Changed<Health>>) {
///     for (health, previous_health) in &query {
///         println!("Health reduced by {}", previous_health.0.0 - health.0);
///     }
/// }
/// ```
#[derive(Component, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Previous<T: Component + Clone, S: ScheduleLabel + Clone = Last>(pub T, PhantomData<S>);

impl<T, S> Previous<T, S>
where
    T: Component + Clone,
    S: ScheduleLabel + Clone,
{
    pub fn new(value: T) -> Self {
        Previous(value, PhantomData)
    }
}

impl<T, S> From<T> for Previous<T, S>
where
    T: Component + Clone,
    S: ScheduleLabel + Clone,
{
    fn from(value: T) -> Self {
        Previous::new(value)
    }
}

/// A type alias for [`Previous<T, FixedLast>`] to be used with [`FixedPreviousPlugin<T>`].
///
/// # Examples
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_previous::*;
///
/// #[derive(Component, Clone)]
/// struct Health(pub u32);
///
/// fn main() {
///     App::new()
///         .add_plugins(FixedPreviousPlugin::<Health>::default())
///         .add_systems(Update, print_differences)
///         .run();
/// }
///
/// fn print_differences(query: Query<(&Health, &FixedPrevious<Health>), Changed<Health>>) {
///     for (health, previous_health) in &query {
///         println!("Health reduced by {}", previous_health.0.0 - health.0);
///     }
/// }
/// ```
pub type FixedPrevious<T> = Previous<T, FixedLast>;

/// A Plugin to activate the [`Previous`] component for a given component `T`.
/// The parameter `S` defines the schedule where [`Previous<T>`] components are
/// set back to the value of `T`. This should be after all of your game logic,
/// so it is set to [`Last`] by default. For [`FixedLast`], the type alias [`FixedPreviousPlugin`]
/// is provided.
///
/// If the schedule implements [`DefaultSchedule`] (which all standard schedules do),
/// you can use `PreviousPlugin::<T, S>::default()` (`S` may be omitted, defaults to [`Last`]).
/// Otherwise, you will either have to implement [`DefaultSchedule`] for your schedule,
/// or provide a schedule with `PreviousPlugin::<T, S>::new(schedule)`.
///
/// # Examples
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_previous::*;
///
/// #[derive(Component, Clone)]
/// struct Health(pub u32);
///
/// fn main() {
///     App::new()
///         .add_plugins(PreviousPlugin::<Health>::default())
///         .add_systems(Update, print_differences)
///         .run();
/// }
///
/// fn print_differences(query: Query<(&Health, &Previous<Health>), Changed<Health>>) {
///     for (health, previous_health) in &query {
///         println!("Health reduced by {}", previous_health.0.0 - health.0);
///     }
/// }
/// ```
///
/// Custom schedule:
///
/// ```
/// # use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
/// # use bevy_previous::*;
/// #
/// # #[derive(DefaultSchedule, ScheduleLabel, Debug, Clone, Hash, PartialEq, Eq)]
/// # struct GameLogic;
///
/// #[derive(Component, Clone)]
/// struct Health(pub u32);
///
/// #[derive(DefaultSchedule, ScheduleLabel, Debug, Clone, Hash, PartialEq, Eq)]
/// struct AfterGameLogic;
///
/// // create a type alias to reduce boilerplate
/// type Previous<T> = bevy_previous::Previous<T, AfterGameLogic>;
///
/// App::new()
///     .add_plugins(PreviousPlugin::<Health, AfterGameLogic>::default());
/// ```
///
/// Or:
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_previous::*;
/// # mod other_lib {
/// #   use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
/// #   #[derive(ScheduleLabel, Debug, Clone, Hash, PartialEq, Eq)]
/// #   pub struct Schedule;
/// # }
///
/// // doesn't impl DefaultSchedule
/// use other_lib::Schedule;
///
/// #[derive(Component, Clone)]
/// struct Health(pub u32);
///
/// // create a type alias to reduce boilerplate
/// type Previous<T> = bevy_previous::Previous<T, Schedule>;
///
/// App::new()
///     .add_plugins(PreviousPlugin::<Health, Schedule>::new(Schedule))
///     .run();
/// ```
#[derive(Debug, Clone)]
pub struct PreviousPlugin<T: Component + Clone, S: ScheduleLabel + Clone = Last> {
    schedule: S,
    _t: PhantomData<T>,
}

/// A type alias for [`PreviousPlugin<T, FixedLast>`] to be used with [`FixedPrevious<T>`].
///
/// *See [PreviousPlugin] for more info*
pub type FixedPreviousPlugin<T> = PreviousPlugin<T, FixedLast>;

impl<T, S> Plugin for PreviousPlugin<T, S>
where
    T: Component + Clone,
    S: ScheduleLabel + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_systems(self.schedule.clone(), update::<T>);
    }
}

type UpdateFilter<T> = Or<(Without<Previous<T>>, Changed<T>)>;
fn update<T: Component + Clone>(
    mut commands: Commands,
    query: Query<(Entity, &T), UpdateFilter<T>>,
) {
    for (entity, t) in &query {
        commands
            .entity(entity)
            .insert(Previous::<T>::new(t.clone()));
    }
}

impl<T, S> PreviousPlugin<T, S>
where
    T: Component + Clone,
    S: ScheduleLabel + Clone,
{
    pub fn new(schedule: S) -> PreviousPlugin<T, S> {
        PreviousPlugin {
            schedule,
            _t: PhantomData,
        }
    }
}

impl<T: Component + Clone, S: ScheduleLabel + Clone> Default for PreviousPlugin<T, S>
where
    T: Component + Clone,
    S: ScheduleLabel + Clone + DefaultSchedule,
{
    fn default() -> Self {
        Self::new(S::default())
    }
}

/// A trait to provide the default value for a schedule label.
///
/// For most schedule labels, that are unit structs, `#[derive(DefaultSchedule)]`
/// will work.
/// For schedule labels that aren't unit structs, implementing [`DefaultSchedule`]
/// doesn't make much sense anyways.
///
/// Why not just use [`Default`]? None of the bevy schedule labels actually implement
/// [`Default`], and so this crate had to make it's own trait.
pub trait DefaultSchedule {
    fn default() -> Self;
}

mod default_schedule_impls {
    use super::DefaultSchedule;

    use bevy::app::*;

    macro_rules! default_schedule_impls {
        ($($schedule:ident),*) => {
            $(
                impl DefaultSchedule for $schedule {
                    fn default() -> Self {
                        $schedule
                    }
                }
            )*
        };
    }

    default_schedule_impls!(PreStartup, Startup, PostStartup);
    default_schedule_impls!(Main, First, PreUpdate, Update, PostUpdate, Last);
    default_schedule_impls!(
        FixedMain,
        FixedFirst,
        FixedPreUpdate,
        FixedUpdate,
        FixedPostUpdate,
        FixedLast
    );
}
