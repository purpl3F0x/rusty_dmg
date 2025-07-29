use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::mpsc::Sender;

use egui::ColorImage;
use egui::Key;
use egui::TextureOptions;

include!("shader_widget.rs");
// use Custom3d;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    scale_factor: f32,
    running: bool,

    #[serde(skip)]
    frame_ready: Arc<(Mutex<bool>, Condvar)>,

    #[serde(skip)]
    keypad_channel_sender: MaybeUninit<Sender<(u8, u8)>>,

    #[serde(skip)]
    screen_window: FrameWindow,
    #[serde(skip)]
    background_window: FrameWindow,
    #[serde(skip)]
    oam_window: FrameWindow,
}

impl Default for App {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            running: true,

            frame_ready: Arc::default(),

            keypad_channel_sender: MaybeUninit::uninit(),

            screen_window: FrameWindow::new(
                egui::Id::new("gameboy_frame"),
                "GameBoy".to_string(),
                (160, 144),
                Arc::default(),
            ),

            background_window: FrameWindow::new(
                egui::Id::new("background_frame"),
                "Background".to_string(),
                (256, 256),
                Arc::default(),
            ),

            oam_window: FrameWindow::new(
                egui::Id::new("oam_frame"),
                "OAM".to_string(),
                (10 * 5, 10 * 8),
                Arc::default(),
            ),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        frame_ready: Arc<(Mutex<bool>, Condvar)>,
        screen_buffer: Arc<Mutex<ColorImage>>,
        background_buffer: Arc<Mutex<ColorImage>>,
        sprites_buffer: Arc<Mutex<ColorImage>>,
        keypad_channel_sender: Sender<(u8, u8)>,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        let mut app: App = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        app.frame_ready = frame_ready;
        app.screen_window.image = screen_buffer;
        app.background_window.image = background_buffer;
        app.oam_window.image = sprites_buffer;
        app.oam_window.scale_factor = 3.0;
        app.keypad_channel_sender = MaybeUninit::new(keypad_channel_sender);

        app.screen_window.create_texture(&cc.egui_ctx);
        app.background_window.create_texture(&cc.egui_ctx);
        app.oam_window.create_texture(&cc.egui_ctx);

        app
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Sync with emulator thread
        // let (lock, cvar) = &*self.frame_ready;
        // let mut is_frame_ready = lock.lock().unwrap();
        // is_frame_ready = cvar
        //     .wait_while(is_frame_ready, |is_frame_ready| !*is_frame_ready)
        //     .unwrap();
        // *is_frame_ready = false;
        // drop(is_frame_ready);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {});

        self.screen_window.scale_factor = self.scale_factor;
        self.screen_window.show(ctx);
        self.background_window.show(ctx);
        self.oam_window.show(ctx);

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                // ui.collapsing("Log", |ui| {
                //     ui.add(egui::Label::new(
                //         "Log output will appear here...\n\n\nHi...",
                //     ));
                // });
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        let time = ctx.input(|input| input.stable_dt);
                        ui.label(format!("FPS: {:3.0}", (1.0 / time).round()));

                        ui.add(
                            egui::Slider::new(&mut self.scale_factor, 1.0..=8.0)
                                .text("Scale Factor"),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        egui::widgets::global_theme_preference_buttons(ui);
                        egui::warn_if_debug_build(ui);
                    });
                });
            });
        });

        ctx.input(|input| {
            let input = (
                (input.key_down(Key::A) as u8
                    | (input.key_down(Key::S) as u8) << 1
                    | (input.key_down(Key::Space) as u8) << 2
                    | (input.key_down(Key::Enter) as u8) << 3),
                (input.key_down(Key::ArrowRight) as u8
                    | (input.key_down(Key::ArrowLeft) as u8) << 1
                    | (input.key_down(Key::ArrowUp) as u8) << 2
                    | (input.key_down(Key::ArrowDown) as u8) << 3),
            );

            unsafe {
                let sender = self.keypad_channel_sender.assume_init_mut();
                _ = sender.send(input);
            }
        });

        ctx.request_repaint();
    }
}

struct FrameWindow {
    pub id: egui::Id,
    pub name: String,
    pub size: (usize, usize),
    pub scale_factor: f32,
    pub image: Arc<Mutex<ColorImage>>,
    buffer_as_texture: Option<egui::TextureHandle>,
}

impl FrameWindow {
    fn new(
        id: egui::Id,
        name: String,
        size: (usize, usize),
        image: Arc<Mutex<ColorImage>>,
    ) -> Self {
        Self {
            id,
            name,
            size,
            scale_factor: 1.0,
            image,
            buffer_as_texture: None,
        }
    }

    pub fn create_texture(&mut self, ctx: &egui::Context) {
        let image = self.image.lock().unwrap();
        self.buffer_as_texture = Some(ctx.load_texture(
            self.name.clone(),
            image.clone(),
            TextureOptions {
                magnification: egui::TextureFilter::Nearest,
                minification: egui::TextureFilter::Linear,
                ..Default::default()
            },
        ));
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.name.clone())
            .default_pos([20.0, 20.0])
            .resizable(false)
            .show(ctx, |ui| {
                let buffer_as_texture_mut = self.buffer_as_texture.as_mut().unwrap();
                buffer_as_texture_mut.set(
                    self.image.lock().unwrap().clone(),
                    TextureOptions {
                        magnification: egui::TextureFilter::Nearest,
                        minification: egui::TextureFilter::Linear,
                        ..Default::default()
                    },
                );

                let img = egui::Image::from_texture(self.buffer_as_texture.as_ref().unwrap())
                    .fit_to_original_size(self.scale_factor as f32);

                ui.add(img);
            });
    }
}
