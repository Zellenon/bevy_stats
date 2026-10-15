use bevy::prelude::{Component, Entity, Message, Reflect};
use core::marker::PhantomData;

use crate::stat_change::StatChange;
use crate::{RPGResource, RPGStat};

#[derive(Component, Debug, Reflect, PartialEq, Clone, Copy)]
pub struct StatModifier<T>
where
    T: RPGStat,
{
    pub change: StatChange<T>,
    pub _phantom: PhantomData<T>,
}

#[derive(PartialOrd, Ord, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ModStyle {
    AddMul,
    MulAdd,
    AverageDifferences,
    SumDifferences,
}

#[derive(PartialOrd, Ord, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub enum ResourceModUpdateStyle {
    ScaleBoth,
    ScaleOnBuff,
    ScaleOnDebuff,
    NoScale,
}

#[derive(Message, Debug, Reflect, PartialEq, Eq, Clone, Copy)]
pub struct DeleteStatMod(pub Entity);

#[derive(Message, Debug, Reflect, PartialEq, Clone, Copy)]
pub struct StatChangeMessage<T>
where
    T: RPGStat,
{
    pub change: StatValueChange<T>,
    pub target: Entity,
}

#[derive(Message, Debug, Reflect, PartialEq, Clone, Copy)]
pub struct ResourceChangeMessage<T>
where
    T: RPGResource,
{
    pub change: StatValueChange<T>,
    pub target: Entity,
}
