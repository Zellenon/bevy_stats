use bevy::prelude::{Component, Entity, Message, Reflect};
use std::marker::PhantomData;

use crate::{RPGResource, RPGStat};

#[derive(PartialOrd, Ord, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ChangeType {
    Added,
    Multiplied,
}

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
pub struct StatChange<T>
where
    T: RPGStat,
{
    pub mod_type: ChangeType,
    pub value: f32,
    pub _phantom: PhantomData<T>,
}

impl<T> StatChange<T>
where
    T: RPGStat,
{
    pub fn new(value: f32, mod_type: ChangeType) -> Self {
        Self {
            mod_type,
            value,
            _phantom: PhantomData,
        }
    }

    pub fn offset(value: f32) -> Self {
        Self {
            mod_type: ChangeType::Added,
            value,
            _phantom: PhantomData,
        }
    }

    pub fn multiplier(value: f32) -> Self {
        Self {
            mod_type: ChangeType::Multiplied,
            value,
            _phantom: PhantomData,
        }
    }
    pub fn apply(&self, stat: f32) -> f32 {
        match T::can_negative() {
            true => match self.mod_type {
                ChangeType::Added => stat + self.value,
                ChangeType::Multiplied => stat * self.value,
            },
            false => match self.mod_type {
                ChangeType::Added => (stat + self.value).max(0.),
                ChangeType::Multiplied => (stat * self.value).max(0.),
            },
        }
    }
}
