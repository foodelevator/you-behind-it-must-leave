use std::process::exit;

use bevy::{prelude::*, input::{ButtonState, mouse::MouseButtonInput}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use chrono::Utc;
use rand::Rng;
use rand::rngs::StdRng;

use crate::Class::*;
use crate::Penalty::*;
use crate::Type::*;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let cur = Utc::now().timestamp_millis();
    let rng = rand::SeedableRng::seed_from_u64(cur as u64);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (set_health_bars, fight_tick, fight_start_pre, fight_start, draft_start, draft_tick, discard_start, discard_tick))
        .add_event::<FightStart>()
        .add_event::<DraftStart>()
        .add_event::<FightPreload>()
        .add_event::<DiscardStart>()
        .insert_resource(FightTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(Random(rng))
        .init_resource::<Party>()
        .insert_resource(DraftSettings { power: 1 })
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct Random(StdRng);

#[derive(Event, Clone, Copy)]
struct DraftStart;

#[derive(Event)]
struct FightStart;

#[derive(Event)]
struct FightPreload;

#[derive(Component)]
struct DraftCardIndex(usize);

#[derive(Component, Clone)]
struct MinionType {
    class: Class,
    penalties: Vec<Penalty>,
    types: Vec<Type>,
    max_countdown: usize,
    start_countdown: usize,
    max_health: usize,
}

impl MinionType {
    fn get_minion(&self) -> Minion {
        Minion {
            countdown: self.start_countdown,
            health: self.max_health,
        }
    }
}

#[derive(Component)]
struct Minion {
    countdown: usize,
    health: usize,
}

#[derive(Clone)]
enum Penalty {
    IncreasedHealth,
    IncreasedDamage,
    IncreasedAmount,
    ReducedCountdown,
}

#[derive(Clone)]
enum Class {
    Arcanist,
    Cleric,
    Warrior,
    Scout,
    Pyromancer,
    Priest,
    Rogue,
    Tactician,
    Necromancer,
    Bulwark,
    Oracle,
    Sage,
    Archmage,
    Pope,
    Invincible,
    General,
}

impl Class {
    fn get_type(&self) -> MinionType {
        match self {
            &Arcanist => { MinionType {
                class: Arcanist,
                penalties: vec![IncreasedDamage],
                types: vec![Caster],
                max_countdown: 1,
                start_countdown: 3,
                max_health: 3,
            } }
            &Cleric => { MinionType {
                class: Cleric,
                penalties: vec![IncreasedHealth],
                types: vec![Divine],
                max_countdown: 3,
                start_countdown: 3,
                max_health: 8,
            } }
            &Warrior => { MinionType {
                class: Warrior,
                penalties: vec![IncreasedAmount],
                types: vec![Martial],
                max_countdown: 2,
                start_countdown: 1,
                max_health: 10,
            } }
            &Scout => { MinionType {
                class: Scout,
                penalties: vec![ReducedCountdown],
                types: vec![Envoy],
                max_countdown: 1,
                start_countdown: 1,
                max_health: 5,
            } }
            &Pyromancer => { MinionType {
                class: Pyromancer,
                penalties: vec![IncreasedAmount],
                types: vec![Caster],
                max_countdown: 3,
                start_countdown: 3,
                max_health: 5,
            } }
            &Priest => { MinionType {
                class: Priest,
                penalties: vec![IncreasedHealth],
                types: vec![Divine],
                max_countdown: 1,
                start_countdown: 3,
                max_health: 6,
            } }
            &Rogue => { MinionType {
                class: Rogue,
                penalties: vec![IncreasedDamage],
                types: vec![Martial],
                max_countdown: 1,
                start_countdown: 1,
                max_health: 4,
            } }
            &Tactician => { MinionType {
                class: Tactician,
                penalties: vec![ReducedCountdown],
                types: vec![Envoy],
                max_countdown: 4,
                start_countdown: 3,
                max_health: 8,
            } }
            &Necromancer => { MinionType {
                class: Necromancer,
                penalties: vec![IncreasedHealth],
                types: vec![Caster, Divine],
                max_countdown: 5,
                start_countdown: 5,
                max_health: 6,
            } }
            &Oracle => { MinionType {
                class: Oracle,
                penalties: vec![IncreasedAmount],
                types: vec![Divine, Envoy],
                max_countdown: 2,
                start_countdown: 3,
                max_health: 7,
            } }
            &Bulwark => { MinionType {
                class: Bulwark,
                penalties: vec![ReducedCountdown],
                types: vec![Martial, Envoy],
                max_countdown: 2,
                start_countdown: 3,
                max_health: 12,
            } }
            &Sage => { MinionType {
                class: Sage,
                penalties: vec![IncreasedDamage],
                types: vec![Caster, Martial],
                max_countdown: 3,
                start_countdown: 4,
                max_health: 6,
            } }
            &Archmage => { MinionType {
                class: Archmage,
                penalties: vec![IncreasedDamage, ReducedCountdown],
                types: vec![Caster],
                max_countdown: 5,
                start_countdown: 1,
                max_health: 15,
            } }
            &Pope => { MinionType {
                class: Pope,
                penalties: vec![IncreasedAmount, IncreasedHealth],
                types: vec![Divine],
                max_countdown: 3,
                start_countdown: 2,
                max_health: 18,
            } }
            &Invincible => { MinionType {
                class: Invincible,
                penalties: vec![ReducedCountdown, IncreasedDamage],
                types: vec![Martial],
                max_countdown: 7,
                start_countdown: 7,
                max_health: 20,
            } }
            &General => { MinionType {
                class: General,
                penalties: vec![IncreasedHealth, IncreasedAmount],
                types: vec![Envoy],
                max_countdown: 4,
                start_countdown: 4,
                max_health: 19,
            } }
            _ => { panic!() }
        }
    }

    fn get_sprite(&self) -> &'static str {
        match self {
            Arcanist => "hood.png",
            Cleric => "hedjet-white-crown.png",
            Warrior => "light-helm.png",
            Scout => "robin-hood-hat.png",
            Pyromancer => "pyromaniac.png",
            Priest => "spiked-halo.png",
            Rogue => "cowled.png",
            Tactician => "warlord-helmet.png",
            Necromancer => "crowned-skull.png",
            Oracle => "alien-stare.png",
            Bulwark => "frog-mouth-helm.png",
            Sage => "graduate-cap.png",
            Archmage => "crown.png",
            Pope => "pope-crown.png",
            Invincible => "black-knight-helm.png",
            General => "elf-helmet.png",
            _ => unreachable!()
        }
    }
}

#[derive(Clone)]
enum Type {
    Martial,
    Caster,
    Divine,
    Envoy
}

#[derive(Component)]
struct Army {
    health: usize,
    max_health: usize,
    damage: usize,
    countdown: usize,
    max_countdown: usize,
}

#[derive(Resource)]
struct FightTimer(Timer);

#[derive(Component)]
struct HealthBar();

#[derive(Resource, Default, Deref, DerefMut)]
struct Party(Vec<(MinionType, usize)>);

#[derive(Resource)]
struct DraftState {
    power_left: usize,
}

#[derive(Resource)]
struct FightState;

#[derive(Resource)]
struct DraftSettings {
    power: usize,
}

#[derive(Resource)]
struct DiscardState;

#[derive(Event)]
struct DiscardStart;

fn setup(
    mut commands: Commands,
    mut draft_start: EventWriter<DraftStart>,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical(100.0);
    commands.spawn(camera_bundle);

    draft_start.send(DraftStart);
}

fn set_health_bars(
    minions: Query<(Entity, &MinionType, &Minion), Changed<Minion>>, 
    armies: Query<(Entity, &Army), Changed<Army>>,
    healthbars: Query<&Children>, 
    mut transforms: Query<&mut Transform, With<HealthBar>>,
) {
    for (entity, t, minion) in minions.iter() {
        for child in healthbars.iter_descendants(entity) {
            let Ok(mut transform) = transforms.get_mut(child) else { continue; };
            transform.scale.x = 500.0 * minion.health as f32 / t.max_health as f32;
        }
    }

    for (entity, army) in armies.iter() {
        for child in healthbars.iter_descendants(entity) {
            let Ok(mut transform) = transforms.get_mut(child) else { continue; };
            transform.scale.x = 500.0 * army.health as f32 / army.max_health as f32;
        }
    }
}

fn draft_start(
    mut commands: Commands,
    mut ev: EventReader<DraftStart>,
    mut rand: ResMut<Random>,
    asset_server: Res<AssetServer>,
    draft_settings: Res<DraftSettings>,
) {
    if ev.is_empty() { return; }
    ev.clear();

    spawn_draft_cards(&mut commands, draft_settings.power.min(16), &mut rand.0, &asset_server);

    commands.insert_resource(DraftState{ power_left: draft_settings.power });
}

fn draft_tick(
    mut commands: Commands,
    state: Option<ResMut<DraftState>>,
    mut mouse_input: EventReader<MouseButtonInput>,
    mut party: ResMut<Party>,
    minions: Query<(Entity, &MinionType, &DraftCardIndex)>,
    windows: Query<&Window>,
    mut rand: ResMut<Random>,
    asset_server: Res<AssetServer>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut discard_start: EventWriter<DiscardStart>,
) {
    let Some(mut state) = state else { return; };

    for event in mouse_input.read() {
        if event.button != MouseButton::Left || event.state != ButtonState::Pressed {
            continue;
        }
        let window = windows.get(event.window).unwrap();
        let pos = window.cursor_position().unwrap_or_default();
        let (camera, camera_transform) = camera.single();
        let x = camera.viewport_to_world_2d(camera_transform, pos).unwrap().x;
        let mut entities: Vec<_> = minions.iter().collect();
        assert_eq!(entities.len(), 3);
        entities.sort_by_key(|m| m.2.0);
        let index = if x < -20.0 {
            0
        } else if x < 20.0 {
            1
        } else {
            2
        };
        party.push((entities[index].1.clone(), 0));
        for (entity, _, _) in minions.iter() {
            commands.entity(entity).despawn_recursive();
        }
        state.power_left -= state.power_left.min(16);

        if state.power_left == 0 && party.len() < 3 {
            state.power_left = 1;
        } else if state.power_left == 0 {
            discard_start.send(DiscardStart);
            commands.remove_resource::<DraftState>();
            return;
        }
        spawn_draft_cards(&mut commands, state.power_left.min(16), &mut rand.0, &asset_server);
    }
}

fn spawn_draft_cards(
    commands: &mut Commands,
    power: usize,
    rand: &mut StdRng,
    asset_server: &AssetServer,
) {
    let spacing = 40.0;

    let count = 3;
    for i in 0..3 {
        let class = generate_class(power, rand);
        let image = asset_server.load(class.get_sprite());
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new((spacing * (count - 1) as f32 * 0.5) + (i as f32 * -spacing), 0.0, 0.0),
                    scale: Vec3::new(0.05, 0.05, 0.0),
                    rotation: default(),
                },
                texture: image,
                ..default()
            },
            class.get_type(),
            DraftCardIndex(i),
        ));
    }
}

fn discard_start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    party: Res<Party>,
    mut ev: EventReader<DiscardStart>,
) {
    if ev.is_empty() { return; }
    ev.clear();

    for (minion, _bounty) in party.iter() {
        spawn_minion(&mut commands, &asset_server, minion);
    }

    commands.insert_resource(DiscardState);
}

fn discard_tick(
    mut commands: Commands,
    state: Option<Res<DiscardState>>,
    mut fight_start: EventWriter<FightPreload>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut party: ResMut<Party>,
    minions: Query<Entity, With<MinionType>>,
    mut redraw: EventWriter<FightStart>,
) {
    if state.is_none() { return; }

    redraw.send(FightStart);

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        party.remove(0);
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        party.remove(1);
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        party.remove(2);
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        party.remove(3);
    }
    if keyboard_input.just_pressed(KeyCode::Digit5) {
        party.remove(4);
    }
    if keyboard_input.just_pressed(KeyCode::Digit6) {
        party.remove(5);
    }
    if keyboard_input.just_pressed(KeyCode::Digit7) {
        party.remove(6);
    }
    if keyboard_input.just_pressed(KeyCode::Digit8) {
        party.remove(7);
    }
    if keyboard_input.just_pressed(KeyCode::Digit9) {
        party.remove(8);
    }
    if keyboard_input.just_pressed(KeyCode::Digit0) {
        party.remove(9);
    }

    if keyboard_input.just_pressed(KeyCode::Enter) {
        for entity in minions.iter() {
            commands.entity(entity).despawn_recursive();
        }
        fight_start.send(FightPreload);
        commands.remove_resource::<DiscardState>();
    }
}

fn fight_start_pre(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev: EventReader<FightPreload>,
    mut evw: EventWriter<FightStart>,
    draft_settings: Res<DraftSettings>,
    party: Res<Party>,
) {
    if ev.is_empty() { return; }
    ev.clear();
    let difficulty = draft_settings.power;
    let mut health = difficulty;
    let mut amount = difficulty / 5;
    let mut countdown = difficulty / 5;
    let mut damage = difficulty / 2;

    for (minion, bounty) in party.iter() {
        spawn_minion(&mut commands, &asset_server, minion);
        if difficulty % 5 == 0 {
            for penalty in &minion.penalties {
                match penalty {
                    IncreasedAmount => amount += bounty / 2,
                    IncreasedDamage => damage += bounty,
                    IncreasedHealth => health += bounty * 2,
                    ReducedCountdown => countdown += bounty / 2,
                }
            }
        }
    }

    for i in 0..(amount+1) {
        spawn_army(&mut commands, &asset_server, health, damage, countdown);
    }

    commands.insert_resource(FightState);

    evw.send(FightStart);
}

fn spawn_minion(commands: &mut Commands, asset_server: &AssetServer, minion: &MinionType) {
    let texture = asset_server.load(minion.class.get_sprite());
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(0.01, 0.01, 0.0),
                ..default()
            },
            texture,
            sprite: Sprite {
                color: Color::rgb(0.8, 0.24, 0.52),
                ..default()
            },
            ..default()
        },
        minion.clone(),
        minion.get_minion(),
    )).with_children(|parent| {
        parent.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, -4.0 / 0.01, 0.0),
                    scale: Vec3::new(500.0, 100.0, 0.0),
                    rotation: default(),
                },
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..default()
                },
                ..Default::default()
            },
            HealthBar(),
        ));
    });
}

fn fight_start(
    mut commands: Commands,
    mut minions: Query<(&mut Transform, &MinionType, &mut Minion), Without<Army>>, 
    mut armies: Query<&mut Transform, With<Army>>, 
    mut ev: EventReader<FightStart>,
) {
    if ev.is_empty() { return; }
    ev.clear();

    let m_iterator = minions.iter_mut();
    let m_count = m_iterator.len();
    let m_spacing = if m_count > 1 {
        (80.0 / (m_count - 1) as f32).min(10.0)
    } else {
        0.0
    };

    for (i, (mut p, t, mut m)) in m_iterator.enumerate() {
        if m_count == 1 {
            p.translation.x = 0.0;
        } else {
            p.translation.x = (m_spacing * (m_count - 1) as f32 * 0.5) + (i as f32 * -m_spacing);
        }
        p.translation.y = -40.0;
    }

    let a_iterator = armies.iter_mut();
    let a_count = a_iterator.len();
    let mut a_spacing = 0.0;
    if a_count > 1 {
        a_spacing = (80.0 / (a_count - 1) as f32).min(10.0);
    }

    for (i, mut p) in a_iterator.enumerate() {
        if a_count == 1 {
            p.translation.x = 0.0;
        } else {
            p.translation.x = (a_spacing * (a_count - 1) as f32 * 0.5) + (i as f32 * -a_spacing);
        }
        p.translation.y = 40.0;
    }
}

fn fight_tick(
    mut commands: Commands,
    mut minions: Query<(Entity, &MinionType, &mut Minion), Without<Army>>,
    mut armies: Query<(Entity, &mut Army)>,
    mut timer: ResMut<FightTimer>,
    mut party: ResMut<Party>,
    time: Res<Time>,
    mut rand: ResMut<Random>,
    state: Option<Res<FightState>>,
    mut draft_start: EventWriter<DraftStart>,
    mut redraw: EventWriter<FightStart>,
    mut draft_settings: ResMut<DraftSettings>,
    asset_server: Res<AssetServer>,
) {
    if state.is_none() {
        return
    }

    if !timer.0.tick(time.delta()).just_finished() { return; }

    redraw.send(FightStart);

    let a_iterator = armies.iter_mut();
    let mut a_count: usize = a_iterator.len();
    if a_count == 0 {
        draft_settings.power += 1;
        draft_start.send(DraftStart);
        commands.remove_resource::<FightState>();
        for (entity, _, _) in minions.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for (_, bounty) in party.iter_mut() {
            *bounty += 1;
        }
        return;
    }

    let mut army_attacks: Vec<usize> = vec!();
    let mut minion_heals: Vec<usize> = vec!();
    let mut minion_aoe_heals: Vec<usize> = vec!();
    let mut minion_aoe_overheals: Vec<usize> = vec!();
    let mut minion_reductions: Vec<usize> = vec!();
    let mut minion_aoe_reductions: Vec<usize> = vec!();
    let mut army_entities = vec![];
    for (e, mut a) in a_iterator {
        a.countdown -= 1;
        if a.countdown == 0 {
            a.countdown = a.max_countdown;
            army_attacks.push(a.damage);
        }
        army_entities.push(e);
    }

    let mut minion_entities = vec!();

    let m_iterator = minions.iter_mut();
    let mut m_count = m_iterator.len();
    if m_count == 0 { return; }

    for (e, m, mut o) in m_iterator {
        minion_entities.push(e);
        if a_count == 0 {
            return;
        }
        if o.health == 0 {
            continue;
        }
        if o.countdown > 0 {
            o.countdown -= 1;
        }
        if o.countdown > 0 {
            continue;
        }
        o.countdown = m.max_countdown;
        match m.class {
            Arcanist => {
                let target = army_entities[rand.gen_range(0..a_count)];
                let mut army = armies.get_mut(target).unwrap();
                if army.1.health <= 3 {
                    commands.entity(target).despawn_recursive();
                    a_count -= 1;
                } else {
                    army.1.health -= 3;
                }
            }
            Cleric => {
                minion_heals.push(2);
            }
            Warrior => {
                let target = army_entities[rand.gen_range(0..a_count)];
                let mut army = armies.get_mut(target).unwrap();
                if army.1.health <= 2 {
                    commands.entity(target).despawn_recursive();
                    a_count -= 1;
                } else {
                    army.1.health -= 2;
                }
            }
            Scout => {
                minion_reductions.push(1);
            }
            Pyromancer => {
                for mut army in armies.iter_mut() {
                    if army.1.health <= 2 {
                        commands.entity(army.0).despawn_recursive();
                        a_count -= 1;
                    } else {
                        army.1.health -= 2;
                    }
                }
            }
            Priest => {
                minion_heals.push(1);
            }
            Rogue => {
                let target = army_entities[rand.gen_range(0..a_count)];
                let mut army = armies.get_mut(target).unwrap();
                if army.1.health <= 5 {
                    commands.entity(target).despawn_recursive();
                    a_count -= 1;
                } else {
                    army.1.health -= 5;
                }
            }
            Tactician => {
                minion_reductions.push(3);
            }
            Necromancer => {
                spawn_minion(&mut commands, &asset_server, &Warrior.get_type());
            }
            Oracle => {
                minion_heals.push(1);
            }
            Bulwark => {
                minion_aoe_overheals.push(1);   
            }
            Sage => {
                let target = army_entities[rand.gen_range(0..a_count)];
                let mut army = armies.get_mut(target).unwrap();
                army.1.countdown += 3;
            }
            Archmage => {
                for mut army in armies.iter_mut() {
                    if army.1.health <= 1 {
                        commands.entity(army.0).despawn_recursive();
                        a_count -= 1;
                    } else {
                        army.1.health -= 1;
                    }
                }
            }
            Pope => {
                minion_aoe_heals.push(3);
            }
            Invincible => {
                let target = army_entities[rand.gen_range(0..a_count)];
                let mut army = armies.get_mut(target).unwrap();
                commands.entity(target).despawn_recursive();
            }
            General => {
                minion_aoe_reductions.push(1);
            }
            _ => unimplemented!()
        }
    }

    for heal in minion_heals {
        let target = rand.gen_range(0..m_count);
        let mut minion = minions.get_mut(minion_entities[target]).unwrap();
        if minion.2.health > minion.1.max_health {
            continue;
        } else if minion.1.max_health - minion.2.health < heal {
            minion.2.health = minion.1.max_health;
        } else {
            minion.2.health += heal;
        }
    }

    for heal in minion_aoe_heals {
        for mut minion in minions.iter_mut() {
            if minion.2.health > minion.1.max_health {
                continue;
            } else if minion.2.health - minion.1.max_health < heal {
                minion.2.health = minion.1.max_health;
            } else {
                minion.2.health += heal;
            }
        }
    }
    
    for heal in minion_aoe_overheals {
        for mut minion in minions.iter_mut() {
            minion.2.health += heal;
        }
    }
    
    for reduction in minion_reductions {
        let target = rand.gen_range(0..m_count);
        let mut minion = minions.get_mut(minion_entities[target]).unwrap();
        if minion.2.countdown < reduction { continue }
        minion.2.countdown -= reduction;
    }

    for reduction in minion_aoe_reductions {
        for mut minion in minions.iter_mut() {
            if minion.2.countdown < reduction { continue }
            minion.2.countdown -= reduction;
        }
    }

    for attack in army_attacks {
        if m_count == 0 {
            println!("you lost!");
            exit(0);
        }
        let target = rand.gen_range(0..m_count);
        let mut minion = minions.get_mut(minion_entities[target]).unwrap();
        if minion.2.health <= attack {
            minion_entities.remove(target);
            commands.entity(minion.0).despawn_recursive();
            m_count -= 1;
        } else {
            minion.2.health -= attack;
        }
    }
    println!("{0:?}, {1:?}", m_count, minion_entities.len());

}

fn spawn_army(commands: &mut Commands, asset_server: &AssetServer, health: usize, damage: usize, countdown: usize) {
    let warlord_helmet_image = asset_server.load("warlord-helmet.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(0.01, 0.01, 0.0),
                ..default()
            },
            texture: warlord_helmet_image,
            sprite: Sprite {
                color: Color::rgb(0.8, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        Army {
            max_health: 10+health,
            health: 10+health,
            damage: 1+damage,
            countdown: (5-countdown).max(1),
            max_countdown: (5-countdown).max(1),
        }
    )).with_children(|parent| {
        parent.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, -4.0 / 0.01, 0.0),
                    scale: Vec3::new(500.0, 100.0, 0.0),
                    rotation: default(),
                },
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    ..default()
                },
                ..Default::default()
            },
            HealthBar(),
        ));
    });

}

fn generate_class(power: usize, rng: &mut StdRng) -> Class {
    match rng.gen_range(0..power) {
        0..=4 => { match rng.gen_range(0..4) {
            0 => return Arcanist,
            1 => return Cleric,
            2 => return Warrior,
            3 => return Scout,
            _ => unreachable!()
        } },
        5..=8 => { match rng.gen_range(0..4) {
            0 => return Pyromancer,
            1 => return Priest,
            2 => return Rogue,
            3 => return Tactician,
            _ => unreachable!()
        } },
        9..=12 => { match rng.gen_range(0..4) {
            0 => return Necromancer,
            1 => return Oracle,
            2 => return Bulwark,
            3 => return Sage,
            _ => unreachable!()
        } },
        _ => { match rng.gen_range(0..4) {
            0 => return Archmage,
            1 => return Pope,
            2 => return Invincible,
            3 => return General,
            _ => unreachable!()
        } }
    }
}
