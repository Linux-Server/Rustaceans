/*
This is getting startde of rust book. It covers
1. Installing Rust on Linux, macOS, and Windows
2. Writing a program that prints Hello, world!
3. Using cargo, Rust’s package manager and build system


////Writing and Running a Rust Program

1. Create a main.rs  source file in the getting started folder and paste hello world programs
2.the file name always end up with .rs extension
3.if ur file name has more than one word use underscore _ ex: hello_world.rs
4.Now save the file and rust the following command in terminal
``` rustc main.rs ```
5. which will generate a binary executable of the  progra
6. run the execubal ein any operating system
 ```
./main
 ```
 7. which will print the welcome message on your terminal


 /// ANATOMY OF RUST
 1. the main function is special , which will run first on every executable
 2. the func sig of main fn have no parameters and returns nothing
 3.the function body is wrapped around with curly braces
 4.if you want to stick with standard style we can use rustfmt which is installed by default with rust
 


*/

fn main() {
    // 1. Rust intentation should go with 4 spaces instead of 1 tab.
    //2. println! is rust macro which end with exclmation
    // each line should end with a semicolom
    println!("Welcome to getting started!");
}


/*
///COMPILING AND RUN
1. bfore running any program , u should compile the prgarm to generate the excutable file
2.  rustc main.rs
3. which will generate the binary executable
4. if are more familiar with dynamic programming lang such, Ruby,Python and JS , you mignt not doing the compiling and running seprately
5. Rust is ahead of time lang that mean you cancmpile your program give the executable to someone else , where thay can run the exec on their system without even rust installed on their system
6.just compile is is enogh for simple programs as your project grows, you should use more sophisticated approach like cargo 

*/