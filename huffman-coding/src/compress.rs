use bit_vec::BitVec;
use rayon::prelude::*;
use rmp_serde;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};


use crate::huffman-coding::{self, Tree};
use Tree::*;
