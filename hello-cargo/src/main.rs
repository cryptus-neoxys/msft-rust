fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn main() {
    let heart_eyed_cat = '😻';
    let mut hello = String::from("Hello, ");
    hello.push('w');
    hello.push_str("orld!");
    println!("&str: {}\nString: {}", heart_eyed_cat, hello);
    print_type_of(&heart_eyed_cat);
    print_type_of(&hello);
}