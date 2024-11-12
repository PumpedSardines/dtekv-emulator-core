use super::{unsafe_cpu::UnsafeCpu, website_source_code, Cors, CpuEvent, GuiEvent, ResponseEvent};
use crate::cpu::Cpu;
use crate::exception;
use image::ImageFormat;
use std::io::Cursor;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{
    http::{header::CONTENT_TYPE, Request},
    WebViewBuilder,
};

pub fn start(cpu: Cpu) {
    println!("Starting emulator");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("DTEK-V Emulator")
        .with_inner_size(LogicalSize::new(1200.0, 800.0))
        .with_min_inner_size(LogicalSize::new(1000.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let (cpu_tx, cpu_rx): (Sender<CpuEvent>, Receiver<CpuEvent>) = mpsc::channel();
    let (gui_tx, gui_rx): (Sender<GuiEvent>, Receiver<GuiEvent>) = mpsc::channel();

    let unsafe_cpu = UnsafeCpu::new(cpu);
    let send_cpu = Arc::new(unsafe_cpu);

    start_cpu_thread(Arc::clone(&send_cpu), cpu_tx, gui_rx);

    let web_view_cpu = Arc::clone(&send_cpu);
    let builder = WebViewBuilder::new()
        .with_devtools(true)
        .with_custom_protocol("wry".into(), move |_webview_id, request| {
            let cpu = unsafe { (*web_view_cpu).get() };

            match get_wry_response(request) {
                Ok(ResponseEvent::Response(r)) => r.map(Into::into),
                Ok(ResponseEvent::GuiEvent(GuiEvent::VgaUpdate)) => {
                    let mut buffer = Vec::new();
                    let img = cpu.bus.vga.to_rbg_image();
                    img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
                        .unwrap();

                    http::Response::builder()
                        .header(CONTENT_TYPE, "image/png")
                        .cors()
                        .body(buffer)
                        .unwrap()
                        .map(Into::into)
                }
                Ok(ResponseEvent::GuiEvent(g)) => {
                    gui_tx.send(g).unwrap();

                    http::Response::builder()
                        .header(CONTENT_TYPE, "text/plain")
                        .cors()
                        .body(b"OK".to_vec())
                        .unwrap()
                        .map(Into::into)
                }
                Err(e) => http::Response::builder()
                    .header(CONTENT_TYPE, "text/plain")
                    .status(500)
                    .body(e.to_string().as_bytes().to_vec())
                    .unwrap()
                    .map(Into::into),
            }
        })
        .with_url("wry://localhost");

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let webview = builder.build(&window).unwrap();
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        for result in cpu_rx.try_iter() {
            match result {
                CpuEvent::Uart(c) => webview
                    .evaluate_script(&format!("window.__dtekv__.uartWrite(`{}`)", c))
                    .unwrap(),
                CpuEvent::HexDisplays(a, b, c, d, e, f) => webview
                    .evaluate_script(&format!(
                        "window.__dtekv__.updateHexDisplays([{}, {}, {}, {}, {}, {}])",
                        a, b, c, d, e, f
                    ))
                    .unwrap(),
                CpuEvent::VgaUpdate => webview
                    .evaluate_script("window.__dtekv__.updateVga()")
                    .unwrap(),
            }
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}

fn get_wry_response(
    request: Request<Vec<u8>>,
) -> Result<ResponseEvent, Box<dyn std::error::Error>> {
    let path = request.uri().path();

    match path {
        "/" => {
            let content = website_source_code::INDEX_HTML.bytes().collect::<Vec<u8>>();
            let response = http::Response::builder()
                .header(CONTENT_TYPE, "text/html")
                .cors()
                .body(content)?;
            return Ok(ResponseEvent::Response(response));
        }
        "/gui/events/ready" => Ok(ResponseEvent::GuiEvent(GuiEvent::Ready)),
        "/gui/events/button/pressed" => Ok(ResponseEvent::GuiEvent(GuiEvent::ButtonPressed)),
        "/gui/events/button/released" => Ok(ResponseEvent::GuiEvent(GuiEvent::ButtonReleased)),
        "/gui/events/vga/update" => Ok(ResponseEvent::GuiEvent(GuiEvent::VgaUpdate)),
        "/gui/events/switch/toggle" => {
            let uri = request.uri().to_string();
            let url = url::Url::parse(&uri).map_err(|_| "Failed to parse URL")?;
            let query = url
                .query_pairs()
                .collect::<std::collections::HashMap<_, _>>();

            let index = query.get("index").ok_or("Missing index")?;
            let on = query.get("on").ok_or("Missing on")?;

            let index = index.parse::<u32>()?;
            let on = on.parse::<bool>()?;

            Ok(ResponseEvent::GuiEvent(GuiEvent::SwitchToggle(index, on)))
        }
        "/css/style.css" => {
            let content = website_source_code::CSS_STYLE_CSS
                .bytes()
                .collect::<Vec<u8>>();
            let response = http::Response::builder()
                .header(CONTENT_TYPE, "text/css")
                .cors()
                .body(content)?;
            return Ok(ResponseEvent::Response(response));
        }
        "/js/index.js" => {
            let content = website_source_code::JS_INDEX_JS
                .bytes()
                .collect::<Vec<u8>>();
            let response = http::Response::builder()
                .header(CONTENT_TYPE, "application/javascript")
                .cors()
                .body(content)?;
            return Ok(ResponseEvent::Response(response));
        }
        _ => Err("404".into()),
    }
}

fn start_cpu_thread(
    unsafe_cpu: Arc<UnsafeCpu>,
    cpu_tx: Sender<CpuEvent>,
    gui_rx: Receiver<GuiEvent>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let cpu: &mut Cpu = unsafe { (*unsafe_cpu).get_mut() };
        cpu.reset();
        #[cfg(debug_assertions)]
        const CLOCK_CYCLES: u64 = 20_000;
        #[cfg(not(debug_assertions))]
        const CLOCK_CYCLES: u64 = 250_000;

        #[cfg(not(debug_assertions))]
        cpu.enable_wait_cycles();

        let mut last_update = Instant::now();
        const DESIRED_FPS: u32 = 120;
        let duration = Duration::from_millis(1000 / DESIRED_FPS as u64);

        loop {
            match gui_rx.recv().unwrap() {
                GuiEvent::Ready => {
                    break;
                }
                _ => {}
            }
        }

        loop {
            for _ in 0..CLOCK_CYCLES {
                cpu.clock();
            }
            cpu.bus.clock();

            // We don't want to check for interrupts every cycle
            if cpu.bus.switch.should_interrupt() {
                cpu.external_interrupt(exception::SWITCH_INTERRUPT);
            } else if cpu.bus.button.should_interrupt() {
                cpu.external_interrupt(exception::BUTTON_INTERRUPT);
            } else if cpu.bus.timer.should_interrupt() {
                cpu.external_interrupt(exception::TIMER_INTERRUPT);
            }

            while let Some(c) = cpu.bus.uart.next() {
                cpu_tx.send(CpuEvent::Uart(c)).unwrap()
            }

            for event in gui_rx.try_iter() {
                match event {
                    GuiEvent::ButtonPressed => cpu.bus.button.set(true),
                    GuiEvent::ButtonReleased => cpu.bus.button.set(false),
                    GuiEvent::SwitchToggle(index, on) => cpu.bus.switch.set(index, on),
                    GuiEvent::VgaUpdate => {}
                    GuiEvent::Ready => {}
                }
            }

            cpu_tx
                .send(CpuEvent::HexDisplays(
                    cpu.bus.hex_display.get(0),
                    cpu.bus.hex_display.get(1),
                    cpu.bus.hex_display.get(2),
                    cpu.bus.hex_display.get(3),
                    cpu.bus.hex_display.get(4),
                    cpu.bus.hex_display.get(4),
                ))
                .unwrap();
            
            if cpu.bus.vga.has_changed() {
                cpu_tx.send(CpuEvent::VgaUpdate).unwrap();
                cpu.bus.vga.reset_has_changed();
            }

            while last_update.elapsed() < duration {
                thread::sleep(Duration::from_millis(1));
            }
            last_update = Instant::now();
        }
    })
}
