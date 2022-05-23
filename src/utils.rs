use std::io::{stdin, stdout, Write};

pub fn read_line(question: &str) -> String {
    let mut answer = String::new();
    print!("{}?", question);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut answer)
        .expect("Error while reading answer for question.");
    if let Some('\n') = answer.chars().next_back() {
        answer.pop();
    }
    if let Some('\r') = answer.chars().next_back() {
        answer.pop();
    }
    answer
}
