// this graph module builds a similarity graph of students based on habits and uses BFS to find behaviorally similar peers.

use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::{Bfs, IntoNeighbors, VisitMap, Visitable};
use crate::utils::Student;
use std::collections::HashMap;


// this function called create_similarity_graph builds a graph where each node is a student, and the edges connect students with similar habits.
// input: list of students
// output: graph of students + map from student_id to node index

// high-level logic: students are connected if their Euclidean distance is < 5.0, meaning they are considered "similar.
// the edge weights store the distance (lower = more similar).

pub fn create_similarity_graph(students: &[Student]) -> (Graph<Student, f32>, HashMap<String, NodeIndex>) {
    let mut graph = Graph::<Student, f32>::new();
    let mut node_map = HashMap::new();

    // adds each student as a graph node
    for student in students {
        let node_index = graph.add_node(student.clone());
        node_map.insert(student.student_id.clone(), node_index);
    }
    
    // adds edges between similar students
    for (i, student_a) in students.iter().enumerate() {
        for student_b in &students[i + 1..] {
            let distance = euclidean_distance(student_a, student_b);
            if distance < 5.0 { // only connects students who are similar (threshold = 5.0)
                graph.add_edge(
                    *node_map.get(&student_a.student_id).unwrap(),
                    *node_map.get(&student_b.student_id).unwrap(),
                    distance, // distance is used as edge weight
                );
            }
        }
    }

    (graph, node_map)
}


// this function called the euclidean_distance calculates similarity between two students based on encoded features. lower distance = more similar.
// input: two students
// output: Euclidean distance between them

fn euclidean_distance(student_a: &Student, student_b: &Student) -> f32 {
    let a = student_a.encode_features();
    let b = student_b.encode_features();

    let sum = a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>();

    (sum as f32).sqrt()
}


// this function run_bfs_from_start runs a breadth-first search (BFS) starting from a specific student node.
// input: graph and starting node
// output: list of similar students found through BFS

// high-level logic: BFS is used to explore all students connected/"similar" to the selected one in the similarity graph.

pub fn run_bfs_from_start(graph: &Graph<Student, f32>, start: NodeIndex) -> Vec<Student> {
    let mut visited = graph.visit_map();
    let mut bfs = Bfs::new(graph, start);
    let mut result = Vec::new();

    while let Some(node) = bfs.next(graph) {
        visited.visit(node);
        if let Some(student) = graph.node_weight(node) {
            result.push(student.clone());
        }
    }

    result
}

