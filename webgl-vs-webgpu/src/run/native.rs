use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};

impl crate::App {
    pub fn run() {
        env_logger::init();

        let (event_loop, instance) = pollster::block_on(Self::create_action_instance());
        Self::start_event_loop(event_loop, instance);
    }

    async fn create_action_instance() -> (EventLoop<()>, Self) {
        let event_loop = EventLoopBuilder::new().build();

        let window = WindowBuilder::new()
            .with_title("WebGL VS WebGPU")
            .build(&event_loop)
            .unwrap();

        // 计算一个默认显示高度
        let height = (750.0 * window.scale_factor()) as u32;
        let width = (height as f32 * 1.6) as u32;
        window.set_inner_size(PhysicalSize::new(width, height));

        let app = app_surface::AppSurface::new(window).await;
        let instance = Self::new(app, &event_loop).await;

        let adapter_info = instance.get_adapter_info();
        let gpu_info = format!(
            "正在使用 {}, 后端图形接口为 {:?}。",
            adapter_info.name, adapter_info.backend
        );
        println!("{gpu_info}");

        (event_loop, instance)
    }

    fn start_event_loop(event_loop: EventLoop<()>, instance: Self) {
        let mut app = instance;

        fn resize_app_surface(app: &mut crate::App, physical_size: &PhysicalSize<u32>) {
            app.resize(physical_size);
        }

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { ref event, .. } => {
                    app.egui_layer.on_ui_event(event);

                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            if physical_size.width == 0 || physical_size.height == 0 {
                                // 处理最小化窗口的事件
                            } else {
                                app.resize(physical_size);
                            }
                        }
                        WindowEvent::ScaleFactorChanged {
                            scale_factor: _,
                            new_inner_size,
                        } => {
                            app.resize(new_inner_size);
                        }
                        _ => {}
                    }
                }
                Event::RedrawRequested(window_id) if window_id == app.current_window_id() => {
                    app.render();
                }
                Event::RedrawEventsCleared => {
                    // 除非手动请求，RedrawRequested 将只会触发一次。
                    app.request_redraw();
                }
                _ => {}
            }
        });
    }
}
