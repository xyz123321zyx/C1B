use std::ops::Index;

use crate::{
    debugger,
    models::{C1BState, Tab, state, tab},
};
use eframe::egui;
use eframe::egui::Widget;

pub struct TabUI {
    colors: TabUIColors,
    actions: TabUIEvents,
    sizes: TabUISizes,
    scroll_to_end: bool,
}

pub enum TabUIEvents {
    NewTab(String),
    CloseTab(usize),
    SwitchTab(usize),
    RestoreTab(usize),
    RestoreAllTab,
}

pub enum TabUIColors {
    NonActive,
    Hover,
    Active,
    Divider,
    NonActiveText,
    ActiveText,
}

pub enum TabUISizes {
    TabButtonWidth,
    TabButtonHeight,
    TabFontSize,
    TabCloseButtonWidth,
    TabCloseButtonHeight,
    TabCloseButtonFontSize,
    TabDividerWidth,
    ActiveTabTopRounding,
    ActiveTabBottomRounding,
    TabSpacing,
    TabDividerVerticalMargin,
    TabCloseButtonRounding,
    InActiveTabHoverRounding,
}

impl TabUISizes {
    pub const fn size(self) -> f32 {
        match self {
            Self::TabButtonWidth => 250.0,
            Self::TabButtonHeight => 36.0,
            Self::TabFontSize => 12.0,
            Self::TabCloseButtonWidth => 23.0,
            Self::TabCloseButtonHeight => 23.0,
            Self::TabCloseButtonFontSize => 18.0,
            Self::TabDividerWidth => 2.0,
            Self::ActiveTabTopRounding => 8.0,
            Self::ActiveTabBottomRounding => 100.0,
            Self::TabSpacing => 15.0,
            Self::TabDividerVerticalMargin => 8.0,
            Self::TabCloseButtonRounding => 10.0,
            Self::InActiveTabHoverRounding => 4.0,
        }
    }
}

impl TabUIColors {
    pub const fn color(self) -> egui::Color32 {
        match self {
            Self::NonActive => egui::Color32::from_rgb(211, 227, 253),
            Self::Hover => egui::Color32::from_rgb(180, 200, 240),
            Self::Active => egui::Color32::from_rgb(255, 255, 255),
            Self::Divider => egui::Color32::from_rgb(168, 199, 250),
            Self::NonActiveText => egui::Color32::from_rgb(31, 31, 31),
            Self::ActiveText => egui::Color32::from_rgb(0, 0, 0),
        }
    }
}

impl TabUI {
    pub fn render_tab_bar(ui: &mut egui::Ui, state: &C1BState) {
        // debugger::StateDebugger::print_debug_state(state);
        egui::Frame::none()
            .fill(self::TabUIColors::NonActive.color())
            .inner_margin(egui::Margin::ZERO)
            .stroke(egui::Stroke::NONE)
            .show(ui, |ui| {
                let tab_frame = egui::Frame {
                    fill: self::TabUIColors::NonActive.color(),
                    stroke: egui::Stroke::NONE,
                    shadow: egui::epaint::Shadow::NONE,
                    inner_margin: egui::Margin {
                        top: 4,
                        bottom: 4,
                        left: 4,
                        right: 4,
                    },
                    ..Default::default()
                };

                tab_frame.show(ui, |ui| {
                    TabUI::render_scrollable_tabs(ui, state);
                });
            });
    }

    pub fn render_scrollable_tabs(ui: &mut egui::Ui, state: &C1BState) {
        egui::ScrollArea::horizontal()
            .auto_shrink([false; 2])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                let mut tab_to_close: Option<usize> = None;

                ui.horizontal(|ui| {
                    // Render all tabs
                    // debugger::StateDebugger::print_debug_state(state);
                    for (i, tab) in state.tabmanager.tabs.iter().enumerate() {
                        let tab_response = TabUI::render_single_tab(ui, i, &tab, state);

                        // if let Some(action) = tab_response {
                        //     match action {
                        //         TabUIActions::Close => tab_to_close = Some(i),
                        //         TabUIActions::Activate => self.manager.active_index = i,
                        //     }
                        // }
                    }

                    // Space and plus button
                    ui.add_space(8.0);
                    TabUI::render_plus_button(ui, state);
                });

                // Handle tab closure
                // self.handle_tab_closure(tab_to_close);
            });
    }

    pub fn render_single_tab(
        ui: &mut egui::Ui,
        index: usize,
        tab: &Tab,
        state: &C1BState,
    ) -> Option<TabUIEvents> {
        let is_active = index == state.tabmanager.active_tab_id;
        ui.add_space(TabUISizes::TabSpacing.size()); // add left spacing before tab
        // Allocate space for the tab
        let tab_size = egui::vec2(
            TabUISizes::TabButtonWidth.size(),
            TabUISizes::TabButtonHeight.size(),
        );
        let rect = egui::Rect::from_min_size(ui.cursor().min, tab_size);
        let resp = ui.allocate_rect(rect, egui::Sense::click());

        // Paint tab background
        if is_active {
            TabUI::render_active_tab_background(ui, &resp);
        } else {
            TabUI::render_inactive_tab_background(ui, &resp);
        }

        // Paint tab text
        let font_id = egui::FontId::proportional(TabUISizes::TabFontSize.size());
        let text_color = if is_active {
            TabUIColors::ActiveText.color()
        } else {
            TabUIColors::NonActiveText.color()
        };

        if let Some(title) = &tab.title {
            let text_pos = rect.min + egui::vec2(10.0, tab_size.y / 2.0);
            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_CENTER,
                title,
                font_id,
                text_color,
            );
        }

        let close_clicked = TabUI::render_close_button(ui, &resp);

        // Paint divider between tabs
        TabUI::render_tab_divider(ui, index, &rect, state);
        let close_clicked = TabUI::render_close_button(ui, &resp);

        // if close_clicked {
        //     Some(TabUIActions::Close)
        // } else if resp.clicked() {
        //     Some(TabUIActions::Activate)
        // } else {
        //     None
        // }
        None
    }

    pub fn render_active_tab_background(ui: &mut egui::Ui, resp: &egui::Response) {
        let bottom_expand = 4.0;
        let rect = resp.rect;
        let active_rect =
            egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.max.y + bottom_expand));

        // Main active tab shape
        ui.painter().rect_filled(
            active_rect,
            egui::Rounding {
                nw: TabUISizes::ActiveTabTopRounding.size() as u8,
                ne: TabUISizes::ActiveTabTopRounding.size() as u8,
                sw: 0,
                se: 0,
            },
            self::TabUIColors::Active.color(),
        );
        TabUI::render_tab_curves_for_active_tab(ui, &active_rect);
    }

    pub fn render_tab_curves_for_active_tab(ui: &mut egui::Ui, active_rect: &egui::Rect) {
        //Don't try to touch this function — unless you're looking for a brand-new way to waste your time.
        let side_width = 15.0;

        let left_rect = egui::Rect::from_min_max(
            egui::pos2(active_rect.min.x - side_width, active_rect.min.y),
            egui::pos2(active_rect.min.x, active_rect.max.y),
        );

        ui.painter().rect_filled(
            left_rect,
            egui::Rounding::ZERO,
            self::TabUIColors::Active.color(),
        );
        ui.painter().rect_filled(
            left_rect,
            egui::Rounding {
                se: TabUISizes::ActiveTabBottomRounding.size() as u8,
                ..egui::Rounding::ZERO
            },
            self::TabUIColors::NonActive.color(),
        );

        let right_rect = egui::Rect::from_min_max(
            egui::pos2(active_rect.max.x, active_rect.min.y),
            egui::pos2(active_rect.max.x + side_width, active_rect.max.y),
        );

        ui.painter().rect_filled(
            right_rect,
            egui::Rounding::ZERO,
            self::TabUIColors::Active.color(),
        );
        ui.painter().rect_filled(
            right_rect,
            egui::Rounding {
                sw: TabUISizes::ActiveTabBottomRounding.size() as u8,
                ..egui::Rounding::ZERO
            },
            self::TabUIColors::NonActive.color(),
        );
    }

    pub fn render_inactive_tab_background(ui: &mut egui::Ui, resp: &egui::Response) {
        // Animate hover effect
        let t = ui.ctx().animate_bool(resp.id, resp.hovered());
        let shrink = 2.0 * t;

        let reduced_rect = egui::Rect::from_min_max(
            egui::pos2(resp.rect.min.x, resp.rect.min.y + shrink),
            egui::pos2(resp.rect.max.x, resp.rect.max.y - shrink),
        );

        // Interpolate color between normal and hover
        let color = self::TabUIColors::Hover.color().linear_multiply(t)
            + self::TabUIColors::NonActive
                .color()
                .linear_multiply(1.0 - t);

        ui.painter().rect_filled(
            reduced_rect,
            TabUISizes::InActiveTabHoverRounding.size(),
            color,
        );
    }

    pub fn render_tab_divider(
        ui: &mut egui::Ui,
        index: usize,
        tab_rect: &egui::Rect,
        state: &C1BState,
    ) {
        // Only show divider between inactive tabs
        if index != state.tabmanager.active_tab_id && index + 1 != state.tabmanager.active_tab_id {
            let divider_x = tab_rect.max.x + 8.0;
            let top = tab_rect.top() + TabUISizes::TabDividerVerticalMargin.size();
            let bottom = tab_rect.bottom() - TabUISizes::TabDividerVerticalMargin.size();

            ui.painter().line_segment(
                [egui::pos2(divider_x, top), egui::pos2(divider_x, bottom)],
                egui::Stroke::new(
                    TabUISizes::TabDividerWidth.size(),
                    self::TabUIColors::Divider.color(),
                ),
            );
        }
    }

    pub fn render_tab_favicon(
        ui: &mut egui::Ui,
        resp: &egui::Response,
        index: usize,
        state: &C1BState,
    ) {
        // Define the favicon area with fixed size
        let favicon_size = 16.0;
        let favicon_start = resp.rect.left_center() + egui::vec2(10.0, 0.0);
        let favicon_rect =
            egui::Rect::from_center_size(favicon_start, egui::vec2(favicon_size, favicon_size));

        // Get favicon URL from tab data
        if let Some(favicon_url) = &state.tabmanager.tabs.index(index).favicon_url {
            // Show loading animation behind the image
            TabUI::render_loading_animation(ui, favicon_start, favicon_size, index, state);

            // Create a child UI for the favicon area
            let mut child_ui = ui.child_ui(
                favicon_rect,
                egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                None,
            );

            // Render the image (egui will handle loading and caching)
            let response = egui::Image::from_uri(favicon_url)
                .max_size(egui::vec2(favicon_size, favicon_size))
                .fit_to_exact_size(egui::vec2(favicon_size, favicon_size))
                .ui(&mut child_ui);

            // Continue requesting repaints for animation until image loads
            // The animation will naturally disappear when the image renders on top
            ui.ctx().request_repaint();
        } else {
            // Fallback: render a default icon or placeholder
            TabUI::render_favicon_placeholder(ui, favicon_start, favicon_size, index, state);
        }
    }

    fn render_loading_animation(
        ui: &mut egui::Ui,
        favicon_start: egui::Pos2,
        favicon_size: f32,
        index: usize,
        state: &C1BState,
    ) {
        let time = ui.input(|i| i.time) as f32;

        let color = if index == state.tabmanager.active_tab_id {
            TabUIColors::ActiveText.color()
        } else {
            TabUIColors::NonActiveText.color()
        };

        // Create a pulsing circle animation
        let pulse = (time * 3.0).sin().abs(); // 3 Hz pulsing
        let radius = (favicon_size * 0.2) + (pulse * favicon_size * 0.1);

        ui.painter().circle_filled(
            favicon_start,
            radius,
            color.gamma_multiply(0.3 + pulse * 0.2),
        );

        // Add rotating dots around the circle
        let rotation = time * 4.0; // 4 radians per second
        let orbit_radius = favicon_size * 0.25;

        for i in 0..3 {
            let angle = rotation + (i as f32) * std::f32::consts::TAU / 3.0;
            let dot_pos =
                favicon_start + egui::vec2(angle.cos() * orbit_radius, angle.sin() * orbit_radius);

            ui.painter()
                .circle_filled(dot_pos, 1.0, color.gamma_multiply(0.6));
        }
    }

    fn render_favicon_placeholder(
        ui: &mut egui::Ui,
        favicon_start: egui::Pos2,
        favicon_size: f32,
        index: usize,
        state: &C1BState,
    ) {
        let placeholder_color = if index == state.tabmanager.active_tab_id {
            TabUIColors::ActiveText.color()
        } else {
            TabUIColors::NonActiveText.color()
        };

        // Draw a simple circle as placeholder
        ui.painter().circle_filled(
            favicon_start,
            favicon_size / 2.0,
            placeholder_color.gamma_multiply(0.3),
        );

        // Draw a simple document icon inside
        let icon_rect = egui::Rect::from_center_size(
            favicon_start,
            egui::vec2(favicon_size * 0.6, favicon_size * 0.6),
        );
        ui.painter().rect_stroke(
            icon_rect,
            2.0,
            egui::Stroke::new(1.0, placeholder_color),
            egui::StrokeKind::Outside,
        );
    }
    pub fn render_tab_text(
        ui: &mut egui::Ui,
        resp: &egui::Response,
        index: usize,
        state: &C1BState,
    ) {
        let font_id = egui::FontId::proportional(TabUISizes::TabFontSize.size());
        let text_color = if index == state.tabmanager.active_tab_id {
            TabUIColors::ActiveText.color()
        } else {
            TabUIColors::NonActiveText.color()
        };

        //text area with fixed width
        let text_start = resp.rect.left_center() + egui::vec2(10.0, 0.0);
        let text_width = 170.0;
        let text_rect = egui::Rect::from_min_size(
            text_start - egui::vec2(0.0, 10.0),
            egui::vec2(text_width, 20.0),
        );

        // Set clip rectangle to hide overflow
        ui.set_clip_rect(text_rect);

        let title = state
            .tabmanager
            .tabs
            .index(index)
            .title
            .as_deref() // Option<&str>
            .unwrap_or("<untitled>");

        ui.painter().text(
            text_start,
            egui::Align2::LEFT_CENTER,
            title, // now a &str
            font_id,
            text_color,
        );

        // Reset clip rectangle to original
        ui.set_clip_rect(resp.rect);
    }

    fn render_close_button(ui: &mut egui::Ui, resp: &egui::Response) -> bool {
        let close_rect = egui::Rect::from_min_size(
            egui::pos2(resp.rect.max.x - 30.0, resp.rect.center().y - 11.0),
            egui::vec2(
                TabUISizes::TabCloseButtonWidth.size(),
                TabUISizes::TabCloseButtonHeight.size(),
            ),
        );

        // Check if the close button area is hovered before drawing the button
        let is_hovered = ui.rect_contains_pointer(close_rect);

        // Draw black background manually when hovered
        if is_hovered {
            ui.painter().rect_filled(
                close_rect,
                TabUISizes::TabCloseButtonRounding.size(),
                TabUIColors::Divider.color(),
            );
        }

        let close_resp = ui.put(
            close_rect,
            egui::Button::new(
                egui::RichText::new("×")
                    .font(egui::FontId::proportional(
                        TabUISizes::TabCloseButtonFontSize.size(),
                    ))
                    .color(egui::Color32::BLACK)
                    .strong(),
            )
            .rounding(TabUISizes::TabCloseButtonRounding.size())
            .frame(false),
        );

        close_resp.clicked()
    }

    pub fn render_plus_button(ui: &mut egui::Ui, state: &C1BState) {
        // println!("+");
        let plus_button = ui.add_sized(
            [30.0, 30.0],
            egui::Button::new(
                egui::RichText::new("+")
                    .font(egui::FontId::proportional(15.0))
                    .color(egui::Color32::BLACK)
                    .strong(),
            )
            .fill(egui::Color32::from_rgb(211, 227, 253))
            .rounding(15.0),
        );

        //         if state.scroll_to_end {
        //             ui.scroll_to_rect(plus_button.rect, Some(egui::Align::Center));
        //             // state.scroll_to_end = false;
        //         }
        //         if plus_button.clicked() {
        // println!("Clicked");
        //         }
    }
}
