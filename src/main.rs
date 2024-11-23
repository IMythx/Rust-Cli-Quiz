use std::io::{self, Write};

use colored::Colorize;

struct Quiz {
    questions: [Question; 5],
    score: u32,
}

impl Quiz {
    fn init(&self) {
        println!(
            "{}",
            "
██████╗ ██╗   ██╗███████╗████████╗     ██████╗██╗     ██╗     ██████╗ ██╗   ██╗██╗███████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝    ██╔════╝██║     ██║    ██╔═══██╗██║   ██║██║╚══███╔╝
██████╔╝██║   ██║███████╗   ██║       ██║     ██║     ██║    ██║   ██║██║   ██║██║  ███╔╝
██╔══██╗██║   ██║╚════██║   ██║       ██║     ██║     ██║    ██║▄▄ ██║██║   ██║██║ ███╔╝
██║  ██║╚██████╔╝███████║   ██║       ╚██████╗███████╗██║    ╚██████╔╝╚██████╔╝██║███████╗
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝        ╚═════╝╚══════╝╚═╝     ╚══▀▀═╝  ╚═════╝ ╚═╝╚══════╝
    "
            .green()
        );

        println!("Welcome to this {} cli quiz app!!🤓 \n", "Rusty".green());
    }

    fn from(questions: [Question; 5]) -> Self {
        Self {
            questions,
            score: 0,
        }
    }

    fn start(&mut self) {
        for (index, question) in self.questions.iter_mut().enumerate() {
            loop {
                println!(
                    "\n{}-{}     {} {} \n",
                    index + 1,
                    question.question.bold(),
                    "Remaining attempts:".bold(),
                    match question.attempts {
                        3 => question.attempts.to_string().green().bold(),
                        2 => question.attempts.to_string().yellow().bold(),
                        _ => question.attempts.to_string().red().bold(),
                    }
                );

                println!(
                    "{}-{}   {}-{}   {}-{}   {}-{} \n",
                    "A".to_string().bold(),
                    question.choices[0],
                    "B".to_string().bold(),
                    question.choices[1],
                    "C".to_string().bold(),
                    question.choices[2],
                    "D".to_string().bold(),
                    question.choices[3]
                );

                let mut answer: String = String::new();

                print!("your answer(case insensitive): ");

                io::stdout().flush().unwrap();

                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read line");

                let answer = answer.trim().to_lowercase();

                if answer == question.answer.to_lowercase() {
                    self.score += 1;

                    break;
                } else {
                    println!("\nWrong Answer! Try again!🤡 \n");

                    question.attempts -= 1;
                }
                if question.attempts == 0 {
                    println!("\nYou have run out of attempts!💀 \n");

                    break;
                }
            }
        }
    }

    fn end(&mut self) {
        println!(
            "{}",
            "
 ██████╗ ██╗   ██╗██╗███████╗     ██████╗ ██████╗ ███╗   ███╗██████╗ ██╗     ███████╗████████╗███████╗██████╗ ██╗
██╔═══██╗██║   ██║██║╚══███╔╝    ██╔════╝██╔═══██╗████╗ ████║██╔══██╗██║     ██╔════╝╚══██╔══╝██╔════╝██╔══██╗██║
██║   ██║██║   ██║██║  ███╔╝     ██║     ██║   ██║██╔████╔██║██████╔╝██║     █████╗     ██║   █████╗  ██║  ██║██║
██║▄▄ ██║██║   ██║██║ ███╔╝      ██║     ██║   ██║██║╚██╔╝██║██╔═══╝ ██║     ██╔══╝     ██║   ██╔══╝  ██║  ██║╚═╝
╚██████╔╝╚██████╔╝██║███████╗    ╚██████╗╚██████╔╝██║ ╚═╝ ██║██║     ███████╗███████╗   ██║   ███████╗██████╔╝██╗
 ╚══▀▀═╝  ╚═════╝ ╚═╝╚══════╝     ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚══════╝╚══════╝   ╚═╝   ╚══════╝╚═════╝ ╚═╝
"
                .green()
        );

        println!(
            "\n{} {} {} {}\n",
            "Your score is".bold(),
            self.score.to_string().bold(),
            "out of ".bold(),
            self.questions.len().to_string().bold()
        );

        for (index, question) in self.questions.iter_mut().enumerate() {
            if question.attempts > 0 {
                println!(
                    "✅ you answered the #{} question from the {}\n",
                    (index + 1).to_string().bold(),
                    match question.attempts {
                        3 => {
                            [
                                "first".green().to_string(),
                                " attempt 😱 like a 🗿".to_string(),
                            ]
                            .join("")
                        }
                        2 => [
                            "second".yellow().to_string(),
                            " attempt 😳 like a 😎".to_string()
                        ]
                        .join(""),
                        _ => [
                            "third".red().to_string(),
                            " attempt 😲 like a 🥴".to_string()
                        ]
                        .join(""),
                    }
                );
            } else {
                println!(
                    "{} {} {}\n",
                    "❌ you exceeded the".red(),
                    [
                        "#".red().to_string(),
                        (index + 1).to_string().red().to_string()
                    ]
                    .join(""),
                    "question maximum attempts 🫩 😑, your a 💩🤡".red(),
                );
            }
        }
    }
}

struct Question {
    question: String,
    choices: [String; 4],
    answer: String,
    attempts: u32,
}

impl Question {
    fn new(question: String, choices: [String; 4], answer: String) -> Self {
        Self {
            question,
            choices,
            answer,
            attempts: 3,
        }
    }
}

fn main() {
    let questions = get_questions();

    let mut q = Quiz::from(questions);

    q.init();

    q.start();

    q.end();
}

fn get_questions() -> [Question; 5] {
    let question_one = Question::new(
        String::from("What is the time complexity of searching for an element in a balanced binary search tree (BST)?"),
        [
            "O(n)".to_string(),
            "O(log n)".to_string(),
            "O(n log n)".to_string(),
            "O(1)".to_string(),
        ],
        "B".to_string(),
    );

    let question_two = Question::new(
        String::from("Which of these languages is not object-oriented?"),
        [
            "Python".to_string(),
            "Java".to_string(),
            "C".to_string(),
            "Ruby".to_string(),
        ],
        "C".to_string(),
    );

    let question_three = Question::new(
        String::from("What does the 'volatile' keyword in C/C++ indicate?"),
        [
            "A variable is read-only.".to_string(),
            "A variable can be modified in an interrupt. ".to_string(),
            "A variable is stored in cache memory.".to_string(),
            "A variable is automatically synchronized across threads.".to_string(),
        ],
        "B".to_string(),
    );

    let question_four = Question::new(
        String::from("Which algorithm is used in merge sort?"),
        [
            "Divide and Conquer ".to_string(),
            "Greedy Algorithm".to_string(),
            "Dynamic Programming".to_string(),
            "Backtracking".to_string(),
        ],
        "A".to_string(),
    );

    let question_five = Question::new(
        String::from(
            "Which of the following languages introduced the concept of 'garbage collection'?",
        ),
        [
            "C".to_string(),
            "Lisp".to_string(),
            "Fortran".to_string(),
            "COBOL".to_string(),
        ],
        "B".to_string(),
    );

    [
        question_one,
        question_two,
        question_three,
        question_four,
        question_five,
    ]
}
