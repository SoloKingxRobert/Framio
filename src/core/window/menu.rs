use eframe::egui::{
    self, Align2, Color32, CornerRadius, Frame, Id, Sense, Stroke, TextStyle, vec2,
};

const MIN_DROPDOWN_WIDTH: f32 = 240.0;
const MAX_DROPDOWN_WIDTH: f32 = 280.0;

static mut OPEN_MENU: Option<&'static str> = None;

pub fn handle_global_click(ctx: &egui::Context) {
    if ctx.input(|i| i.pointer.any_click()) {
        unsafe { OPEN_MENU = None };
    }
}

pub fn render_menus(ctx: &egui::Context, ui: &mut egui::Ui) {
    let bar_rect = ui.max_rect();
    for &(id, label) in &[("file", "File"), ("edit", "Edit"), ("view", "View")] {
        let is_open = unsafe { OPEN_MENU == Some(id) };

        let menu_rect = ui.allocate_space(vec2(50.0, bar_rect.height())).1;
        let menu_id = Id::new(("menu_btn", id));
        let response = ui.interact(menu_rect, menu_id, Sense::click());

        if response.hovered() {
            ui.painter().rect_filled(
                menu_rect,
                CornerRadius::ZERO,
                Color32::from_rgb(98, 98, 98),
            );
            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
            unsafe { OPEN_MENU = Some(id) };
        }

        ui.painter().text(
            menu_rect.center(),
            Align2::CENTER_CENTER,
            label,
            TextStyle::Button.resolve(ui.style()),
            Color32::WHITE,
        );

        if is_open {
            egui::Area::new(Id::new(id))
                .order(egui::Order::Foreground)
                .fixed_pos(menu_rect.left_bottom())
                .show(ctx, |ui| {
                    Frame::popup(&ctx.style())
                        .fill(ctx.style().visuals.panel_fill)
                        .stroke(Stroke {
                            width: 1.0,
                            color: Color32::from_rgb(80, 80, 80),
                        })
                        .corner_radius(CornerRadius::same(4))
                        .show(ui, |ui| match id {
                            "file" => file_menu(ctx, ui),
                            "edit" => edit_menu(ctx, ui),
                            "view" => view_menu(ctx, ui),
                            _ => {}
                        });
                });
        }

        ui.add_space(6.0);
    }
}

fn file_menu(ctx: &egui::Context, ui: &mut egui::Ui) {
    draw_menu_items(ctx, ui, &["New", "Open", "Save"], "file_item");
}

fn edit_menu(ctx: &egui::Context, ui: &mut egui::Ui) {
    draw_menu_items(ctx, ui, &["Undo", "Redo"], "edit_item");
}

fn view_menu(ctx: &egui::Context, ui: &mut egui::Ui) {
    draw_menu_items(ctx, ui, &["Zoom In", "Zoom Out"], "view_item");
}

fn draw_menu_items(ctx: &egui::Context, ui: &mut egui::Ui, items: &[&str], id_prefix: &str) {
    let style = &ctx.style();

    let content_w = ui.fonts(|f| {
        items.iter()
            .map(|&item| {
                let layout = f.layout_no_wrap(
                    item.to_owned(),
                    TextStyle::Button.resolve(style),
                    Color32::WHITE,
                );
                layout.size().x
            })
            .fold(0.0, f32::max)
    });

    let target_w = (content_w + 20.0).clamp(MIN_DROPDOWN_WIDTH, MAX_DROPDOWN_WIDTH);
    ui.set_min_width(target_w);
    ui.set_max_width(target_w);

    let elide = |ui: &egui::Ui, s: &str, max_w: f32| -> String {
        let ts = TextStyle::Button.resolve(ui.style());
        let fits = ui.fonts(|f| {
            f.layout_no_wrap(s.to_owned(), ts.clone(), Color32::WHITE).size().x <= max_w
        });
        if fits { return s.to_owned(); }
        let mut base = s.to_owned();
        while !base.is_empty() {
            base.pop();
            let candidate = format!("{base}…");
            let fits = ui.fonts(|f| {
                f.layout_no_wrap(candidate.clone(), ts.clone(), Color32::WHITE).size().x <= max_w
            });
            if fits { return candidate; }
        }
        "…".to_owned()
    };

    for item in items {
        let rect = ui.allocate_space(vec2(target_w, 22.0)).1;
        let id = Id::new((id_prefix, item));
        let resp = ui.interact(rect, id, Sense::click());

        if resp.hovered() {
            ui.painter().rect_filled(
                rect.shrink(1.0),
                CornerRadius::same(4.0 as u8),
                Color32::from_rgb(60, 120, 200),
            );
        }

        let drawn = elide(ui, item, target_w - 28.0);
        ui.painter().text(
            rect.left_center() + vec2(8.0, 0.0),
            Align2::LEFT_CENTER,
            drawn,
            TextStyle::Button.resolve(ui.style()),
            Color32::WHITE,
        );
    }
}
