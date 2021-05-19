use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{self, output::OutputEvent, WidgetType},
    EguiContext, EguiPlugin,
};
use bevy_tts::*;
use difference::Changeset;

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

fn start_menu(
    context: Res<EguiContext>,
    mut tts: ResMut<Tts>,
    mut exit: EventWriter<AppExit>,
    mut ran: Local<bool>,
    mut checked: Local<bool>,
    mut username: Local<String>,
) {
    context.ctx().memory().options.screen_reader = true;
    egui::CentralPanel::default().show(context.ctx(), |ui| {
        let start = ui.button("Start");
        if start.clicked() {
            tts.speak("Start clicked", true).unwrap();
            println!("Start clicked");
        }
        ui.checkbox(&mut *checked, "Check me");
        ui.horizontal(|ui| {
            ui.label("Username");
            ui.text_edit_singleline(&mut username);
        });
        if ui.button("Quit").clicked() {
            exit.send(AppExit);
        }
        if !*ran {
            start.request_focus();
            *ran = true;
        }
    });
}

fn screen_reader(
    context: Res<EguiContext>,
    mut tts: ResMut<Tts>,
    mut cached_text_selection: Local<Option<std::ops::RangeInclusive<usize>>>,
) {
    let events = &context.ctx().output().events;
    for event in events {
        match event {
            OutputEvent::Clicked(widget_info) => {
                *cached_text_selection = None;
                if let Some(selected) = widget_info.selected {
                    if widget_info.typ == WidgetType::Checkbox {
                        if selected {
                            tts.speak("Checked", true).unwrap();
                        } else {
                            tts.speak("Unchecked", true).unwrap();
                        }
                    } else if selected {
                        tts.speak("Selected", true).unwrap();
                    } else {
                        tts.speak("Unselected", true).unwrap();
                    }
                }
            }
            OutputEvent::FocusGained(widget_info) => {
                *cached_text_selection = None;
                tts.speak(widget_info.description(), true).unwrap();
            }
            OutputEvent::TextSelectionChanged(widget_info) => {
                if let Some(text_selection) = widget_info.text_selection.clone() {
                    if widget_info.text_selection != *cached_text_selection {
                        if let Some(text) = &widget_info.current_text_value {
                            if text_selection.start() < &text.len() {
                                let str = &text[text_selection.clone()];
                                tts.speak(str, true).unwrap();
                            }
                        }
                    }
                    *cached_text_selection = Some(text_selection);
                }
            }
            OutputEvent::ValueChanged(widget_info) => {
                *cached_text_selection = None;
                if let (Some(text), Some(prev)) = (
                    &widget_info.current_text_value,
                    &widget_info.prev_text_value,
                ) {
                    let changes = Changeset::new(&prev, &text, "");
                    for change in changes.diffs {
                        tts.stop().unwrap();
                        use difference::Difference;
                        match change {
                            Difference::Add(str) => {
                                tts.speak(str, false).unwrap();
                            }
                            Difference::Rem(str) => {
                                tts.speak(str, false).unwrap();
                            }
                            _ => {}
                        };
                    }
                }
            }
            v => {
                *cached_text_selection = None;
                println!("{:?}", v);
            }
        };
    }
}
