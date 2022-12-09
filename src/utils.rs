use std::fs;

pub(crate) fn read_input_to_string(day: u32) -> std::io::Result<String> {
    fs::read_to_string(format!("./src/day{day}/input.txt"))
}
