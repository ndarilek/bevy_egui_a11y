use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(start_menu.system())
        .add_system(screen_reader.system())
        .run();
}

fn start_menu(context: Res<EguiContext>) {
    context.ctx().memory().options.screen_reader = true;
    egui::CentralPanel::default().show(context.ctx(), |ui| {
        let start = ui.button("Start");
        start.request_focus();
        if start.clicked() {
            println!("Start clicked");
        }
        if ui.button("Quit").clicked() {
            println!("Quit clicked");
        }
    });
}

fn screen_reader(context: Res<EguiContext>, mut last_seen_event: Local<usize>) {
    let events = &context.ctx().output().events;
    if events.len() > 0 {
        println!("{:?}", events);
    }
    let events = &events[0..events.len()];
    for event in events {
        println!("{:?}", event);
    }
    *last_seen_event = events.len();
}
