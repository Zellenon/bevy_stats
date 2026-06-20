# Bevy Stats

> **Note:**
> Bevy Stats is a work-in-progress project. I don't currently offer any real support for it while it is still in development, though I am open to feedback and changes.

This is a simple Bevy implementation of Stats (Strength, Damage, Armor, etc) and Resources (Health, Mana, etc) that can be patched into any game to speed up gameplay loop implementation

## Quickstart

Following the examples in [examples](https://github.com/Zellenon/bevy_stats/tree/main/examples/basic.rs),

Stats and Resources can be declared with the corresponding Macros:

```rust,no_run
DeclareResources!(Health, Mana);
DeclareStat!(Damage);
```

- And remember to register them with the app:

```rust,no_run
app.register_stat::<Damage>();
app.register_resource::<Health>();
```

And then the Components can then be added to any struct serving as an Entity!

```rust,no_run
struct Fighter {
    name: Name,
    health: Resource<Health>,
    damage: Stat<Damage>,
}
```

Follow the examples further to see how Messages and the ValueChange enum can be used for health loss, lasting debuffs, stat changes on levelup, and more!
