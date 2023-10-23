use bevy::prelude::*;

pub use resource::{RPGResource, Resource};
pub use stat::{RPGStat, Stat};
pub use statmod::DeleteStatMod;
use systems::delete_stat_mod;
pub mod resource;
pub mod stat;
pub mod statmod;
pub mod systems;

pub struct StatPlugin;

impl Plugin for StatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeleteStatMod>();
        app.add_systems(Update, delete_stat_mod); //TODO: See if Update is the right time to be
                                                  //running this
    }
}

#[macro_export]
macro_rules! DeclareResources {
    ($($a:ident),*) => {
        $(
        #[derive(TypePath)]
        pub struct $a;
        impl RPGStat for $a {}
        impl RPGResource for $a {}
        )*
    };
}

#[macro_export]
macro_rules! DeclareStat {
    ($($a:ident),*) => {
        $(
        #[derive(TypePath)]
        pub struct $a;
        impl RPGStat for $a {}
        )*
    };
}
