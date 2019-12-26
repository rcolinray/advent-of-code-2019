use crate::grid::Grid;
use crate::maze::Block;
use crate::point::Point2D;

use std::collections::HashMap;

pub struct PathCache {
    paths: HashMap<(Point2D, Point2D), Vec<Point2D>>,
}

impl PathCache {
    pub fn new(
        grid: &Grid<Block>,
        entrance: &Point2D,
        exit: &Point2D,
        portals: &HashMap<Point2D, Point2D>,
    ) -> Self {
        let mut paths = HashMap::new();
        let all_portals = portals.keys().collect::<Vec<_>>();

        if let Some(points) = grid.find_path(entrance, exit) {
            paths.insert((*entrance, *exit), points);
        }

        for &portal in all_portals.iter() {
            if let Some(points) = grid.find_path(entrance, portal) {
                paths.insert((*entrance, *portal), points);
            }

            if let Some(points) = grid.find_path(&portal, exit) {
                paths.insert((*portal, *exit), points);
            }

            for &end_portal in all_portals.iter() {
                if portal == end_portal {
                    continue;
                } else if let Some(pair_portal) = portals.get(portal) {
                    if pair_portal == end_portal {
                        continue;
                    }
                }

                if let Some(points) = grid.find_path(portal, end_portal) {
                    paths.insert((*portal, *end_portal), points);
                }
            }
        }

        PathCache { paths }
    }

    pub fn get_path(&self, start: &Point2D, end: &Point2D) -> Option<&Vec<Point2D>> {
        self.paths.get(&(*start, *end))
    }

    pub fn has_path(&self, start: &Point2D, end: &Point2D) -> bool {
        self.get_path(start, end).is_some()
    }
}
