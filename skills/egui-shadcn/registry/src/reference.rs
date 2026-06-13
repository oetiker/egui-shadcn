//! The reference screen: a settings form with tabs. Built entirely from the
//! egui_shadcn theme + layout helpers + components. Reused by the snapshot eval
//! and the `settings` example binary.

use crate::components::button::{Button, ButtonVariant};
use crate::components::card::{card_description, card_title};
use crate::components::input::Input;
use crate::components::label::label;
use crate::components::separator::separator;
use crate::components::switch::toggle;
use crate::components::tabs::tab_bar;
use crate::{layout, Theme};

#[derive(Default)]
pub struct SettingsState {
    pub tab: usize,
    pub name: String,
    pub email: String,
    pub marketing_emails: bool,
    pub security_emails: bool,
}

/// Render the settings screen into a CentralPanel. Call `Theme::apply` on the
/// context once per frame before calling this.
#[allow(deprecated)] // CentralPanel::show is deprecated in egui 0.34; the replacement
                     // show_inside() requires &mut Ui, but we expose a ctx-level API
                     // so the caller does not need to manage a root panel.
pub fn settings_ui(ctx: &egui::Context, state: &mut SettingsState) {
    let t = Theme::current(ctx);
    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(t.palette.background)
                .inner_margin(egui::Margin::same(32)),
        )
        .show(ctx, |ui| {
            layout::vstack(ui, 24.0, |ui| {
                card_title(ui, "Settings");
                card_description(ui, "Manage your account settings and preferences.");
                tab_bar(ui, &mut state.tab, &["Account", "Notifications"]);
                if state.tab == 0 {
                    layout::card(ui, |ui| {
                        layout::vstack(ui, 16.0, |ui| {
                            card_title(ui, "Account");
                            card_description(ui, "Update your account details.");
                            separator(ui);
                            layout::vstack(ui, 6.0, |ui| {
                                label(ui, "Name");
                                ui.add(Input::new(&mut state.name).hint("Your name").max_width(f32::INFINITY));
                            });
                            layout::vstack(ui, 6.0, |ui| {
                                label(ui, "Email");
                                ui.add(Input::new(&mut state.email).hint("you@example.com").max_width(f32::INFINITY));
                            });
                            layout::space_between(
                                ui,
                                |_ui| {},
                                |ui| {
                                    let _ = ui.add(Button::new("Save").variant(ButtonVariant::Default));
                                },
                            );
                        });
                    });
                } else {
                    layout::card(ui, |ui| {
                        layout::vstack(ui, 16.0, |ui| {
                            card_title(ui, "Notifications");
                            card_description(ui, "Choose what you want to be notified about.");
                            separator(ui);
                            layout::space_between(
                                ui,
                                |ui| {
                                    label(ui, "Marketing emails");
                                },
                                |ui| {
                                    let _ = toggle(ui, &mut state.marketing_emails);
                                },
                            );
                            layout::space_between(
                                ui,
                                |ui| {
                                    label(ui, "Security emails");
                                },
                                |ui| {
                                    let _ = toggle(ui, &mut state.security_emails);
                                },
                            );
                        });
                    });
                }
            });
        });
}
