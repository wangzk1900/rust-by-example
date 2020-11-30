// Line comments which go to the end of the line.
/* Block comments which go to the closing delimiter. */

/// Generate library docs for the following item.
fn main() {
    //! Generate library docs for the enclosing item.
    println!("Hello, world!");

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
