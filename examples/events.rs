use std::time::Duration;

use bevy::prelude::*;
use bevy_webview::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WebviewPlugin::with_engine(webview_engine::headless))
        .add_webview_input_event::<LoginRequest>("login")
        .add_webview_input_event::<CloseRequest>("close")
        .add_webview_output_event::<AppTime>("app_time")
        .add_startup_system(setup)
        .add_system(login_handler)
        .add_system(send_time_to_all_webviews_system)
        .add_system(close_handler)
        .run();
}

#[derive(Component)]
struct TimeReceiver;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(WebviewUIBundle {
            webview: Webview {
                html: Some(include_str!("events.html").into()),
                color: Color::rgb_u8(58, 58, 58),
                ..Default::default()
            },
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(50.)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TimeReceiver);

    commands.insert_resource(TimeTick(Timer::new(Duration::from_millis(1_000), TimerMode::Repeating)));
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
}

impl Resource for LoginRequest {}

#[derive(Serialize, Debug)]
pub struct AppTime {
    seconds_since_startup: f64,
}
impl Resource for AppTime {}

fn login_handler(mut login_request_events: WebviewEventReader<LoginRequest>) {
    // iterate `T`event types
    for event in login_request_events.iter() {
        println!("Login request for username: {:?}", event.username);
    }

    // iterate with entities
    for (event, entity) in login_request_events.iter_with_entity() {
        println!(
            "Login request for username: {:?} from webview {:?}",
            event.username, entity
        );
    }
}

#[derive(Deserialize, Debug)]
pub struct CloseRequest;

impl Resource for CloseRequest {}
fn close_handler(
    mut close_request_events: WebviewEventReader<CloseRequest>,
    mut commands: Commands,
) {
    for (_close_request, entity) in close_request_events.iter_with_entity() {
        // handle the event programmatically (e.g. state change, or other logic)
        // and finally close the webview
        println!("`CloseRequest` called");

        commands.entity(entity).despawn_recursive();
    }
}

struct TimeTick(Timer);
impl Resource for TimeTick {}

fn send_time_to_all_webviews_system(
    mut app_time: WebviewEventWriter<AppTime>,
    time: Res<Time>,
    mut tick: ResMut<TimeTick>,
) {
    if tick.0.tick(time.delta()).just_finished() {
        app_time.send(AppTime {
            seconds_since_startup: time.elapsed_seconds_f64(),
        });
    }
}
