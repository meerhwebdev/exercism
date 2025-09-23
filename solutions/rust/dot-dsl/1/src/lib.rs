pub mod graph {
    use std::collections::HashMap;
    use graph_items::{edge::Edge, node::Node};

    pub mod graph_items {
        pub mod edge{
            use super::super::*;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                pub first:String,
                pub second:String,
                pub attrs:HashMap<String, String>
            }
            #[allow(dead_code)]
            impl Edge {
                pub fn new(first:&str, second:&str) -> Self {
                    Self {
                        first:first.into(),
                        second:second.into(),
                        attrs:HashMap::new()
                    }
                }

                pub fn attr(&self, key:&str) -> Option<&str> {
                    self.attrs.get(key).map(String::as_str)
                }

                pub fn with_attrs(mut self, attrs:&[(&str, &str)])->Self {
                    self.attrs = attrs.iter().map(|(k, v)|  (k.to_string(), v.to_string())).collect();
                    self
                }
            }
        }
        pub mod node {
            use super::super::*;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                pub name:String,
                pub attrs:HashMap<String, String>
            }
            #[allow(dead_code)]
            impl Node {
                pub fn new(name:&str) -> Self {
                    Self {
                        name:name.to_string(),
                        attrs:HashMap::new()
                    }
                }

                pub fn attr(&self, key:&str) -> Option<&str> {
                    self.attrs.get(key).map(String::as_str)
                }
                pub fn with_attrs(mut self, attrs:&[(&str, &str)])->Self {
                    self.attrs = attrs.iter().map(|(k, v)|  (k.to_string(), v.to_string())).collect();
                    self
                }
            }
        }
    }

    pub struct Graph {
        pub edges:Vec<Edge>,
        pub nodes:Vec<Node>,
        pub attrs:HashMap<String, String>
    }
    #[allow(dead_code)]
    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes:vec![],
                edges:vec![],
                attrs:HashMap::new(),
            }
        }
        pub fn node(&self, name:&str) -> Option<Node> {
            self.nodes.iter().find(|&node| node.name == name).cloned()
        }
        pub fn with_nodes(mut self, nodes:&[Node])->Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges:&[Edge])-> Self {
            self.edges = edges.to_vec();
            self
        }
        
        pub fn with_attrs(mut self, attrs:&[(&str, &str)])->Self {
            self.attrs = attrs.iter().map(|(k, v)|  (k.to_string(), v.to_string())).collect();
            self
        }
    }
}