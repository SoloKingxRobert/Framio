use eframe::{egui, NativeOptions};
mod core;
mod ui;

fn main() -> eframe::Result<()> {
    // Disable OS decorations so only your custom top bar is visible
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([900.0, 600.0])
            .with_decorations(false),
        ..Default::default()
    };

    // Preload the Framio logo at once
    let logo_bytes = include_bytes!("Assets/Logo/Framio.png");
    let logo_image = image::load_from_memory(logo_bytes)
        .expect("Failed to load logo image")
        .to_rgba8();
    let size = [logo_image.width() as usize, logo_image.height() as usize];
    let pixels = logo_image.into_raw();

    // Create the Shell instance outside the closure so it persists
    let mut shell = core::window::shell::Shell::default();

    eframe::run_simple_native("Framio", native_options, move |ctx, _frame| {
        // Apply branded dark theme
        ui::theme::dark::apply(ctx);

        // Upload logo texture to GPU (could also be preloaded once)
        let logo_texture = ctx.load_texture(
            "Framio",
            egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
            egui::TextureOptions::LINEAR,
        );

        // 1. Handle resize edges
        let _edge = core::window::resize::handle_resize(ctx);

        // 2. Draw the top navigation bar with logo
        shell.top_nav(ctx, &logo_texture);

        // 3. Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello from Framio!");
            ui.label("Frameless + Resize + Drag + Nav Menu is active.");
        });
    })
}
