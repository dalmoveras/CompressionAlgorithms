use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use Tree::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree<T> {
    Leaf{
        frequency: u64,
        token: T,
    },
    Node {
        frequency: u64,
        left_node: Box<Tree<T>>,
        right_node: Box<Tree<T>>,
    },
}

impl<T: Clone> Tree<T>{
    pub fn frequency(&self) -> u64 {
        match self {
            Leaf {frequency, ..} => *frequency,
            Node {frequency, ..} => *frequency,
        }
    }

    pub fn token(&self) -> Option<T> {
        match self {
            Leaf {token, ..} => Some(token.clone()),
            Node { .. } => None,
        }
    }

    pub fn left(&self) -> Option<&Tree<T>> {
        match self {
            Node {left_node, ..}=> Some(left_node),
            Leaf { .. } => None,
        }
    }

    pub fn right(&self) -> Option<&Tree<T>> {
        match self {
            Node {right_node, ..}=> Some(right_node),
            Leaf { .. } => None,
        }
    }
}//end impl Tree


