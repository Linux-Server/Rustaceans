/*
1. cargo is rust build system and package manager
2.cargo allw the dev to build your code,download external crates and build thos libraries
3.the simple rust progm we have written doesnt have any dependecies
4. cargo is the opttimum chose if you are bulding a bigger project
5.cargo comes with the official rust installe
6.if you installed rust through other means please check whether cargo is installed by...
````
cargo --version
```

/// CREATING PROJECT WITH CARGO

1. ``` cargo new hello_world ```
2.this command will creat a new directory  and project called hello_world
3. now open th e folder you will see two file a Cargo.toml and src/main.rs
4. it will also generate a .gitignore file , if you have not initialize any github repo
5.Cargo..toml file keep tract of your dependecies and package details

//BUILDING PROJECT

1.you can build you hello world program with cargo using  cargo build
2. the build command will generate a target folder , where its keeping binary executable in debug folder
3. we just compiled uor code using cargo build and
4. cargo run command helps us to compil and run the program with a single command
5. cargo also provide a commannd called cargo check, which will check for cmpile time error without gereating the executable
6. if your project is so big and need fequent check, then this method is more feasible than cargo build
7. when you project is finally ready to released you you can use 
cargo build --release
which will generate another folder in the target directory


*/


fn main() {
    println!("Hello, world by debug!");
}

/*
Let’s recap what we’ve learned so far about Cargo:

We can create a project using cargo new.
We can build a project using cargo build.
We can build and run a project in one step using cargo run.
We can build a project without producing a binary to check for errors using cargo check.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

*/
