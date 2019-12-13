mod hull;
mod intcode;
mod point;
mod robot;

use hull::{Hull, WHITE};
use intcode::{load_program, AsyncComputer};
use point::Point;
use robot::Robot;

fn main() {
    let program = load_program("./paint.intcode");
    let cpu = AsyncComputer::new(&program);
    let mut robot = Robot::new(cpu);
    let mut hull = Hull::new();

    // part 2
    hull.paint(Point::at(0, 0), WHITE);

    robot.start();
    while robot.is_running() {
        robot.scan(&hull);
        robot.paint(&mut hull);
        println!(
            "{}",
            hull.render(robot.get_location(), robot.get_direction())
        );
    }

    // println!("part 1: {}", hull.num_painted());
}

#[cfg(test)]
mod test {
    use super::hull::{Hull, BLACK, WHITE};
    use super::intcode::IntcodeComputer;
    use super::point::Point;
    use super::robot::{Robot, TURN_LEFT, TURN_RIGHT};
    use std::collections::VecDeque;

    struct MockComputer {
        expected_inputs: VecDeque<i64>,
        mock_outputs: VecDeque<i64>,
        running: bool,
    }

    impl MockComputer {
        fn new(expected_inputs: Vec<i64>, mock_outputs: Vec<i64>) -> Self {
            MockComputer {
                expected_inputs: VecDeque::from(expected_inputs),
                mock_outputs: VecDeque::from(mock_outputs),
                running: false,
            }
        }
    }

    impl IntcodeComputer for MockComputer {
        fn start(&mut self) {
            self.running = true;
        }
        fn stop(&mut self) {
            self.running = false;
        }
        fn is_running(&self) -> bool {
            self.running
        }
        fn send_input(&mut self, data: i64) {
            let expected = self.expected_inputs.pop_front().unwrap();
            assert_eq!(data, expected);
        }
        fn recv_output(&mut self) -> Option<i64> {
            let result = self.mock_outputs.pop_front().unwrap();
            if self.mock_outputs.len() == 0 {
                self.running = false;
            }
            Some(result)
        }
    }

    #[test]
    fn test_robot() {
        let cpu = MockComputer::new(
            vec![BLACK, BLACK, BLACK, BLACK, WHITE, BLACK, BLACK, BLACK],
            vec![
                WHITE, TURN_LEFT, BLACK, TURN_LEFT, WHITE, TURN_LEFT, WHITE, TURN_LEFT, BLACK,
                TURN_RIGHT, WHITE, TURN_LEFT, WHITE, TURN_LEFT,
            ],
        );
        let mut robot = Robot::new(cpu);
        robot.start();
        let mut hull = Hull::new();
        while robot.is_running() {
            robot.scan(&hull);
            robot.paint(&mut hull);
        }

        let location = robot.get_location();
        let direction = robot.get_direction();
        assert_eq!(location, Point::at(0, 1));
        assert_eq!(direction, Point::at(-1, 0));

        assert_eq!(hull.get_color(&Point::at(0, 0)), BLACK);
        assert_eq!(hull.get_color(&Point::at(-1, 0)), BLACK);
        assert_eq!(hull.get_color(&Point::at(-1, -1)), WHITE);
        assert_eq!(hull.get_color(&Point::at(0, -1)), WHITE);
        assert_eq!(hull.get_color(&Point::at(1, 0)), WHITE);
        assert_eq!(hull.get_color(&Point::at(1, 1)), WHITE);

        let output = hull.render(location, direction);
        assert_eq!(
            output,
            "⬛⬛⬛⬛⬛\n⬛⬛🤖⬜⬛\n⬛⬛⬛⬜⬛\n⬛⬜⬜⬛⬛\n⬛⬛⬛⬛⬛\n"
        )
    }
}
