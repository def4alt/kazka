use bevy::prelude::*;

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

pub struct PlayerPlugin;

#[derive(Component)]
pub struct PlayerCamera;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementSettings>()
            .add_startup_system(setup_player)
            .add_startup_system(startup_grab_cursor)
            .add_system(player_move)
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
        if let Some(mut transform) = transforms.iter_mut().next() {
            let mut movement = Vec3::ZERO;

            for key in keyboard_input.get_pressed() {
                if window.cursor_locked() {
                    match key {
                        KeyCode::A => movement.x -= 1.0,
                        KeyCode::D => movement.x += 1.0,
                        KeyCode::S => movement.y -= 1.0,
                        KeyCode::W => movement.y += 1.0,
                        _ => (),
                    }
                }
            }

            movement = movement.normalize_or_zero();
            movement *= settings.speed * time.delta().as_secs_f32();

            let local_y = transform.local_y();
            let local_x = transform.local_x();
            transform.translation += local_x * movement.x + local_y * movement.y;
        }
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
