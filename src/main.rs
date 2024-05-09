use rand::prelude::*;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

// use std::fs::{self, OpenOptions};
// use regex::Regex;

// fn fill_csv_files(category: u8) {
//     let path_csv = format!("./questions/category_{}.csv", category);
//     let path_txt = format!("./raw/category_{}.txt", category);

//     let file = OpenOptions::new()
//         .write(true)
//         .open(path_csv)
//         .unwrap();
//     let mut wtr = csv::WriterBuilder::new()
//         .delimiter(b';')
//         .from_writer(file);

//     wtr.write_record(&["number", "question", "answer_1", "answer_2", "answer_3", "answer_4"]).unwrap();

//     let contents = fs::read_to_string(path_txt)
//         .expect("Should have been able to read the file");

//     let lines = contents.split("\n");

//     let regex_question = Regex::new(r"(\d+)\.(.+)").unwrap();
//     let regex_answer = Regex::new(r"(\d)\)(.+)").unwrap();
//     let regex_ignore = Regex::new(r"\[\d+\]").unwrap();

//     let mut number: String = String::new();
//     let mut question: String = String::new();
//     let mut answers: [String; 4] = [String::new(), String::new(), String::new(), String::new()];

//     for line in lines {
//         if regex_ignore.is_match(line) { continue };

//         // if line is question -> write to csv
//         let cap = regex_question.captures(line);
//         if cap.is_some() {
//             if question.len() > 0 &&
//                 answers[0].len() > 0 &&
//                 answers[1].len() > 0 &&
//                 answers[2].len() > 0 &&
//                 answers[3].len() > 0 {

//                 // println!("{}. {}:\n\n{}\n{}\n{}\n{}\n", &number, &question, &answers[0], &answers[1], &answers[2], &answers[3]);
//                 wtr.write_record(&[&number, &question, &answers[0], &answers[1], &answers[2], &answers[3]]).unwrap();
//                 number.clear();
//                 question.clear();
//                 answers = [String::new(), String::new(), String::new(), String::new()];
//             }

//             let cap = cap.unwrap();
//             number = cap[1].to_string();
//             question = cap[2].to_string();
//             continue;
//         }

//         let cap = regex_answer.captures(line);
//         if cap.is_some() {
//             let cap = cap.unwrap();
//             let mut q_nr: usize = cap[1].parse().unwrap();
//             q_nr -= 1;
//             answers[q_nr] = cap[2].to_string();
//             continue;
//         }

//         // append non-matching line to previous string
//         if answers[0].len() == 0 {
//             question += " ";
//             question += line;
//         } else if answers[1].len() == 0 {
//             answers[0] += " ";
//             answers[0] += line;
//         } else if answers[2].len() == 0 {
//             answers[1] += " ";
//             answers[1] += line;
//         } else if answers[3].len() == 0 {
//             answers[2] += " ";
//             answers[2] += line;
//         } else {
//             answers[3] += " ";
//             answers[3] += line;
//         }
//     }

//     wtr.write_record(&[&number, &question, &answers[0], &answers[1], &answers[2], &answers[3]]).unwrap();
//     wtr.flush().unwrap();
// }

struct Question {
    number: u8,
    question: String,
    answers: [String; 4],
}

impl Clone for Question {
    fn clone(&self) -> Self {
        Question {
            number: self.number,
            question: self.question.clone(),
            answers: self.answers.clone(),
        }
    }
}

struct Quiz {
    name: String,
    questions: Vec<Question>,
}

impl Quiz {
    fn get_correct_answer_for_q_nr(&self, q_nr: u8) -> &str {
        let question = self.questions.iter().find(|q| q.number == q_nr);

        if question.is_none() {
            return "";
        }

        return &question.unwrap().answers[0];
    }
}

fn get_quizes_from_csvs() -> Vec<Quiz> {
    let mut quizes: Vec<Quiz> = Vec::new();

    for i in 1..=7 {
        let path = format!("./questions/category_{}.csv", i);

        let file = File::open(path).unwrap();
        let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);

        let name = match i {
            1 => "Mobiler Seefunkdienst und Weltweites Seenot- und Sicherheitsfunksystem (GMDSS)",
            2 => "Funkeinrichtungen und Seefunkstellen",
            3 => "Digitaler Selektivruf (DSC)",
            4 => "UKW (VHF) – Sprechfunk",
            5 => "Betriebsverfahren und Rangfolgen",
            6 => {
                "Nautische und Meteorologische
Warnnachrichten (NAVTEX)"
            }
            7 => "Suche und Rettung (SAR), Seenotfunkbake (EPIRB) und Radartransponder (SART)",
            _ => "Unknown",
        };

        let mut quiz = Quiz {
            name: name.to_string(),
            questions: Vec::new(),
        };

        rdr.records().for_each(|record| {
            let record = record.unwrap();
            let question = Question {
                number: record[0].parse().unwrap(),
                question: record[1].to_string(),
                answers: [
                    record[2].to_string(),
                    record[3].to_string(),
                    record[4].to_string(),
                    record[5].to_string(),
                ],
            };

            quiz.questions.push(question);
        });

        quizes.push(quiz);
    }

    return quizes;
}

fn get_input(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}: ", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Ungültige Eingabe!");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    return s;
}

fn get_input_number(prompt: &str, max: usize) -> usize {
    loop {
        let input = get_input(prompt);

        match input.parse() {
            Ok(num) => {
                if num > max || num < 1 {
                    println!("Bitte eine Zahl zwischen 1 und {} eingeben!", max);
                    continue;
                }
                return num;
            }
            Err(_) => {
                println!("Bitte nur Zahlen eingeben!");
            }
        }
    }
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn game_loop() {
    let quizes = get_quizes_from_csvs();

    let mut _rng = rand::thread_rng();

    loop {
        clear_terminal();
        println!("Wähle ein Quiz aus:\n\n");

        for (i, quiz) in quizes.iter().enumerate() {
            println!("{}. {} [{} Fragen]", i + 1, quiz.name, quiz.questions.len());
        }
        println!("{}. Beenden\n\n", quizes.len() + 1);

        let mut quiz_nr = get_input_number("Quiznummer", quizes.len() + 1);
        if quiz_nr == quizes.len() + 1 {
            clear_terminal();
            println!("Tschüss...");
            sleep(Duration::from_secs(2));

            clear_terminal();
            break;
        }

        quiz_nr -= 1;

        let mut rand_questions = quizes[quiz_nr].questions.clone();
        _rng = rand::thread_rng();
        rand_questions.shuffle(&mut _rng);

        let mut score = 0;

        for (i, question) in rand_questions.iter().enumerate() {
            clear_terminal();
            println!("--- {} ---", quizes[quiz_nr].name);
            println!("Score: {}\n\n", score);

            println!(
                "Frage {}/{}: {}\n",
                i + 1,
                rand_questions.len(),
                question.question
            );

            let mut rand_answers = question.answers.clone();
            _rng = rand::thread_rng();
            rand_answers.shuffle(&mut _rng);

            for (j, answer) in rand_answers.iter().enumerate() {
                println!("{}) {}", j + 1, answer);
            }

            let user_answer = get_input_number("Antwort", 4) - 1;

            let correct_answer = quizes[quiz_nr].get_correct_answer_for_q_nr(question.number);

            if rand_answers[user_answer] == correct_answer {
                score += 1;
                println!("\nKorrekt!");
            } else {
                println!(
                    "\nFalsch!\nDie richtige Antwort war: \"{}\"",
                    &correct_answer
                );
            }

            get_input("\nDrücke Enter um fortzufahren...");
        }

        clear_terminal();
        println!(
            "Du hast {} von {} Fragen richtig beantwortet!",
            score,
            rand_questions.len()
        );
        sleep(Duration::from_secs(5));
    }
}

fn main() {
    // for i in 1..=7 {
    //     println!("Setting up category {}", i);
    //     fill_csv_files(i);
    // }

    game_loop();
}
