use std::num::ParseIntError;

// fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
//     let first_number = first_number_str.parse::<i32>().unwrap();
//     let second_number = second_number_str.parse::<i32>().unwrap();
//     first_number * second_number
// }

// fn main() {
//     let twenty = multiply("10", "2");
//     println!("double is {}", twenty);

//     let tt = multiply("t", "2");
//     println!("double is {}", tt);
// }

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
