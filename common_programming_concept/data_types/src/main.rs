/*
//DATA TYPES
1.Every value in rust is  a certain data type
2.data types are mainly two, scalar and compound data types
3.rust is statically typed language
4.it must know all type of the varibales at compile time
5.the compiler can usually infer data type based on the value we assign ti it
6. In some cases, developer need to annotate type, where compiler is unable to infer the data type

//SCALAR TYPES

1.scalar types represent single value
2.ex: integer,bool,charf,float
3.Integer types => signed and unsigned integer ex ;i32 anfd u32 takes 32 bits of space
4. unsign = u8 = 255
5.isize and usize depends on the system arch
6.integer type default is i32

// COMPOUND TYPES

1.cant hold mutiple value in a single type
2. there are two primitive compound types, tuples and array
3.tuples : general way of grouping together varienty of data with different types
4. cannot grow and cannot shrink
5. array can only hold datra of same type

*/




fn main() {
    // here we need to give an explicit type for guess variables
   let guess = "42".parse().expect("Unable to parse");
   // example of tuple
   let tup = (10,true,6.7);
   //to take value out of tuple, we should use pattern matching to destructute
   let (x,y,z) = tup //destructuring a tuple
   //another way of getting value out of tuple is using periods(.)
   let one = tup.0;


   ///Array
   let x = [1,2,3,4,5]; //array holds same type of data

}

/*
A tuple without any value are called : unit type => ()
expressions expresitely return unit type , if they not return anything else

*/