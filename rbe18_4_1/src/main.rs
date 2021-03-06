use std::num::ParseIntError;

// fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
//     vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
// }

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    println!("The first doubled is {:?}", double_first(numbers));

    let empty = vec![];
    println!("The first doubled is {:?}", double_first(empty));

    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", double_first(strings));
}
