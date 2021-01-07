use bevy::{prelude::*, text::CalculatedSize};
use std::ops::Deref;

struct ThingTimer(Timer);
#[derive(Default)]
struct State {
    score: u32,
}

fn startup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        // UI camera
        .spawn(CameraUiBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Auto),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.5).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    margin: Rect {
                        left: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    value: "0".to_string(),
                    font: font.clone(),
                    style: TextStyle {
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        });
}

/*
fn test(
    mut query: Query<&mut Text>, state: Res<State>
) {
    if let Some(mut text) = query.iter_mut().next() {
        text.value = format!("Score: {}", state.score);
    }
}*/

fn test(mut queries: QuerySet<(Query<&Text>, Query<&mut Text>)>, state: Res<State>) {
    if let Some(text) = queries.q0().iter().next() {
        let score_text = format!("Score: {}", state.score);

        if (text.value != score_text) {
            if let Some(mut text) = queries.q1_mut().iter_mut().next() {
                text.value = score_text;
            }
        }
    }
}

fn timed_thing(mut timer: ResMut<ThingTimer>, mut state: ResMut<State>, mut time: Res<Time>) {
    if timer.0.tick(time.delta_seconds()).just_finished() {
        info!("tick");
        state.score += 1;
    }
}

fn recalced(query: Query<&CalculatedSize, Changed<CalculatedSize>>) {
    for (i, size) in query.iter().enumerate() {
        info!("recalced {} {} {}", i, size.size.width, size.size.height);
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<State>()
        .add_resource(ThingTimer(Timer::from_seconds(1.0, true)))
        .add_startup_system(startup.system())
        .add_system(test.system())
        .add_system(timed_thing.system())
        .add_stage_after(
            stage::POST_UPDATE,
            "post_post_update",
            SystemStage::parallel(),
        )
        .add_system_to_stage("post_post_update", recalced.system())
        .run();
}
