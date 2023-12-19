use std::{
    collections::HashMap,
    num::{ParseIntError, TryFromIntError},
    str::FromStr,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkParseError {
    #[error("Invalid input error")]
    InvalidInputError,
    #[error("Can't parse number error")]
    ParseIntError(#[from] ParseIntError),
    #[error("Can't parse number error")]
    TryFromIntError(#[from] TryFromIntError),
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = NetworkParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(NetworkParseError::InvalidInputError),
        }
    }
}

#[derive(Debug)]
pub struct Node {
    index: String,
    left: String,
    right: String,
}

impl Node {
    pub fn new(index: String, left: String, right: String) -> Self {
        Self { index, left, right }
    }
}

impl FromStr for Node {
    type Err = NetworkParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, node) = s.split_once(" = ").expect("Should split");
        let s = &node[1..node.len() - 1];
        let (left, right) = s
            .split_once(", ")
            .ok_or(NetworkParseError::InvalidInputError)?;

        Ok(Node::new(id.to_owned(), left.to_owned(), right.to_owned()))
    }
}

pub struct Network {
    directions: Vec<Direction>,
    nodes: Vec<Node>,
    node_index: HashMap<String, usize>,
}

impl Network {
    pub fn new(
        directions: Vec<Direction>,
        nodes: Vec<Node>,
        node_index: HashMap<String, usize>,
    ) -> Self {
        Self {
            directions,
            nodes,
            node_index,
        }
    }

    pub fn walk(&self) -> Result<usize, NetworkParseError> {
        let mut steps = 0;
        let mut current_step = &String::from("AAA");
        let last_node = &String::from("ZZZ");
        let mut direction_iter = self.directions.iter();

        while current_step != last_node {
            let mut direction = direction_iter.next();

            if direction.is_none() {
                direction_iter = self.directions.iter();
                direction = direction_iter.next();
            }

            let node = self
                .node_index
                .get(current_step)
                .and_then(|i| self.nodes.get(*i))
                .ok_or(NetworkParseError::InvalidInputError)?;

            if let Some(direction) = direction {
                current_step = match direction {
                    Direction::Left => &node.left,
                    Direction::Right => &node.right,
                };
                steps += 1;
            } else {
                return Err(NetworkParseError::InvalidInputError);
            }
        }

        Ok(steps)
    }
}

impl TryFrom<String> for Network {
    type Error = NetworkParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut lines = value.lines();

        let directions = lines.next().expect("Should exist");
        let directions: Vec<Direction> = directions
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<_>, _>>()
            .expect("Should parse");

        lines.next().expect("Should skip next line");

        let mut node_index_map: HashMap<String, usize> = HashMap::new();
        let mut nodes: Vec<Node> = vec![];

        for node in lines {
            let node: Node = node.parse().expect("Should parse");
            node_index_map.insert(node.index.to_owned(), nodes.len());

            nodes.push(node);
        }

        Ok(Network::new(directions, nodes, node_index_map))
    }
}
