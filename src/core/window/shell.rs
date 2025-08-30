use eframe::egui::{
    self, Align, Align2, Color32, CornerRadius, Frame, Layout, Sense, Stroke,
    TextureHandle, ViewportCommand, vec2,
};
use egui::load::SizedTexture;
use super::{TOP_BAR_HEIGHT, NAV_BUTTON_SIZE, NAV_SPACING};
use super::menu;
use super::drag::DragState;

const LOGO_SIZE: f32 = 24.0;

pub struct Shell {
    drag_state: DragState,
}

impl Default for Shell {
    fn default() -> Self {
        Self {
            drag_state: DragState::default(),
        }
    }
}

impl Shell {
    pub fn top_nav(&mut self, ctx: &egui::Context, logo_texture: &TextureHandle) {
        // Close any open menu if the user clicks elsewhere
        menu::handle_global_click(ctx);

        let base = ctx.style().visuals.panel_fill;
        let [r, g, b, a] = base.to_array();
        let nav_bg = Color32::from_rgba_premultiplied(
            r.saturating_sub(8),
            g.saturating_sub(8),
            b.saturating_sub(8),
            a,
        );

        egui::TopBottomPanel::top("top_bar")
            .exact_height(TOP_BAR_HEIGHT)
            .frame(Frame {
                stroke: Stroke::NONE,
                fill: nav_bg,
                ..Default::default()
            })
            .show_separator_line(false)
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    // Logo
                    ui.add(egui::Image::new(SizedTexture {
                        id: logo_texture.id(),
                        size: vec2(LOGO_SIZE + 15.0, LOGO_SIZE + 15.0),
                    }));
                    ui.add_space(3.0);

                    // Menus
                    menu::render_menus(ctx, ui);

                    // Drag zone
                    let _ = self.drag_state.handle_drag(ctx, TOP_BAR_HEIGHT);

                    // Window control buttons
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        let is_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));

                        if control_button(ui, "X", Color32::from_rgb(232, 17, 35)).clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }

                        let max_icon = if is_maximized { "▣" } else { "▢" };
                        if control_button(ui, max_icon, ui.visuals().widgets.hovered.bg_fill).clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
                        }

                        if control_button(ui, "—", ui.visuals().widgets.hovered.bg_fill).clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                        }

                        ui.add_space(NAV_SPACING);
                    });
                });
            });
    }
}

fn control_button(ui: &mut egui::Ui, label: &str, hover_fill: Color32) -> egui::Response {
    let is_window_control = matches!(label, "—" | "▢" | "▣" | "X");
    let button_width = if is_window_control {
        NAV_BUTTON_SIZE * 1.65
    } else {
        NAV_BUTTON_SIZE
    };

    let (rect, resp) = ui.allocate_exact_size(vec2(button_width, TOP_BAR_HEIGHT), Sense::click());

    let effective_hover_fill = if is_window_control {
        match label {
            "—" | "▢" | "▣" => Color32::from_rgb(98, 98, 98),
            "X" => Color32::from_rgb(232, 17, 35),
            _ => hover_fill,
        }
    } else {
        hover_fill
    };

    if resp.hovered() {
        ui.painter()
            .rect_filled(rect, CornerRadius::ZERO, effective_hover_fill);
    }

    let font_id = if is_window_control {
        egui::FontId::proportional(
            ui.style().text_styles[&egui::TextStyle::Button].size * 1.1,
        )
    } else {
        egui::TextStyle::Button.resolve(ui.style())
    };

    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        font_id,
        ui.visuals().text_color(),
    );

    resp
}
