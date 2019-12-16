use crate::droid::{Direction, Droid};
use crate::point::Point;
use crate::section_map::SectionMap;

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

pub struct Ai {
    pos: Point,
    frontier: VecDeque<Point>,
    explored: HashSet<Point>,
    plan: VecDeque<Direction>,
    oxygen_system: Option<Point>,
}

impl Ai {
    pub fn new(map: &SectionMap) -> Self {
        let pos = Point::at(0, 0);
        let mut explored = HashSet::new();
        explored.insert(pos);
        let mut frontier = VecDeque::new();
        let neighbors = map.get_neighbors(pos);
        frontier.extend(neighbors.iter());
        Ai {
            pos,
            frontier,
            explored,
            plan: VecDeque::new(),
            oxygen_system: None,
        }
    }

    pub fn get_oxygen_system(&self) -> Point {
        self.oxygen_system.expect("Failed to get oxygen system")
    }

    pub fn is_found(&self) -> bool {
        self.oxygen_system.is_some()
    }

    pub fn done_exploring(&self) -> bool {
        self.frontier.len() == 0
    }

    pub fn get_path_length(&self, map: &SectionMap) -> usize {
        let start = self
            .oxygen_system
            .expect("Expected oxygen system to be found");
        let goal = Point::at(0, 0);
        plan_path(start, goal, map).len()
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    pub fn update(&mut self, droid: &mut Droid, map: &mut SectionMap) {
        let dir = self.get_next_dir(map);
        if dir.is_none() {
            return;
        }

        let try_dir = dir.unwrap();
        let result = droid.try_move(try_dir);

        let attempted_pos = self.pos + try_dir.to_vector();
        self.explored.insert(attempted_pos);

        if !result.is_wall() {
            self.pos = attempted_pos;
            let neighbors = map.get_neighbors(self.pos);
            let mut unexplored = neighbors
                .iter()
                .filter(|neighbor| {
                    !self.explored.contains(neighbor) && !self.frontier.contains(neighbor)
                })
                .cloned()
                .collect::<VecDeque<Point>>();
            unexplored.extend(self.frontier.iter());
            self.frontier = unexplored;
        }

        if result.is_oxygen_system() {
            self.oxygen_system = Some(self.pos);
        }

        map.set_result(attempted_pos, result);
    }

    fn get_next_dir(&mut self, map: &SectionMap) -> Option<Direction> {
        if self.plan.len() == 0 && self.frontier.len() > 0 {
            let mut next_goal = self
                .frontier
                .pop_front()
                .expect("Failed to get point from frontier");
            while self.explored.contains(&next_goal) && self.frontier.len() > 0 {
                next_goal = self
                    .frontier
                    .pop_front()
                    .expect("Failed to get point from frontier");
            }
            self.plan = plan_path(self.pos, next_goal, map);
        }

        if self.plan.len() > 0 {
            Some(
                self.plan
                    .pop_front()
                    .expect("Failed to get point from plan"),
            )
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Node {
    point: Point,
    cost: usize,
}

impl Node {
    fn new(point: Point, cost: usize) -> Self {
        Node { point, cost }
    }
}

impl Ord for Node {
    fn cmp(&self, rhs: &Node) -> std::cmp::Ordering {
        rhs.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

fn plan_path(start: Point, goal: Point, map: &SectionMap) -> VecDeque<Direction> {
    let mut frontier = BinaryHeap::new();
    let mut total_costs = HashMap::new();
    let mut came_from = HashMap::new();

    total_costs.insert(start, 0);
    frontier.push(Node::new(start, 0));

    while frontier.len() > 0 {
        let current = frontier.pop().expect("Failed to get next from frontier");
        if current.point == goal {
            break;
        }

        let neighbors = map.get_neighbors(current.point);
        let valid_neighbors = neighbors
            .iter()
            .filter(|&neighbor| map.is_empty(neighbor) || *neighbor == goal);
        for &neighbor in valid_neighbors {
            let current_cost = total_costs
                .get(&current.point)
                .expect("Failed to get total cost for current");
            let next_cost = current_cost + 1;
            if !total_costs.contains_key(&neighbor)
                || next_cost
                    < *total_costs
                        .get(&neighbor)
                        .expect("Failed to get total cost for next")
            {
                total_costs.insert(neighbor, next_cost);
                frontier.push(Node::new(neighbor, next_cost));
                came_from.insert(neighbor, current.point);
            }
        }
    }

    let mut path = VecDeque::new();
    let mut current = goal;
    loop {
        if let Some(&prev) = came_from.get(&current) {
            let dir = Direction::from_vector(&(current - prev));
            path.push_front(dir);
            current = prev;
        } else {
            break;
        }
    }

    path
}
