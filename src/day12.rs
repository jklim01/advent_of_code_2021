use std::str;
use std::iter;
use std::fmt;
use std::error::Error;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
enum ParseNodeError {
    EmptyString, InvalidCase
}
impl fmt::Display for ParseNodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseNodeError::EmptyString => write!(f, "node cannot be empty string"),
            ParseNodeError::InvalidCase => write!(f, "all characters must have same case")
        }
    }
}
impl Error for ParseNodeError {}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Node {
    Start, End, BigCave(String), SmallCave(String)
}
impl str::FromStr for Node {
    type Err = ParseNodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() { return Err(ParseNodeError::EmptyString); }
        if s == "start" { return Ok(Node::Start); }
        if s == "end" { return Ok(Node::End); }
        if s.chars().all(|c| c.is_uppercase()) { return Ok(Node::BigCave(s.to_owned())); }
        if s.chars().all(|c| c.is_lowercase()) { return Ok(Node::SmallCave(s.to_owned())); }
        Err(ParseNodeError::InvalidCase)
    }
}

#[derive(Debug)]
enum ParseCaveError {
    NodeError((usize, ParseNodeError)), InvalidConnection(usize), MissingStart, MissingEnd
}
impl fmt::Display for ParseCaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseCaveError::NodeError(e) => write!(f, "ParseNodeError on line {}", e.0),
            ParseCaveError::InvalidConnection(i) => write!(f, "invalid connection format on line {}", i),
            ParseCaveError::MissingStart => write!(f, "missing \"start\" node"),
            ParseCaveError::MissingEnd => write!(f, "mising \"end\" node"),
        }
    }
}
impl Error for ParseCaveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseCaveError::NodeError((_, ref e)) => Some(e),
            _ => None,
        }
    }
}
impl From<(usize, ParseNodeError)> for ParseCaveError {
    fn from(err: (usize, ParseNodeError)) -> ParseCaveError {
        ParseCaveError::NodeError(err)
    }
}
#[derive(Debug)]
struct CaveMap(HashMap<Node, HashSet<Node>>);
impl iter::FromIterator<(Node, Node)> for CaveMap {
    fn from_iter<I: IntoIterator<Item = (Node, Node)>>(iter: I) -> Self {
        let mut cave = CaveMap(HashMap::new());
        for (node1, node2) in iter {
            if node1 == node2 { continue; }
            cave.0.entry(node1.clone()).or_insert_with(HashSet::new).insert(node2.clone());
            cave.0.entry(node2).or_insert_with(HashSet::new).insert(node1);
        }
        cave
    }
}
impl str::FromStr for CaveMap {
    type Err = ParseCaveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s.lines().enumerate().map(|(i, line)| {
            let line_number = i + 1;
            let mut nodes = line.split('-');
            if nodes.clone().count() != 2 {
                return Err(ParseCaveError::InvalidConnection(line_number));
            }

            let node1 = nodes.next().unwrap().trim()
                .parse::<Node>()
                .map_err(|e| (line_number, e))?;
            let node2 = nodes.next().unwrap().trim()
                .parse::<Node>()
                .map_err(|e| (line_number, e))?;
            Ok((node1, node2))
        })
        .collect::<Result<CaveMap, _>>();

        if let Ok(ref cave) = res {
            if !cave.0.contains_key(&Node::Start) { return Err(ParseCaveError::MissingStart); }
            if !cave.0.contains_key(&Node::End)   { return Err(ParseCaveError::MissingEnd); }
        }
        res
    }
}
impl CaveMap {
    fn count_paths_p1(&self) -> u32 {
        let mut path_count = 0;
        let mut visited_small_caves = HashSet::new();
        let mut stack: Vec<StackElem> = self.0.get(&Node::Start).unwrap().iter()
            .map(StackElem::NextNode)
            .collect();

        while let Some(elem) = stack.pop() {
            match elem {
                StackElem::StackDiv(node) => {
                    // The presence of `StackDiv(node)` signifies that all paths further down from `node` have
                    // been traversed. Before moving ot the next branch at the same depth as `node`, we will
                    // remove the record of visiting it from `visited_small_caves`.
                    // ! `StackDiv` will only appear for `node == Node::SmallCave(_)`
                    visited_small_caves.remove(node);
                    continue;
                },

                StackElem::NextNode(node) => {
                    match node {
                        Node::End => {
                            path_count += 1;
                            continue;
                        },
                        Node::SmallCave(_) => {
                            // make a record of visiting the SmallCave, and push on its corresponding StackDiv
                            visited_small_caves.insert(node);
                            stack.push(StackElem::StackDiv(node));
                        },
                        _ => (),
                    }
                    // push all valid next paths onto the stack
                    stack.extend(
                    self.0.get(node).unwrap().iter()
                            .filter(|x| (x != &&Node::Start) && (!visited_small_caves.contains(x)))
                            .map(StackElem::NextNode)
                    );
                }
            }
        }
        path_count
    }

    fn count_paths_p2(&self) -> u32 {
        let mut path_count = 0;
        let mut visited_small_caves: HashMap<&Node, u8> = HashMap::new();
        let mut second_visited = None;
        let mut stack: Vec<StackElem> = self.0.get(&Node::Start).unwrap().iter()
            .map(StackElem::NextNode)
            .collect();

        while let Some(elem) = stack.pop() {
            match elem {
                StackElem::StackDiv(node) => {
                    // The presence of `StackDiv(node)` signifies that all paths further down from `node` have
                    // been traversed. Before moving on to the next branch at the same depth as `node`, we will
                    // decrement the count of visiting it in `visited_small_caves`.
                    // ! `StackDiv` will only appear for `node == Node::SmallCave(_)`
                    *visited_small_caves.get_mut(node).unwrap() -= 1;
                    if second_visited == Some(node) { second_visited = None; }
                    continue;
                },

                StackElem::NextNode(node) => {
                    match node {
                        Node::End => {
                            path_count += 1;
                            continue;
                        },
                        Node::SmallCave(_) => {
                            let record = visited_small_caves.entry(node).or_insert(0);
                            *record += 1;
                            if *record == 2 { second_visited = Some(node); }
                            stack.push(StackElem::StackDiv(node));
                        },
                        _ => (),
                    }
                    // push all valid next paths onto the stack
                    if second_visited == None {
                        stack.extend(
                        self.0.get(node).unwrap().iter()
                                .filter(|x|
                                    (x != &&Node::Start) && (visited_small_caves.get(x).unwrap_or(&0) != &2)
                                )
                                .map(StackElem::NextNode)
                        );
                    }
                    else {
                        stack.extend(
                        self.0.get(node).unwrap().iter()
                                .filter(|x|
                                    (x != &&Node::Start) && (visited_small_caves.get(x).unwrap_or(&0) == &0)
                                )
                                .map(StackElem::NextNode)
                        );
                    }
                }
            }
        }
        path_count
    }
}

enum StackElem<'a> {
    NextNode(&'a Node), StackDiv(&'a Node)
}

pub fn day12_main(file_data: &str) -> (u32, u32) {
    let cave = file_data.parse::<CaveMap>().unwrap_or_else(|e| {
        if let Some(src) = e.source() { panic!("Error parsing cave! : {},  caused by \"{}\"", e, src); }
        else { panic!("Error parsing cave! : {}", e); }
    });
    let p1_path_count = cave.count_paths_p1();
    println!("[Part 1] The number of all possible paths is {}.", p1_path_count);
    let p2_path_count = cave.count_paths_p2();
    println!("[Part 2] The number of all possible paths is {}.", p2_path_count);

    (p1_path_count, p2_path_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let test_data =
            "start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end";

        assert_eq!(day12_main(test_data), (10, 36));
    }

    #[test]
    fn it_works_2() {
        let test_data =
            "dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc";

        assert_eq!(day12_main(test_data), (19, 103));
    }

    #[test]
    fn it_works_3() {
        let test_data =
            "fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW";

        assert_eq!(day12_main(test_data), (226, 3509));
    }
}