use std::time::Instant;

use insploray::Vec2;
use imgui::{TextureId};
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowId};

use wgpu::wgt::TextureViewDescriptor;

use crate::ui::app_window::AppWindow;
use crate::ui::Viewport;
use crate::ui::utils::create_texture_from_pixels;

#[derive(Default)]
pub struct App {
    window: Option<AppWindow>,
    // viewport_renderer: RayTracer,

    viewport : Viewport,
}

impl App {
    fn handle_redraw_request(&mut self) 
    {
        let window = self.window.as_mut().unwrap();
        let imgui = window.imgui.as_mut().unwrap();
        // let delta_s = imgui.last_frame.elapsed();
        let now = Instant::now();

        imgui
            .context
            .io_mut()
            .update_delta_time(now - imgui.last_frame);

        imgui.last_frame = now;

        let frame = match window.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("dropped frame: {e:?}");
                return;
            }
        };

        imgui
            .platform
            .prepare_frame(imgui.context.io_mut(), &window.window)
            .expect("Failed to prepare frame");

        let ui = imgui.context.frame();
        let mut frame_id: Option<TextureId> = None;

        // create windows
        {
            // write docking mechanism
            ui.dockspace_over_main_viewport();

            let mut viewport_size: [f32; 2] = [0.0, 0.0];

            let style_guard = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));

            ui.window("Viewport")
                .size([500.0, 500.0], imgui::Condition::FirstUseEver)
                .position([0.0, 0.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    viewport_size = ui.content_region_avail();
                    let width = viewport_size[0] as u32;
                    let height = viewport_size[1] as u32;

                    if ui.is_window_focused() {

                        // let camera = self.viewport_renderer.active_camera;
                        self.viewport.handle_input(
                            ui,
                            width,
                            height,
                        );
                    }

                    self.viewport.set_dimensions(width, height);
                    self.viewport
                        .prepare_buffer();

                    let [c_w, c_h] = self.viewport.renderer.get_current_size();

                    let dimensions = self.viewport.renderer.get_current_size();
                    let texture_id = create_texture_from_pixels(
                        self.viewport.get_buffer(),
                        dimensions,
                        &window.device,
                        &window.queue,
                        &mut imgui.renderer,
                    );

                    imgui::Image::new(texture_id, [c_w as f32, c_h as f32])
                        .uv0(Vec2::new(0.0, 1.0))
                        .uv1(Vec2::new(1.0, 0.0))
                        .build(ui);

                    frame_id = Some(texture_id);
                });

            drop(style_guard);

            ui.window("Settings")
                .size([300.0, 200.0], imgui::Condition::FirstUseEver)
                .position([500.0, 200.0], imgui::Condition::FirstUseEver)
                .build(|| {
                    let duration = self.viewport.renderer.get_last_render_time();

                    let mut updated = false;
                    ui.text(format!("Last Render : {duration:?}"));
                    ui.text(format!("Image : {viewport_size:?}"));
                    ui.button("Render").then(|| {
                        updated |= true;
                    });

                    let camera = self.viewport.camera.read().unwrap();
                    let mut focal_length = camera.focal_length;
                    let mut sensor_size = camera.sensor_size;
                    drop(camera);

                    if imgui::Drag::new("Focal Length")
                        .build(ui, &mut focal_length) && focal_length > 0.0  {
                        self.viewport.camera.write().unwrap()
                            .set_focal_length(focal_length);
                        updated |= true;
                    }

                    if imgui::Drag::new("Sensor Size")
                        .build(ui, &mut sensor_size) && sensor_size > 0.0 {
                        self.viewport.camera.write().unwrap()
                            .set_sensor_size(sensor_size);
                        updated |= true;
                    }
                    
                    if updated {
                        self.viewport.renderer
                            .update(
                                viewport_size[0] as u32,
                                viewport_size[1] as u32,
                            );
                    }
                });
            
            self.viewport.draw_scene_setting_window(ui, &viewport_size);
        }

        let mut encoder = window
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        if imgui.last_cursor != ui.mouse_cursor() {
            imgui.last_cursor = ui.mouse_cursor();
            imgui.platform.prepare_render(ui, &window.window);
        }

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(imgui.clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        imgui
            .renderer
            .render(
                imgui.context.render(),
                &window.queue,
                &window.device,
                &mut rpass,
            )
            .expect("Rendering Failed!...");

        drop(rpass);

        imgui.renderer.textures.remove(frame_id.unwrap());

        window.queue.submit(Some(encoder.finish()));
        frame.present();
    }

}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window = Some(AppWindow::new(event_loop))
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {

        match event {
            WindowEvent::Resized(size) => {
                let window = self.window.as_mut().unwrap();
                window.surface_desc.width = size.width;
                window.surface_desc.height = size.height;

                window
                    .surface
                    .configure(&window.device, &window.surface_desc);
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => self.handle_redraw_request(),
            _ => (),
        }

        let window = self.window.as_mut().unwrap();
        let imgui = window.imgui.as_mut().unwrap();

        imgui.platform.handle_event::<()>(
            imgui.context.io_mut(),
            &window.window,
            &Event::WindowEvent { window_id, event },
        );
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: ()) {
        let window = self.window.as_mut().unwrap();
        let imgui = window.imgui.as_mut().unwrap();
        imgui.platform.handle_event::<()>(
            imgui.context.io_mut(),
            &window.window,
            &Event::UserEvent(event),
        );
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        let window = self.window.as_mut().unwrap();
        let imgui = window.imgui.as_mut().unwrap();
        imgui.platform.handle_event::<()>(
            imgui.context.io_mut(),
            &window.window,
            &Event::DeviceEvent { device_id, event },
        );
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let window = self.window.as_mut().unwrap();
        let imgui = window.imgui.as_mut().unwrap();

        window.window.request_redraw();
        imgui.platform.handle_event::<()>(
            imgui.context.io_mut(),
            &window.window,
            &Event::AboutToWait
        );
    }
}
