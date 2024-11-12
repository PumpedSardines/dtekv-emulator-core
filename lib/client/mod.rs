// Mirror the file path with the name the constant
mod website_source_code {
    use const_format::str_replace;

    #[cfg(target_os = "macos")]
    pub const HOST: &'static str = "wry://localhost";
    #[cfg(target_os = "windows")]
    pub const HOST: &'static str = "http://wry.com";

    pub const INDEX_HTML: &str =
        str_replace!(include_str!("./website/index.html"), "{{HOST}}", HOST);
    pub const HELP_INDEX: &str =
        str_replace!(include_str!("./website/help.html"), "{{HOST}}", HOST);

    pub const CSS_STYLE_CSS: &str = include_str!("./website/css/style.css");
    pub const JS_INDEX_JS: &str = include_str!("./website/js/index.js");
    pub const JS_DTEKV_JS: &str =
        str_replace!(include_str!("./website/js/__dtekv__.js"), "{{HOST}}", HOST);
}

trait Cors {
    fn cors(self) -> Self;
}
impl Cors for http::response::Builder {
    fn cors(self) -> Self {
        self.header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .header("Access-Control-Max-Age", "3600")
            .header("Access-Control-Allow-Credentials", "true")
            .header("Access-Control-Expose-Headers", "*")
            .header("Access-Control-Allow-Headers", "*")
    }
}

pub use tao::dpi::LogicalSize;

pub mod start;
pub use start::start;
mod shared_cpu;

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
    Reset,
    OpenLinkInBrowser(String),
    Load,
}

enum CpuEvent {
    Uart(char),
    HexDisplays(u8, u8, u8, u8, u8, u8),
    VgaUpdate,
    OpenLinkInBrowser(String)
}
