use bevy::{prelude::*, window::PresentMode};
use grid_plane::{GridPlanePlugin, GridAxis};
mod utils;
use utils::camera_controls;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy grid plane example".into(),
                resolution: (800., 600.).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
            
        })
        .set(ImagePlugin::default_nearest())
    )

    // Use this code if you want to change some of the options.
    .add_plugin(GridPlanePlugin {
        grid_axis: GridAxis::Zx,
        ..default()
    })
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
