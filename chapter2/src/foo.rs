pub fn public_foo() {
    println!("foo::public_foo()");

    cat::public_cat();
}

pub mod bar{
    pub fn public_bar() {
        println!("bar::public_bar()")
    }
}

mod cat{
    pub fn public_cat() {
        println!("cat::public_cat()")
    }
}
