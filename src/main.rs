use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{self, output::OutputEvent},
    EguiContext, EguiPlugin,
};
use bevy_tts::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Simple Bevy egui Screen Reader".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(TtsPlugin)
        .add_system(start_menu.system())
        .add_system_to_stage(CoreStage::PostUpdate, screen_reader.system())
        .run();
}

fn start_menu(context: Res<EguiContext>, mut exit: EventWriter<AppExit>, mut ran: Local<bool>) {
    context.ctx().memory().options.screen_reader = true;
    egui::CentralPanel::default().show(context.ctx(), |ui| {
        let start = ui.button("Start");
        if start.clicked() {
            println!("Start clicked");
        }
        if ui.button("Quit").clicked() {
            exit.send(AppExit);
        }
        if !*ran {
            start.request_focus();
            *ran = true;
        }
    });
}

fn screen_reader(context: Res<EguiContext>, mut tts: ResMut<Tts>) {
    let events = &context.ctx().output().events;
    for event in events {
        match event {
            OutputEvent::FocusGained(widget_info) => {
                tts.speak(widget_info.description(), true).unwrap();
            }
        };
    }
}
