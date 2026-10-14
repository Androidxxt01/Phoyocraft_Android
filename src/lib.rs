//! Prova de conceito: janela Android com wgpu (Vulkan).
//! Toque na tela para mudar a cor do "canvas".
//! Proximo passo: depender dos crates do PhotoCraft e renderizar um PSD aqui.

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use photocraft_psd as _;

struct Gfx {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl Gfx {
    fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).expect("surface");
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .expect("adapter");
        log::info!("GPU: {:?}", adapter.get_info());
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_limits: wgpu::Limits::downlevel_defaults()
                    .using_resolution(adapter.limits()),
                ..Default::default()
            },
            None,
        ))
        .expect("device");
        let size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .expect("config");
        surface.configure(&device, &config);
        Self { window, surface, device, queue, config }
    }

    fn resize(&mut self, w: u32, h: u32) {
        if w > 0 && h > 0 {
            self.config.width = w;
            self.config.height = h;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn draw(&self, color: wgpu::Color) {
        let frame = match self.surface.get_current_texture() {
            Ok(f) => f,
            Err(_) => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
        };
        let view = frame.texture.create_view(&Default::default());
        let mut enc = self.device.create_command_encoder(&Default::default());
        {
            let _pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }
        self.queue.submit(Some(enc.finish()));
        self.window.pre_present_notify();
        frame.present();
    }
}

struct App {
    gfx: Option<Gfx>,
    color: wgpu::Color,
}

impl ApplicationHandler for App {
    // No Android a superficie so existe entre resumed e suspended.
    fn resumed(&mut self, el: &ActiveEventLoop) {
        let window = Arc::new(el.create_window(Window::default_attributes()).expect("window"));
        self.gfx = Some(Gfx::new(window));
        if let Some(g) = &self.gfx {
            g.window.request_redraw();
        }
    }

    fn suspended(&mut self, _el: &ActiveEventLoop) {
        self.gfx = None;
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(s) => {
                if let Some(g) = &mut self.gfx {
                    g.resize(s.width, s.height);
                    g.window.request_redraw();
                }
            }
            WindowEvent::Touch(t) => {
                if let Some(g) = &self.gfx {
                    let s = g.window.inner_size();
                    self.color = wgpu::Color {
                        r: (t.location.x / s.width.max(1) as f64).clamp(0.0, 1.0),
                        g: (t.location.y / s.height.max(1) as f64).clamp(0.0, 1.0),
                        b: 0.6,
                        a: 1.0,
                    };
                    g.window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(g) = &self.gfx {
                    g.draw(self.color);
                }
            }
            _ => {}
        }
    }
}

fn run(event_loop: EventLoop<()>) {
    let mut app = App {
        gfx: None,
        color: wgpu::Color { r: 0.1, g: 0.2, b: 0.4, a: 1.0 },
    };
    event_loop.run_app(&mut app).unwrap();
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    let event_loop = EventLoop::builder().with_android_app(app).build().unwrap();
    run(event_loop);
}
