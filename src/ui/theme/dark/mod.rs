use eframe::egui;
use eframe::epaint::CornerRadius;

use crate::ui::theme::colors::*;     // HALO_BG_DARK / HALO_TEXT_DARK / HALO_ACCENT / HALO_PRIMARY
use crate::ui::theme::typography::*; // TITLE_SIZE / BODY_SIZE
use crate::ui::theme::spacing::*;    // PADDING_SMALL / PADDING_MEDIUM / PADDING_LARGE

pub fn apply(ctx: &egui::Context) {
    // Copy the current style so we can change it
    let mut style = (*ctx.style()).clone();

    // --- Base mode settings ---
    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(HALO_TEXT_DARK);

    // --- Background layers for depth ---
    style.visuals.panel_fill       = HALO_BG_DARK;
    style.visuals.window_fill      = egui::Color32::from_rgb(30, 31, 34);
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(37, 38, 41);
    style.visuals.faint_bg_color   = egui::Color32::from_rgb(60, 63, 65);

    // --- Widget background colours ---
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(60, 63, 65);
    style.visuals.widgets.hovered.bg_fill  = HALO_ACCENT;
    style.visuals.widgets.active.bg_fill   = HALO_PRIMARY;

    // --- Widget corner radii (cast f32 → u8) ---
    style.visuals.widgets.inactive.corner_radius = CornerRadius::same(PADDING_SMALL as u8);
    style.visuals.widgets.hovered.corner_radius  = CornerRadius::same(PADDING_SMALL as u8);
    style.visuals.widgets.active.corner_radius   = CornerRadius::same(PADDING_SMALL as u8);

    // --- Non-interactive widget stroke ---
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke {
        width: 1.0,
        color: egui::Color32::from_rgb(60, 63, 65),
    };

    // --- Layout spacing (f32 constants) ---
    style.spacing.item_spacing   = egui::vec2(PADDING_MEDIUM, PADDING_SMALL);
    style.spacing.button_padding = egui::vec2(PADDING_MEDIUM, PADDING_SMALL);

    // Margin::same() in your egui version expects i8, so cast
    style.spacing.window_margin  = egui::Margin::same(PADDING_LARGE as i8);

    // --- Typography sizes ---
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::proportional(TITLE_SIZE),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::proportional(BODY_SIZE),
    );

    // Apply the modified style back to the context
    ctx.set_style(style);
}
