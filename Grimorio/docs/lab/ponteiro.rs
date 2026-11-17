fn main () {
    ex1()
}

fn ex1 (){
    let mut x: i8 = 5;
    let mut y: i8 = x;

    y = 10;

    println!("x = {}", x);
    println!("----A & B----------------------");

    let mut a: i8 = 5;
    let b: &mut i8 = &mut a; 

    *b = 10;

    println!("print b = {}", b); 
    println!("print a = {}", a); 
}