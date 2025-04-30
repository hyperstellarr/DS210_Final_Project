use serde::Deserialize;

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
    pub fn to_category(&self) -> usize {
        match self.exam_score {
            s if s < 60.0 => 0,
            s if s < 80.0 => 1,
            _ => 2,
        }
    }


