use egui::Color32;

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{sync::Arc, vec};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    scale_factor: f32,
    running: bool,

    #[serde(skip)]
    screen_buffer: Arc<Mutex<Vec<u8>>>,
    #[serde(skip)]
    background_buffer: Arc<Mutex<Vec<u8>>>,
    #[serde(skip)]
    sprites_buffer: Arc<Mutex<Vec<u8>>>,

    #[serde(skip)]
    frame_ready: Arc<AtomicBool>,

    #[serde(skip)]
    screen_window: FrameWindow,
    #[serde(skip)]
    background_window: FrameWindow,
    #[serde(skip)]
    oam_window: FrameWindow,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            running: true,

            screen_buffer: Arc::new(Mutex::new(vec![])),
            background_buffer: Arc::new(Mutex::new(vec![])),
            sprites_buffer: Arc::new(Mutex::new(vec![])),

            frame_ready: Arc::new(AtomicBool::new(false)),

            screen_window: FrameWindow::new(
                egui::Id::new("gameboy_frame"),
                "GameBoy".to_string(),
                (160, 144),
            ),

            background_window: FrameWindow::new(
                egui::Id::new("background_frame"),
                "Background".to_string(),
                (256, 256),
            ),

            oam_window: FrameWindow::new(egui::Id::new("oam_frame"), "OAM".to_string(), (10 * 5, 10 * 8)),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        frame_ready: Arc<AtomicBool>,
        screen_buffer: Arc<Mutex<Vec<u8>>>,
        background_buffer: Arc<Mutex<Vec<u8>>>,
        sprites_buffer: Arc<Mutex<Vec<u8>>>,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut app: TemplateApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        app.frame_ready = frame_ready;
        app.screen_buffer = screen_buffer;
        app.background_buffer = background_buffer;
        app.sprites_buffer = sprites_buffer;
        app
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.screen_window.scale_factor = self.scale_factor;
            self.screen_window
                .show(ctx, &self.screen_buffer.lock().unwrap());
            self.background_window
                .show(ctx, &self.background_buffer.lock().unwrap());

            self.oam_window
                .show(ctx, &self.sprites_buffer.lock().unwrap());
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    let time = ctx.input(|input| input.stable_dt);
                    ui.label(format!("FPS: {:3.0}", 1.0 / time));

                    ui.add(
                        egui::Slider::new(&mut self.scale_factor, 1.0..=8.0).text("Scale Factor"),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        if self.frame_ready.load(Ordering::Relaxed) {
            self.frame_ready.store(false, Ordering::Relaxed);
        }

        ctx.request_repaint();
    }
}

struct FrameWindow {
    pub id: egui::Id,
    pub name: String,
    pub size: (usize, usize),
    pub scale_factor: f32,
}

impl FrameWindow {
    fn new(id: egui::Id, name: String, size: (usize, usize)) -> Self {
        Self {
            id,
            name,
            size,
            scale_factor: 1.0,
        }
    }

    pub fn show(&self, ctx: &egui::Context, buffer: &Vec<u8>) {
        egui::Window::new(self.name.clone())
            .default_pos([20.0, 20.0])
            .resizable(false)
            .show(ctx, |ui| {
                let buffer_as_image = egui::ColorImage::from_rgba_unmultiplied(
                    [self.size.0, self.size.1],
                    buffer.as_ref(),
                );
                let buffer_as_texture =
                    ctx.load_texture("frame", buffer_as_image, egui::TextureOptions::default());

                let img = egui::Image::from_texture(&buffer_as_texture)
                    .fit_to_original_size(self.scale_factor as f32);
                ui.add(img);
            });
    }
}
