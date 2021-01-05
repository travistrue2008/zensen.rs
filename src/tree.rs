use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    InvalidNodeId,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Node {
    id: u32,
    parent_id: Option<u32>,
    child_ids: Vec<u32>,
}

impl Node {
    pub fn new(id: u32, parent_id: Option<u32>, child_ids: Vec<u32>) -> Node {
        Node { id, parent_id, child_ids }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn parent_id(&self) -> Option<u32> {
        self.parent_id
    }

    pub fn child_ids(&self) -> &Vec<u32> {
        &self.child_ids
    }
}

#[derive(Default, Clone)]
pub struct Tree {
    next_id: u32,
    arena: Vec<Node>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            next_id: 0,
            arena: Vec::new(),
        }
    }

    pub fn add(&mut self) -> u32 {
        self.next_id += 1;
        self.arena.push(Node {
            id: self.next_id,
            parent_id: None,
            child_ids: Vec::new(),
        });

        self.next_id
    }

    pub fn insert(&mut self, parent_id: u32) -> Result<u32> {
        if let Ok(index) = self.get_index(parent_id) {
            self.next_id += 1;
            self.arena[index as usize].child_ids.push(self.next_id);
            self.arena.push(Node {
                id: self.next_id,
                parent_id: Some(parent_id),
                child_ids: Vec::new(),
            });

            return Ok(self.next_id)
        }

        Err(Error::InvalidNodeId)
    }

    pub fn remove(&mut self, id: u32) -> Result<Vec<u32>> {
        if let Ok(index) = self.get_index(id) {
            let start_index = index as usize;
            let node_iter = self.arena[start_index..].iter();
            let removed_ids = node_iter.fold(vec![id], |accum, curr| {
                if accum.contains(&curr.id) {
                    [&accum[..], &curr.child_ids[..]].concat()
                } else {
                    accum
                }
            });

            self.arena = self.arena
                .iter()
                .filter(|&item| !removed_ids.contains(&item.id))
                .map(|node| {
                    let child_ids: Vec<u32> = node.child_ids
                        .iter()
                        .filter(|&id| !removed_ids.contains(id))
                        .map(|id| id.clone())
                        .collect();

                    Node {
                        child_ids,
                        ..node.clone()
                    }
                })
                .collect();

            Ok(removed_ids)
        } else {
            Err(Error::InvalidNodeId)
        }
    }

    pub fn get_index(&self, id: u32) -> Result<u32> {
        for (index, node) in self.arena.iter().enumerate() {
            if id == node.id {
                return Ok(index as u32)
            }
        }

        Err(Error::InvalidNodeId)
    }

    pub fn get_node(&self, id: u32) -> Option<Node> {
        for node in self.arena() {
            if node.id == id {
                return Some(node.clone())
            }
        }

        None
    }

    pub fn arena (&self) -> &Vec<Node> {
        &self.arena
    }

    fn fmt_node (&self, f: &mut std::fmt::Formatter<'_>, node: &Node, depth: usize) {
        write!(f, "{:indent$}{} | {:?}\n", "", node.id, node.child_ids, indent = depth * 2).unwrap();

        node.child_ids
            .iter()
            .map(|&id| self.get_node(id).unwrap())
            .for_each(|node| self.fmt_node(f, &node, depth + 1));
    }
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n").unwrap();

        for node in self.arena() {
            if node.parent_id == None {    
                self.fmt_node(f, node, 1);
            }
        }

        Ok(())
    }
}
