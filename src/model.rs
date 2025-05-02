// this model module trains the model using decision tree algorithm. this module contains the logic for training a decision tree model based on the student dataset,
// where the features are student habits and the target labels are exam score categories.


use linfa::prelude::*;
use linfa_trees::DecisionTree;
use ndarray::{Array1, Array2};
use crate::utils::Student;


// struct TrainedModel: represents a decision tree model that has been trained using the student dataset; it contains the trained decision tree and is used for making predictions.
pub struct TrainedModel { 
    model: DecisionTree<f64, usize>,
}

// this function trains a decision tree using student data
// inputs: slice of Student structs
// output: TrainedModel (or error)
pub fn train_model(students: &[Student]) -> Result<TrainedModel, Box<dyn std::error::Error>> {
    // extracts features and labels
    let features = students.iter().map(|s| s.encode_features()).collect::<Vec<_>>();
    let labels = students.iter().map(|s| s.to_category()).collect::<Vec<_>>();

    // convert to arrays (14 features per student)
    let feature_array = Array2::from_shape_vec((students.len(), 14), features.into_iter().flatten().collect())?;
    let label_array = Array1::from_vec(labels);

    // trains the decision tree with specified parameters
    let model = DecisionTree::params()
        .max_depth(Some(5))
        .min_weight_split(10.0)
        .min_weight_leaf(5.0)
        .fit(&Dataset::new(feature_array, label_array))?;

    Ok(TrainedModel { model })
}

// this function predicts the exam category for a single student
// inputs: trained model, student
// output: predicted category (0, 1, or 2)
pub fn predict(model: &TrainedModel, student: &Student) -> usize {
    let input = Array2::from_shape_vec((1, 14), student.encode_features()).unwrap();
    model.model.predict(&input)[0]
}


