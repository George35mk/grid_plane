use bevy::prelude::*;
use grid_plane::GridPlanePlugin;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins)
    .add_plugin(GridPlanePlugin::default())
    .add_startup_system(setup)
    .add_system(camera_controls)
    .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // add the cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.0).into()),
        material: materials.add(Color::SEA_GREEN.into()),
        ..default()
    });

    // add the camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 8., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

}


pub fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();


    let speed = 5.0;
    let rotate_speed = 0.3;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    }
    if keyboard.pressed(KeyCode::Up) {
        camera.translation.y += camera.translation.y * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::Down) {
        camera.translation.y -= camera.translation.y * time.delta_seconds() * speed;
    }
}