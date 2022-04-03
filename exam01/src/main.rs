fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    another_function();


    let mut a  = 100;
    for cnt in 0..10{
        println!("[{:3}]={:10}", cnt,a);
        a += 1;
    }

}


fn another_function() {
    println!("Another function.");
}