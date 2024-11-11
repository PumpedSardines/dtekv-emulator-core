// Mirror the file path with the name the constant
mod website_source_code {
    pub const INDEX_HTML: &str = include_str!("./website/index.html");
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
