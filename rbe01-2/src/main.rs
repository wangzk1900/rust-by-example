// format!: write formatted text to String
// print!: same as format! but the text is printed to the console (io::stdout).
// println!: same as print! but a newline is appended.
// eprint!: same as format! but the text is printed to the standard error (io::stderr).
// eprintln!: same as eprint!but a newline is appended.

// 打印某个数据类型的值，该类型需要实现 std::fmt 中的 trait，常用的有两个：fmt::Debug 和 fmt::Display
// 基本数据类型已经实现了这两个 trait。

fn main() {
    // In general, the `{}` will be automatically replace with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // There are various optional patterns this work with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't.", 1, 2);

    // You can right-align text with a specified width. This will output "     1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bond");

    // Create a structure name `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
}
