use bevy::{
    prelude::{
        App, Bundle, ClearColor, Color, Commands, DespawnRecursiveExt, Entity, Event, EventReader,
        EventWriter, IntoSystemConfigs, Name, Query, Startup, Update, With,
    },
    reflect::TypePath,
    DefaultPlugins,
};
use bevy_stats::{
    statmod::{ResourceChangeEvent, StatValueChange},
    systems::{change_resource, StatRegisterable},
    RPGResource, RPGStat, Resource, Stat,
};
use rand::seq::SliceRandom;

#[derive(TypePath)]
pub struct Health;
#[derive(TypePath)]
pub struct Damage;

impl RPGStat for Health {}
impl RPGResource for Health {}
impl RPGStat for Damage {}

#[derive(Bundle)]
struct Fighter {
    name: Name,
    health: Resource<Health>,
    damage: Stat<Damage>,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(ClearColor(Color::rgb(
        0xA9 as f32 / 255.0,
        0xA9 as f32 / 255.0,
        0xAF as f32 / 255.0,
    )))
    .add_event::<FightEvent>();

    // ------ Important Stuff ---------

    app.register_stat::<Damage>();
    app.register_resource::<Health>();

    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            (choose_fight, do_attack)
                .chain()
                .before(change_resource::<Health>),
            die_on_hp0.after(change_resource::<Health>),
        ),
    );

    // --------------------------------

    app.run();
    Ok(())
}

#[derive(Event)]
struct FightEvent {
    attacker: Entity,
    defender: Entity,
}

fn setup(mut commands: Commands) {
    commands.spawn(Fighter {
        name: "Fred".into(),
        health: Resource::new(5.),
        damage: Stat::new(2.5),
    });

    commands.spawn(Fighter {
        name: "Mike".into(),
        health: Resource::new(5.),
        damage: Stat::new(2.),
    });

    commands.spawn(Fighter {
        name: "Bill".into(),
        health: Resource::new(10.),
        damage: Stat::new(0.5),
    });

    commands.spawn(Fighter {
        name: "RHYP".into(),
        health: Resource::new(2.),
        damage: Stat::new(1.),
    });
}

// Everything past this point just makes them slap each other randomly.

// Picks 2 random fighters to slap each other
fn choose_fight(fighters: Query<Entity, With<Stat<Damage>>>, mut fights: EventWriter<FightEvent>) {
    let fighters: Vec<_> = fighters.iter().collect();
    if fighters.len() < 2 {
        return;
    }
    let fighters: Vec<_> = fighters
        .choose_multiple(&mut rand::thread_rng(), 2)
        .collect();
    fights.send(FightEvent {
        attacker: *fighters[0],
        defender: *fighters[1],
    });
}

// Once the 2 fighters are chosen, have the attacker send a "health decrease event" to the
// defender.
fn do_attack(
    mut fights: EventReader<FightEvent>,
    damage_stats: Query<&Stat<Damage>>,
    names: Query<&Name>,
    mut damages: EventWriter<ResourceChangeEvent<Health>>,
) {
    for FightEvent { attacker, defender } in fights.iter() {
        let damage = damage_stats.get(*attacker).unwrap().current_value();
        println!(
            "{} hits {}. He takes {} damage.",
            names.get(*attacker).unwrap(),
            names.get(*defender).unwrap(),
            damage
        );
        damages.send(ResourceChangeEvent {
            change: StatValueChange::offset(-0.5 * damage),
            target: *defender,
        })
    }
}

// People die when they are dead.
fn die_on_hp0(
    healths: Query<(Entity, &Resource<Health>)>,
    mut commands: Commands,
    names: Query<&Name>,
) {
    for (e, health) in healths.iter() {
        if health.current_value() <= 0. {
            println!("{} dies!", names.get(e).unwrap());
            commands.entity(e).despawn_recursive()
        }
    }
}
