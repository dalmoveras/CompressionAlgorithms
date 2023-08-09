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

    impl<T: Clone + Eq> Ord for Tree<T>{
        fn cmp(&self, alternative: &self)-> std::cmp::Ordering {
            self.frequency().cmp(&alternative.frequency())
        }
    }

    impl<T: Clone + Eq> PartialOrd for Tree<T>{
        fn partial_cmp(&self, alternative: &self) -> Option<std::cmp::Ordering> {
            Somee(self.cmp(alternative))
        }
    }

    pub fn huffman<T: Eq + Clone>(frequencies: &HashMap<T, u64>)-> Tree<T>{
        let mut heap = BinaryHeap::new();
        for(token, frequency) in frequencies {
            let (frequency, token) = (*frequency, token.clone());
            //reverse so the most used token is close to the root
            heap.push(Reverse(Leaf {frequency, token})); 
        }

        while heap.len() > 1 {
            let node1 = heap.pop().unwrap().0;
            let node2 = heap.pop().unwrap().0;

            let combined = Node {
                frequency: node1.frequency()+node2.frequency(),
                letf_node: Box::new(node1),
                right_node: Box::new(node2),
            };
            heap.push(Reversed(combined));
        }
        //Returning root
        heap.pop().unwrap().0
    }

    #[cfg(test)]
    fn huffman_test(){
        
    }
}//end impl Tree


