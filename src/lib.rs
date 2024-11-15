use bevy::prelude::*;

use resource::{ensure_max_stat, ensure_max_stat_with_percentage};
pub use resource::{RPGResource, Resource};
use stat::{
    update_modded_stats_addmul, update_modded_stats_avediff, update_modded_stats_muladd,
    update_modded_stats_sumdiff,
};
pub use stat::{RPGStat, Stat};
pub use statmod::{DeleteStatMod, ModStyle, ResourceChangeEvent, StatChangeEvent};
use systems::{change_resource, delete_stat_mod};

pub mod resource;
pub mod stat;
pub mod statmod;
pub mod systems;

#[derive(Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub struct StatPlugin;

impl Plugin for StatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeleteStatMod>();
        app.add_systems(Update, delete_stat_mod); //TODO: See if Update is the right time to be
                                                  //running this
    }
}

pub trait StatRegisterable {
    fn register_stat<T: RPGStat + TypePath>(&mut self) -> &mut App;
    fn register_resource<T: RPGResource + TypePath>(&mut self) -> &mut App;
}

impl StatRegisterable for App {
    fn register_stat<T: RPGStat + TypePath>(&mut self) -> &mut App {
        self.register_type::<Stat<T>>();
        self.add_event::<StatChangeEvent<T>>();

        match T::modstyle() {
            ModStyle::AddMul => {
                self.add_systems(Update, update_modded_stats_addmul::<T>);
            }
            ModStyle::MulAdd => {
                self.add_systems(Update, update_modded_stats_muladd::<T>);
            }
            ModStyle::AverageDifferences => {
                self.add_systems(Update, update_modded_stats_avediff::<T>);
            }
            ModStyle::SumDifferences => {
                self.add_systems(Update, update_modded_stats_sumdiff::<T>);
            }
        }
        return self;
    }

    fn register_resource<T: RPGResource + TypePath>(&mut self) -> &mut App {
        self.register_stat::<T>();
        self.register_type::<Resource<T>>();
        self.add_event::<ResourceChangeEvent<T>>();

        self.add_systems(
            Update,
            (
                change_resource::<T>,
                ensure_max_stat::<T>,
                ensure_max_stat_with_percentage::<T>,
            ),
        );
        return self;
    }
}

#[macro_export]
macro_rules! DeclareResources {
    ($($a:ident),*) => {
        $(
        #[derive(Reflect, Clone, Copy)]
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
        #[derive(Reflect, Clone, Copy)]
        pub struct $a;
        impl RPGStat for $a {}
        )*
    };
}
