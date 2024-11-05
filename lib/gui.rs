#![allow(clippy::unnecessary_wraps)]
use crate::Cpu;
use ggez::{
    event,
    glam::{self, *},
    graphics::{self, ImageFormat, Rect},
    winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize, Pixel},
    Context, GameResult,
};

struct MainState {
    cpu: Cpu,
    width: f32,
    height: f32,
    scale_factor: f64,
    switches: Vec<Switch>,
    hex_displays: Vec<HexDisplay>,
    button: Button,
    vga_display: VgaDisplay,
}

impl MainState {
    fn new(ctx: &mut Context, cpu: Cpu) -> GameResult<MainState> {
        let mut switches: Vec<Switch> = vec![];
        let mut hex_displays: Vec<HexDisplay> = vec![];

        for i in 0..10 {
            switches.push(Switch {
                x: 1000.0 + 60.0 * (i as f32),
                y: 1600.0 - 100.0 - 70.0,
                index: 9 - i,
            });
        }

        for i in 0..6 {
            hex_displays.push(HexDisplay {
                x: 150.0 * (i as f32),
                y: 1600.0 - 240.0,
                index: 5 - i,
            });
        }

        let mut button = Button::new(ctx);
        button.x = 1800.0 - 100.0 - 70.0;
        button.y = 1600.0 - 100.0 - 70.0;

        let vga_display = VgaDisplay {
            x: (1800.0 - 320.0 * 5.0) / 2.0,
            y: 50.0,
            image: None,
        };

        Ok(MainState {
            cpu,
            width: 900.0,
            height: 800.0,
            scale_factor: ctx.gfx.window().scale_factor(),
            switches,
            vga_display,
            hex_displays,
            button,
        })
    }
}

struct Switch {
    x: f32,
    y: f32,
    index: u32,
}

impl Switch {
    fn draw(&self, canvas: &mut graphics::Canvas, cpu: &Cpu) {
        let on = cpu.bus.switch.get_switch(self.index);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect([self.x, self.y, 50.0, 100.0].into())
                .color([0.5, 0.5, 0.5, 1.0]),
        );
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect([self.x + 15.0, self.y + 10.0, 20.0, 80.0].into())
                .color(if on {
                    [0.0, 0.8, 0.0, 1.0]
                } else {
                    [0.8, 0.0, 0.0, 1.0]
                }),
        );
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(
                    if on {
                        [self.x + 15.0, self.y + 10.0, 20.0, 30.0]
                    } else {
                        [self.x + 15.0, self.y + 10.0 + 50.0, 20.0, 30.0]
                    }
                    .into(),
                )
                .color([0.2, 0.2, 0.2, 1.0]),
        );
    }

    fn handle_mouse_down(&mut self, x: f32, y: f32, cpu: &mut Cpu) {
        if Rect::new(self.x, self.y, 50.0, 100.0).contains([x, y]) {
            cpu.bus
                .switch
                .set_switch(self.index, !cpu.bus.switch.get_switch(self.index));
        }
    }
}

struct HexDisplay {
    x: f32,
    y: f32,
    index: u32,
}

impl HexDisplay {
    fn draw(&self, canvas: &mut graphics::Canvas, cpu: &Cpu) -> GameResult {
        // Bit indexes of the different segments
        //         0
        // 5                1
        //         6
        // 4                2
        //         3        7

        let bit_mask = cpu.bus.hex_display.get_display(self.index);

        let shapes: Vec<([f32; 4], bool)> = vec![
            (
                [self.x + 35.0, self.y + 25.0, 80.0, 10.0],
                bit_mask & (1 << 0) == 0,
            ), // Top
            (
                [self.x + 25.0 + 80.0 + 10.0, self.y + 35.0, 10.0, 80.0],
                bit_mask & (1 << 1) == 0,
            ), // Top right
            (
                [
                    self.x + 25.0 + 80.0 + 10.0,
                    self.y + 35.0 + 90.0,
                    10.0,
                    80.0,
                ],
                bit_mask & (1 << 2) == 0,
            ), // Bottom right
            (
                [self.x + 35.0, self.y + 25.0 + 90.0 + 90.0, 80.0, 10.0],
                bit_mask & (1 << 3) == 0,
            ), // Bottom
            (
                [self.x + 25.0, self.y + 35.0 + 90.0, 10.0, 80.0],
                bit_mask & (1 << 4) == 0,
            ), // Bottom left
            (
                [self.x + 25.0, self.y + 35.0, 10.0, 80.0],
                bit_mask & (1 << 5) == 0,
            ), // Top left
            (
                [self.x + 35.0, self.y + 25.0 + 90.0, 80.0, 10.0],
                bit_mask & (1 << 6) == 0,
            ), // Middle
            (
                [self.x + 150.0 - 25.0, self.y + 240.0 - 25.0, 10.0, 10.0],
                bit_mask & (1 << 7) == 0,
            ), // Decimal dot
        ];

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect([self.x, self.y, 150.0, 240.0].into())
                .color([0.0, 0.0, 0.0, 1.0]),
        );

        for (shape, on) in shapes {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(shape.into())
                    .color(if on {
                        [1.0, 0.0, 0.0, 1.0]
                    } else {
                        [0.1, 0.1, 0.1, 1.0]
                    }),
            );
        }

        Ok(())
    }
}

struct VgaDisplay {
    x: f32,
    y: f32,
    image: Option<graphics::Image>,
}

impl VgaDisplay {
    fn update(&mut self, ctx: &mut Context, cpu: &Cpu) {
        let mut pixel_data = [0u8; 320 * 240 * 4];
        for x in 0..320 {
            for y in 0..240 {
                let (r, g, b) = cpu.bus.vga.get_pixel(x, y);
                let index = ((y * 320 + x) * 4) as usize;

                pixel_data[index] = r;
                pixel_data[index + 1] = g;
                pixel_data[index + 2] = b;
                pixel_data[index + 3] = 255;
            }
        }

        let image =
            graphics::Image::from_pixels(ctx, &pixel_data, ImageFormat::Rgba8UnormSrgb, 320, 240);

        self.image = Some(image);
    }

    fn draw(&self, canvas: &mut graphics::Canvas, _: &Cpu) {
        let image = self.image.as_ref().unwrap();
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        canvas.draw(
            image,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(self.x, self.y))
                .scale([5.0, 5.0]),
        );
        canvas.set_default_sampler();
    }
}

struct Button {
    x: f32,
    y: f32,
    circle_on: graphics::Mesh,
    circle_off: graphics::Mesh,
}

impl Button {
    fn new(ctx: &mut Context) -> Self {
        let circle_off = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            40.0,
            1.0,
            [0.5, 0.0, 0.0, 1.0].into(),
        )
        .unwrap();

        let circle_on = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            40.0,
            1.0,
            [0.0, 0.5, 0.0, 1.0].into(),
        )
        .unwrap();

        Button {
            x: 0.0,
            y: 0.0,
            circle_on,
            circle_off,
        }
    }

    fn handle_mouse_down(&mut self, x: f32, y: f32, cpu: &mut Cpu) {
        if Rect::new(self.x, self.y, 100.0, 100.0).contains([x, y]) {
            cpu.bus.button.set(true);
        }
    }

    fn handle_mouse_up(&mut self, cpu: &mut Cpu) {
        if cpu.bus.button.get() {
            cpu.bus.button.set(false);
        }
    }

    fn draw(&self, canvas: &mut graphics::Canvas, cpu: &Cpu) -> GameResult {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect([self.x, self.y, 100.0, 100.0].into())
                .color([0.5, 0.5, 0.5, 1.0]),
        );

        if cpu.bus.button.get() {
            canvas.draw(&self.circle_on, Vec2::new(self.x + 50.0, self.y + 50.0));
        } else {
            canvas.draw(&self.circle_off, Vec2::new(self.x + 50.0, self.y + 50.0));
        }

        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let scale_factor = ctx.gfx.window().scale_factor();

        if self.scale_factor != scale_factor {
            // self.scale_factor = scale_factor;
            // let size = LogicalSize::new(self.width as f64, self.height as f64);
            // ctx.gfx.window().set_inner_size(size);
        }

        // 60hz * 500 000 = 30 000 000 hz
        // The update runs 60 times a second and the clock cycle on the dtekv chip is 30 000 000 hz
        //
        // However: This lagged the game too much, so I had to reduce the amount of cycles, so this
        // is slower than the actual chip
        for _ in 0..200_000 {
            self.cpu.clock();

            if self.cpu.bus.switch.should_interrupt() {
                self.cpu.external_interrupt_switch();
            } else if self.cpu.bus.button.should_interrupt() {
                self.cpu.external_interrupt_button();
            } else if self.cpu.bus.timer.should_interrupt() {
                self.cpu.external_interrupt_timer();
            }
        }

        self.cpu.bus.timer.update();
        self.vga_display.update(ctx, &self.cpu);

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        let scaling_factor = ctx.gfx.window().scale_factor() as f32;

        let x = x / scaling_factor;
        let y = y / scaling_factor;
        let x = x * 2.0;
        let y = y * 2.0;

        for switch in self.switches.iter_mut() {
            switch.handle_mouse_down(x, y, &mut self.cpu);
        }

        self.button.handle_mouse_down(x, y, &mut self.cpu);

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        self.button.handle_mouse_up(&mut self.cpu);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.2, 0.2, 0.2, 1.0]));
        let scaling_factor = ctx.gfx.window().scale_factor() as f32;
        canvas.set_screen_coordinates(
            [0.0, 0.0, 3600.0 / scaling_factor, 3200.0 / scaling_factor].into(),
        );

        self.vga_display.draw(&mut canvas, &self.cpu);

        for hex_display in self.hex_displays.iter() {
            hex_display.draw(&mut canvas, &self.cpu)?;
        }

        for switch in self.switches.iter() {
            switch.draw(&mut canvas, &self.cpu);
        }

        self.button.draw(&mut canvas, &self.cpu)?;

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn gui(bin: Vec<u8>) -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("DTEKV"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(900.0, 800.0)
                .resize_on_scale_factor_change(true),
        );
    let (mut ctx, event_loop) = cb.build()?;
    let size = ctx.gfx.window().inner_size();
    let dpi = ctx.gfx.window().scale_factor() as f32;

    let sw = size.width as f32 * dpi;
    let sh = size.height as f32 * dpi;

    let size = PhysicalSize::new(sw as f64, sh as f64);
    ctx.gfx.window().set_inner_size(size);

    let mut cpu = Cpu::new();

    // let size = LogicalSize::new(900.0, 800.0);
    // ctx.gfx.window().set_inner_size(size);

    cpu.bus.mem.load_data_at(0, bin);
    // Mie is set to 1 always
    cpu.csr.set_mstatus_mie(true);
    // When a reset signal is sent to the chip, the pc goes to 4, not 0
    cpu.pc = 4;

    let mut state = MainState::new(&mut ctx, cpu)?;
    state.vga_display.update(&mut ctx, &state.cpu);
    event::run(ctx, event_loop, state)
}
