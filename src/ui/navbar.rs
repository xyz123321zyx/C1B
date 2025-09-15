use eframe::egui::{self, ColorImage, TextureHandle};
use std::sync::OnceLock;

use crate::models::C1BState;

pub struct NavBarUI {
    sizes: NavBarUISizes,
    pub colors: NavBarUIColors,
    events: NavBarUIEvents,
}

// Navigation button size constants (add these to your constants file)
pub enum NavBarUISizes {
    IconSize,
    HoverRounding,
    ButtonSpacing,
}

impl NavBarUISizes {
    pub fn size(&self) -> f32 {
        match self {
            NavBarUISizes::IconSize => 18.0,
            NavBarUISizes::HoverRounding => 6.0,
            NavBarUISizes::ButtonSpacing => 8.0,
        }
    }
}

// Navigation button color constants (add these to your constants file)
pub enum NavBarUIColors {
    IconTint,
    HoverBackground,
}

impl NavBarUIColors {
    pub const fn color(&self) -> egui::Color32 {
        match self {
            NavBarUIColors::IconTint => egui::Color32::from_rgba_premultiplied(0, 0, 0, 0),
            NavBarUIColors::HoverBackground => {
                egui::Color32::from_rgba_premultiplied(255, 255, 255, 30)
            }
        }
    }
}

// Back button with manual hover detection

pub enum NavBarUIEvents {}

// Include the icon bytes
const HOME_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/home.png");
const FORWARD_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/arrow_forward.png");
const BACKWARD_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/arrow_backward.png");
const REFRESH_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/refresh.png");
const CUT_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/cut.png");
const COPY_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/copy.png");
const PASTE_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/paste.png");
const RDP_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/rdp.png");
const DOWNLOAD_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/downloads.png");
const MENU_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/navbar/menu.png");

// Static storage for the icon - loaded once and reused
static HOME_ICON: OnceLock<TextureHandle> = OnceLock::new();
static FORWARD_ICON: OnceLock<TextureHandle> = OnceLock::new();
static BACKWARD_ICON: OnceLock<TextureHandle> = OnceLock::new();
static REFRESH_ICON: OnceLock<TextureHandle> = OnceLock::new();
static CUT_ICON: OnceLock<TextureHandle> = OnceLock::new();
static COPY_ICON: OnceLock<TextureHandle> = OnceLock::new();
static PASTE_ICON: OnceLock<TextureHandle> = OnceLock::new();
static RDP_ICON: OnceLock<TextureHandle> = OnceLock::new();
static DOWNLOAD_ICON: OnceLock<TextureHandle> = OnceLock::new();
static MENU_ICON: OnceLock<TextureHandle> = OnceLock::new();

impl NavBarUI {
    pub fn custom_icon_button(
        ui: &mut egui::Ui,
        icon: &TextureHandle,
        size: [f32; 2],
    ) -> egui::Response {
        let button_response = ui.add_sized(size, egui::Image::new(icon));
        //Make the buttons light to mactch the current color pallete
        ui.painter().rect_filled(
            button_response.rect,
            4.0,
            egui::Color32::from_rgba_premultiplied(70, 70, 70, 70),
        );
        if button_response.hovered() {
            ui.painter().rect_stroke(
                button_response.rect.expand(1.0),
                4.0,
                egui::Stroke::new(0.5, egui::Color32::from_rgb(100, 100, 100)),
                egui::epaint::StrokeKind::Outside,
            );
        }
        button_response
    }

    fn get_home_icon(ctx: &egui::Context) -> &'static TextureHandle {
        HOME_ICON.get_or_init(|| {
            let image = image::load_from_memory(HOME_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture("home_icon", color_image, egui::TextureOptions::default())
        })
    }

    fn get_forward_icon(ctx: &egui::Context) -> &'static TextureHandle {
        FORWARD_ICON.get_or_init(|| {
            let image = image::load_from_memory(FORWARD_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture("forward_icon", color_image, egui::TextureOptions::default())
        })
    }

    fn get_backward_icon(ctx: &egui::Context) -> &'static TextureHandle {
        BACKWARD_ICON.get_or_init(|| {
            let image = image::load_from_memory(BACKWARD_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture(
                "backward_icon",
                color_image,
                egui::TextureOptions::default(),
            )
        })
    }

    fn get_refresh_icon(ctx: &egui::Context) -> &'static TextureHandle {
        REFRESH_ICON.get_or_init(|| {
            let image = image::load_from_memory(REFRESH_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture("refresh_icon", color_image, egui::TextureOptions::default())
        })
    }
    fn get_cut_icon(ctx: &egui::Context) -> &'static TextureHandle {
        CUT_ICON.get_or_init(|| {
            let image = image::load_from_memory(CUT_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture("cut_icon", color_image, egui::TextureOptions::default())
        })
    }

    fn get_copy_icon(ctx: &egui::Context) -> &'static TextureHandle {
        COPY_ICON.get_or_init(|| {
            let image = image::load_from_memory(COPY_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture("copy_icon", color_image, egui::TextureOptions::default())
        })
    }

    fn get_paste_icon(ctx: &egui::Context) -> &'static TextureHandle {
        PASTE_ICON.get_or_init(|| {
            let image = image::load_from_memory(PASTE_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture("paste_icon", color_image, egui::TextureOptions::default())
        })
    }

    fn get_rdp_icon(ctx: &egui::Context) -> &'static TextureHandle {
        RDP_ICON.get_or_init(|| {
            let image = image::load_from_memory(RDP_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture("rdp_icon", color_image, egui::TextureOptions::default())
        })
    }
    fn get_download_icon(ctx: &egui::Context) -> &'static TextureHandle {
        DOWNLOAD_ICON.get_or_init(|| {
            let image = image::load_from_memory(DOWNLOAD_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture(
                "download_icon",
                color_image,
                egui::TextureOptions::default(),
            )
        })
    }
    fn get_menu_icon(ctx: &egui::Context) -> &'static TextureHandle {
        MENU_ICON.get_or_init(|| {
            let image = image::load_from_memory(MENU_ICON_BYTES).unwrap();
            let rgba_image = image.to_rgba8();
            let size = [rgba_image.width() as usize, rgba_image.height() as usize];
            let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba_image);
            ctx.load_texture("menu_icon", color_image, egui::TextureOptions::default())
        })
    }

    pub fn render_navigation_bar(ui: &mut egui::Ui, state: &C1BState) {
        egui::Frame::none()
            .fill(egui::Color32::WHITE)
            .show(ui, |ui| {
                egui::ScrollArea::horizontal()
                    .auto_shrink([false; 2])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .enable_scrolling(false)
                    .show(ui, |ui| {
                        ui.set_height(50.0);
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space(8.0);
                            let home_icon = Self::get_home_icon(ui.ctx());
                            let home_button = Self::custom_icon_button(ui, home_icon, [20.0, 20.0]);
                            // Divider
                            Self::render_divider(ui);
                            let backward_icon = Self::get_backward_icon(ui.ctx());
                            let backward_button =
                                Self::custom_icon_button(ui, backward_icon, [20.0, 20.0]);
                            ui.add_space(8.0);
                            let forward_icon = Self::get_forward_icon(ui.ctx());
                            let forward_button =
                                Self::custom_icon_button(ui, forward_icon, [20.0, 20.0]);
                            ui.add_space(8.0);
                            let refresh_icon = Self::get_refresh_icon(ui.ctx());
                            let refresh_button =
                                Self::custom_icon_button(ui, refresh_icon, [20.0, 20.0]);
                            ui.add_space(8.0);
                            Self::render_divider(ui);

                            let fixed_elements_width = 50.0;
                            let total_available = ui.available_width();
                            let available_width = (total_available - fixed_elements_width).max(0.0);
                            let min_width = 150.0; // Minimum width for the address bar
                            let max_width = f32::INFINITY; // No maximum limit, let it expand fully
                            let address_bar_width = available_width.clamp(min_width, max_width);

                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(address_bar_width, 50.0),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    egui::Frame::none()
                                        .fill(egui::Color32::from_rgb(237, 242, 250))
                                        .rounding(10.0)
                                        .outer_margin(egui::Margin::same(8))
                                        .show(ui, |ui| {
                                            ui.set_width(address_bar_width - 8.0);
                                            ui.set_height(34.0);
                                            ui.with_layout(
                                                egui::Layout::left_to_right(egui::Align::Center),
                                                |ui| {
                                                    ui.add_space(25.0);
                                                    let copy_icon = Self::get_copy_icon(ui.ctx());
                                                    let refresh_button = Self::custom_icon_button(
                                                        ui,
                                                        copy_icon,
                                                        [16.0, 16.0],
                                                    );
                                                    ui.add_space(15.0);

                                                    let cut_icon = Self::get_cut_icon(ui.ctx());
                                                    let refresh_button = Self::custom_icon_button(
                                                        ui,
                                                        cut_icon,
                                                        [16.0, 16.0],
                                                    );
                                                    ui.add_space(15.0);

                                                    let paste_icon = Self::get_paste_icon(ui.ctx());
                                                    let paste_button = Self::custom_icon_button(
                                                        ui,
                                                        paste_icon,
                                                        [16.0, 16.0],
                                                    );
                                                    ui.add_space(15.0);

                                                    let rdp_icon = Self::get_rdp_icon(ui.ctx());
                                                    let rdp_button = Self::custom_icon_button(
                                                        ui,
                                                        rdp_icon,
                                                        [16.0, 16.0],
                                                    );
                                                    ui.add_space(15.0);
                                                },
                                            );
                                        });
                                },
                            );

                            // let download_icon = Self::get_download_icon(ui.ctx());
                            // let download_button =
                            //     Self::custom_icon_button(ui, download_icon, [20.0, 20.0]);
                            // ui.add(
                            //     egui::Button::new(
                            //         egui::RichText::new("N")
                            //             .strong()
                            //             .color(egui::Color32::WHITE),
                            //     )
                            //     .fill(egui::Color32::from_rgb(1, 87, 155))
                            //     .rounding(20.0)
                            //     .min_size(egui::Vec2::new(20.0, 20.0)),
                            // );
                            let menu_icon = Self::get_menu_icon(ui.ctx());
                            let menu_button = Self::custom_icon_button(ui, menu_icon, [20.0, 20.0]);
                        });
                    });
            });
    }

    pub fn render_divider(ui: &mut egui::Ui) {
        ui.add_space(4.0);
        ui.allocate_ui_with_layout(
            egui::Vec2::new(2.0, 20.0),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                let rect = ui.available_rect_before_wrap();
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(rect.min, egui::Vec2::new(2.0, rect.height())),
                    0.0,
                    egui::Color32::from_rgba_unmultiplied(60, 60, 60, 60),
                );
            },
        );
        ui.add_space(4.0);
    }

    
}
