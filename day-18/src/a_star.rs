use std::cmp::{Ord, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::iter::FromIterator;
use std::time::SystemTime;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node<T> {
    node: T,
    cost: usize,
}

impl<T> Node<T> {
    fn new(node: T, cost: usize) -> Self {
        Node { node, cost }
    }
}

impl<T> Ord for Node<T>
where
    T: Eq + PartialEq,
{
    fn cmp(&self, other: &Node<T>) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T> PartialOrd for Node<T>
where
    T: Eq + PartialEq,
{
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait Searchable<T>
where
    T: Clone + Hash + Eq + PartialEq,
{
    fn distance(&self, start: &T, end: &T) -> usize;
    fn get_neighbors(&self, node: &T) -> Vec<T>;
    fn debug(&self, _current: &T, _goal: &T, _frontier: &HashSet<T>, _explored: &HashSet<T>) {}
}

pub fn search<T, G, H>(start: &T, goal: &T, graph: &G, heuristic: H) -> Option<Vec<T>>
where
    T: Clone + Hash + Eq + PartialEq + std::fmt::Debug,
    G: Searchable<T>,
    H: Fn(&T, &T) -> usize,
{
    // let now = SystemTime::now();

    let mut frontier = BinaryHeap::<Node<T>>::new();
    let initial_cost = 0;
    frontier.push(Node::new(start.clone(), initial_cost));

    let mut came_from = HashMap::<T, T>::new();

    let mut cost_so_far = HashMap::<T, usize>::new();
    cost_so_far.insert(start.clone(), initial_cost);

    while !frontier.is_empty() {
        let current = frontier.pop().expect("Failed to pop value from frontier");

        // let explored = cost_so_far.keys().cloned().collect::<HashSet<_>>();
        // let unique_frontier = frontier
        //     .iter()
        //     .map(|node| node.node.clone())
        //     .collect::<HashSet<_>>();
        // graph.debug(&current.node, goal, &unique_frontier, &explored);

        if current.node == *goal {
            // println!("A* search took {}us", now.elapsed().unwrap().as_micros());
            return Some(reconstruct_path(&current.node, came_from));
        }

        let current_cost_so_far = *cost_so_far.get(&current.node).expect(&format!(
            "Failed to get cost so far for current node: {:?}",
            current.node
        ));
        let neighbors = graph.get_neighbors(&current.node);
        for neighbor in neighbors {
            let new_cost = current_cost_so_far + graph.distance(&current.node, &neighbor);
            let neighbor_cost_so_far = cost_so_far.get(&neighbor);
            if neighbor_cost_so_far.is_none() || new_cost < *neighbor_cost_so_far.unwrap() {
                cost_so_far.insert(neighbor.clone(), new_cost);
                let priority = new_cost + heuristic(&neighbor, goal);
                frontier.push(Node::new(neighbor.clone(), priority));
                came_from.insert(neighbor, current.node.clone());
            }
        }
    }

    None
}

fn reconstruct_path<T>(end: &T, came_from: HashMap<T, T>) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut current = end;
    let mut path = VecDeque::new();
    path.push_back(current.clone());

    while let Some(next) = came_from.get(current) {
        path.push_front(next.clone());
        current = next;
    }

    Vec::from_iter(path.into_iter())
}

#[cfg(test)]
mod tests {}
