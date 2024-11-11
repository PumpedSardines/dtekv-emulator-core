// Mirror the file path with the name the constant
mod website_source_code {
    use const_format::str_replace;

    #[cfg(target_os = "macos")]
    pub const HOST: &'static str = "wry://localhost/";
    #[cfg(target_os = "windows")]
    pub const HOST: &'static str = "http://wry.com/";

    pub const INDEX_HTML: &str = str_replace!(include_str!("./website/index.html"), "{{HOST}}", HOST);
    pub const CSS_STYLE_CSS: &str = include_str!("./website/css/style.css");
    pub const JS_INDEX_JS: &str = include_str!("./website/js/index.js");
}

pub use tao::dpi::LogicalSize;

pub mod start;
pub use start::start;
mod unsafe_cpu;

enum ResponseEvent {
    GuiEvent(GuiEvent),
    Response(http::Response<Vec<u8>>),
}

#[derive(Debug)]
enum GuiEvent {
    ButtonPressed,
    ButtonReleased,
    SwitchToggle(u32, bool),
    VgaUpdate,
    Ready,
}

enum CpuEvent {
    Uart(char),
    HexDisplays(u8, u8, u8, u8, u8, u8),
}
