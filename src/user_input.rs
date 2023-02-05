use std::io;

pub fn get_user_input() -> Option<Input> {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let input_without_newline = user_input.trim_end();
    match input_without_newline {
        "1" => Some(Input::MarkCell(0)),
        "2" => Some(Input::MarkCell(1)),
        "3" => Some(Input::MarkCell(2)),
        "4" => Some(Input::MarkCell(3)),
        "5" => Some(Input::MarkCell(4)),
        "6" => Some(Input::MarkCell(5)),
        "7" => Some(Input::MarkCell(6)),
        "8" => Some(Input::MarkCell(7)),
        "9" => Some(Input::MarkCell(8)),
        _ => None,
    }
}

#[derive(std::fmt::Debug)]
pub enum Input {
    MarkCell(usize),
}
