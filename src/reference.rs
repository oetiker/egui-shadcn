//! The reference screen: a settings form with tabs that doubles as a gallery of
//! every ported component. Built entirely from the egui_shadcn theme + layout
//! helpers + components. Reused by the snapshot eval and the `settings` example.

use crate::components::badge::{badge, BadgeVariant};
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::card::{card_description, card_title};
use crate::components::checkbox::checkbox;
use crate::components::input::Input;
use crate::components::label::label;
use crate::components::select::select;
use crate::components::separator::separator;
use crate::components::switch::toggle;
use crate::components::tabs::tab_bar;
use crate::{layout, Theme};

#[derive(Default)]
pub struct SettingsState {
    pub tab: usize,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: usize,
    pub marketing_emails: bool,
    pub security_emails: bool,
    pub weekly_digest: bool,
}

/// Render the settings screen into the given root `Ui`. Call `Theme::apply` on
/// `ui.ctx()` once per frame before calling this. The caller owns the root `Ui`
/// (e.g. from `eframe::run_ui_native`); this fills it with a `CentralPanel`.
pub fn settings_ui(ui: &mut egui::Ui, state: &mut SettingsState) {
    let t = Theme::current(ui.ctx());
    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(t.palette.background)
                .inner_margin(egui::Margin::same(32)),
        )
        .show_inside(ui, |ui| {
            layout::vstack(ui, 24.0, |ui| {
                card_title(ui, "Settings");
                card_description(ui, "Manage your account settings and preferences.");
                tab_bar(ui, &mut state.tab, &["Account", "Notifications", "Components"]);
                match state.tab {
                    0 => account_tab(ui, state),
                    1 => notifications_tab(ui, state),
                    _ => components_tab(ui),
                }
            });
        });
}

/// A labelled control: a 6px-gap stack of `label` over the control.
fn field(ui: &mut egui::Ui, name: &str, control: impl FnOnce(&mut egui::Ui)) {
    layout::vstack(ui, 6.0, |ui| {
        label(ui, name);
        control(ui);
    });
}

fn account_tab(ui: &mut egui::Ui, state: &mut SettingsState) {
    layout::card(ui, |ui| {
        layout::vstack(ui, 16.0, |ui| {
            card_title(ui, "Account");
            card_description(ui, "Update your account details.");
            separator(ui);
            field(ui, "Name", |ui| {
                ui.add(Input::new(&mut state.name).hint("Your name").max_width(f32::INFINITY));
            });
            field(ui, "Email", |ui| {
                ui.add(Input::new(&mut state.email).hint("you@example.com").max_width(f32::INFINITY));
            });
            field(ui, "Password", |ui| {
                ui.add(
                    Input::new(&mut state.password)
                        .hint("••••••••")
                        .password(true)
                        .max_width(f32::INFINITY),
                );
            });
            field(ui, "Role", |ui| {
                select(ui, "role", &mut state.role, &["Member", "Admin", "Owner"]);
            });
            separator(ui);
            layout::space_between(
                ui,
                |_ui| {},
                // right_to_left: the first widget added lands rightmost, so add the
                // primary action (Save) first to keep it on the right of Cancel.
                |ui| {
                    let _ = ui.add(Button::new("Save").variant(ButtonVariant::Default));
                    let _ = ui.add(Button::new("Cancel").variant(ButtonVariant::Outline));
                },
            );
        });
    });
}

fn notifications_tab(ui: &mut egui::Ui, state: &mut SettingsState) {
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
            layout::space_between(
                ui,
                |ui| {
                    label(ui, "Weekly digest");
                },
                |ui| {
                    let _ = checkbox(ui, &mut state.weekly_digest);
                },
            );
            separator(ui);
            label(ui, "Channel status");
            layout::row(ui, 8.0, |ui| {
                badge(ui, "Active", BadgeVariant::Default);
                badge(ui, "Beta", BadgeVariant::Secondary);
                badge(ui, "Deprecated", BadgeVariant::Destructive);
                badge(ui, "Optional", BadgeVariant::Outline);
            });
        });
    });
}

fn components_tab(ui: &mut egui::Ui) {
    layout::card(ui, |ui| {
        layout::vstack(ui, 16.0, |ui| {
            card_title(ui, "Components");
            card_description(ui, "Buttons and badges in every variant.");
            separator(ui);
            label(ui, "Button variants");
            layout::row(ui, 8.0, |ui| {
                let _ = ui.add(Button::new("Default").variant(ButtonVariant::Default));
                let _ = ui.add(Button::new("Secondary").variant(ButtonVariant::Secondary));
                let _ = ui.add(Button::new("Destructive").variant(ButtonVariant::Destructive));
            });
            layout::row(ui, 8.0, |ui| {
                let _ = ui.add(Button::new("Outline").variant(ButtonVariant::Outline));
                let _ = ui.add(Button::new("Ghost").variant(ButtonVariant::Ghost));
                let _ = ui.add(Button::new("Link").variant(ButtonVariant::Link));
            });
            label(ui, "Button sizes");
            layout::row(ui, 8.0, |ui| {
                let _ = ui.add(Button::new("Small").variant(ButtonVariant::Secondary).size(ButtonSize::Sm));
                let _ = ui.add(Button::new("Default").variant(ButtonVariant::Secondary).size(ButtonSize::Default));
                let _ = ui.add(Button::new("Large").variant(ButtonVariant::Secondary).size(ButtonSize::Lg));
            });
            separator(ui);
            label(ui, "Badges");
            layout::row(ui, 8.0, |ui| {
                badge(ui, "Default", BadgeVariant::Default);
                badge(ui, "Secondary", BadgeVariant::Secondary);
                badge(ui, "Destructive", BadgeVariant::Destructive);
                badge(ui, "Outline", BadgeVariant::Outline);
            });
        });
    });
}
