//! Unified application launcher for the Lunola Creative Suite.
//!
//! Provides a launcher screen that can open any of the three apps (wordcraft, deckcraft,
//! gridcraft). The launcher is accessible from the Backstage view and shows all available
//! apps with their icons and descriptions.

use egui::{RichText, Sense, Ui};

use crate::lunola::SuiteApp;
use crate::SlideApp;

/// The app IDs that can be launched.
pub use crate::lunola::SuiteApp as AppId;

/// Returns the display name for an app.
pub fn app_name(app: SuiteApp) -> &'static str {
    app.display_name()
}

/// Returns the description for an app.
pub fn app_description(app: SuiteApp) -> &'static str {
    app.description()
}

/// Attempt to launch the given app. In a desktop context this would spawn the process;
/// in the web context it shows a message. Returns true if the launch was initiated.
pub fn launch_app(_app: SuiteApp) -> bool {
    false
}

/// Draws the launcher UI: a grid of app tiles that can be clicked to launch.
pub fn show_launcher(_app: &mut SlideApp, ui: &mut Ui) {
    ui.label(RichText::new("Lunola Creative Suite").font(crate::theme::bold(22.0)));
    ui.add_space(8.0);
    ui.label(RichText::new("Choose an app to open").font(crate::theme::font(13.0)));
    ui.add_space(20.0);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
        for suite_app in SuiteApp::ALL {
            app_tile(ui, suite_app);
        }
    });
}

fn app_tile(ui: &mut Ui, suite_app: SuiteApp) {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(180.0, 160.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, suite_app.display_name()));

    let p = ui.painter();
    let hovered = resp.hovered();
    let t = crate::theme::Tokens::get(ui.ctx());
    let bg = if hovered { t.hover } else { t.card };
    p.rect_filled(rect, 8.0, bg);
    p.rect_stroke(
        rect,
        8.0,
        egui::Stroke::new(if hovered { 2.0 } else { 1.0 }, if hovered { t.accent } else { t.border }),
        egui::StrokeKind::Inside,
    );

    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(rect.center().x, rect.top() + 40.0),
        egui::vec2(48.0, 48.0),
    );
    if let Some(icon) = crate::icons::Icon::from_name(suite_app.icon()) {
        crate::icons::paint(p, icon_rect, icon, t.text, false);
    }

    let name_galley = p.layout(
        suite_app.display_name().to_string(),
        crate::theme::font(14.0),
        t.text,
        rect.width() - 16.0,
    );
    p.galley(
        egui::pos2(rect.center().x - name_galley.size().x / 2.0, rect.top() + 100.0),
        name_galley,
        t.text,
    );

    let desc_galley = p.layout(
        suite_app.description().to_string(),
        crate::theme::font(11.0),
        t.text_dim,
        rect.width() - 16.0,
    );
    p.galley(
        egui::pos2(rect.center().x - desc_galley.size().x / 2.0, rect.top() + 122.0),
        desc_galley,
        t.text_dim,
    );

    if resp.clicked() {
        launch_app(suite_app);
    }
}
