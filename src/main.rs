use std::ops::DerefMut;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::Action::*;
use crate::Penalty::*;
use crate::Type::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(pluginsWorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_event::<RoundStart>()
        .run();
}

#[derive(Event)]
struct RoundStart;

struct Party {
    members: Vec<MinionType>,
}

#[derive(Component)]
struct MinionType {
    action: Action,
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

enum Penalty {
    IncreasedHealth,
}

enum Action {
    Melee,
}

enum Type {
    Warrior,
    Caster,
}

struct Army {
    health: usize,
    damage: usize,
    countdown: usize,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut round_start: EventWriter<RoundStart>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection::default(),
        ..Default::default()
    });

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(120.0, 1000.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.7),
                ..default()
            },
            ..default()
        },
        MinionType {
            action: Action::Melee,
            start_countdown: 1,
            max_countdown: 2,
            max_health: 10,
            penalties: vec![IncreasedHealth],
            types: vec![Warrior],
        },
        Minion {
            countdown: 1,
            health: 10,
        },
    ));

    round_start.send(RoundStart);
}

fn round_start(mut window: Query<(&Window)>, mut query: Query<(&mut Transform, &mut MinionType, &mut Minion)>, mut ev: EventReader<RoundStart>) {
    if ev.is_empty() { return; }
    ev.clear();

    for (mut p, mut t, mut m) in query.iter_mut() {
        let window = window.get_single().expect().resolution.height;
        *m = t.get_minion();
    }
}
