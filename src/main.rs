use bevy::{prelude::*, text::CalculatedSize};
use rand::{thread_rng, Rng};

pub static FONT_SIZE_LABEL: f32 = 24.0;

#[derive(Default)]
struct FontHandles {
    jptext: Handle<Font>,
}

struct TowerSlotLabel;
struct TowerSlotLabelMatched;
struct TowerSlotLabelUnmatched;
struct TowerSlotLabelBg;

fn randomly_change_text(
    time: Res<Time>,
    mut timer: ResMut<Timer>,
    mut matched_query: Query<&mut Text, With<TowerSlotLabelMatched>>,
    mut unmatched_query: Query<&mut Text, With<TowerSlotLabelUnmatched>>,
    full_query: Query<(&Text, &Parent), With<TowerSlotLabel>>,
    children_query: Query<&Children>,
) {
    timer.tick(time.delta_seconds());
    if !timer.finished() {
        return;
    }

    for (full, parent) in full_query.iter() {
        let mut rng = thread_rng();
        let (new_matched, new_unmatched) = full
            .value
            .split_at(rng.gen_range(1..(full.value.chars().count() - 1)));

        if let Ok(children) = children_query.get(**parent) {
            for child in children.iter() {
                if let Ok(mut matched) = matched_query.get_mut(*child) {
                    matched.value = new_matched.to_string();
                }
                if let Ok(mut unmatched) = unmatched_query.get_mut(*child) {
                    unmatched.value = new_unmatched.to_string();
                }
            }
        }
    }
}

fn update_tower_slot_labels(
    mut left_query: Query<
        (
            &mut Transform,
            &mut GlobalTransform,
            &CalculatedSize,
            &Parent,
        ),
        (With<TowerSlotLabelUnmatched>, Changed<CalculatedSize>),
    >,
    mut right_query: Query<
        (&mut Transform, &mut GlobalTransform, &CalculatedSize),
        With<TowerSlotLabelMatched>,
    >,
    full_query: Query<&CalculatedSize, With<TowerSlotLabel>>,
    mut bg_query: Query<(&mut Sprite, &GlobalTransform), With<TowerSlotLabelBg>>,
    children_query: Query<&Children>,
) {
    for (mut left_t, mut left_gt, left_size, parent) in left_query.iter_mut() {
        // can probably just add Children to bg_query and use that here.
        if let Ok(children) = children_query.get(**parent) {
            // My iterator/result-fu is not enough for this.
            let mut full_width = 0.0;
            let mut global_x = 0.0;

            for child in children.iter() {
                if let Ok(full_size) = full_query.get(*child) {
                    full_width = full_size.size.width;
                }
            }

            if let Ok((mut bg_sprite, gt)) = bg_query.get_mut(**parent) {
                bg_sprite.size.x = full_width + 8.0;
                global_x = gt.translation.x;
            }

            // Muckign around with GlobalTransform seems completely necessary to prevent weird
            // positioning judder, but it seems to mess up heirarchical positioning. So we'll
            // just grab the parent's position and do that ourselves.

            left_t.translation.x = global_x + full_width / 2.0 - left_size.size.width / 2.0;
            left_gt.translation.x = global_x + full_width / 2.0 - left_size.size.width / 2.0;

            for child in children.iter() {
                if let Ok((mut right_t, mut right_gt, right_size)) = right_query.get_mut(*child) {
                    right_t.translation.x =
                        global_x - full_width / 2.0 + right_size.size.width / 2.0;
                    right_gt.translation.x =
                        global_x - full_width / 2.0 + right_size.size.width / 2.0;
                }
            }
        }
    }
}

fn setup(
    commands: &mut Commands,
    mut font_handles: ResMut<FontHandles>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    font_handles.jptext = asset_server.load("fonts/NotoSansJP-Light.otf");

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 99.0)),
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.5).into()),
            sprite: Sprite::new(Vec2::new(108.0, FONT_SIZE_LABEL)),
            ..Default::default()
        })
        .with(TowerSlotLabelBg)
        .with_children(|parent| {
            parent
                .spawn(Text2dBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
                    text: Text {
                        value: "".to_string(),
                        font: font_handles.jptext.clone(),
                        style: TextStyle {
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            font_size: FONT_SIZE_LABEL,
                            color: Color::GREEN,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(TowerSlotLabelMatched);
            parent
                .spawn(Text2dBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
                    text: Text {
                        value: "".to_string(),
                        font: font_handles.jptext.clone(),
                        style: TextStyle {
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            font_size: FONT_SIZE_LABEL,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(TowerSlotLabelUnmatched);
            parent
                .spawn(Text2dBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 98.0)),
                    text: Text {
                        value: "Lorem ipsum dolor sit amet".to_string(),
                        font: font_handles.jptext.clone(),
                        style: TextStyle {
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            font_size: FONT_SIZE_LABEL,
                            color: Color::INDIGO,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(TowerSlotLabel);
        });
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<FontHandles>()
        .add_resource(Timer::from_seconds(0.1, true))
        .add_stage_after(
            stage::POST_UPDATE,
            "after_text_size_calculated",
            SystemStage::parallel(),
        )
        .add_startup_system(setup.system())
        .add_system(randomly_change_text.system())
        .add_system_to_stage(
            "after_text_size_calculated",
            update_tower_slot_labels.system(),
        )
        .run();
}
