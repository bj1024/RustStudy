use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {} {}", x,type_of(x));
    x = 6;
    println!("The value of x is: {}", x);

    another_function();

    type_exam();

    let mut a  = 100;
    for cnt in 0..10{
        println!("[{:3}]={:10}", cnt,a);
        a += 1;
    }

}
fn type_exam(){
    println!("function typeExam");
    }


fn another_function() {
    println!("Another function.");
}