#[allow(dead_code)]
#[derive(Debug)]
struct Test {
    student: String,
    score: i32,
}

impl Test {
    fn new(student: String, score: i32) -> Self {
        Test {
            student,
            score,
        }
    }
}

fn main() {
    let scores = vec![
        Test::new("Ron".to_string(), 58),
        Test::new("John".to_string(), 75),
        Test::new("AMy".to_string(), 24)
    ];

    for score in scores {
        if score.score < 34 {
            println!("{} Failed with Percentage {}", score.student, score.score);
        } else {
            println!("{} Passed with Percentage {}", score.student, score.score);
        }
    }
}
