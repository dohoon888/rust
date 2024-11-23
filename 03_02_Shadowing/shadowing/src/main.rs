fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    // it's work. because first spaces is shadowed by second spaces 
    // let spaces = "   ";
    // let spaces = spaces.len();


    //  it's not work. when spaces is defined string variable. but second operating is return numberable value. 
    // let mut spaces = "   ";
    // spaces = spaces.len();

}