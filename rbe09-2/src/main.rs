fn main() {
    fn function (i: i32) -> i32 { i + 1 }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i | i + 1; // error[E0282]: type annotations needed

    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_annotated(i));

    let one = || 1;
    println!("closure returning one: {}", one());
}
