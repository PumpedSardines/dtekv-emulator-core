// This example runs a board game program on the emulator

pub fn main() {
    let bin: Vec<u8> = include_bytes!("./board_games.bin").to_vec();
    dtekv_emulator::gui(bin).unwrap();
}
