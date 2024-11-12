fn main() {
    let bin_path = std::env::args()
        .nth(1)
        .expect("Usage: dtekv-emulator <bin>");
    let bin = std::fs::read(bin_path).expect("Failed to read bin file");
    let mut cpu = dtekv_emulator::cpu::Cpu::new();
    cpu.bus.load_at(0, bin);
    dtekv_emulator::client::start(cpu);
}
