pub const VGA_BUFFER_LOWER_ADDR: u32 = 0x08000000;
pub const VGA_BUFFER_HIGHER_ADDR: u32 = 0x80257ff;

// No default implementation of the vga_buffer,
// I feel like this component is too platform dependent and should be implemented by the user
