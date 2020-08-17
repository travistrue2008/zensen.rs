use std::vec::Vec;
use std::result;

use speculate::speculate;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidNodeId,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    id: u32,
    parent_id: Option<u32>,
    child_ids: Vec<u32>,
}

impl Node {
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

speculate! {
    describe "tree" {
        const ID_INVALID: u32 = 1000;

        before {
            let mut tree = Tree::new();
        }

        context "when adding a node" {
            before {
                let id1 = tree.add();
            }

            it "returns the correct ID" {    
                assert_eq!(id1, 1);
            }

            it "is associated with the correct index" {    
                assert_eq!(tree.get_index(id1), Ok(0));
            }

            it "adds the node to the arena" {
                assert_eq!(tree.arena(), &vec![
                    Node {
                        id: 1,
                        parent_id: None,
                        child_ids: Vec::new(),
                    },
                ]);
            }

            context "when removing an invalid node" {
                before {
                    let sub_id = tree.remove(ID_INVALID);
                }
    
                it "returns an error" {                
                    assert_eq!(sub_id, Err(Error::InvalidNodeId));
                }
            }

            context "when removing a node" {
                before {
                    let result = tree.remove(id1);
                }

                it "returns the IDs of the removed nodes" {
                    assert_eq!(result, Ok(vec![id1]));
                }

                it "removes it from the arena" {
                    assert_eq!(tree.arena(), &vec![]);
                }
            }

            context "when adding a second node" {
                before {
                    let id2 = tree.add();
                }
    
                it "returns the correct ID" {
                    assert_eq!(id2, 2);
                }

                it "adds the node to the arena" {
                    assert_eq!(tree.arena(), &vec![
                        Node {
                            id: id1,
                            parent_id: None,
                            child_ids: Vec::new(),
                        },
                        Node {
                            id: id2,
                            parent_id: None,
                            child_ids: Vec::new(),
                        },
                    ]);
                }

                context "when inserting a sub-node into an invalid node" {
                    before {
                        let sub_id = tree.insert(ID_INVALID);
                    }
        
                    it "returns an error" {                
                        assert_eq!(sub_id, Err(Error::InvalidNodeId));
                    }
                }

                context "when adding a sub-node" {
                    before {
                        let id3 = tree.insert(id2).unwrap();
                        let child_node = tree.get_node(id3).unwrap();
                        let parent_node = tree.get_node(id2).unwrap();
                    }
    
                    it "returns the correct ID" {
                        assert_eq!(id3, 3);
                    }
    
                    it "has a parent ID" {
                        assert_eq!(child_node.parent_id(), Some(id2));
                    }
    
                    it "adds the node as a child to its parent" {
                        assert_eq!(parent_node.child_ids(), &vec![id3]);
                    }
    
                    it "adds the node to the arena" {
                        assert_eq!(tree.arena(), &vec![
                            Node {
                                id: id1,
                                parent_id: None,
                                child_ids: Vec::new(),
                            },
                            Node {
                                id: id2,
                                parent_id: None,
                                child_ids: vec![id3],
                            },
                            Node {
                                id: id3,
                                parent_id: Some(id2),
                                child_ids: Vec::new(),
                            },
                        ]);
                    }

                    context "when adding a second sub-node" {
                        before {
                            let id4 = tree.insert(id2).unwrap();
                            let child_node = tree.get_node(id4).unwrap();
                            let parent_node = tree.get_node(id2).unwrap();
                        }
        
                        it "returns the correct ID" {
                            assert_eq!(id4, 4);
                        }
        
                        it "has a parent ID" {
                            assert_eq!(child_node.parent_id(), Some(id2));
                        }
        
                        it "adds the node as a child to its parent" {
                            assert_eq!(parent_node.child_ids(), &vec![id3, id4]);
                        }
        
                        it "adds the node to the arena" {
                            assert_eq!(tree.arena(), &vec![
                                Node {
                                    id: id1,
                                    parent_id: None,
                                    child_ids: Vec::new(),
                                },
                                Node {
                                    id: id2,
                                    parent_id: None,
                                    child_ids: vec![id3, id4],
                                },
                                Node {
                                    id: id3,
                                    parent_id: Some(id2),
                                    child_ids: Vec::new(),
                                },
                                Node {
                                    id: id4,
                                    parent_id: Some(id2),
                                    child_ids: Vec::new(),
                                },
                            ]);
                        }

                        context "when removing a leaf-most node" {
                            before {
                                let result = tree.remove(id3);
                            }

                            it "returns the IDs of the removed nodes" {
                                assert_eq!(result, Ok(vec![id3]));
                            }
            
                            it "removes it from the arena" {
                                assert_eq!(tree.arena(), &vec![
                                    Node {
                                        id: id1,
                                        parent_id: None,
                                        child_ids: Vec::new(),
                                    },
                                    Node {
                                        id: id2,
                                        parent_id: None,
                                        child_ids: vec![id4],
                                    },
                                    Node {
                                        id: id4,
                                        parent_id: Some(id2),
                                        child_ids: Vec::new(),
                                    },
                                ]);
                            }
                        }

                        context "when removing a node that has children" {
                            before {
                                let result = tree.remove(id2);
                            }

                            it "returns the IDs of the target node and all children" {
                                assert_eq!(result, Ok(vec![id2, id3, id4]));
                            }
            
                            it "removes it and its children from the arena" {
                                assert_eq!(tree.arena(), &vec![
                                    Node {
                                        id: id1,
                                        parent_id: None,
                                        child_ids: Vec::new(),
                                    },
                                ]);
                            }
                        }

                        context "when adding another top-level node" {
                            before {
                                let id5 = tree.add();
                            }

                            it "adds the node to the arena" {
                                assert_eq!(tree.arena(), &vec![
                                    Node {
                                        id: id1,
                                        parent_id: None,
                                        child_ids: Vec::new(),
                                    },
                                    Node {
                                        id: id2,
                                        parent_id: None,
                                        child_ids: vec![id3, id4],
                                    },
                                    Node {
                                        id: id3,
                                        parent_id: Some(id2),
                                        child_ids: Vec::new(),
                                    },
                                    Node {
                                        id: id4,
                                        parent_id: Some(id2),
                                        child_ids: Vec::new(),
                                    },
                                    Node {
                                        id: id5,
                                        parent_id: None,
                                        child_ids: Vec::new(),
                                    },
                                ]);
                            }

                            it "is associated with the correct index" {
                                assert_eq!(tree.get_index(id5), Ok(4));
                            }

                            context "when removing a sub-node" {
                                before {
                                    tree.remove(id2).unwrap();
                                }

                                it "is associated with the correct index" {
                                    assert_eq!(tree.get_index(id5), Ok(1));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
