use jerris::class::Class;

fn main() {
    match Class::from_file(std::env::args().next().unwrap()) {
        Ok(class) => {
            println!("{:#?}", class);
        }
        Err(e) => println!("{e}")
    }
}