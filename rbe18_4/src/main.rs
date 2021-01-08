fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    println!("The first doubled is {}", double_first(numbers));

    // let empty = vec![];
    // println!("The first doubled is {}", double_first(empty));

    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {}", double_first(strings));
}
