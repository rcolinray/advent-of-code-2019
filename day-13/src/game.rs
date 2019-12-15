use std::collections::{HashMap, VecDeque};

use ggez::event::{quit, EventHandler};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Drawable, Mesh, MeshBuilder, Rect};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{Context, GameResult};

use crate::intcode::Computer;
use crate::point::Point;

const BLOCK_SIZE: f32 = 10.0;

pub struct Game {
    cpu: Computer,
    score: i64,
    output: VecDeque<i64>,
    walls: Vec<Mesh>,
    blocks: HashMap<Point, Mesh>,
    ball: Option<Mesh>,
    paddle: Option<Mesh>,
}

impl Game {
    pub fn new(cpu: Computer, _ctx: &mut Context) -> Self {
        Game {
            cpu,
            score: 0,
            output: VecDeque::new(),
            walls: Vec::new(),
            blocks: HashMap::new(),
            ball: None,
            paddle: None,
        }
    }

    fn process_instruction(&mut self) -> (i64, i64, i64) {
        let x = self.output.pop_front().expect("Failed to get output");
        let y = self.output.pop_front().expect("Failed to get output");
        let t = self.output.pop_front().expect("Failed to get output");
        (x, y, t)
    }

    fn remove_block(&mut self, point: Point) {
        self.blocks.remove(&point);
    }

    fn rect_at(&self, point: Point) -> Rect {
        Rect::new(
            BLOCK_SIZE * point.x as f32,
            BLOCK_SIZE * point.y as f32,
            BLOCK_SIZE,
            BLOCK_SIZE,
        )
    }

    fn add_wall(&mut self, point: Point, ctx: &mut Context) {
        let rect = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                self.rect_at(point),
                Color::from_rgb(0xe5, 0x31, 0x70),
            )
            .build(ctx)
            .expect("Failed to build wall");
        self.walls.push(rect);
    }

    fn add_block(&mut self, point: Point, ctx: &mut Context) {
        let rect = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                self.rect_at(point),
                Color::from_rgb(0xf2, 0x5f, 0x4c),
            )
            .build(ctx)
            .expect("Failed to build block");
        self.blocks.insert(point, rect);
    }

    fn update_paddle(&mut self, point: Point, ctx: &mut Context) {
        let rect = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                self.rect_at(point),
                Color::from_rgb(0xff, 0x89, 0x06),
            )
            .build(ctx)
            .expect("Failed to build paddle");
        self.paddle = Some(rect);
    }

    fn update_ball(&mut self, point: Point, ctx: &mut Context) {
        let rect = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                self.rect_at(point),
                Color::from_rgb(0xff, 0xff, 0xfe),
            )
            .build(ctx)
            .expect("Failed to build ball");
        self.ball = Some(rect);
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.cpu.set_input(0);

        for _ in 0..10000 {
            self.cpu.step();
            if self.cpu.is_blocked() {
                break;
            }
        }

        loop {
            match self.cpu.get_output() {
                Some(output) => self.output.push_back(output),
                None => break,
            };
        }

        while self.output.len() >= 3 {
            let (x, y, t) = self.process_instruction();
            if x == -1 && y == 0 {
                self.score = t;
                println!("score: {}", self.score);
            } else {
                let point = Point::at(x, y);
                match t {
                    0 => self.remove_block(point),
                    1 => self.add_wall(point, ctx),
                    2 => self.add_block(point, ctx),
                    3 => self.update_paddle(point, ctx),
                    4 => self.update_ball(point, ctx),
                    _ => (),
                };
            }
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => self.cpu.set_input(-1),
            KeyCode::Right => self.cpu.set_input(1),
            KeyCode::Escape => quit(ctx),
            _ => (),
        };
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        self.cpu.set_input(0);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for wall in self.walls.iter() {
            wall.draw(ctx, DrawParam::default())
                .expect("Failed to draw wall");
        }

        for block in self.blocks.values() {
            block
                .draw(ctx, DrawParam::default())
                .expect("Failed to draw block");
        }

        if let Some(paddle) = self.paddle.as_ref() {
            paddle
                .draw(ctx, DrawParam::default())
                .expect("Failed to draw paddle");
        }

        if let Some(ball) = self.ball.as_ref() {
            ball.draw(ctx, DrawParam::default())
                .expect("Failed to draw ball");
        }

        graphics::present(ctx)
    }
}
