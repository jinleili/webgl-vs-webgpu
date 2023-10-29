use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::WindowBuilder,
};

use super::CustomJsTriggerEvent;
use wasm_bindgen::{prelude::*, JsCast};

const CANVAS_SIZE_NEED_CHANGE: &str = "canvas_size_need_change";
#[allow(unused)]
const ALL_CUSTOM_EVENTS: [&str; 1] = [CANVAS_SIZE_NEED_CHANGE];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = Function, js_name = "prototype.call.call")]
    fn call_catch(this: &JsValue) -> Result<(), JsValue>;
    fn canvas_resize_completed();
}

fn try_call_canvas_resize_completed() {
    let run_closure = Closure::once_into_js(canvas_resize_completed);
    if call_catch(&run_closure).is_err() {
        log::error!("js 端没有定义 canvas_resize_completed 函数");
    }
}

impl crate::App {
    pub fn run() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("无法初始化日志库");
        wasm_bindgen_futures::spawn_local(async move {
            let (event_loop, instance) = Self::create_action_instance().await;
            let run_closure =
                Closure::once_into_js(move || Self::start_event_loop(event_loop, instance));

            // 处理运行过程中抛出的 JS 异常。
            // 否则 wasm_bindgen_futures 队列将中断，且不再处理任何任务。
            if let Err(error) = call_catch(&run_closure) {
                let is_control_flow_exception =
                    error.dyn_ref::<js_sys::Error>().map_or(false, |e| {
                        e.message().includes("Using exceptions for control flow", 0)
                    });

                if !is_control_flow_exception {
                    web_sys::console::error_1(&error);
                }
            }
        });
    }

    fn get_container_id() -> &'static str {
        let container_id = if cfg!(feature = "webgl") {
            "webgl-container"
        } else {
            "webgpu-container"
        };
        container_id
    }

    async fn create_action_instance() -> (EventLoop<CustomJsTriggerEvent>, Self) {
        let event_loop = EventLoopBuilder::<CustomJsTriggerEvent>::with_user_event().build();
        let proxy = event_loop.create_proxy();

        let window = WindowBuilder::new()
            .with_title("WebGL VS WebGPU")
            .build(&event_loop)
            .unwrap();

        // 计算一个默认显示高度
        let height = (700.0 * window.scale_factor()) as u32;
        let width = (height as f32 * 1.6) as u32;

        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .map(|doc| {
                let canvas = window.canvas();

                if let Some(container) = doc.get_element_by_id(Self::get_container_id()) {
                    let rect = container.get_bounding_client_rect();
                    let scale_factor = window.scale_factor();
                    let w = rect.width() * scale_factor;
                    let h = rect.height() * scale_factor;
                    window.set_inner_size(PhysicalSize::new(w, h));
                    canvas.style().set_css_text(
                        &(canvas.style().css_text()
                            + "background-color: black; display: block; margin: 0px;"),
                    );
                    let _ = container.append_child(&web_sys::Element::from(canvas));

                    let target: web_sys::EventTarget = container.into();
                    let call_back = Closure::wrap(Box::new(move |event: web_sys::Event| {
                        // let event_name: &'static str = event.type_().as_str();
                        let event_name: &'static str = Box::leak(event.type_().into_boxed_str());
                        let _ = proxy.send_event(CustomJsTriggerEvent {
                            ty: event_name,
                            _value: String::new(),
                        });
                    }) as Box<dyn FnMut(_)>);

                    // Add html element's event listener
                    for e in ALL_CUSTOM_EVENTS.iter() {
                        target
                            .add_event_listener_with_callback(e, call_back.as_ref().unchecked_ref())
                            .unwrap();
                    }
                    call_back.forget();
                } else {
                    window.set_inner_size(PhysicalSize::new(width, height));
                    let canvas = window.canvas();
                    canvas.style().set_css_text(
                        &(canvas.style().css_text()
                            + "background-color: black; display: block; margin: 20px auto;"),
                    );
                    doc.body()
                        .map(|body| body.append_child(&web_sys::Element::from(canvas)));
                }
            })
            .expect("Couldn't append canvas to document body.");

        let app = app_surface::AppSurface::new(window).await;
        let instance = Self::new(app, &event_loop).await;

        let adapter_info = instance.get_adapter_info();
        let gpu_info = format!(
            "正在使用 {}, 后端图形接口为 {:?}。",
            adapter_info.name, adapter_info.backend
        );
        log::info!("{gpu_info}");

        (event_loop, instance)
    }

    fn start_event_loop(event_loop: EventLoop<CustomJsTriggerEvent>, instance: Self) {
        let mut app = instance;
        // 一帧内的 cpu 耗时
        let mut cpu_usage: f64 = 0.0;
        // 上一帧开始到当前帧开始，时间间隔
        let mut frame_usage: f64 = 0.0;
        let mut last_frame_start_time: f64 = now_m_sec();

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
                                app.resize(&physical_size);
                                try_call_canvas_resize_completed();
                            }
                        }
                        WindowEvent::ScaleFactorChanged {
                            scale_factor: _,
                            new_inner_size,
                        } => {
                            app.resize(&new_inner_size);
                            try_call_canvas_resize_completed();
                        }
                        _ => {}
                    }
                }
                Event::UserEvent(event) => {
                    if event.ty == CANVAS_SIZE_NEED_CHANGE {
                        if let Some(doc) = web_sys::window().and_then(|win| win.document()) {
                            if let Some(container) = doc.get_element_by_id(Self::get_container_id())
                            {
                                let window = app.get_view_mut();
                                let rect = container.get_bounding_client_rect();
                                let scale_factor = window.scale_factor();
                                let w = rect.width() * scale_factor;
                                let h = rect.height() * scale_factor;
                                window.set_inner_size(PhysicalSize::new(w, h));
                            }
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == app.current_window_id() => {
                    let frame_start = now_m_sec();
                    frame_usage = frame_start - last_frame_start_time;
                    app.render();
                    cpu_usage = now_m_sec() - frame_start;
                    last_frame_start_time = frame_start;
                    app.egui_layer
                        .on_new_frame(now_m_sec(), frame_usage as f32, cpu_usage as f32);
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

/// 网页当前时间, 毫秒
pub fn now_m_sec() -> f64 {
    web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance")
        .now()
        / 1000.
}
