fn copy(x: u32) {
    println!("Inside {}", x);
}

fn borrow(s: String) -> String {
    println!("Inside {}", s);
    return s;
}

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    let x = 5;
    copy(x);
    borrow(s);
    println!("Outside {}", x);
    println!("Outside {}", s);
}
