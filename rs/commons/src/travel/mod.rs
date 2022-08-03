// Algorithm I want :

// 1. Generate graph
// 2. Compute possible paths, including refuel (happens when the ship cannot go anywhere)
// 3. Filter out generated paths

use crate::Route;
use std::collections::HashMap;
use log::trace;

pub struct Graph {
    pub planets: Vec<String>,
    pub links: Vec<Link>,
}

#[derive(Debug)]
pub struct Link {
    from: String,
    to: String,
    length: u32,
}

#[derive(Debug, Clone)]
pub struct PathStep {
    pub to: String,
    pub cost: u32
}

impl Graph {
    // Generate a new graph based on provided routes. Planets are automatically guessed from paths provided.
    pub fn new(routes: &Vec<Route>) -> Self {
        let mut res = Self {
            links: vec![],
            planets: vec![],
        };

        for route in routes {
            if !res.planets.contains(&route.origin) {
                res.planets.push(route.origin.clone());
            }

            res.links.push(Link {
                from: route.origin.clone(),
                to: route.destination.clone(),
                length: route.travel_time,
            })
        }

        res
    }

    // Generates a quick lookup table to easily map names into possible links.
    // Currently only returns the "from" links, as the algorithm does not make use of the "to" links.
    fn lookuptable_links(&self) -> HashMap<String, Vec<&Link>> {
        let mut hashmap: HashMap<String, Vec<&Link>> = HashMap::new();

        for link in self.links.iter() {
            // hashmap.entry(link.to.clone())
            //     .or_insert(vec![])
            //     .push(&link);

            hashmap
                .entry(link.from.clone())
                .or_insert(vec![])
                .push(&link);
        }

        hashmap
    }

    // This computes every possible path WITHOUT LOOPS (!!) between two points.
    pub fn get_all_paths(&self, start: String, end: String) -> Vec<Vec<PathStep>> {
        let links_lookup = self.lookuptable_links();
        // event!(Level::TRACE, "Found links: {:?}", links_lookup);
        trace!("Found links: {:?}", links_lookup);

        let mut paths: Vec<Vec<PathStep>> = vec![];
        self.build_path(PathStep {to: start, cost: 0}, end, &links_lookup, vec![], &mut paths);
        trace!("{:?}", paths);
        paths
    }

    fn build_path(&self, start: PathStep, end: String, link_map: &HashMap<String, Vec<&Link>>, mut path: Vec<PathStep>, paths: &mut Vec<Vec<PathStep>>) {
        path.push(start.clone());

        if start.to == end {
            paths.push(path);
            return;
        }

        let links = &link_map[&start.to];

        for link in links {
            self.build_path(PathStep { cost: link.length, to: link.to.clone()}, end.clone(), link_map, path.clone(), paths);
        }
    }
}

