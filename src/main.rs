use web_view::*;

pub fn main() {
    // // get args
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() != 2 {
    //     eprintln!("Usage: {} <path to binary>", args[0]);
    //     std::process::exit(1);
    // }
    // let bin = std::fs::read(&args[1]);
    // if bin.is_err() {
    //     eprintln!("Failed to read file: {}", args[1]);
    //     std::process::exit(1);
    // }
    // let bin = bin.unwrap();
    // dtekv_emulator::gui(bin).unwrap();
    let html_content = "<html><body><h1>Hello, World!</h1></body></html>";
	
    web_view::builder()
        .title("My Project")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
