use crate::app::theme::Theme;
use crate::input::keys::VKey;
use crate::keyboard_visual::layout_k26::{k26_physical_layout, KeyDefinition};
use crate::profiles::model::Profile;
use crate::remap::rules::MappingTarget;
use egui::{Color32, CornerRadius, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};
use std::collections::HashSet;

pub struct KeyboardRenderer<'a> {
    pressed_keys: &'a HashSet<VKey>,
    selected_key: Option<VKey>,
    active_profile: &'a Profile,
    unit_size: f32,
    gap: f32,
}

impl<'a> KeyboardRenderer<'a> {
    pub fn new(
        pressed_keys: &'a HashSet<VKey>,
        selected_key: Option<VKey>,
        active_profile: &'a Profile,
    ) -> Self {
        Self {
            pressed_keys,
            selected_key,
            active_profile,
            unit_size: 42.0,
            gap: 4.0,
        }
    }

    pub fn render(&self, ui: &mut Ui) -> Option<VKey> {
        let rows = k26_physical_layout();
        let total_rows = rows.len() as f32;
        let max_row_width = 16.0;

        let total_w = max_row_width * (self.unit_size + self.gap);
        let total_h = total_rows * (self.unit_size + self.gap);

        let (rect, _response) = ui.allocate_exact_size(Vec2::new(total_w, total_h), Sense::hover());
        let painter = ui.painter_at(rect);

        let mut clicked_key = None;

        // Background chassis with sharp retro edges
        painter.rect_filled(
            rect.expand(6.0),
            CornerRadius::same(1),
            Color32::from_rgb(0x0A, 0x0A, 0x0F),
        );
        painter.rect_stroke(
            rect.expand(6.0),
            CornerRadius::same(1),
            Stroke::new(1.0_f32, Theme::BORDER_SUBTLE),
            StrokeKind::Inside,
        );

        let mut y = rect.min.y;

        for row in &rows {
            let mut x = rect.min.x;

            for key in row {
                let w = key.width_u * self.unit_size + (key.width_u - 1.0).max(0.0) * self.gap;
                let h = self.unit_size;
                let key_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h));

                let is_pressed = self.pressed_keys.contains(&key.vkey);
                let is_selected = self.selected_key == Some(key.vkey);
                let mapping = self.active_profile.mappings.get(&key.vkey);

                let key_response = ui.interact(
                    key_rect,
                    ui.id().with(key.vkey),
                    Sense::click(),
                );

                if key_response.clicked() {
                    clicked_key = Some(key.vkey);
                }

                self.paint_keycap(
                    &painter,
                    key_rect,
                    key,
                    is_pressed,
                    is_selected,
                    mapping,
                    key_response.hovered(),
                );

                x += w + self.gap;
            }

            y += self.unit_size + self.gap;
        }

        clicked_key
    }

    fn paint_keycap(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        key: &KeyDefinition,
        is_pressed: bool,
        is_selected: bool,
        mapping: Option<&MappingTarget>,
        is_hovered: bool,
    ) {
        // Outer glow simulation when selected or physically pressed
        if is_selected {
            painter.rect_stroke(
                rect.expand(1.5),
                CornerRadius::same(1),
                Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(0x3D, 0xA9, 0xFC, 80)),
                StrokeKind::Outside,
            );
        } else if is_pressed {
            painter.rect_stroke(
                rect.expand(1.5),
                CornerRadius::same(1),
                Stroke::new(1.0_f32, Color32::from_rgba_premultiplied(0x3D, 0xFC, 0xA0, 100)),
                StrokeKind::Outside,
            );
        }

        let base_fill = if is_pressed {
            Color32::from_rgb(0x1B, 0x48, 0x38) // CRT phosphor active state
        } else if is_selected {
            Color32::from_rgb(0x18, 0x2A, 0x44) // Electric blue base
        } else if is_hovered {
            Color32::from_rgb(0x22, 0x22, 0x32)
        } else {
            Theme::BG_KEYCAP
        };

        let border_stroke = if is_pressed {
            Stroke::new(1.5_f32, Theme::ACCENT_GREEN)
        } else if is_selected {
            Stroke::new(1.5_f32, Theme::ACCENT_BLUE)
        } else if mapping.is_some() {
            Stroke::new(1.0_f32, Theme::ACCENT_VIOLET)
        } else {
            Stroke::new(1.0_f32, Theme::BORDER_SUBTLE)
        };

        painter.rect_filled(rect, CornerRadius::same(1), base_fill);
        painter.rect_stroke(rect, CornerRadius::same(1), border_stroke, StrokeKind::Inside);

        let inset = rect.shrink2(Vec2::new(2.5, 2.5));
        let top_fill = if is_pressed {
            Color32::from_rgb(0x22, 0x64, 0x4E)
        } else if is_selected {
            Color32::from_rgb(0x15, 0x24, 0x3A)
        } else {
            Color32::from_rgb(0x12, 0x12, 0x1B)
        };
        painter.rect_filled(inset, CornerRadius::same(1), top_fill);

        let text_color = if is_pressed {
            Color32::WHITE
        } else if is_selected {
            Theme::ACCENT_BLUE
        } else {
            Theme::TEXT_PRIMARY
        };

        painter.text(
            inset.center(),
            egui::Align2::CENTER_CENTER,
            key.vkey.label(),
            egui::FontId::monospace(12.0),
            text_color,
        );

        if let Some(sub) = key.secondary_label {
            painter.text(
                inset.min + Vec2::new(3.0, 2.0),
                egui::Align2::LEFT_TOP,
                sub,
                egui::FontId::monospace(9.0),
                Theme::TEXT_MUTED,
            );
        }

        if let Some(target) = mapping {
            let (badge_text, badge_color) = match target {
                MappingTarget::Key(dest) => (dest.label(), Theme::ACCENT_VIOLET),
                MappingTarget::Block => ("✕", Theme::ACCENT_RED),
            };

            painter.text(
                inset.max - Vec2::new(3.0, 2.0),
                egui::Align2::RIGHT_BOTTOM,
                badge_text,
                egui::FontId::monospace(9.0),
                badge_color,
            );
        }
    }
}
