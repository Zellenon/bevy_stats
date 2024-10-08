use bevy::prelude::Reflect;
use bevy::prelude::{Component, Entity, Event};
use std::marker::PhantomData;

use crate::{RPGResource, RPGStat};

#[derive(PartialOrd, Ord, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ModStyle {
    AddMul,
    MulAdd,
    AverageDifferences,
    SumDifferences,
}

#[derive(PartialOrd, Ord, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ModType {
    Offset,
    Multiplier,
}

#[derive(PartialOrd, Ord, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum MultiplierStyle {
    Additive,
    Multiplicative,
}

#[derive(PartialOrd, Ord, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ResourceModUpdateStyle {
    ScaleBoth,
    ScaleOnBuff,
    ScaleOnDebuff,
    NoScale,
}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
pub struct StatValueChange<T>
where
    T: RPGStat,
{
    pub mod_type: ModType,
    pub value: f32,
    pub _phantom: PhantomData<T>,
}

impl<T> StatValueChange<T>
where
    T: RPGStat,
{
    pub fn new(value: f32, mod_type: ModType) -> Self {
        Self {
            mod_type,
            value,
            _phantom: PhantomData,
        }
    }

    pub fn offset(value: f32) -> Self {
        Self {
            mod_type: ModType::Offset,
            value,
            _phantom: PhantomData,
        }
    }

    pub fn multiplier(value: f32) -> Self {
        Self {
            mod_type: ModType::Multiplier,
            value,
            _phantom: PhantomData,
        }
    }
    pub fn apply(&self, stat: f32) -> f32 {
        match T::can_negative() {
            true => match self.mod_type {
                ModType::Offset => stat + self.value,
                ModType::Multiplier => stat * self.value,
            },
            false => match self.mod_type {
                ModType::Offset => (stat + self.value).max(0.),
                ModType::Multiplier => (stat * self.value).max(0.),
            },
        }
    }
}

#[derive(Component, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub struct StatModifier;

#[derive(Event, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub struct DeleteStatMod(pub Entity);

#[derive(Event, Debug, Reflect, PartialEq, Clone, Copy)]
pub struct StatChangeEvent<T>
where
    T: RPGStat,
{
    pub change: StatValueChange<T>,
    pub target: Entity,
}

#[derive(Event, Debug, Reflect, PartialEq, Clone, Copy)]
pub struct ResourceChangeEvent<T>
where
    T: RPGResource,
{
    pub change: StatValueChange<T>,
    pub target: Entity,
}
