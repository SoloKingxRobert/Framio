use eframe::egui::{self, CursorIcon};
use super::EDGE_THICKNESS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    None,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub fn handle_resize(ctx: &egui::Context) -> Edge {
    let rect = ctx.screen_rect();
    let pointer_pos = ctx.input(|i| i.pointer.hover_pos());

    if let Some(pos) = pointer_pos {
        let near_left   = pos.x <= rect.left()   + EDGE_THICKNESS;
        let near_right  = pos.x >= rect.right()  - EDGE_THICKNESS;
        let near_top    = pos.y <= rect.top()    + EDGE_THICKNESS;
        let near_bottom = pos.y >= rect.bottom() - EDGE_THICKNESS;

        let edge = match (near_left, near_right, near_top, near_bottom) {
            (true,  false, true,  false) => Edge::TopLeft,
            (false, true,  true,  false) => Edge::TopRight,
            (true,  false, false, true ) => Edge::BottomLeft,
            (false, true,  false, true ) => Edge::BottomRight,
            (true,  false, false, false) => Edge::Left,
            (false, true,  false, false) => Edge::Right,
            (false, false, true,  false) => Edge::Top,
            (false, false, false, true ) => Edge::Bottom,
            _ => Edge::None,
        };

        // Cursor feedback
        match edge {
            Edge::Left | Edge::Right => ctx.set_cursor_icon(CursorIcon::ResizeHorizontal),
            Edge::Top | Edge::Bottom => ctx.set_cursor_icon(CursorIcon::ResizeVertical),
            Edge::TopLeft | Edge::BottomRight => ctx.set_cursor_icon(CursorIcon::ResizeNwSe),
            Edge::TopRight | Edge::BottomLeft => ctx.set_cursor_icon(CursorIcon::ResizeNeSw),
            Edge::None => {}
        }

        // Actual OS resize on press
        if ctx.input(|i| i.pointer.any_pressed()) {
            use egui::{ViewportCommand, ResizeDirection::*};
            match edge {
                Edge::Left        => ctx.send_viewport_cmd(ViewportCommand::BeginResize(West)),
                Edge::Right       => ctx.send_viewport_cmd(ViewportCommand::BeginResize(East)),
                Edge::Top         => ctx.send_viewport_cmd(ViewportCommand::BeginResize(North)),
                Edge::Bottom      => ctx.send_viewport_cmd(ViewportCommand::BeginResize(South)),
                Edge::TopLeft     => ctx.send_viewport_cmd(ViewportCommand::BeginResize(NorthWest)),
                Edge::TopRight    => ctx.send_viewport_cmd(ViewportCommand::BeginResize(NorthEast)),
                Edge::BottomLeft  => ctx.send_viewport_cmd(ViewportCommand::BeginResize(SouthWest)),
                Edge::BottomRight => ctx.send_viewport_cmd(ViewportCommand::BeginResize(SouthEast)),
                Edge::None => {}
            }
        }

        return edge;
    }

    Edge::None
}
