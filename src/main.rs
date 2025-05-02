// this is the main file of the program. it loads student data, trains a prediction model,
// it also shows a simple text-based menu for user interaction.

use std::error::Error;
use std::io::{self, Write};

use csv::ReaderBuilder;

use crate::model::{predict, train_model};
use crate::utils::Student;
use crate::graph::{create_similarity_graph, run_bfs_from_start};
use petgraph::graph::NodeIndex;

// modules containing reusable code
mod utils;
mod model;
mod graph;


// main function: it loads student data from a CSV file. next, it trains a prediction model and shows a simple menu for the user to choose options like:
//  1. Predict a student's exam score
//  2. Show similar students based on habits
//  3. Exit the program

fn main() -> Result<(), Box<dyn Error>> {
    // loads the dataset
    let file_path = "student_habits_performance.csv";
    let students = load_students(file_path)?;

    // trains the decision tree model on the loaded student data
    let trained_model = train_model(&students)?;

    // loops to keep showing the menu until the user chooses to exit
    loop {
        println!("\n--- MENU ---");
        println!("1. Predict a student's exam score");
        println!("2. Show similar students (BFS Graph)");
        println!("3. Exit");
        print!("Enter your choice (1/2/3): ");
        io::stdout().flush()?; // this ensures the prompt appears before waiting for input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?; // reads the user input
        match choice.trim() {
            "1" => {
                // option 1: predicts student exam score
                println!("\n--- Enter student details ---");
                let new_student = get_student_from_input();
                let prediction = predict(&trained_model, &new_student);

                // shows the predicted score category in words
                let category = match prediction {
                    0 => "Below 60",
                    1 => "Between 60 and 79",
                    2 => "80 or higher",
                    _ => "Unknown",
                };
                println!(" Predicted exam score category: {}", category);
            }
            "2" => { // option 2 shows similar students using a graph
                println!("\n--- Student Similarity Graph ---");
                let (graph, node_map) = create_similarity_graph(&students);
                println!("Graph constructed with {} nodes and {} edges.", graph.node_count(), graph.edge_count());

                // asks the user which student to start from (from the dataset)
                print!("Enter a student ID to start BFS from: ");
                io::stdout().flush()?;
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input)?;
                let id_input = id_input.trim();

                // find similar students by walking through the graph
                if let Some(&start_index) = node_map.get(id_input) {
                    let bfs_result = run_bfs_from_start(&graph, start_index);
                    println!("\nSimilar students to {}:", id_input);
                    for student in bfs_result {
                        println!("- {}", student.student_id);
                    }
                } else {
                    println!(" Student ID not found.");
                }
            }
            "3" => {// option 3: exits the program
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),

        
        }
    }

    Ok(())
}

// this function loads students from a CSV file
// inputs: a file path pointing to the CSV file
// output: a list (vector) of Student objects
// high-level logic: it uses a CSV reader to go through each line in the file and turns each row into a Student and adds it to a list

fn load_students(file_path: &str) -> Result<Vec<Student>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut students = Vec::new();

    // goes through each row in the CSV and converts it to a Student
    for result in rdr.deserialize() {
        let student: Student = result?;
        students.push(student);
    }

    Ok(students)
}

// this function collects user input to create a new student (used for prediction).
// outputs a Student object with details entered by the user
// logic: asks for each student field one at a time (like age, gender, etc.) and converts text input to numbers when needed
fn get_student_from_input() -> Student {
    // this is a helper function for getting a number input
    fn prompt<T: std::str::FromStr>(message: &str) -> T {
        loop {
            print!("{}", message);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if let Ok(value) = input.trim().parse::<T>() {
                return value;
            } else {
                println!("Invalid input. Try again.");
            }
        }
    }

    // this is a helper function for getting a text input
    fn prompt_string(message: &str) -> String {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    // asks for student info and return it as a Student object
    Student {
        student_id: prompt_string("Student ID: "),
        age: prompt("Age: "),
        gender: prompt_string("Gender (Male/Female): "),
        study_hours_per_day: prompt("Study hours per day: "),
        social_media_hours: prompt("Social media hours per day: "),
        netflix_hours: prompt("Netflix hours per day: "),
        part_time_job: prompt_string("Part-time job? (Yes/No): "),
        attendance_percentage: prompt("Attendance percentage: "),
        sleep_hours: prompt("Sleep hours per day: "),
        diet_quality: prompt_string("Diet quality (Poor/Fair/Good): "),
        exercise_frequency: prompt("Exercise frequency per week: "),
        parental_education_level: prompt_string("Parental education level (High School/Bachelor/Master): "),
        internet_quality: prompt_string("Internet quality (Poor/Average/Good): "),
        mental_health_rating: prompt("Mental health rating (1–10): "),
        extracurricular_participation: prompt_string("Extracurricular participation? (Yes/No): "),
        exam_score: 0.0, // Placeholder
    }
}

// testing: contains unit tests to verify core functionality
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    // test loading students from a sample CSV file
    #[test]
    fn test_load_students_from_sample_csv() {
        let test_csv = "test_students.csv";

        // creates a simple test CSV file
        let mut file = File::create(test_csv).unwrap();
        writeln!(file, "student_id,age,gender,study_hours_per_day,social_media_hours,netflix_hours,part_time_job,attendance_percentage,sleep_hours,diet_quality,exercise_frequency,parental_education_level,internet_quality,mental_health_rating,extracurricular_participation,exam_score").unwrap();
        writeln!(file, "S001,18,Male,3,1,1,No,95,8,Good,3,Bachelor,Good,7,Yes,85").unwrap();

        // loads the data
        let result = load_students(test_csv);
        assert!(result.is_ok());
        let students = result.unwrap();
        assert_eq!(students.len(), 1);
        assert_eq!(students[0].student_id, "S001");

        // deletes test file
        std::fs::remove_file(test_csv).unwrap();
    }

    // test prediction model output for a known student
    #[test]
    fn test_predict_with_known_student() {
        let student = Student {
            student_id: "Test".to_string(),
            age: 18.0,
            gender: "Male".to_string(),
            study_hours_per_day: 5.0,
            social_media_hours: 1.0,
            netflix_hours: 1.0,
            part_time_job: "No".to_string(),
            attendance_percentage: 95.0,
            sleep_hours: 8.0,
            diet_quality: "Good".to_string(),
            exercise_frequency: 3.0,
            parental_education_level: "Bachelor".to_string(),
            internet_quality: "Good".to_string(),
            mental_health_rating: 8.0,
            extracurricular_participation: "Yes".to_string(),
            exam_score: 85.0,
        };

        let model = train_model(&[student.clone()]).unwrap();
        let prediction = predict(&model, &student);
        assert_eq!(prediction, 2); // 85 should fall into category 2 (80+)
    }

    // test encoding feature length and content structure
    #[test]
    fn test_encode_features_length_and_values() {
        let student = Student {
            student_id: "S001".to_string(),
            age: 20.0,
            gender: "Female".to_string(),
            study_hours_per_day: 3.0,
            social_media_hours: 2.0,
            netflix_hours: 1.5,
            part_time_job: "Yes".to_string(),
            attendance_percentage: 88.0,
            sleep_hours: 7.0,
            diet_quality: "Fair".to_string(),
            exercise_frequency: 2.0,
            parental_education_level: "Master".to_string(),
            internet_quality: "Average".to_string(),
            mental_health_rating: 6.0,
            extracurricular_participation: "No".to_string(),
            exam_score: 72.0,
        };

        let features = student.encode_features();
        assert_eq!(features.len(), 14); // should always encode to 14 features
    }

    // test that Euclidean distance is 0 for identical students
    #[test]
    fn test_euclidean_distance_zero_for_identical() {
        let student = Student {
            student_id: "S001".to_string(),
            age: 20.0,
            gender: "Female".to_string(),
            study_hours_per_day: 3.0,
            social_media_hours: 2.0,
            netflix_hours: 1.5,
            part_time_job: "Yes".to_string(),
            attendance_percentage: 88.0,
            sleep_hours: 7.0,
            diet_quality: "Fair".to_string(),
            exercise_frequency: 2.0,
            parental_education_level: "Master".to_string(),
            internet_quality: "Average".to_string(),
            mental_health_rating: 6.0,
            extracurricular_participation: "No".to_string(),
            exam_score: 72.0,
        };

        let distance = crate::graph::euclidean_distance(&student, &student);
        assert!(distance < 1e-6); // near-zero distance for exact match
    }
}
