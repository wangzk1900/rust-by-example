mod my_mod {
    // 默认私有，可见范围是当前模块
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // pub 表示公有，可见范围是全局
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // `pub(in path)` 指定可见范围
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // `pub(self)` 可见范围：当前模块
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // `pub(super)` 可见范围：父模块
        pub(super) fn public_function_in_super_mod(){
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        println!("called `my_mod::nested::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 可见范围：当前的 crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    mod private_nesed {
        #[allow(dead_code)]
        pub fn function(){
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    function();
    my_mod::function();

    // 公有项，包括在嵌套模块内的，都可以在父模块外部访问
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

}
