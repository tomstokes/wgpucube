use egui_wgpu::ScreenDescriptor;
use std::fmt;
use std::sync::Arc;

pub(crate) struct EguiInterface {
    state: egui_winit::State,
    renderer: egui_wgpu::Renderer,
}

impl fmt::Debug for EguiInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EguiInterface")
            .field("state", &"<egui_winit::State>")
            .field("renderer", &"<egui_wgpu::Renderer>")
            .finish()
    }
}

impl EguiInterface {
    pub fn new(
        device: &wgpu::Device,
        window: &winit::window::Window,
        output_color_format: wgpu::TextureFormat,
    ) -> Self {
        let context = egui::Context::default();
        let state = egui_winit::State::new(
            context,
            egui::ViewportId::default(), // Default (ROOT) viewport
            window,
            None, // TODO: Native pixels per point (window.scale_factor() ?)
            None, // Default theme
            None, // TODO: Is default max texture size sufficient?
        );
        let render_options = egui_wgpu::RendererOptions {
            msaa_samples: 0,
            depth_stencil_format: None,
            dithering: false,
            predictable_texture_filtering: false,
        };
        let renderer = egui_wgpu::Renderer::new(device, output_color_format, render_options);

        Self { state, renderer }
    }

    pub(crate) fn handle_input(
        &mut self,
        window: &winit::window::Window,
        event: &winit::event::WindowEvent,
    ) {
        // TODO: Use the EventResponse
        let _ = self.state.on_window_event(window, event);
    }

    pub fn render(
        &mut self,
        window: &Arc<winit::window::Window>,
        view: &wgpu::TextureView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        frame_stats: &crate::frame_counter::FrameStatistics,
    ) {
        // Extract accumulated input from the window
        let input = self.state.take_egui_input(window);

        // Run the egui code for one frame
        let full_output = self
            .state
            .egui_ctx()
            .run(input, |_ui| self.draw_window(frame_stats));

        // Handle any platform output from egui such as updating cursor or IME
        self.state
            .handle_platform_output(window, full_output.platform_output);

        // Tessellate egui shapes from the computed frame into triangles for rendering
        let triangles = self
            .state
            .egui_ctx()
            .tessellate(full_output.shapes, self.state.egui_ctx().pixels_per_point());

        // Handle any requested texture updates from the computed frame
        for (texture_id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *texture_id, image_delta);
        }

        // Upload uniform, vertex, and index buffers to the GPU
        let window_size = window.inner_size();
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [window_size.width, window_size.height],
            pixels_per_point: window.scale_factor() as f32, // TODO: * scale_factor ?
        };
        self.renderer
            .update_buffers(device, queue, encoder, &triangles, &screen_descriptor);

        // Render pass for the egui output
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
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

        // Render the computed egui output
        self.renderer.render(
            &mut render_pass.forget_lifetime(),
            &triangles,
            &screen_descriptor,
        );

        // Free any textures that egui marks to be freed
        for texture_id in full_output.textures_delta.free {
            self.renderer.free_texture(&texture_id);
        }
    }

    fn draw_window(&self, frame_stats: &crate::frame_counter::FrameStatistics) {
        egui::Window::new("Rendering Statistics")
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .show(self.state.egui_ctx(), |ui| {
                let ui_builder = egui::UiBuilder::new();
                ui.scope_builder(ui_builder, |ui| {
                    egui::Grid::new("stats_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Frame Rate");
                            ui.label(format!("{:.2} fps", frame_stats.frames_per_second));
                            ui.end_row();

                            ui.label("Frame Time");
                            ui.label(format!("{:.2} ms", frame_stats.average_frame_time_ms));
                            ui.end_row();
                        });
                });
            });
    }
}
