use bevy::{
    ecs::query::ReadOnlyWorldQuery,
    prelude::{App, Commands, Entity, EventReader, Query, Update},
    reflect::TypePath,
};

use crate::{
    resource::{ensure_max_stat, ensure_max_stat_with_percentage},
    stat::{
        update_modded_stats_addmul, update_modded_stats_avediff, update_modded_stats_muladd,
        update_modded_stats_sumdiff,
    },
    statmod::{
        DeleteStatMod, ModStyle, ModType, ResourceChangeEvent, StatChangeEvent, StatValueChange,
    },
    RPGResource, RPGStat, Resource, Stat,
};

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

pub(crate) fn mul_stats<T: RPGStat, F: ReadOnlyWorldQuery>(
    base: f32,
    statlist: &Vec<Entity>,
    mods: &Query<&StatValueChange<T>, F>,
) -> f32 {
    statlist
        .iter()
        .filter_map(|w| mods.get(*w).ok())
        .filter(|w| w.mod_type == ModType::Multiplier)
        .fold(base, |w, v| w * (1. + v.value)) // TODO: Add handling for additive multiplierstyle
}

pub(crate) fn mul_diff<T: RPGStat, F: ReadOnlyWorldQuery>(
    base: f32,
    statlist: &Vec<Entity>,
    mods: &Query<&StatValueChange<T>, F>,
) -> f32 {
    statlist
        .iter()
        .filter_map(|w| mods.get(*w).ok())
        .filter(|w| w.mod_type == ModType::Multiplier)
        .fold(0., |w, v| w + v.value * base) // TODO: Add handling for additive multiplierstyle
}

pub(crate) fn add_stats<T: RPGStat, F: ReadOnlyWorldQuery>(
    base: f32,
    statlist: &Vec<Entity>,
    mods: &Query<&StatValueChange<T>, F>,
) -> f32 {
    statlist
        .iter()
        .filter_map(|w| mods.get(*w).ok())
        .filter(|w| w.mod_type == ModType::Offset)
        .fold(base, |w, v| w + v.value)
}

pub fn delete_stat_mod(mut commands: Commands, mut events: EventReader<DeleteStatMod>) {
    for DeleteStatMod(entity) in events.iter() {
        commands.get_entity(*entity).unwrap().despawn();
    }
}

pub fn change_stat<T: RPGStat>(
    mut events: EventReader<StatChangeEvent<T>>,
    mut query: Query<&mut Stat<T>>,
) {
    for StatChangeEvent { change, target } in events.iter() {
        let base = change.apply(query.get(*target).unwrap().base);
        query.get_mut(*target).unwrap().base = base;
    }
}

pub fn change_resource<T: RPGResource>(
    mut events: EventReader<ResourceChangeEvent<T>>,
    mut query: Query<(&mut Resource<T>, &Stat<T>)>,
) {
    for ResourceChangeEvent { change, target } in events.iter() {
        let (new_base, new_percentage) = {
            let response = query.get(*target);
            if let Err(_) = response {
                continue;
            }
            let (base, stat) = response.unwrap();
            let mut new_base = change.apply(base.current);
            if !T::can_negative() {
                new_base = new_base.max(0.);
            }
            if !T::can_overmax() {
                new_base = new_base.min(stat.current);
            }
            (new_base, new_base / stat.current)
        };
        let mut resource = query.get_mut(*target).unwrap().0;
        resource.current = new_base;
        resource.percent = new_percentage;
    }
}
