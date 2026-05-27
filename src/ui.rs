use eframe::egui::{self, Ui};
use crate::MyApp;

pub fn render(app: &mut MyApp, ui: &mut Ui) {
    ui.heading("STREAM-TOOL");
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Channel: ");
        ui.text_edit_singleline(&mut app.channel);
    });
    ui.separator();

    let twitch_btn_label = if app.twitch_connected { "Twitch Déconnecter" } else { "Twitch Connecter" };
    if ui.button(twitch_btn_label).clicked() {
        if !app.twitch_connected {
            app.runtime.spawn(crate::twitch_irc::twitch_irc_main(
                app.tx.clone(),
                app.channel.clone(),
            ));
            app.twitch_connected = true;
        }
    }

    ui.add(
        egui::Image::new(egui::include_image!("../assets/icon/twitch.png"))
            .corner_radius(5)
            .tint(egui::Color32::PURPLE)
            .max_size(egui::vec2(30.0, 30.0)),
    );

    ui.separator();

    // Récupère les nouveaux messages IRC
    while let Ok(msg) = app.rx.try_recv() {
        app.messages.push(msg);
        if app.messages.len() > 100 {
            app.messages.remove(0);
        }
    }

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for msg in &app.messages {
                ui.label(msg);
                ui.separator();
            }
        });
}