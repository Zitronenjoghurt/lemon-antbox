use egui_wgpu::{wgpu, ScreenDescriptor};
use egui_winit::EventResponse;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct Gfx {
    window: Arc<Window>,
    pixels: Pixels<'static>,
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    screen_descriptor: ScreenDescriptor,
    textures: egui::TexturesDelta,
    paint_jobs: Vec<egui::ClippedPrimitive>,
}

impl Gfx {
    pub fn new(window: Arc<Window>, width: u32, height: u32) -> Self {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;

        let surface = SurfaceTexture::new(size.width, size.height, window.clone());
        let pixels = Pixels::new(width, height, surface).unwrap();

        let egui_ctx = egui::Context::default();
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(scale_factor),
            Some(max_texture_size),
        );

        let egui_renderer =
            egui_wgpu::Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: scale_factor,
        };

        Self {
            window,
            pixels,
            egui_ctx,
            egui_state,
            egui_renderer,
            screen_descriptor,
            textures: egui::TexturesDelta::default(),
            paint_jobs: Vec::new(),
        }
    }

    pub fn window_pos_to_pixel(&self, pos: (f32, f32)) -> Option<(isize, isize)> {
        self.pixels
            .window_pos_to_pixel(pos)
            .ok()
            .map(|(x, y)| (x as isize, y as isize))
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height).unwrap();
        self.screen_descriptor.size_in_pixels = [width, height];
    }

    pub fn prepare(&mut self, ui_fn: impl FnOnce(&egui::Context)) {
        let raw_input = self.egui_state.take_egui_input(&self.window);
        let output = self.egui_ctx.run(raw_input, ui_fn);

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(&self.window, output.platform_output);
        self.paint_jobs = self
            .egui_ctx
            .tessellate(output.shapes, self.screen_descriptor.pixels_per_point);
    }

    pub fn render(&mut self) {
        let paint_jobs = &self.paint_jobs;
        let screen = &self.screen_descriptor;
        let textures = &self.textures;
        let renderer = &mut self.egui_renderer;

        self.pixels
            .render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target);

                for (id, delta) in &textures.set {
                    renderer.update_texture(&context.device, &context.queue, *id, delta);
                }

                renderer.update_buffers(
                    &context.device,
                    &context.queue,
                    encoder,
                    paint_jobs,
                    screen,
                );

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("egui"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: render_target,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    renderer.render(&mut rpass, paint_jobs, screen);
                }

                Ok(())
            })
            .unwrap();

        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.egui_renderer.free_texture(id);
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn pixels_frame(&mut self) -> &mut [u8] {
        self.pixels.frame_mut()
    }

    pub fn set_pixels_per_point(&mut self, pixels_per_point: f32) {
        self.screen_descriptor.pixels_per_point = pixels_per_point;
    }

    pub fn on_egui_window_event(&mut self, window_event: &WindowEvent) -> EventResponse {
        self.egui_state.on_window_event(&self.window, window_event)
    }
}
