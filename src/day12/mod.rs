use fnv::{FnvHashMap, FnvHashSet};

#[derive(Debug, Clone)]
pub struct CaveSystem {
    system: Vec<CaveNode>,
}

impl CaveSystem {
    fn new() -> Self {
        Self { system: Vec::new() }
    }

    fn add_connections(&mut self, left: &CaveNodeType, right: &CaveNodeType) {
        let mut left_added = false;
        let mut right_added = false;

        for node in self.system.iter_mut() {
            if &node.base == left {
                node.add_connection(right);
                left_added = true;
            } else if &node.base == right {
                node.add_connection(left);
                right_added = true;
            }
        }

        if !left_added {
            self.insert_node(left, right);
        }
        if !right_added {
            self.insert_node(right, left);
        }
    }

    fn insert_node(&mut self, node_type: &CaveNodeType, connection: &CaveNodeType) {
        self.system
            .push(CaveNode::new(node_type.to_owned(), connection.to_owned()));
    }

    fn get_node(&self, node_type: &CaveNodeType) -> &CaveNode {
        for node in self.system.iter() {
            if &node.base == node_type {
                return node;
            }
        }

        unreachable!("Tried to get node that doesn't exist");
    }

    fn start_exploration(&self) -> usize {
        self.system
            .iter()
            .filter(|node| node.base == CaveNodeType::Start)
            .map(|start| {
                start
                    .connections
                    .iter()
                    .map(|conn| {
                        let mut path = FnvHashSet::default();
                        path.insert(conn);
                        self.explore_paths(&mut path, self.get_node(conn))
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn explore_paths(&self, path: &mut FnvHashSet<&CaveNodeType>, node: &CaveNode) -> usize {
        let mut total = 0;

        for conn in &node.connections {
            match conn {
                CaveNodeType::End => total += 1,
                CaveNodeType::Start => total += 0,
                CaveNodeType::Large(_) => {
                    let mut path = path.clone();
                    path.insert(&conn);
                    total += self.explore_paths(&mut path, self.get_node(&conn));
                }
                CaveNodeType::Small(_) => {
                    if path.contains(&conn) {
                        total += 0;
                    } else {
                        let mut path = path.clone();
                        path.insert(&conn);
                        total += self.explore_paths(&mut path, self.get_node(&conn));
                    }
                }
            }
        }

        total
    }

    fn start_exploration_with_revisit(&self) -> usize {
        self.system
            .iter()
            .filter(|node| node.base == CaveNodeType::Start)
            .map(|start| {
                start
                    .connections
                    .iter()
                    .map(|conn| {
                        let mut path = FnvHashMap::default();
                        path.insert(conn, 1);
                        self.explore_paths_with_revist(&mut path, self.get_node(conn))
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn explore_paths_with_revist(
        &self,
        path: &mut FnvHashMap<&CaveNodeType, usize>,
        node: &CaveNode,
    ) -> usize {
        let mut total = 0;

        for conn in &node.connections {
            match conn {
                CaveNodeType::End => total += 1,
                CaveNodeType::Start => total += 0,
                CaveNodeType::Large(_) => {
                    let mut path = path.clone();
                    *path.entry(&conn).or_insert(0) += 1;
                    total += self.explore_paths_with_revist(&mut path, self.get_node(&conn));
                }
                CaveNodeType::Small(_) => {
                    if path
                        .iter()
                        .filter(|(&key, _)| {
                            if let &CaveNodeType::Small(_) = key {
                                true
                            } else {
                                false
                            }
                        })
                        .filter(|(_, &val)| val == 2)
                        .next()
                        .is_some()
                        && path.get(conn).is_some()
                    {
                        total += 0;
                    } else {
                        let mut path = path.clone();
                        *path.entry(&conn).or_insert(0) += 1;
                        total += self.explore_paths_with_revist(&mut path, self.get_node(&conn));
                    }
                }
            }
        }

        total
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaveNode {
    base: CaveNodeType,
    connections: Vec<CaveNodeType>,
}

impl CaveNode {
    fn new(base: CaveNodeType, connection: CaveNodeType) -> Self {
        Self {
            base,
            connections: vec![connection],
        }
    }

    fn add_connection(&mut self, connection: &CaveNodeType) {
        if !self.connections.contains(&connection) {
            self.connections.push(connection.to_owned());
        }
    }

    // fn explore(&self, path: &mut FnvHashSet<&CaveNodeType>) -> usize {
    //     0
    // }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaveNodeType {
    Start,
    Small(String),
    Large(String),
    End,
}

impl From<&str> for CaveNodeType {
    fn from(input: &str) -> Self {
        match input {
            "start" => Self::Start,
            "end" => Self::End,
            _ => {
                let mut uppercase = true;

                for c in input.chars() {
                    if c.is_lowercase() {
                        uppercase = false;
                        break;
                    }
                }

                if uppercase {
                    Self::Large(input.to_owned())
                } else {
                    Self::Small(input.to_owned())
                }
            }
        }
    }
}

pub fn input_generator(input: &str) -> CaveSystem {
    let mut cave_system = CaveSystem::new();
    input.lines().into_iter().for_each(|line| {
        let mut caves = line.split("-");

        let left = CaveNodeType::from(caves.next().expect("Error retrieving left value"));
        let right = CaveNodeType::from(caves.next().expect("Error retrieving right value"));

        cave_system.add_connections(&left, &right);
    });

    cave_system
}

pub fn part1(input: &CaveSystem) -> usize {
    input.start_exploration()
}

pub fn part2(input: &CaveSystem) -> usize {
    input.start_exploration_with_revisit()
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2021/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(&input), $val);
            }
        };
    }

    test!(part1, 10);
    test!(part2, 36);
}
