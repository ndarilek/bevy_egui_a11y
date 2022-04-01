use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{self, output::OutputEvent, TextEdit, WidgetType},
    EguiContext, EguiPlugin,
};
use bevy_egui_kbgp::prelude::*;
use bevy_tts::*;
use difference::Changeset;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Simple Bevy egui Screen Reader".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(KbgpPlugin)
        .add_plugin(TtsPlugin)
        .add_system(start_menu)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            screen_reader.before(bevy_egui::EguiSystem::ProcessOutput),
        )
        .run();
}

fn start_menu(
    context: Res<EguiContext>,
    mut tts: ResMut<Tts>,
    mut exit: EventWriter<AppExit>,
    mut checked: Local<bool>,
    mut username: Local<String>,
    mut password: Local<String>,
) {
    context.ctx().memory().options.screen_reader = true;
    egui::CentralPanel::default().show(context.ctx(), |ui| {
        if ui
            .button("Start")
            .kbgp_initial_focus()
            .kbgp_navigation()
            .clicked()
        {
            tts.speak("Start clicked", true).unwrap();
            println!("Start clicked");
        }
        ui.checkbox(&mut *checked, "Check me").kbgp_navigation();
        ui.horizontal(|ui| {
            ui.label("Username").kbgp_navigation();
            ui.text_edit_singleline(&mut *username).kbgp_navigation();
        });
        ui.horizontal(|ui| {
            ui.label("Password").kbgp_navigation();
            ui.add(TextEdit::singleline(&mut *password).password(true))
                .kbgp_navigation();
        });
        if ui.button("Quit").kbgp_navigation().clicked() {
            exit.send(AppExit);
        }
    });
}

fn screen_reader(context: Res<EguiContext>, mut tts: ResMut<Tts>) {
    for event in &context.ctx().output().events {
        println!("{:?}", event);
        match event {
            OutputEvent::Clicked(widget_info) => {
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
                tts.speak(widget_info.description(), true).unwrap();
            }
            OutputEvent::TextSelectionChanged(widget_info) => {
                if let Some(text_selection) = widget_info.text_selection.clone() {
                    if let Some(text) = &widget_info.current_text_value {
                        if text_selection.start() < &text.len() {
                            let str = &text[text_selection.clone()];
                            tts.speak(str, true).unwrap();
                        }
                    }
                }
            }
            OutputEvent::ValueChanged(widget_info) => {
                if let (Some(text), Some(prev)) = (
                    &widget_info.current_text_value,
                    &widget_info.prev_text_value,
                ) {
                    let changes = Changeset::new(&prev, &text, "");
                    for change in changes.diffs {
                        tts.stop().unwrap();
                        use difference::Difference;
                        match change {
                            Difference::Add(str) => tts.speak(str, false).unwrap(),
                            Difference::Rem(str) => tts.speak(str, false).unwrap(),
                            _ => None,
                        };
                    }
                }
            }
            v => {
                println!("{:?}", v);
            }
        };
    }
}
