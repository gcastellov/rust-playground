pub mod graph {

    use std::collections::HashMap;

    pub mod graph_items {

        pub mod node {

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: Vec<(String, String)>
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: Vec::default()
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    let mut node = self.clone();
                    
                    attrs
                        .iter()
                        .map(|(key, value)|(key.to_string(), value.to_string()))
                        .for_each(|tuple|node.attrs.push(tuple));

                    node
                }

                pub fn get_attr(&self, key_attr: &str) -> Option<&str> {
                    if let Some((_, value)) = self.attrs.iter().find(|(key, _)|*key == key_attr.to_string()) {
                        Some(value.as_str())
                    } else {
                        None
                    }
                }
            }
        }

        pub mod edge {

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: Vec<(String, String)>
            }
    
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: Vec::default()
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    let mut edge = self.clone();
                    attrs
                        .iter()
                        .map(|(key, value)|(key.to_string(), value.to_string()))
                        .for_each(|tuple|edge.attrs.push(tuple));

                    edge
                }
            }
        }
    }

    #[derive(Clone)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>
    }
    
    impl Graph {
        pub fn new() -> Self {
            Graph { 
                nodes: Vec::default(),
                edges: Vec::default(),
                attrs: HashMap::default()
            }
        }

        pub fn with_nodes(&self, nodes: &Vec<graph_items::node::Node>) -> Self {
            let mut graph = self.clone();
            nodes
                .iter()
                .map(|node|node.clone())
                .for_each(|node|graph.nodes.push(node));

            graph
        }

        pub fn with_edges(&self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            let mut graph = self.clone();
            edges
                .iter()
                .map(|edge|edge.clone())
                .for_each(|edge|graph.edges.push(edge));

            graph
        }

        pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
            let mut graph = self.clone();
            attrs
                .iter()
                .map(|(key, value)|(key.to_string(), value.to_string()))
                .for_each(|(key, value)|{graph.attrs.insert(key, value);});

            graph
        }

        pub fn get_node(&self, node: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|n|n.name == node.to_string())
        }
    }
}

