use crate::tree::*;

use speculate::speculate;

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
                    Node::new(1, None, Vec::new()),
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
                        Node::new(id1, None, Vec::new()),
                        Node::new(id2, None, Vec::new()),
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
                            Node::new(id1, None, Vec::new()),
                            Node::new(id2, None, vec![id3]),
                            Node::new(id3, Some(id2), Vec::new()),
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
                                Node::new(id1, None, Vec::new()),
                                Node::new(id2, None, vec![id3, id4]),
                                Node::new(id3, Some(id2), Vec::new()),
                                Node::new(id4, Some(id2), Vec::new()),
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
                                    Node::new(id1, None, Vec::new()),
                                    Node::new(id2, None, vec![id4]),
                                    Node::new(id4, Some(id2), Vec::new()),
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
                                    Node::new(id1, None, Vec::new()),
                                ]);
                            }
                        }

                        context "when adding another top-level node" {
                            before {
                                let id5 = tree.add();
                            }

                            it "adds the node to the arena" {
                                assert_eq!(tree.arena(), &vec![
                                    Node::new(id1, None, Vec::new()),
                                    Node::new(id2, None, vec![id3, id4]),
                                    Node::new(id3, Some(id2), Vec::new()),
                                    Node::new(id4, Some(id2), Vec::new()),
                                    Node::new(id5, None, Vec::new()),
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
