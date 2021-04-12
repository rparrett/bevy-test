use bevy::{prelude::*, sprite::SpriteSettings};

struct Bar;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut transform = Transform::from_xyz(-800.0, 0.0, 0.0);
    transform.scale = Vec3::new(1.0, 20.0, 1.0);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
        transform,
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        ..Default::default()
    }).insert(Bar);
}

fn rotate(mut query: Query<&mut Transform, With<Bar>>, time: Res<Time>) {
    for mut t in query.iter_mut() {
        t.rotation *= Quat::from_rotation_z(time.delta_seconds());
    }
}

fn main() {
    App::build()
        .insert_resource(SpriteSettings {
            frustum_culling_enabled: true,
        })        
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotate.system())
        .run();
}
