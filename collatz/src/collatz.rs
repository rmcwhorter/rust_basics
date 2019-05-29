extern crate serde;
use serde::*;
use std::fmt;

pub struct Arena<T> {
    pub nodes: Vec<Node<T>>,
}

#[derive(Copy, Clone)]
pub struct NodeId {
    pub index: usize,
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.index)
    }
}

pub struct Node<T> {
    pub id: NodeId,
    pub child: Option<NodeId>,

    /// The actual data which will be stored within the tree
    pub data: T,
}

impl fmt::Display for Node<i128> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "ID: {} Data: {} Child's ID: {}", self.id, self.data, match &self.child {
            Some(x) => x.index,
            None => std::usize::MAX
        })
    }
}

impl Arena<i128> {
    pub fn new_node(&mut self, data: i128) -> NodeId {
        // Get the next free index
        let next_index = self.nodes.len();

        // Push the node into the arena
        self.nodes.push(Node {
            id: NodeId { index: next_index },
            child: None,
            data: data,
        });

        // Return the node identifier
        NodeId { index: next_index }
    }

    pub fn search_id(&mut self, value: i128) -> Option<NodeId> {
        for n in &self.nodes {
            if n.data == value {
                return Some(NodeId { index: n.id.index});
            }
        }
        return None;
    }

    pub fn search_bool(&mut self, value: i128) -> bool {
        for n in &self.nodes {
            if n.data == value {
                return true;
            }
        }
        return false;
    }

    pub fn augment(&mut self, mut value: i128, prior: Option<NodeId>) {
        let mut ident;
        if !self.search_bool(value) {
            //The value doesn't exist in the tree yet, so we have to add it.
            //From there, we will pass the new collatz number, as well as passing the optional 'prior' argument, so that we will link the old node to the next node.
            ident = self.new_node(value);
            if value % 2 == 0 {
                self.augment(value/2, Some(ident));
            } else {
                self.augment(3*value + 1, Some(ident));
            }
        } else {
            ident = self.search_id(value).unwrap();
        }
        match prior {
            Some(x) => self.nodes[x.index].child = Some(ident),
            None => (),
        }
    }
}



pub fn collatz(mut n: i128) -> Vec<i128> {
    let mut out: Vec<i128> = vec![];
    while n != 1 {
        out.push(n);
        if n % 2 == 0 {
            n = n/2;
        } else {
            n = 3*n + 1;
        }
    }
    out.push(1);
    return out;
}