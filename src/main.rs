use bevy::{prelude::*, text::CalculatedSize};

struct WordTimer(Timer);

struct Words {
    words: Vec<String>,
}

struct TextResizedEvent;

struct LeftText;
struct RightText;
struct InvisText;

#[derive(Default)]
struct State {
    word: String,
    word_index: usize,
}

fn startup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    state: Res<State>,
) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    
    let mut halves = state.word.split(",");
    let left_half = halves.next().unwrap().to_string();
    let right_half = halves.next().unwrap().to_string();

    let mut both = left_half.clone();
    both.push_str(&right_half);

    commands
        // UI camera
        .spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.7).into()),
            sprite: Sprite::new(Vec2::new(10.0, 42.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Text2dBundle {
                    text: Text {
                        value: left_half,
                        font: font.clone(),
                        style: TextStyle {
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            font_size: 32.0,
                            color: Color::GREEN,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(LeftText);
            parent
                .spawn(Text2dBundle {
                    text: Text {
                        value: right_half,
                        font: font.clone(),
                        style: TextStyle {
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(RightText);
            parent
                .spawn(Text2dBundle {
                    text: Text {
                        value: both,
                        font: font.clone(),
                        style: TextStyle {
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            font_size: 32.0,
                            color: Color::NONE,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(InvisText);
        });
}

fn test(
    mut left_queries: QuerySet<(
        Query<&Text, With<LeftText>>,
        Query<&mut Text, With<LeftText>>,
    )>,
    mut right_queries: QuerySet<(
        Query<&Text, With<RightText>>,
        Query<&mut Text, With<RightText>>,
    )>,
    mut invis_queries: QuerySet<(
        Query<&Text, With<InvisText>>,
        Query<&mut Text, With<InvisText>>,
    )>,
    state: Res<State>,
) {
    let mut halves = state.word.split(",");
    let left_half = halves.next().unwrap().to_string();
    let right_half = halves.next().unwrap().to_string();

    let mut both = left_half.clone();
    both.push_str(&right_half);

    if let Some(left) = left_queries.q0().iter().next() {
        if left.value != left_half {
            if let Some(mut left) = left_queries.q1_mut().iter_mut().next() {
                left.value = left_half.clone();
            }
        }
    }

    if let Some(right) = right_queries.q0().iter().next() {
        if right.value != right_half {
            if let Some(mut right) = right_queries.q1_mut().iter_mut().next() {
                right.value = right_half.clone();
            }
        }
    }

    if let Some(invis) = invis_queries.q0().iter().next() {
        if invis.value != both {
            if let Some(mut invis) = invis_queries.q1_mut().iter_mut().next() {
                invis.value = both.clone();
            }
        }
    }
}

fn timed_thing(
    mut timer: ResMut<WordTimer>,
    mut state: ResMut<State>,
    words: Res<Words>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta_seconds()).just_finished() {
        state.word = words.words.get(state.word_index).unwrap().to_string();

        state.word_index += 1;
        if state.word_index >= words.words.len() {
            state.word_index = 0;
        }

        info!("tick");
    }
}

fn resize_bg(
    size_query: Query<&CalculatedSize, (With<InvisText>, Changed<CalculatedSize>)>,
    mut bg_query: Query<&mut Sprite>,
    mut events: ResMut<Events<TextResizedEvent>>,
) {
    for size in size_query.iter() {
        for mut bg in bg_query.iter_mut() {
            // This seems to work just fine after POST_UPDATE, unlike modifying translations
            bg.size.x = size.size.width + 8.0;
            events.send(TextResizedEvent {});
        }
    }
}

fn reposition_text(
    size_query: Query<&CalculatedSize, With<InvisText>>,
    mut left_query: Query<(&mut Transform, &CalculatedSize), With<LeftText>>,
    mut right_query: Query<(&mut Transform, &CalculatedSize), With<RightText>>,
    events: Res<Events<TextResizedEvent>>,
    mut reader: Local<EventReader<TextResizedEvent>>,
) {
    for _ in reader.iter(&events) {
        let mut invis_width = 0.0;
        for size in size_query.iter() {
            invis_width = size.size.width;
            info!("invis_width {}", invis_width);
        }

        for (mut t, s) in left_query.iter_mut() {
            info!("left_width: {}", s.size.width);
            t.translation.x = 0.0 - invis_width / 2.0 + s.size.width / 2.0;
        }

        for (mut t, s) in right_query.iter_mut() {
            info!("right_width: {}", s.size.width);
            t.translation.x = 0.0 + invis_width / 2.0 - s.size.width / 2.0;
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<TextResizedEvent>()
        .add_resource(State {
            word: "Z,ero".to_string(),
            word_index: 0,
        })
        .add_resource(WordTimer(Timer::from_seconds(1.0, true)))
        .add_resource(Words {
            words: vec![
                "Ze,ro".to_string(),
                "On,e".to_string(),
                "T,wo".to_string(),
                "Th,ree".to_string(),
            ],
        })
        .add_startup_system(startup.system())
        .add_system(test.system())
        .add_system(timed_thing.system())
        .add_system(reposition_text.system())
        // bevy_text changes CalculatedSize in POST_UPDATe, so we need do deal with it
        // after that.
        .add_stage_after(
            stage::POST_UPDATE,
            "post_post_update",
            SystemStage::parallel(),
        )
        .add_system_to_stage("post_post_update", resize_bg.system())
        .run();
}
