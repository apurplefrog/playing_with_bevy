use bevy::{input::mouse::MouseMotion, prelude::*, window};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup /*prepare_game*/,))
        .add_systems(Update, (camera_motion, movement, jumping))
        .run();
}

#[derive(Component)]
struct Camera;

pub fn prepare_game(mut primary_window: Query<&mut Window, With<window::PrimaryWindow>>) {
    let primary_window = &mut primary_window.single_mut();
    primary_window.cursor.visible = false;
    primary_window.mode = window::WindowMode::BorderlessFullscreen;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(400.0)),
        material: materials.add(Color::srgb_u8(0, 0, 255)),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Camera,
    ));
}

fn camera_motion(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut evr_motion: EventReader<MouseMotion>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();
    for mouse_event in evr_motion.read() {
        let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
        let sensitivity = 0.1;
        pitch -= (sensitivity * mouse_event.delta.y).to_radians();
        yaw -= (sensitivity * mouse_event.delta.x).to_radians();

        pitch = pitch.clamp(-1.54, 1.54);

        camera.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

fn movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();
    let speed = 3.4;

    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        let dir = (*camera.forward()).xz();
        direction += Vec3::from([dir[0], 0.0, dir[1]]);
        direction += *camera.forward();
    }

    if keys.pressed(KeyCode::KeyS) {
        let dir = (*camera.back()).xz();
        direction += Vec3::from([dir[0], 0.0, dir[1]]);
        direction += *camera.back();
    }

    if keys.pressed(KeyCode::KeyD) {
        let dir = (*camera.right()).xz();
        direction += Vec3::from([dir[0], 0.0, dir[1]]);
        direction += *camera.right();
    }

    if keys.pressed(KeyCode::KeyA) {
        let dir = (*camera.left()).xz();
        direction += Vec3::from([dir[0], 0.0, dir[1]]);
        direction += *camera.left();
    }

    direction.y = 0.0;

    camera.translation += direction.normalize_or_zero() * speed * time.delta_seconds();
}

fn jumping(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut camera = camera_query.get_single_mut().unwrap();

    if keys.pressed(KeyCode::Space) {
        let og_y = camera.translation.y;
        let mut t: f32 = 0.0;
        let scale = 2.0;
        while og_y >= camera.translation.y {
            camera.translation.y = scale * ((-4.9) * t.powf(2.0) + 10.0 * t);
            t += time.delta_seconds();
            println!("{}", camera.translation.y);
        }
    }
}
