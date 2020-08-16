use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Node {
    id: u32,
    parent_id: Option<u32>,
    children: Vec<u32>,
}

impl Node {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn children(&self) -> &Vec<u32> {
        &self.children
    }
}

#[derive(Default)]
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
            children: Vec::new(),
        });

        self.next_id
    }

    pub fn insert(&mut self, parent_id: u32) -> Option<u32> {
        if let Some(index) = self.get_index(parent_id) {
            self.next_id += 1;
            self.arena[index as usize].children.push(self.next_id);
            self.arena.push(Node {
                id: self.next_id,
                parent_id: Some(parent_id),
                children: Vec::new(),
            });

            return Some(self.next_id)
        }

        None
    }

    pub fn remove(&mut self, id: u32) -> Option<Vec<u32>> {
        if let Some(index) = self.get_index(id) {
            let start_index = index as usize;
            let mut removed_indices = vec![index];

            for (index, node) in self.arena[start_index..].iter().enumerate() {
                let index = (index + start_index) as u32;

                if removed_indices.contains(&index) {
                    let indices: Vec<u32> = node.children
                        .iter()
                        .map(|&id| self.get_index(id).unwrap())
                        .collect();

                    removed_indices = [&removed_indices[..], &indices[..]].concat();
                }
            }

            self.arena = self.arena
                .iter()
                .enumerate()
                .filter(|&(index, _)| {
                    let i = index as u32;

                    !removed_indices.contains(&i)
                })
                .map(|(_, e)| {
                    let children: Vec<u32> = e.children
                        .iter()
                        .filter(|&child_id| *child_id != id)
                        .map(|e| e.clone())
                        .collect();

                    Node {
                        children,
                        ..e.clone()
                    }
                })
                .collect();

            Some(removed_indices)
        } else {
            None
        }
    }

    pub fn get_index(&self, id: u32) -> Option<u32> {
        for (index, node) in self.arena.iter().enumerate() {
            if id == node.id() {
                return Some(index as u32)
            }
        }

        None
    }

    pub fn get_node(&self, id: u32) -> Option<Node> {
        for node in self.arena() {
            if node.id() == id {
                return Some(node.clone())
            }
        }

        None
    }

    pub fn arena (&self) -> &Vec<Node> {
        &self.arena
    }

    fn print_children(&self, node: &Node, depth: usize) {
        let nodes: Vec<Node> = node.children
            .iter()
            .map(|&id| self.get_node(id).unwrap())
            .collect();
    
        for node in nodes {
            if node.parent_id != None {
                println!("{:indent$}{} | {:?}", "", node.id(), node.children, indent = depth * 2);
    
                if node.children.len() > 0 {
                    self.print_children(&node, depth + 1);
                }
            }
        }
    }
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n").unwrap();

        for node in self.arena() {
            if node.parent_id == None {
                write!(f, "{} | {:?}\n", node.id(), node.children).unwrap();
    
                if node.children.len() > 0 {
                    self.print_children(node, 1);
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let mut tree = Tree::new();

    tree.add();
    tree.add();

    let id1 = tree.add();
    tree.insert(id1);
    tree.insert(id1);

    let id2 = tree.insert(id1).unwrap();
    tree.insert(id2);
    tree.insert(id2);
    tree.insert(id2);
    tree.insert(id2);

    tree.insert(id1);

    println!("tree: {:?}", tree);

    tree.remove(1);
    tree.remove(6);
    println!("tree: {:?}", tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_a_node() {
        let mut tree = Tree::new();
        let id1 = tree.add();

        assert_eq!(id1, 1);
        assert_eq!(tree.get_index(id1), Some(0));
    }

    #[test]
    fn it_adds_nodes() {
        let mut tree = Tree::new();
        let id1 = tree.add();
        let id2 = tree.add();
        let id3 = tree.add();
        let id4 = tree.add();

        assert_eq!(id1, 1);
        assert_eq!(tree.get_index(id1), Some(0));

        assert_eq!(id2, 2);
        assert_eq!(tree.get_index(id2), Some(1));

        assert_eq!(id3, 3);
        assert_eq!(tree.get_index(id3), Some(2));

        assert_eq!(id4, 4);
        assert_eq!(tree.get_index(id4), Some(3));
    }

    #[test]
    fn it_does_not_add_sub_node_for_invalid_parent_id() {
        const ID_INVALID: u32 = 1000;

        let mut tree = Tree::new();

        let sub_id_none = tree.insert(ID_INVALID);

        assert_eq!(sub_id_none, None);
    }

    #[test]
    fn it_adds_sub_nodes_to_valid_parent_node() {
        let mut tree = Tree::new();

        tree.add();
        tree.add();
        let id = tree.add();

        let sub_id1 = tree.insert(id);
        let sub_id2 = tree.insert(id);
        let sub_id3 = tree.insert(id);
        let node = tree.get_node(id).unwrap();

        let id2 = tree.add();

        assert_eq!(id, 3);
        assert_eq!(sub_id1, Some(4));
        assert_eq!(sub_id2, Some(5));
        assert_eq!(sub_id3, Some(6));
        assert_eq!(node.children(), &vec![4, 5, 6]);
        assert_eq!(id2, 7);
    }

    #[test]
    fn it_removes_a_node() {
        let mut tree = Tree::new();

        let id1 = tree.add();
        let id2 = tree.add();
        let id3 = tree.add();
        let removed_indices = tree.remove(id3);

        assert_eq!(removed_indices, Some(vec![2]));
        assert_eq!(tree.get_index(id1), Some(0));
        assert_eq!(tree.get_index(id2), Some(1));
        assert_eq!(tree.get_index(id3), None);
    }

    #[test]
    fn it_removes_a_sub_node_and_children() {
        let mut tree = Tree::new();

        let id1 = tree.add();
        let id2 = tree.add();
        let id3 = tree.add();

        let id4 = tree.insert(id3).unwrap();
        let id5 = tree.insert(id3).unwrap();
        let id6 = tree.insert(id3).unwrap();

        let id7 = tree.insert(id6).unwrap();
        let id8 = tree.insert(id6).unwrap();
        let id9 = tree.insert(id6).unwrap();

        let id10 = tree.insert(id3).unwrap();

        let removed_indices = tree.remove(id6);

        assert_eq!(removed_indices, Some(vec![5, 6, 7, 8]));
        assert_eq!(tree.get_index(id1), Some(0));
        assert_eq!(tree.get_index(id2), Some(1));
        assert_eq!(tree.get_index(id3), Some(2));
        assert_eq!(tree.get_index(id4), Some(3));
        assert_eq!(tree.get_index(id5), Some(4));
        assert_eq!(tree.get_index(id6), None);
        assert_eq!(tree.get_index(id7), None);
        assert_eq!(tree.get_index(id8), None);
        assert_eq!(tree.get_index(id9), None);
        assert_eq!(tree.get_index(id10), Some(5));
    }
}
