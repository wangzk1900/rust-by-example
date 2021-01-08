use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

fn mutiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
fn main() {
    print(mutiply("10", "2"));
    print(mutiply("t", "2"));
}
