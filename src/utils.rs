use std::fs;

use ndarray::{Array2, ArrayBase, Axis, Ix2, RawData};

pub(crate) fn read_input_to_string(day: u32) -> std::io::Result<String> {
    fs::read_to_string(format!("./src/day{day}/input.txt"))
}

pub(crate) fn pretty_print(arr: &Array2<char>) -> String {
    let mut result = String::new();
    for row in arr.rows() {
        for elem in row {
            result.push(*elem);
        }
        result.push('\n');
    }

    result.trim().to_owned()
}

/// See: https://github.com/rust-ndarray/ndarray/issues/866
pub(crate) fn rot90<S>(arr: &mut ArrayBase<S, Ix2>)
where
    S: RawData,
{
    arr.swap_axes(0, 1);
    arr.invert_axis(Axis(0));
}
