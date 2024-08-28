use bevy::{
    ecs::system::QueryLens,
    prelude::{Commands, Entity, EventReader, Query},
};

use crate::{
    statmod::{DeleteStatMod, ModType, ResourceChangeEvent, StatChangeEvent, StatValueChange},
    RPGResource, RPGStat, Resource, Stat,
};

pub(crate) fn mul_stats<T: RPGStat>(
    base: f32,
    statlist: &Vec<Entity>,
    mods: &mut QueryLens<&StatValueChange<T>>,
) -> f32 {
    let mods = mods.query();
    statlist
        .iter()
        .filter_map(|w| mods.get(*w).ok())
        .filter(|w| w.mod_type == ModType::Multiplier)
        .fold(base, |w, v| w * (1. + v.value)) // TODO: Add handling for additive multiplierstyle
}

pub(crate) fn mul_diff<T: RPGStat>(
    base: f32,
    statlist: &Vec<Entity>,
    mods: &mut QueryLens<&StatValueChange<T>>,
) -> f32 {
    let mods = mods.query();
    statlist
        .iter()
        .filter_map(|w| mods.get(*w).ok())
        .filter(|w| w.mod_type == ModType::Multiplier)
        .fold(0., |w, v| w + v.value * base) // TODO: Add handling for additive multiplierstyle
}

pub(crate) fn add_stats<T: RPGStat>(
    base: f32,
    statlist: &Vec<Entity>,
    mods: &mut QueryLens<&StatValueChange<T>>,
) -> f32 {
    let mods = mods.query();
    statlist
        .iter()
        .filter_map(|w| mods.get(*w).ok())
        .filter(|w| w.mod_type == ModType::Offset)
        .fold(base, |w, v| w + v.value)
}

pub fn delete_stat_mod(mut commands: Commands, mut events: EventReader<DeleteStatMod>) {
    for DeleteStatMod(entity) in events.read() {
        commands.get_entity(*entity).unwrap().despawn();
    }
}

pub fn change_stat<T: RPGStat>(
    mut events: EventReader<StatChangeEvent<T>>,
    mut query: Query<&mut Stat<T>>,
) {
    for StatChangeEvent { change, target } in events.read() {
        let base = change.apply(query.get(*target).unwrap().base);
        query.get_mut(*target).unwrap().base = base;
    }
}

pub fn change_resource<T: RPGResource>(
    mut events: EventReader<ResourceChangeEvent<T>>,
    mut query: Query<(&mut Resource<T>, &Stat<T>)>,
) {
    for ResourceChangeEvent { change, target } in events.read() {
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
