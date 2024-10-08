use std::future::Future;
use std::sync::Arc;
use wgpu::*;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::settings::window::WINDOW_SETTINGS;

pub(crate) struct State<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
    pub window: Arc<Window>,
}

impl <'a> State<'a> {
    pub(crate) async fn new(window: Arc<Window>) -> impl Future<Output = State<'a>> {
        let size = window.inner_size();

        let instance_descriptor = InstanceDescriptor {
            backends: Backends::all(), ..Default::default()
        };
        let instance = Instance::new(instance_descriptor);
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let adapter_descriptor = RequestAdapterOptionsBase {
            power_preference: WINDOW_SETTINGS.power_preference,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let adapter = instance.request_adapter(&adapter_descriptor).await.unwrap();

        let device_descriptor = DeviceDescriptor {
            required_features: Features::empty(),
            required_limits: Limits::default(),
            label: Some("Device"),
            memory_hints: Default::default(),
        };

        let (device, queue) = adapter.request_device(&device_descriptor, None).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats.iter().copied().filter(|f| f.is_srgb()).next()
            .unwrap_or(surface_capabilities.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: if surface_capabilities.present_modes.contains(&WINDOW_SETTINGS.present_mode)
            { WINDOW_SETTINGS.present_mode } else { surface_capabilities.present_modes[0] },
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: WINDOW_SETTINGS.desired_max_buffer,
        };

        surface.configure(&device, &config);

        async move {
            Self {
                window,
                surface,
                device,
                queue,
                config,
                size
            }
        }
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config)
        }
    }

    pub(crate) fn render(&mut self) -> Result<(), SurfaceError> {
        let drawable = self.surface.get_current_texture()?;
        let image_view_descriptor = TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_descriptor);

        let command_encoder_descriptor = CommandEncoderDescriptor {
            label: Some("Render Encoder")
        };
        let mut command_encoder = self.device.create_command_encoder(&command_encoder_descriptor);

        let color_attachment = RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color{
                    r: 0.75,
                    g: 0.5,
                    b: 0.25,
                    a: 1.0
                }),
                store: StoreOp::Store
            }
        };

        let render_pass_descriptor = RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None
        };

        command_encoder.begin_render_pass(&render_pass_descriptor);
        self.queue.submit(std::iter::once(command_encoder.finish()));

        drawable.present();

        Ok(())
    }
}