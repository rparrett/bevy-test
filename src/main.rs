use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::CursorMoved,
    sprite::collide_aabb::{collide, Collision},
};

struct Cursor;
struct Target;
enum TargetHighlight {
    Left,
    Right,
    Top,
    Bottom,
}

fn startup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        ..Default::default()
    }).insert(Cursor);

    let target_size = Vec2::new(100.0, 100.0);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
        sprite: Sprite::new(target_size.clone()),
        ..Default::default()
    }).insert(Target);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        sprite: Sprite::new(Vec2::new(target_size.x / 2.0, target_size.y)),
        transform: Transform::from_translation(Vec3::new(target_size.x / -4.0, 0.0, 0.5)),
        visible: Visible { is_visible: false, ..Default::default() },
        ..Default::default()
    }).insert(TargetHighlight::Left);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        sprite: Sprite::new(Vec2::new(target_size.x / 2.0, target_size.y)),
        transform: Transform::from_translation(Vec3::new(target_size.x / 4.0, 0.0, 0.5)),
        visible: Visible { is_visible: false, ..Default::default() },
        ..Default::default()
    }).insert(TargetHighlight::Right);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        sprite: Sprite::new(Vec2::new(target_size.x, target_size.y / 2.0)),
        transform: Transform::from_translation(Vec3::new(0.0, target_size.y / -4.0, 0.5)),
        visible: Visible { is_visible: false, ..Default::default() },
        ..Default::default()
    }).insert(TargetHighlight::Bottom);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        sprite: Sprite::new(Vec2::new(target_size.x, target_size.y / 2.0)),
        transform: Transform::from_translation(Vec3::new(0.0, target_size.y / 4.0, 0.5)),
        visible: Visible { is_visible: false, ..Default::default() },
        ..Default::default()
    }).insert(TargetHighlight::Top);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup.system())
        .add_system(mouse.system())
        .run();
}

/// This system prints out all mouse events as they come in
fn mouse(
    mut cursor_moved_events: EventReader<CursorMoved>,
    windows: Res<Windows>,
    query: Query<(&Transform, &Sprite), (With<Target>, Without<Cursor>)>,
    mut cursor_query: Query<(&mut Transform, &Sprite), With<Cursor>>,
    mut highlight_query: Query<(&TargetHighlight, &mut Visible)>,
) {
    for event in cursor_moved_events.iter() {
        let window = windows.get(event.id).unwrap();
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let pos = event.position - window_size / 2.0;

        for (mut transform, sprite) in cursor_query.iter_mut() {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;

            for (mut target_transform, target_sprite) in query.iter() {
                let collision = collide(transform.translation, sprite.size, target_transform.translation, target_sprite.size);
                
                for (_, mut visible) in highlight_query.iter_mut() {
                    visible.is_visible = false;
                }
                
                for (highlight, mut visible) in highlight_query.iter_mut() {
                    match (&collision, highlight) {
                        (Some(Collision::Left), TargetHighlight::Left) => visible.is_visible = true,
                        (Some(Collision::Right), TargetHighlight::Right) => visible.is_visible = true,
                        (Some(Collision::Bottom), TargetHighlight::Bottom) => visible.is_visible = true,
                        (Some(Collision::Top), TargetHighlight::Top) => visible.is_visible = true,
                        (Some(Collision::Intersecting), _) => visible.is_visible = true,
                        _ => {}
                    }
                }
                info!("{:?}", collision);
            }
        }
    }
}
