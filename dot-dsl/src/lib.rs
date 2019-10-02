pub mod graph {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(self, att: &[(&str, &str)]) -> Self {
            Graph {
                nodes: self.nodes,
                edges: self.edges,
                attrs: att.iter().map(|&(a, b)| (a.to_string(), b.to_string())).collect(),
            }
        }

        pub fn with_nodes(self, nodes: &Vec<graph_items::node::Node>) -> Self {
            Graph {
                nodes: nodes.clone().to_vec(),
                edges: self.edges,
                attrs: self.attrs,
            }
        }

        pub fn with_edges(self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            Graph {
                nodes: self.nodes,
                edges: edges.clone().to_vec(),
                attrs: self.attrs,
            }
        }

        pub fn get_node(&self, node: &str) -> Option<&graph_items::node::Node> {
            for n in self.nodes.iter() {
                if n.is(node) {
                    return Some(&n);
                }
            }
            None
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                node1: String,
                node2: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Edge {
                        node1: node1.to_string(),
                        node2: node2.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, att: &[(&str, &str)]) -> Self {
                    Edge {
                        node1: self.node1,
                        node2: self.node2,
                        attrs: att.iter().map(|&(a, b)| (a.to_string(), b.to_string())).collect(),
                    }
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;
            
            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                label: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(l: &str) -> Self {
                    Node {
                        label: l.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, att: &[(&str, &str)]) -> Self {
                    Node {
                        label: self.label,
                        attrs: att.iter().map(|&(a, b)| (a.to_string(), b.to_string())).collect(),
                    }
                }

                pub fn is(&self, l: &str) -> bool {
                    self.label == l.to_string()
                }

                pub fn get_attr(&self, att: &str) -> Option<&str> {
                    if let Some(ret) = self.attrs.get(att) {
                        Some(ret)
                    } else {
                        None
                    }
                }
            }
        }
    }
}
