use rusty_jvm::jvm::class::Class;

fn main() {
    let file = std::fs::read("res/HelloWorld.class").unwrap();
    let class = Class::new(&file).unwrap();
    println!("{:#?}", class);
}
