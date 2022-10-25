use rusty_jvm::jvm::{class::Class, interpreter::Interpreter};

fn main() {
    let file = std::fs::read("res/HelloWorld.class").unwrap();
    let class = Class::new(&file).unwrap();
    println!("{:-#?}", class);
    println!("{:#?}", class);
    let mut interpreter = Interpreter::new();
    interpreter.insert_class(class);
    let ret = interpreter
        .run_static_method("HelloWorld", "main", &[])
        .unwrap();
    println!("{:#?}", ret);
}
