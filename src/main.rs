use bevy::prelude::*;

pub struct GamePlugin;

const MOVEMENT_SPEED: f32 = 1.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(camera_movement)
            .run();
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Kazka".to_string(),
            width: 800.0,
            height: 600.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.5, 0.3, 0.2).into()),
        ..default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.3, 0.6, 0.1).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform, With<Camera>>,
) {
    if let Some(mut transform) = transforms.iter_mut().next() {
        let mut movement = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            movement.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            movement.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            movement.y += 1.0;
        }

        movement = movement.normalize_or_zero();
        movement *= MOVEMENT_SPEED * time.delta().as_secs_f32();

        let local_y = transform.local_y();
        let local_x = transform.local_x();
        transform.translation += local_x * movement.x + local_y * movement.y;
    }
}
