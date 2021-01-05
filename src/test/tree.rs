#[cfg(test)]
 mod tests {
    use rspec;

    use crate::tree::*;

    const ID_INVALID: u32 = 1000;

    #[test]
    pub fn suite() {
        #[derive(Debug)]
        struct Environment {
            id1: u32,
            id2: u32,
            id3: u32,
            id4: u32,
            id5: u32,
            insert_result: Result<u32>,
            remove_result: Result<Vec<u32>>,
            child_node: Node,
            parent_node: Node,
            instance: Tree,
        }

        impl Clone for Environment {
            fn clone(&self) -> Self {
                Environment {
                    id1: self.id1,
                    id2: self.id2,
                    id3: self.id3,
                    id4: self.id4,
                    id5: self.id5,
                    insert_result: self.insert_result.clone(),
                    remove_result: self.remove_result.clone(),
                    child_node: self.child_node.clone(),
                    parent_node: self.child_node.clone(),
                    instance: self.instance.clone(),
                }
            }
        }

        impl Default for Environment {
            fn default() -> Self {
                Environment {
                    id1: 0,
                    id2: 0,
                    id3: 0,
                    id4: 0,
                    id5: 0,
                    insert_result: Ok(0),
                    remove_result: Ok(vec![]),
                    child_node: Node::default(),
                    parent_node: Node::default(),
                    instance: Tree::default(),
                }
            }
        }

        rspec::run(&rspec::describe("tree", Environment::default(), |ctx| {
            ctx.before_each(|env| {
                env.instance = Tree::new();
            });

            ctx.when("adding a node", |ctx| {
                ctx.before_each(|env| {
                    env.id1 = env.instance.add();
                });

                ctx.it("returns the correct ID", |env|
                    assert_eq!(env.id1, 1));

                ctx.it("is associated with the correct index", |env|
                    assert_eq!(env.instance.get_index(env.id1), Ok(0)));

                ctx.it("adds the node to the arena", |env|
                    assert_eq!(env.instance.arena(), &vec![
                        Node::new(1, None, Vec::new()),
                    ]));

                ctx.when("when removing an invalid node", |ctx| {
                    ctx.before_each(|env| {
                        env.remove_result = env.instance.remove(ID_INVALID);
                    });

                    ctx.it("returns an error", |env|
                        assert_eq!(env.remove_result, Err(Error::InvalidNodeId)));
                });

                ctx.when ("when removing a node", |ctx| {
                    ctx.before_each( |env| {
                        env.remove_result = env.instance.remove(env.id1);
                    });
    
                    ctx.it("returns the IDs of the removed nodes", |env|
                        assert_eq!(env.remove_result, Ok(vec![env.id1])));
    
                    ctx.it("removes it from the arena", |env|
                        assert_eq!(env.instance.arena(), &vec![]));
                });

                ctx.when("adding a second node", |ctx| {
                    ctx.before_each(|env| {
                        env.id2 = env.instance.add();
                    });
        
                    ctx.it("returns the correct ID", |env|
                        assert_eq!(env.id2, 2));
    
                    ctx.it("adds the node to the arena", |env|
                        assert_eq!(env.instance.arena(), &vec![
                            Node::new(env.id1, None, Vec::new()),
                            Node::new(env.id2, None, Vec::new()),
                        ]));
    
                    ctx.when("inserting a sub-node into an invalid node", |ctx| {
                        ctx.before_each(|env| {
                            env.insert_result = env.instance.insert(ID_INVALID);
                        });
            
                        ctx.it("returns an error", |env|
                            assert_eq!(env.insert_result, Err(Error::InvalidNodeId)));
                    });
    
                    ctx.when("adding a sub-node", |ctx| {
                        ctx.before_each(|env| {
                            env.id3 = env.instance.insert(env.id2).unwrap();
                            env.child_node = env.instance.get_node(env.id3).unwrap();
                            env.parent_node = env.instance.get_node(env.id2).unwrap();
                        });
        
                        ctx.it("returns the correct ID", |env|
                            assert_eq!(env.id3, 3));
        
                        ctx.it("has a parent ID", |env|
                            assert_eq!(env.child_node.parent_id(), Some(env.id2)));
        
                        ctx.it("adds the node as a child to its parent", |env|
                            assert_eq!(env.parent_node.child_ids(), &vec![env.id3]));
        
                        ctx.it ("adds the node to the arena", |env|
                            assert_eq!(env.instance.arena(), &vec![
                                Node::new(env.id1, None, Vec::new()),
                                Node::new(env.id2, None, vec![env.id3]),
                                Node::new(env.id3, Some(env.id2), Vec::new()),
                            ]));
    
                        ctx.when("adding a second sub-node", |ctx| {
                            ctx.before_each(|env| {
                                env.id4 = env.instance.insert(env.id2).unwrap();
                                env.child_node = env.instance.get_node(env.id4).unwrap();
                                env.parent_node = env.instance.get_node(env.id2).unwrap();
                            });
            
                            ctx.it("returns the correct ID", |env|
                                assert_eq!(env.id4, 4));
            
                            ctx.it("has a parent ID", |env|
                                assert_eq!(env.child_node.parent_id(), Some(env.id2)));
            
                            ctx.it("adds the node as a child to its parent", |env|
                                assert_eq!(env.parent_node.child_ids(), &vec![env.id3, env.id4]));
            
                            ctx.it("adds the node to the arena", |env|
                                assert_eq!(env.instance.arena(), &vec![
                                    Node::new(env.id1, None, Vec::new()),
                                    Node::new(env.id2, None, vec![env.id3, env.id4]),
                                    Node::new(env.id3, Some(env.id2), Vec::new()),
                                    Node::new(env.id4, Some(env.id2), Vec::new()),
                                ]));
    
                            ctx.when("removing a leaf-most node", |ctx| {
                                ctx.before_each(|env| {
                                    env.remove_result = env.instance.remove(env.id3);
                                });
    
                                ctx.it("returns the IDs of the removed nodes", |env|
                                    assert_eq!(env.remove_result, Ok(vec![env.id3])));
                
                                ctx.it("removes it from the arena", |env|
                                    assert_eq!(env.instance.arena(), &vec![
                                        Node::new(env.id1, None, Vec::new()),
                                        Node::new(env.id2, None, vec![env.id4]),
                                        Node::new(env.id4, Some(env.id2), Vec::new()),
                                    ]));
                            });
    
                            ctx.when("removing a node that has children", |ctx| {
                                ctx.before_each(|env| {
                                    env.remove_result = env.instance.remove(env.id2);
                                });
    
                                ctx.it("returns the IDs of the target node and all children", |env|
                                    assert_eq!(env.remove_result, Ok(vec![env.id2, env.id3, env.id4])));
                
                                ctx.it("removes it and its children from the arena", |env|
                                    assert_eq!(env.instance.arena(), &vec![
                                        Node::new(env.id1, None, Vec::new()),
                                    ]));
                            });
    
                            ctx.when("adding another top-level node", |ctx| {
                                ctx.before_each(|env| {
                                    env.id5 = env.instance.add();
                                });
    
                                ctx.it("adds the node to the arena", |env|
                                    assert_eq!(env.instance.arena(), &vec![
                                        Node::new(env.id1, None, Vec::new()),
                                        Node::new(env.id2, None, vec![env.id3, env.id4]),
                                        Node::new(env.id3, Some(env.id2), Vec::new()),
                                        Node::new(env.id4, Some(env.id2), Vec::new()),
                                        Node::new(env.id5, None, Vec::new()),
                                    ]));
    
                                ctx.it("is associated with the correct index", |env|
                                    assert_eq!(env.instance.get_index(env.id5), Ok(4)));
    
                                ctx.when("when removing a sub-node", |ctx| {
                                    ctx.before_each(|env| {
                                        env.instance.remove(env.id2).unwrap();
                                    });
    
                                    ctx.it("is associated with the correct index", |env|
                                        assert_eq!(env.instance.get_index(env.id5), Ok(1)));
                                });
                            });
                        });
                    });
                });
            });
        }));
    }
 }
 