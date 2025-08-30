use eframe::egui::{self, Pos2, Rect, vec2};

#[derive(Default)]
pub struct DragState {
    dragging: bool,
    start_pos: Pos2,
}

impl DragState {
    pub fn handle_drag(
        &mut self,
        ctx: &egui::Context,
        top_bar_height: f32,
    ) -> bool {
        let pointer = ctx.input(|i| i.pointer.clone());

        let drag_zone = Rect::from_min_size(
            Pos2::new(0.0, 0.0),
            vec2(ctx.screen_rect().width(), top_bar_height),
        );

        if pointer.any_pressed() {
            if let Some(pos) = pointer.press_origin() {
                if drag_zone.contains(pos) {
                    self.dragging = true;
                    self.start_pos = pos;
                }
            }
        }

        if self.dragging {
            if pointer.any_down() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            } else {
                self.dragging = false;
            }
        }

        self.dragging
    }
}
