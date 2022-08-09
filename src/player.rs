use bevy::{ecs::event::ManualEventReader, input::mouse::MouseMotion, prelude::*};

pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00015,
            speed: 2.0,
        }
    }
}

#[derive(Default)]
struct InputState {
    pitch: f32,
    yaw: f32,
    reader_motion: ManualEventReader<MouseMotion>,
}

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerCamera;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(setup_player)
            .add_startup_system(startup_grab_cursor)
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab);
    }
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PlayerCamera);
}

fn startup_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        error!("Primary window not found for `startup_grap_cursor`");
    }
}

fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

fn player_move(
    keyboard_input: Res<Input<KeyCode>>,
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform, With<PlayerCamera>>,
) {
    if let Some(window) = windows.get_primary() {
        if !window.cursor_locked() {
            return;
        }

        if let Some(mut transform) = transforms.iter_mut().next() {
            let mut movement = Vec3::ZERO;

            let local_x = transform.local_x();
            let local_z = -transform.local_z();
            let forward = Vec3::new(local_z.x, 0.0, local_z.z);
            let right = Vec3::new(local_x.x, 0.0, local_x.z);
            let up = Vec3::Y;

            for key in keyboard_input.get_pressed() {
                match key {
                    KeyCode::A => movement -= right,
                    KeyCode::D => movement += right,
                    KeyCode::S => movement -= forward,
                    KeyCode::W => movement += forward,
                    KeyCode::LShift => movement -= up,
                    KeyCode::Space => movement += up,
                    _ => (),
                }
            }

            movement = movement.normalize_or_zero();
            movement *= settings.speed * time.delta_seconds();

            transform.translation += movement;
        }
    }
}

fn player_look(
    windows: Res<Windows>,
    settings: Res<MovementSettings>,
    motion: Res<Events<MouseMotion>>,
    mut state: ResMut<InputState>,
    mut transforms: Query<&mut Transform, With<PlayerCamera>>,
) {
    if let Some(window) = windows.get_primary() {
        if !window.cursor_locked() {
            return;
        }

        let mut delta_state = state.as_mut();
        if let Some(mut transform) = transforms.iter_mut().next() {
            for ev in delta_state.reader_motion.iter(&motion) {
                let window_scale = window.height().min(window.width());
                delta_state.pitch -=
                    (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                delta_state.yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();

                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
    } else {
        error!("Failed to get primary window at `player_look`")
    }
}

fn cursor_grab(keyboard_input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Some(window) = windows.get_primary_mut() {
            toggle_grab_cursor(window)
        } else {
            error!("Failed to get primary window `cursor_grab`");
        }
    }
}
