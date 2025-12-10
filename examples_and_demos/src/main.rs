#![allow(unused)]

/*
THE BASICS:

Prerequisite Knowledge:

processors read one word at a time -> 4bytes for 32bit systems & 8bytes for 64 bit systems

binary conversions -> 2^0 2^1 2^2 2^3 . . ..

Logic Gates:

AND -> returns 1 only when both inputs are 1 (multiplication) [&& in rust]
OR -> returns 1 if atleast one input is 1 (addition) [|| in rust]
XOR -> returns 1 if input is different and 0 if same (exclusive or) [^ in rust]

Bitwise Shifts:

Left_Shift -> 1<<5 (00000001) becomes (00100000) so output will be 32
Right_Shift -> 128>>2 (10000000) becomes (00100000) so output will be 32




Variables:
let a = 5 ( immutable and type is optional *inferred* )
let a mut = 5 (mut keyword makes the variable mutable)
const MAX_VAL: u32 = 5 (type required, only for constant never use for a variable being set in runtime, inlined *no fixed memory address*)


Basic Types:
signed integer-> (both positive and negative nums)
a:i32 (the number following i can be 8,16,32,64,128)
a:isize (i32 for x32 and i64 for x64 architecture systems)

unsigned integer-> (only positive nums)
a:u32 (num following u can be 8,16,32,64,128)
a:usize (same as isize but for unsigned integers)

floating nums->
a: f32 (32 bit floating point num)
a: f64 (64 bit floating point num)

if no type specified, defaults to i32 if num and f64 if float

println!(i8::MAX) *127 is the max range of i8 integers*

boolean->
a: bool (true/false)

characters->
a: char (4 byte unicode scalar value)

strings->
String keyword (heap-allocated, owned and growable, UTF8-Encoded)
&str keyword (borrowed immutable reference to sequence of UTF-8 bytes, fixed size, efficient)
ONLY USE DOUBLE QUOTES FOR STRINGS SINGLE QUOTES ARE ONLY FOR CHAR

name = "alice";
let mut my_str = String::from("Hello");
my_str.push_str(" World!") *we can append strings to other strings if String*
let info = format!("Hello, {}",name); *inlining variables in strings*
message = info + my_str *we can concat strings*

unit->
0 bytes basically rusts version of null
let _z :() = ();
asssertions return unit that is why we cannot use assert as a condition in an if else statement

Type Conversion:
let v: u16 = 31_u8 as u16; *we can specify type directly on number with _typename, as keyword to convert one type to another*


Printing:
print!("Hello") (prints Hello)
println!("Hello") (prints Hello followed by a newline)


Shadowing:
we can declare a new variable with the same name to overshadow the first one
let x = 5;
{
let x = 12;
assert_eq!(x,5); *assert_eq checks if 2 given values are equal*
}
assert_eq!(x,12);


Unused Variables:
2 ways to hide the warnings that appear when you have unused vars:

#![allow(unused_variables)] *add this to the top of your file*
or
#![allow(unused)] *covers unused variables, functions, assignments,etc*

let _x = 1 *adding underscore before varaible makes it an unused variable, use _x when using the variable also*


Destructuring Assignments:

let (x,y);
(x,..) = (3,4) *.. means to ignore the 2nd value*
[..,y] = [1,2]
assert_eq!([x,y],[3,2])

Functions:
Types of the parameters of the function need to be annotated (specified)
there is no return keyword just giving variable WITHOUT SEMICOLON means it needs to be returned
fn sum(x:i32, y:i32){
    x+y; //this will return unit type since we are not specifying any return
}
fn sum(x:i32, y:i32){
    x+y //this works because we are returning x+y
}

*/

//for each question first call with 0 for question output and 1 for solution output

fn unit_trick(a: i8) {
    // adding a semicolon on the definition of a variable returns unit
    if a == 0 {
        let z = {
            2 * 2;
        };
        println!("{:?}", z);
    } else if a == 1 {
        let z = { 2 * 2 };
        println!("{}", z);
    }
}

fn _ascii_vals(a: i8) {
    //output 97-122 with one line change
    if a == 0 {
        for c in 'a'..='z' {
            //.. means exclusive ..= means inclusive
            println!("{}", c);
        }
    } else if a == 1 {
        for c in 'a'..='z' {
            println!("{}", c as i16);
        }
    }
    //explanation: typecasting the char as int gives us the ascii values
}

fn integer_check_trickq(a: i8) {
    //fix the assertion err
    if a == 0 {
        println!("{}", 0.1 + 0.2);
        assert!(0.1 + 0.2 == 0.3);
        println!("Success!")
    } else if a == 1 {
        assert!(0.1_f32 + 0.2_f32 == 0.3);
        println!("Success");
    }
    //explanation: default is f64 which gives 0.10000000000000002 and 0.20000000000000002
    // by providing type as f32 it doesn't fuck up the calc
}

fn main() {}
