use std::error::Error;
use std::io::{self, Write};

use csv::ReaderBuilder;

use crate::model::{predict, train_model};
use crate::utils::Student;
use crate::graph::{create_similarity_graph, run_bfs_from_start};
use petgraph::graph::NodeIndex;

mod utils;
mod model;
mod graph;

