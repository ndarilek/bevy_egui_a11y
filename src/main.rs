use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self,
        output::{OutputEvent, WidgetEvent},
    },
    EguiContext, EguiPlugin,
};
use bevy_tts::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(TtsPlugin)
        .add_system(start_menu.system())
        .add_system_to_stage(CoreStage::PostUpdate, screen_reader.system())
        .run();
}

fn start_menu(context: Res<EguiContext>) {
    context.ctx().memory().options.screen_reader = true;
    egui::CentralPanel::default().show(context.ctx(), |ui| {
        let start = ui.button("Start");
        if start.clicked() {
            println!("Start clicked");
        }
        if ui.button("Quit").clicked() {
            println!("Quit clicked");
        }
        if context.ctx().memory().focus().is_none() {
            start.request_focus();
        }
    });
}

fn screen_reader(context: Res<EguiContext>, mut tts: ResMut<Tts>) {
    let events = &context.ctx().output().events;
    for event in events {
    let OutputEvent::WidgetEvent(WidgetEvent::Focus, widget_info) = event ;
            tts.speak(widget_info.description(), true).unwrap();
    }
}
