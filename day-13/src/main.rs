mod game;
mod intcode;
mod point;
mod screen;

use ggez::event;
use ggez::graphics::{self, Rect};
use ggez::ContextBuilder;

use game::Game;
use intcode::{load_program, AsyncComputer, Computer, IntcodeComputer};
use point::Point;
use screen::Screen;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mem = load_program("./game.intcode");
    let mut cpu = AsyncComputer::new(&mem);
    let mut screen = Screen::new();

    cpu.start();

    while cpu.is_running() {
        if let Some((x, y, t)) = recv_instruction(&mut cpu) {
            screen.set(Point::at(x, y), t);
        }
    }

    println!("{}", screen.to_string());
    println!("num blocks: {}", screen.num_blocks());
}

fn recv_instruction<T: IntcodeComputer>(cpu: &mut T) -> Option<(i64, i64, i64)> {
    let x = cpu.recv_output();
    let y = cpu.recv_output();
    let t = cpu.recv_output();

    match (x, y, t) {
        (Some(x), Some(y), Some(t)) => Some((x, y, t)),
        _ => None,
    }
}

fn part2() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("day-13", "Advent of Code")
        .build()
        .expect("Failed to create ggez context");

    graphics::set_drawable_size(&mut ctx, 370.0, 260.0).expect("Failed to set drawable size");
    graphics::set_screen_coordinates(&mut ctx, Rect::new(0.0, 0.0, 370.0, 260.0))
        .expect("Failed to set screen coords");

    let mut mem = load_program("./hacked_game.intcode");
    mem[0] = 2; // insert two coins
    let cpu = Computer::new(&mem);
    let mut game = Game::new(cpu, &mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => (),
        Err(_) => (),
    };
}
