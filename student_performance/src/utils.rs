// this utils module defines the Student struct and provides methods to encode student data for machine learning and classification.

use serde::Deserialize;


// the struct Student represents a student with various personal and academic attributes.
// it's also used for encoding features and categorizing exam scores.

#[derive(Debug, Deserialize, Clone)]
pub struct Student {
    pub student_id: String,
    pub age: f64,
    pub gender: String,
    pub study_hours_per_day: f64,
    pub social_media_hours: f64,
    pub netflix_hours: f64,
    pub part_time_job: String,
    pub attendance_percentage: f64,
    pub sleep_hours: f64,
    pub diet_quality: String,
    pub exercise_frequency: f64,
    pub parental_education_level: String,
    pub internet_quality: String,
    pub mental_health_rating: f64,
    pub extracurricular_participation: String,
    pub exam_score: f64,
}

impl Student {
    // converts raw exam score into a category: 0 = below 60, 1 = 60–79, 2 = 80+
    pub fn to_category(&self) -> usize {
        match self.exam_score {
            s if s < 60.0 => 0,
            s if s < 80.0 => 1,
            _ => 2,
        }
    }

    // this function converts all student attributes into a numerical vector. it's used as input for machine learning models
    pub fn encode_features(&self) -> Vec<f64> {
        vec![
            self.age,
            encode_gender(&self.gender),
            self.study_hours_per_day,
            self.social_media_hours,
            self.netflix_hours,
            encode_bool(&self.part_time_job),
            self.attendance_percentage,
            self.sleep_hours,
            encode_diet(&self.diet_quality),
            self.exercise_frequency,
            encode_education(&self.parental_education_level),
            encode_internet(&self.internet_quality),
            self.mental_health_rating,
            encode_bool(&self.extracurricular_participation),
        ]
    }
}



// below are encoding functions which convert string-based fields into numerical values for use in the model input

fn encode_gender(g: &str) -> f64 {
    match g.to_lowercase().as_str() {
        "male" => 0.0,
        "female" => 1.0,
        _ => 0.5,
    }
}

fn encode_bool(b: &str) -> f64 {
    match b.to_lowercase().as_str() {
        "yes" => 1.0,
        "no" => 0.0,
        _ => 0.5,
    }
}

fn encode_diet(d: &str) -> f64 {
    match d.to_lowercase().as_str() {
        "poor" => 0.0,
        "fair" => 0.5,
        "good" => 1.0,
        _ => 0.5,
    }
}

fn encode_education(e: &str) -> f64 {
    match e.to_lowercase().as_str() {
        "high school" => 0.0,
        "bachelor" => 0.5,
        "master" => 1.0,
        _ => 0.5,
    }
}

fn encode_internet(i: &str) -> f64 {
    match i.to_lowercase().as_str() {
        "poor" => 0.0,
        "average" => 0.5,
        "good" => 1.0,
        _ => 0.5,
    }
}
