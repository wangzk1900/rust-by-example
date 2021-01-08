mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct CloseBox<T> {
        contents: T,
    }

    impl<T> CloseBox<T> {
        pub fn new(contents: T) -> CloseBox<T> {
            CloseBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    let open_box = my::OpenBox {
        contents: "public information"
    };

    println!("The open box contains: {}", open_box.contents);

    let _closed_box = my::CloseBox::new("classified information");
    
    println!("Hello, world!");
}
