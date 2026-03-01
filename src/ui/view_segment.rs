//! View Mode Segmented Control for Ferrite
//!
//! This module implements a modern segmented control widget for switching
//! between view modes (Raw, Split, Rendered) in the title bar.
//!
//! The control displays all three view modes as a compact pill-shaped button group,
//! making it immediately clear which modes are available and which is currently active.

use crate::app::modifier_symbol;
use crate::config::ViewMode;
use crate::state::FileType;
use eframe::egui::{self, Color32, Response, RichText, Sense, Vec2};

/// Height of the segmented control.
const SEGMENT_HEIGHT: f32 = 20.0;

/// Width of each segment button.
const SEGMENT_WIDTH: f32 = 26.0;

/// Corner rounding for the pill shape.
const CORNER_ROUNDING: f32 = 10.0;

/// Inner padding for the selected indicator.
const INNER_PADDING: f32 = 2.0;

/// Actions that can be triggered from the view mode segment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewSegmentAction {
    /// Switch to Raw mode
    SetRaw,
    /// Switch to Split mode
    SetSplit,
    /// Switch to Rendered mode
    SetRendered,
}

/// View mode segmented control widget.
///
/// A three-button toggle control for switching between Raw, Split, and Rendered views.
/// File-type aware: disables Split for non-markdown structured files.
#[derive(Debug, Clone, Default)]
pub struct ViewModeSegment;

impl ViewModeSegment {
    /// Create a new view mode segment widget.
    pub fn new() -> Self {
        Self
    }

    /// Show the segmented control and return any triggered action.
    ///
    /// # Arguments
    ///
    /// * `ui` - The egui UI context
    /// * `current_mode` - The currently active view mode
    /// * `file_type` - The current file type (affects which modes are available)
    /// * `is_dark` - Whether the current theme is dark mode
    ///
    /// # Returns
    ///
    /// Optional action if a segment was clicked
    pub fn show(
        &self,
        ui: &mut egui::Ui,
        current_mode: ViewMode,
        file_type: FileType,
        _is_dark: bool,
    ) -> Option<ViewSegmentAction> {
        let mut action: Option<ViewSegmentAction> = None;
        let visuals = ui.visuals();
        let dark_mode = visuals.dark_mode;

        // Determine which modes are available based on file type
        let split_available = file_type.is_markdown();
        let rendered_available = file_type.is_markdown() || file_type.is_structured();

        // Colors - refined for a polished pill appearance
        let bg_color = visuals.faint_bg_color;
        let selected_bg = visuals.selection.bg_fill;
        let hover_bg = visuals.widgets.hovered.weak_bg_fill;
        let text_color = visuals.widgets.inactive.fg_stroke.color;
        let selected_text = visuals.widgets.active.fg_stroke.color;
        let disabled_text = visuals.widgets.noninteractive.fg_stroke.color;
        let border_color = visuals.widgets.noninteractive.bg_stroke.color;

        // Calculate total width
        let total_width = SEGMENT_WIDTH * 3.0;
        let size = Vec2::new(total_width, SEGMENT_HEIGHT);

        // Allocate space for the entire control
        let (rect, _response) = ui.allocate_exact_size(size, Sense::hover());

        // Draw outer border/shadow for depth
        ui.painter().rect_filled(
            rect.expand(1.0),
            CORNER_ROUNDING + 1.0,
            border_color,
        );

        // Draw pill background
        ui.painter()
            .rect_filled(rect, CORNER_ROUNDING, bg_color);

        // Define segment data: (mode, icon, tooltip, action, enabled)
        // Using text icons for cross-platform compatibility
        let segments = [
            (
                ViewMode::Raw,
                "R",
                "Raw Editor",
                ViewSegmentAction::SetRaw,
                true,
            ),
            (
                ViewMode::Split,
                "S",
                "Split View",
                ViewSegmentAction::SetSplit,
                split_available,
            ),
            (
                ViewMode::Rendered,
                "V",
                "Rendered View",
                ViewSegmentAction::SetRendered,
                rendered_available,
            ),
        ];

        // First pass: draw selected indicator
        let mut x_offset = rect.min.x;
        for (mode, _, _, _, _) in segments.iter() {
            if current_mode == *mode {
                let segment_rect = egui::Rect::from_min_size(
                    egui::pos2(x_offset, rect.min.y),
                    Vec2::new(SEGMENT_WIDTH, SEGMENT_HEIGHT),
                );

                let indicator_rect = segment_rect.shrink(INNER_PADDING);
                ui.painter().rect_stroke(
                    indicator_rect,
                    CORNER_ROUNDING - INNER_PADDING,
                    egui::Stroke::new(1.5, selected_bg),
                );
                break;
            }
            x_offset += SEGMENT_WIDTH;
        }

        // Second pass: draw icons and handle interactions
        x_offset = rect.min.x;
        for (mode, icon, tooltip, segment_action, enabled) in segments.iter() {
            let segment_rect = egui::Rect::from_min_size(
                egui::pos2(x_offset, rect.min.y),
                Vec2::new(SEGMENT_WIDTH, SEGMENT_HEIGHT),
            );

            let is_selected = current_mode == *mode;

            // Create clickable area
            let segment_response = ui.allocate_rect(segment_rect, Sense::click());

            // Draw hover effect (only for non-selected, enabled segments)
            if !is_selected && segment_response.hovered() && *enabled {
                let hover_rect = segment_rect.shrink(INNER_PADDING);
                ui.painter().rect_filled(
                    hover_rect,
                    CORNER_ROUNDING - INNER_PADDING,
                    hover_bg,
                );
            }

            // Determine icon color
            let icon_color = if !*enabled {
                disabled_text
            } else if is_selected {
                selected_text
            } else {
                text_color
            };

            // Draw icon - using single letters for consistency and cross-platform support
            ui.painter().text(
                segment_rect.center(),
                egui::Align2::CENTER_CENTER,
                icon,
                egui::FontId::proportional(11.0),
                icon_color,
            );

            // Handle click
            if segment_response.clicked() && *enabled && !is_selected {
                action = Some(*segment_action);
            }

            // Tooltip with keyboard shortcut
            let tooltip_text = if *enabled {
                format!("{} ({}+E to cycle)", tooltip, modifier_symbol())
            } else {
                format!("{} (not available for this file type)", tooltip)
            };
            segment_response.on_hover_text(tooltip_text);

            x_offset += SEGMENT_WIDTH;
        }

        action
    }

    /// Show a compact 2-mode segment for structured files (Raw/Rendered only).
    ///
    /// This variant is used when Split mode is not available.
    pub fn show_two_mode(
        &self,
        ui: &mut egui::Ui,
        current_mode: ViewMode,
        _is_dark: bool,
    ) -> Option<ViewSegmentAction> {
        let mut action: Option<ViewSegmentAction> = None;
        let visuals = ui.visuals();
        let bg_color = visuals.faint_bg_color;
        let selected_bg = visuals.selection.bg_fill;
        let hover_bg = visuals.widgets.hovered.weak_bg_fill;
        let text_color = visuals.widgets.inactive.fg_stroke.color;
        let selected_text = visuals.widgets.active.fg_stroke.color;
        let border_color = visuals.widgets.noninteractive.bg_stroke.color;

        // Two segments only
        let total_width = SEGMENT_WIDTH * 2.0;
        let size = Vec2::new(total_width, SEGMENT_HEIGHT);

        let (rect, _response) = ui.allocate_exact_size(size, Sense::hover());

        // Draw border and background
        ui.painter()
            .rect_filled(rect.expand(1.0), CORNER_ROUNDING + 1.0, border_color);
        ui.painter()
            .rect_filled(rect, CORNER_ROUNDING, bg_color);

        let segments = [
            (ViewMode::Raw, "R", "Raw Editor", ViewSegmentAction::SetRaw),
            (
                ViewMode::Rendered,
                "V",
                "Rendered View",
                ViewSegmentAction::SetRendered,
            ),
        ];

        // Draw selected indicator
        let mut x_offset = rect.min.x;
        for (mode, _, _, _) in segments.iter() {
            if current_mode == *mode || (current_mode == ViewMode::Split && *mode == ViewMode::Raw)
            {
                let segment_rect = egui::Rect::from_min_size(
                    egui::pos2(x_offset, rect.min.y),
                    Vec2::new(SEGMENT_WIDTH, SEGMENT_HEIGHT),
                );
                let indicator_rect = segment_rect.shrink(INNER_PADDING);

                ui.painter().rect_stroke(
                    indicator_rect,
                    CORNER_ROUNDING - INNER_PADDING,
                    egui::Stroke::new(1.5, selected_bg),
                );
                break;
            }
            x_offset += SEGMENT_WIDTH;
        }

        // Draw icons and handle interactions
        x_offset = rect.min.x;
        for (mode, icon, tooltip, segment_action) in segments.iter() {
            let segment_rect = egui::Rect::from_min_size(
                egui::pos2(x_offset, rect.min.y),
                Vec2::new(SEGMENT_WIDTH, SEGMENT_HEIGHT),
            );

            let is_selected =
                current_mode == *mode || (current_mode == ViewMode::Split && *mode == ViewMode::Raw);
            let segment_response = ui.allocate_rect(segment_rect, Sense::click());

            if !is_selected && segment_response.hovered() {
                let hover_rect = segment_rect.shrink(INNER_PADDING);
                ui.painter().rect_filled(
                    hover_rect,
                    CORNER_ROUNDING - INNER_PADDING,
                    hover_bg,
                );
            }

            let icon_color = if is_selected {
                selected_text
            } else {
                text_color
            };

            ui.painter().text(
                segment_rect.center(),
                egui::Align2::CENTER_CENTER,
                icon,
                egui::FontId::proportional(11.0),
                icon_color,
            );

            if segment_response.clicked() && !is_selected {
                action = Some(*segment_action);
            }

            let tooltip_text = format!("{} ({}+E to toggle)", tooltip, modifier_symbol());
            segment_response.on_hover_text(tooltip_text);

            x_offset += SEGMENT_WIDTH;
        }

        action
    }
}

/// Title bar button for quick toggles (Settings, Zen Mode, etc.)
///
/// A compact button styled for the title bar area.
pub struct TitleBarButton;

impl TitleBarButton {
    /// Show a title bar toggle button.
    ///
    /// # Arguments
    ///
    /// * `ui` - The egui UI context
    /// * `icon` - The icon/emoji to display
    /// * `tooltip` - Hover tooltip text
    /// * `is_active` - Whether the button is in active/on state
    /// * `is_dark` - Whether the current theme is dark mode
    ///
    /// # Returns
    ///
    /// The button response for click detection
    pub fn show(
        ui: &mut egui::Ui,
        icon: &str,
        tooltip: &str,
        is_active: bool,
        _is_dark: bool,
    ) -> Response {
        let size = Vec2::new(28.0, 24.0); // Slightly taller for better alignment
        let visuals = ui.visuals();
        let text_color = visuals.widgets.inactive.fg_stroke.color;
        let hover_bg = visuals.widgets.hovered.weak_bg_fill;
        let active_outline = visuals.selection.bg_fill;
        let active_text = visuals.widgets.active.fg_stroke.color;

        let btn = ui.add(
            egui::Button::new(RichText::new(" ").size(14.0)) // Match icon size
                .frame(false)
                .min_size(size),
        );

        // Draw background on hover or if active
        if is_active {
            ui.painter()
                .rect_stroke(
                    btn.rect.shrink(0.5),
                    egui::Rounding::same(3.0),
                    egui::Stroke::new(1.5, active_outline),
                );
        } else if btn.hovered() {
            ui.painter()
                .rect_filled(btn.rect, egui::Rounding::same(3.0), hover_bg);
        }

        // Draw icon centered
        ui.painter().text(
            btn.rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::proportional(14.0),
            if is_active { active_text } else { text_color },
        );

        btn.on_hover_text(tooltip)
    }

    /// Show an auto-save indicator button with special styling.
    ///
    /// Green tint when enabled, muted when disabled.
    pub fn show_auto_save(
        ui: &mut egui::Ui,
        enabled: bool,
        _is_dark: bool,
    ) -> Response {
        let size = Vec2::new(28.0, 24.0); // Match other title bar buttons
        let visuals = ui.visuals();
        
        let icon = if enabled { "⏱" } else { "⏸" };
        let tooltip = if enabled {
            "Auto-Save: ON (click to disable)"
        } else {
            "Auto-Save: OFF (click to enable)"
        };

        // Green tint for enabled, muted for disabled
        let text_color = if enabled {
            visuals.hyperlink_color
        } else {
            visuals.widgets.noninteractive.fg_stroke.color
        };
        let hover_bg = visuals.widgets.hovered.weak_bg_fill;

        let btn = ui.add(
            egui::Button::new(RichText::new(" ").size(14.0)) // Match icon size
                .frame(false)
                .min_size(size),
        );

        if btn.hovered() {
            ui.painter()
                .rect_filled(btn.rect, egui::Rounding::same(3.0), hover_bg);
        }

        // Draw icon centered with appropriate color
        ui.painter().text(
            btn.rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::proportional(14.0),
            text_color,
        );

        btn.on_hover_text(tooltip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_mode_segment_new() {
        let segment = ViewModeSegment::new();
        // Just verify it can be created
        let _ = segment;
    }

    #[test]
    fn test_view_segment_action_equality() {
        assert_eq!(ViewSegmentAction::SetRaw, ViewSegmentAction::SetRaw);
        assert_ne!(ViewSegmentAction::SetRaw, ViewSegmentAction::SetSplit);
    }

    #[test]
    fn test_view_mode_segment_default() {
        let segment = ViewModeSegment::default();
        let _ = segment;
    }
}
