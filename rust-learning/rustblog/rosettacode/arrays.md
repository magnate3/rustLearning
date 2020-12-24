# about array

**err**

```rust

#[derive(Copy, Clone, Debug)]
struct Foo {
    a: u32,
    b: u32,
}

fn main() {
    let mut foo_array = [Foo { a: 10, b: 10 }; 10];
}
```


```rust
use std::{mem, ptr};

#[derive(Debug)]
struct Foo {
    a: u32,
    b: u32,
}

// We're just implementing Drop to prove there are no unnecessary copies.
impl Drop for Foo {
    fn drop(&mut self) {
        println!("Destructor running for a Foo");
    }
}

pub fn main() {
    let array = unsafe {
        // Create an uninitialized array.
        let mut array: [Foo; 10] = mem::uninitialized();

        for (i, element) in array.iter_mut().enumerate() {
            let foo = Foo { a: i as u32, b: 0 };

            // Overwrite `element` without running the destructor of the old value.
            // Since Foo does not implement Copy, it is moved.
            ptr::write(element, foo)
        }

        array
    };

    for element in array.iter() {
        println!("{:?}", element);
    }
}
```

You can use the arrayvec crate:

Cargo.toml
```toml
[package]
name = "initialize_array"
version = "0.1.0"
authors = ["author"]
edition = "2018"

[dependencies]
arrayvec = "0.4.10"
```


src/main.rs

```rust
use arrayvec::ArrayVec; 
use std::iter;

#[derive(Clone)]
struct Foo {
    a: u32,
    b: u32,
}

fn main() {
    let foo_array: [Foo; 10] = iter::repeat(Foo { a: 10, b: 10 })
        .collect::<ArrayVec<_>>()
        .into_inner()
        .unwrap_or_else(|_| unreachable!());
}
```

---

The init array:
lib.rs
```rust

#![no_std]

//! The `array-vec` crate allows you to initialize arrays
//! with an initializer closure that will be called
//! once for each element until the array is filled.
//!
//! This way you do not need to default-fill an array
//! before running initializers. Rust currently only
//! lets you either specify all initializers at once,
//! individually (`[a(), b(), c(), ...]`), or specify
//! one initializer for a `Copy` type (`[a(); N]`),
//! which will be called once with the result copied over.
//!
//! # Examples:
//! ```rust
//! # #![allow(unused)]
//! # extern crate array_init;
//!
//! // Initialize an array of length 10 containing
//! // successive squares
//!
//! let arr: [u32; 50] = array_init::array_init(|i| (i*i) as u32);
//!
//! // Initialize an array from an iterator
//! // producing an array of [1,2,3,4] repeated
//!
//! let four = [1u32,2,3,4];
//! let mut iter = four.iter().cloned().cycle();
//! let arr: [u32; 50] = array_init::from_iter(iter).unwrap();
//!
//! // Closures can also mutate state. We guarantee that they will be called
//! // in order from lower to higher indices.
//!
//! let mut last = 1u64;
//! let mut secondlast = 0;
//! let fibonacci: [u64; 50] = array_init::array_init(|_| {
//!     let this = last + secondlast;
//!     secondlast = last;
//!     last = this;
//!     this
//! });
//! ```

use core::mem::{self, MaybeUninit};
use core::ptr;

/// Trait for things which are actually arrays
///
/// Probably shouldn't implement this yourself,
/// but you can
pub unsafe trait IsArray {
    /// The stored `T`
    type Item;

    fn len() -> usize;
}

#[inline]
/// Initialize an array given an initializer expression
///
/// The initializer is given the index of the element. It is allowed
/// to mutate external state; we will always initialize the elements in order.
///
/// If your initializer panics, any elements that have been initialized
/// will be leaked.
///
/// # Examples
///
/// ```rust
/// # #![allow(unused)]
/// # extern crate array_init;
///
/// // Initialize an array of length 10 containing
/// // successive squares
///
/// let arr: [u32; 50] = array_init::array_init(|i| (i*i) as u32);
///
/// // Initialize an array from an iterator
/// // producing an array of [1,2,3,4] repeated
///
/// let four = [1u32,2,3,4];
/// let mut iter = four.iter().cloned().cycle();
/// let arr: [u32; 50] = array_init::from_iter(iter).unwrap();
///
/// ```
///
pub fn array_init<Array, F>(mut initializer: F) -> Array where Array: IsArray,
                                                               F: FnMut(usize) -> Array::Item {
    let mut ret: MaybeUninit<Array> = MaybeUninit::uninit();
    // Poor Man's array-to-pointer decay from C O:-)
    let mut elem = ret.as_mut_ptr() as usize as *mut Array::Item;

    for i in 0..Array::len() {
        let value = initializer(i);
        unsafe {
            ptr::write(elem, value);
            // Using .add instead of offset to avoid having to check the offset in bytes not
            // overflowing isize. We are guaranteed to be within the array, because we go one by
            // one.
            elem = elem.add(1);
        }
    }

    unsafe { ret.assume_init() }
}

#[inline]
/// Initialize an array given an iterator
///
/// We will iterate until the array is full or the iterator is exhausted. Returns
/// None if the iterator is exhausted before we can fill the array.
///
/// # Examples
///
/// ```rust
/// # #![allow(unused)]
/// # extern crate array_init;
///
/// // Initialize an array from an iterator
/// // producing an array of [1,2,3,4] repeated
///
/// let four = [1u32,2,3,4];
/// let mut iter = four.iter().cloned().cycle();
/// let arr: [u32; 50] = array_init::from_iter(iter).unwrap();
/// ```
///
pub fn from_iter<Array, I>(iter: I) -> Option<Array>
    where I: IntoIterator<Item = Array::Item>,
          Array: IsArray {
    let mut ret: MaybeUninit<Array> = MaybeUninit::uninit();
    // Poor Man's array-to-pointer decay from C O:-)
    let mut elem = ret.as_mut_ptr() as usize as *mut Array::Item;

    let mut count = 0;
    for item in iter.into_iter().take(Array::len()) {
        unsafe {
            ptr::write(elem, item);
            // Using .add instead of offset to avoid having to check the offset in bytes not
            // overflowing isize. We are guaranteed to be within the array, because we go one by
            // one.
            elem = elem.add(1);
        }
        count += 1;
    }

    // crucial for safety!
    if count == Array::len() {
        Some(unsafe { ret.assume_init() })
    } else {
        if mem::needs_drop::<Array::Item>() {
            let mut elem = ret.as_mut_ptr() as usize as *mut Array::Item;
            for _ in 0..count {
                unsafe {
                    ptr::drop_in_place(elem);
                    elem = elem.add(1);
                }
            }
        }
        None
    }
}

macro_rules! impl_is_array {
    ($($size:expr)+) => ($(
        unsafe impl<T> IsArray for [T; $size] {
            type Item = T;

            #[inline]
            fn len() -> usize {
                $size
            }
        }
    )+)
}

// lol

impl_is_array! {
     0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
    16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
    32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47
    48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63
    64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79
    80 81 82 83 84 85 86 87 88 89 90 91 92 93 94 95
    96 97 98 99 100 101 102 103 104 105 106 107 108
    109 110 111 112 113 114 115 116 117 118 119 120
    121 122 123 124 125 126 127 128 129 130 131 132
    133 134 135 136 137 138 139 140 141 142 143 144
    145 146 147 148 149 150 151 152 153 154 155 156
    157 158 159 160 161 162 163 164 165 166 167 168
    169 170 171 172 173 174 175 176 177 178 179 180
    181 182 183 184 185 186 187 188 189 190 191 192
    193 194 195 196 197 198 199 200 201 202 203 204
    205 206 207 208 209 210 211 212 213 214 215 216
    217 218 219 220 221 222 223 224 225 226 227 228
    229 230 231 232 233 234 235 236 237 238 239 240
    241 242 243 244 245 246 247 248 249 250 251 252
    253 254 255 256 257 258 259 260 261 262 263 264
    265 266 267 268 269 270 271 272 273 274 275 276
    277 278 279 280 281 282 283 284 285 286 287 288
    289 290 291 292 293 294 295 296 297 298 299 300
    301 302 303 304 305 306 307 308 309 310 311 312
    313 314 315 316 317 318 319 320 321 322 323 324
    325 326 327 328 329 330 331 332 333 334 335 336
    337 338 339 340 341 342 343 344 345 346 347 348
    349 350 351 352 353 354 355 356 357 358 359 360
    361 362 363 364 365 366 367 368 369 370 371 372
    373 374 375 376 377 378 379 380 381 382 383 384
    385 386 387 388 389 390 391 392 393 394 395 396
    397 398 399 400 401 402 403 404 405 406 407 408
    409 410 411 412 413 414 415 416 417 418 419 420
    421 422 423 424 425 426 427 428 429 430 431 432
    433 434 435 436 437 438 439 440 441 442 443 444
    445 446 447 448 449 450 451 452 453 454 455 456
    457 458 459 460 461 462 463 464 465 466 467 468
    469 470 471 472 473 474 475 476 477 478 479 480
    481 482 483 484 485 486 487 488 489 490 491 492
    493 494 495 496 497 498 499 500 501 502 503 504
    505 506 507 508 509 510 511 512
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq() {
        let seq: [usize; 5] = array_init(|i| i);
        assert_eq!(&[0, 1, 2, 3, 4], &seq);
    }
}

```

---


# Rust Arrays Tutorial

In this Rust tutorial we learn about a data container that can hold multiple values instead of just one, the array.

We learn how to initialize an array with values, how to access array elements with the indexer and with a loop, how to mutate array elements and how to pass an array to a function.

Here's a table of contents of what you'll learn in this lesson:  
(_click on a link to skip to its section_)

-   [What is an array](https://www.koderhq.com/tutorial/rust/array/#what-is-array "Jump directly to: What is an array")
-   [How to create (initialize) an array](https://www.koderhq.com/tutorial/rust/array/#init "Jump directly to: How to create (initialize) an array")
-   [How to access array elements with the indexer](https://www.koderhq.com/tutorial/rust/array/#access-indexer "Jump directly to: How to access array elements with the indexer")
-   [How to access array elements in a loop](https://www.koderhq.com/tutorial/rust/array/#access-loop "Jump directly to: How to access array elements in a loop")
-   [How to change array element values](https://www.koderhq.com/tutorial/rust/array/#mutate "Jump directly to: How to change array element values")
-   [How to pass an array to a function](https://www.koderhq.com/tutorial/rust/array/#parameter "Jump directly to: How to pass an array to a function")
-   [Summary: Points to remember](https://www.koderhq.com/tutorial/rust/array/#summary "Jump directly to: Summary: Points to remember")

Let's jump right in.

## What is an array

An array is a data container that can hold multiple separate values of the same type. We can think of it as a variable that can accept multiple separate values.

An array will be allocated to memory sequentially, that’s to say it will occupy memory blocks one after the other. This typically speeds up access to memory.

## How to create (initialize) an array

Rust provides us with 3 variations in syntax to declare an array.

**SYNTAX:** declare an array

```rust
// variation 1
let variable_name = [value_1, value_2, ...];

// variation 2
let variable_name:[data_type; array_size] = [value_1, value_2, ...];

// variation 3
let variable_name:[data_type; array_size] = [default_value_for_all_elements; array_size];

```

The simplest variation will let the compiler infer the array type from the values we give it.

**EXAMPLE:** automatically infer array type

```rust
fn main() {

    let users = ["John", "Jane", "Jack", "Jill"];

}

```

The example above creates an array with 4 string elements.

The second variation allows us to specify the data type and array size explicitly.

To do this we add a colon after the array name, followed by open and close square brackets. Inside the square brackets we specify first the data type, then an integer value for the amount of values the array can hold. The type and size is separated by a semicolon, not a comma.

**EXAMPLE:** explicitly declare type and size

```rust
fn main() {

    let users:[&str; 4] = ["John", "Jane", "Jack", "Jill"];

}

```

In the example above, we explicitly declare a data type of string and an array size of 4.

The third variation allows us to initialize all the array elements with the same value.

To do this we add only a single value that will be used to populate all the elements in the array, followed by the array size, and separated with a semicolon.

**EXAMPLE:** initialize all array elements with a single value

```rust
fn main() {

    let users:[&str; 4] = ["Unknown"; 4];

}

```

In the example above, all 4 elements in the array will have the value “Unknown”.

## How to access array elements with the indexer

Before we proceed, let’s see how to print the whole array to the console.

**EXAMPLE:** print the full array to the console

```rust
fn main() {

    let users = ["John", "Jane", "Jack", "Jill"];

    println!("{:?}", users);

}

```

We simply specify the array name in the println macro. The :? between the curly braces will do what’s known as a pretty print variation, allowing us to see the whole array at once.

_This helps us quickly see all the values in an array._

Individual array elements are accessed with the indexer. Every element in an array has a corresponding number assigned to it.

0

1

2

3

John

Jane

Jack

Jill

If we want to access a value, we use its index number between square brackets to get the value.

**EXAMPLE:** access individual array elements

```rust
fn main() {

    let users = ["John", "Jane", "Jack", "Jill"];

    println!("{}", users[2]);

}

```

In the example above, we access an element of the array with its corresponding index number.

You may have noticed that we use the number 2, but the value we accessed was “Jack” which is the third element in the array.

Any array or collection with an index, will always start at the number 0, not at 1. This is true for almost all programming languages (lua tables start at 1 for example).

So, if we want to access an element, we just subtract one from its position and we have the correct index.

## How to access array elements in a loop

A more practical and effective method to access elements in an array is to loop over them.

**EXAMPLE:** loop over all array elements

```rust
fn main() {

    let users = ["John", "Jane", "Jack", "Jill"];

    for x in users.iter() {
        println!("User: {}", x);
    }

}

```

.iter() is a function that we can use to iterate over individual elements. All arrays have access to this function and we use it by simply appending .iter() to the array name.

In the for loop, the temporary variable (in this case x) is used to access each individual element.

## How to change array element values

To change elements in an array we have two options.

-   Make the array mutable with mut.
-   Initialize the array anew with the changed values.

Let’s start with a mutable array. We add the  mut  keyword to the array initialization to allow the array to be mutable.

Then, we assign a new value to the array element by using its index.

**EXAMPLE:** mut array

```rust
fn main() {

    let mut users = ["John", "Jane", "Jack", "Jill"];

    println!("Before: {}", users[0]);

    // mutate array element 0
    users[0] = "John Doe";

    println!("After: {}", users[0]);

}

```

The second option is to simply reinitialize the array with different values. This may not be practical in most circumstances but we have the option.

**EXAMPLE:** reinitialize array

```rust
fn main() {

    let users = ["John", "Jane", "Jack", "Jill"];
    println!("Before: {:?}", users);

    let users = ["John Doe", "Jane Doe", "Jack", "Jill"];
    println!("After: {:?}", users);

}

```

This solution is not practical because we’ll often want to mutate only a single value in a large array.

## How to pass an array to a function

To pass an array to a function as a parameter, we simply declare the parameter as an array.

When declaring an array as a parameter we have to include it’s type and size.

**EXAMPLE:** pass an array to a function

```rust
fn main() {

    let num:[i32; 2] = [5, 3];

    println!("{:?}", sum(num));
}

fn sum(arr:[i32; 2]) -> i32 {

    return arr[0] + arr[1];

}

```

In the function definition above, the  **arr**  parameter can accept an array of type i32 with 2 elements.

### Summary: Points to remember

-   An array is a data container that can hold more than one value.
    -   An array is stored sequentially in memory.
    -   An array can only accept a single type of values.
-   An array is initialized with values in one of three syntax variations.
    -   We can let the compiler infer the type from the values automatically.
-   We access individual array elements with the indexer, specifying their corresponding index number between brackets.
-   For loops are perfect to iterate over elements in an array.
-   To be able to change array element values, we make an array mutable with the  **mut**  keyword.
    -   To change the value itself, we assign a new value to the element in the array via the indexer.
-   We can pass an array to a function by declaring the parameter as an array. We also have to declare the type and size explicitly.

---

# Rust - Array

----------

Advertisements

[Previous Page](https://www.tutorialspoint.com/rust/rust_tuple.htm)

[Next Page](https://www.tutorialspoint.com/rust/rust_ownership.htm)

In this chapter, we will learn about an array and the various features associated with it. Before we learn about arrays, let us see how an array is different from a variable.

Variables have the following limitations −

-   Variables are scalar in nature. In other words, a variable declaration can only contain a single value at a time. This means that to store n values in a program n variable declaration will be needed. Hence, the use of variables is not feasible when one needs to store a larger collection of values.
    
-   Variables in a program are allocated memory in random order, thereby making it difficult to retrieve/read the values in the order of their declaration.
    

An array is a homogeneous collection of values. Simply put, an array is a collection of values of the same data type.

## Features of an Array

The features of an array are as listed below −

-   An array declaration allocates sequential memory blocks.
    
-   Arrays are static. This means that an array once initialized cannot be resized.
    
-   Each memory block represents an array element.
    
-   Array elements are identified by a unique integer called the subscript/ index of the element.
    
-   Populating the array elements is known as array initialization.
    
-   Array element values can be updated or modified but cannot be deleted.
    

## Declaring and Initializing Arrays

Use the syntax given below to declare and initialize an array in Rust.

### Syntax

//Syntax1
let variable_name = [value1,value2,value3];

//Syntax2
let variable_name:[dataType;size] = [value1,value2,value3];

//Syntax3
let variable_name:[dataType;size] = [default_value_for_elements,size];

In the first syntax, type of the array is inferred from the data type of the array’s first element during initialization.

### Illustration: Simple Array

The following example explicitly specifies the size and the data type of the array. The  _{:?} syntax of the println!()_  function is used to print all values in the array. The  _len()_  function is used to compute the size of the array.

fn main(){
   let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());
}

### Output

array is [10, 20, 30, 40]
array size is :4

### Illustration: Array without data type

The following program declares an array of 4 elements. The datatype is not explicitly specified during the variable declaration. In this case, the array will be of type integer. The  _len()_  function is used to compute the size of the array.

fn main(){
   let arr = [10,20,30,40];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());
}

### Output

array is [10, 20, 30, 40]
array size is :4

### Illustration: Default values

The following example creates an array and initializes all its elements with a default value of  _-1_.

fn main() {
   let arr:[i32;4] = [-1;4];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());
}

### Output

array is [-1, -1, -1, -1]
array size is :4

### Illustration: Array with for loop

The following example iterates through an array and prints the indexes and their corresponding values. The loop retrieves values from index 0 to 4 (index of the last array element).

fn main(){
   let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());

   for index in 0..4 {
      println!("index is: {} & value is : {}",index,arr[index]);
   }
}

### Output

array is [10, 20, 30, 40]
array size is :4
index is: 0 & value is : 10
index is: 1 & value is : 20
index is: 2 & value is : 30
index is: 3 & value is : 40

### Illustration: Using the iter() function

The iter() function fetches values of all elements in an array.

fn main(){

let arr:[i32;4] = [10,20,30,40];
   println!("array is {:?}",arr);
   println!("array size is :{}",arr.len());

   for val in arr.iter(){
      println!("value is :{}",val);
   }
}

### Output

array is [10, 20, 30, 40]
array size is :4
value is :10
value is :20
value is :30
value is :40

### Illustration: Mutable array

The  _mut_  keyword can be used to declare a mutable array. The following example declares a mutable array and modifies value of the second array element.

fn main(){
   let mut arr:[i32;4] = [10,20,30,40];
   arr[1] = 0;
   println!("{:?}",arr);
}

### Output

[10, 0, 30, 40]

## Passing Arrays as Parameters to Functions

An array can be passed by value or by reference to functions.

### Illustration: Pass by value

fn main() {
   let arr = [10,20,30];
   update(arr);

   print!("Inside main {:?}",arr);
}
fn update(mut arr:[i32;3]){
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}",arr);
}

### Output

Inside update [0, 0, 0]
Inside main [10, 20, 30]

### Illustration: Pass by reference

fn main() {
   let mut arr = [10,20,30];
   update(&mut arr);
   print!("Inside main {:?}",arr);
}
fn update(arr:&mut [i32;3]){
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}",arr);
}

### Output

Inside update [0, 0, 0]
Inside main [0, 0, 0]

## Array Declaration and Constants

Let us consider an example given below to understand array declaration and constants.

fn main() {
   let N: usize = 20;
   let arr = [0; N]; //Error: non-constant used with constant
   print!("{}",arr[10])
}

The compiler will result in an exception. This is because an array's length must be known at compile time. Here, the value of the variable "N" will be determined at runtime. In other words, variables cannot be used to define the size of an array.

However, the following program is valid −

fn main() {
   const N: usize = 20; //reading note: must used const , it cannot use the let. By 20200312.
   // pointer sized
   let arr = [0; N];

   print!("{}",arr[10])
}

The value of an identifier prefixed with the const keyword is defined at compile time and cannot be changed at runtime. usize is pointer-sized, thus its actual size depends on the architecture you are compiling your program for.

[Previous Page](https://www.tutorialspoint.com/rust/rust_tuple.htm)  Print Page

[Next Page](https://www.tutorialspoint.com/rust/rust_ownership.htm)

Advertisements




$$$$$$

---

$$$$$$


# Arrays

[![Task](http://rosettacode.org/mw/images/thumb/b/ba/Rcode-button-task-crushed.png/64px-Rcode-button-task-crushed.png)](http://rosettacode.org/wiki/Category:Solutions_by_Programming_Task "Category:Solutions by Programming Task")

**Arrays**  
You are encouraged to  [solve this task](http://rosettacode.org/wiki/Rosetta_Code:Solve_a_Task "Rosetta Code:Solve a Task")  according to the task description, using any language you may know.

This task is about arrays.

For hashes or associative arrays, please see  [Creating an Associative Array](http://rosettacode.org/wiki/Creating_an_Associative_Array "Creating an Associative Array").

For a definition and in-depth discussion of what an array is, see  [Array](http://rosettacode.org/wiki/Array "Array").

  

Task

Show basic array syntax in your language.

Basically, create an array, assign a value to it, and retrieve an element (if available, show both fixed-length arrays and dynamic arrays, pushing a value into it).

Please discuss at Village Pump: [Arrays](http://rosettacode.org/wiki/Rosetta_Code:Village_Pump/Arrays "Rosetta Code:Village Pump/Arrays").

Please merge code in from these obsolete tasks:

-   [Creating an Array](http://rosettacode.org/wiki/Creating_an_Array "Creating an Array")
-   [Assigning Values to an Array](http://rosettacode.org/wiki/Assigning_Values_to_an_Array "Assigning Values to an Array")
-   [Retrieving an Element of an Array](http://rosettacode.org/wiki/Retrieving_an_Element_of_an_Array "Retrieving an Element of an Array")

  

Related tasks

-   [Collections](http://rosettacode.org/wiki/Collections "Collections")
-   [Creating an Associative Array](http://rosettacode.org/wiki/Creating_an_Associative_Array "Creating an Associative Array")
-   [Two-dimensional array (runtime)](http://rosettacode.org/wiki/Two-dimensional_array_(runtime) "Two-dimensional array (runtime)")

  
  

## Contents

[[hide](http://rosettacode.org/wiki/Arrays#)]

-   [1  360 Assembly](http://rosettacode.org/wiki/Arrays#360_Assembly)
-   [2  8051 Assembly](http://rosettacode.org/wiki/Arrays#8051_Assembly)
-   [3  8th](http://rosettacode.org/wiki/Arrays#8th)
-   [4  ABAP](http://rosettacode.org/wiki/Arrays#ABAP)
-   [5  ACL2](http://rosettacode.org/wiki/Arrays#ACL2)
-   [6  ActionScript](http://rosettacode.org/wiki/Arrays#ActionScript)
-   [7  Ada](http://rosettacode.org/wiki/Arrays#Ada)
-   [8  Aikido](http://rosettacode.org/wiki/Arrays#Aikido)
-   [9  Aime](http://rosettacode.org/wiki/Arrays#Aime)
-   [10  ALGOL 68](http://rosettacode.org/wiki/Arrays#ALGOL_68)
-   [11  ALGOL W](http://rosettacode.org/wiki/Arrays#ALGOL_W)
-   [12  AmigaE](http://rosettacode.org/wiki/Arrays#AmigaE)
-   [13  AntLang](http://rosettacode.org/wiki/Arrays#AntLang)
-   [14  APL](http://rosettacode.org/wiki/Arrays#APL)
-   [15  App Inventor](http://rosettacode.org/wiki/Arrays#App_Inventor)
-   [16  Apex](http://rosettacode.org/wiki/Arrays#Apex)
-   [17  AppleScript](http://rosettacode.org/wiki/Arrays#AppleScript)
-   [18  Arendelle](http://rosettacode.org/wiki/Arrays#Arendelle)
-   [19  Argile](http://rosettacode.org/wiki/Arrays#Argile)
-   [20  ARM Assembly](http://rosettacode.org/wiki/Arrays#ARM_Assembly)
-   [21  Arturo](http://rosettacode.org/wiki/Arrays#Arturo)
-   [22  AutoHotkey](http://rosettacode.org/wiki/Arrays#AutoHotkey)
-   [23  AutoIt](http://rosettacode.org/wiki/Arrays#AutoIt)
-   [24  AWK](http://rosettacode.org/wiki/Arrays#AWK)
-   [25  Axe](http://rosettacode.org/wiki/Arrays#Axe)
-   [26  Babel](http://rosettacode.org/wiki/Arrays#Babel)
    -   [26.1  Create an array](http://rosettacode.org/wiki/Arrays#Create_an_array)
    -   [26.2  Get a single array element](http://rosettacode.org/wiki/Arrays#Get_a_single_array_element)
    -   [26.3  Change an array element](http://rosettacode.org/wiki/Arrays#Change_an_array_element)
    -   [26.4  Select a range of an array](http://rosettacode.org/wiki/Arrays#Select_a_range_of_an_array)
    -   [26.5  Add a new element to an array](http://rosettacode.org/wiki/Arrays#Add_a_new_element_to_an_array)
    -   [26.6  Convert between arrays and lists](http://rosettacode.org/wiki/Arrays#Convert_between_arrays_and_lists)
-   [27  BASIC](http://rosettacode.org/wiki/Arrays#BASIC)
    -   [27.1  Static](http://rosettacode.org/wiki/Arrays#Static)
    -   [27.2  Dynamic](http://rosettacode.org/wiki/Arrays#Dynamic)
    -   [27.3  Applesoft BASIC](http://rosettacode.org/wiki/Arrays#Applesoft_BASIC)
    -   [27.4  Commodore BASIC](http://rosettacode.org/wiki/Arrays#Commodore_BASIC)
-   [28  BASIC256](http://rosettacode.org/wiki/Arrays#BASIC256)
-   [29  Batch File](http://rosettacode.org/wiki/Arrays#Batch_File)
-   [30  BBC BASIC](http://rosettacode.org/wiki/Arrays#BBC_BASIC)
-   [31  bc](http://rosettacode.org/wiki/Arrays#bc)
-   [32  BML](http://rosettacode.org/wiki/Arrays#BML)
-   [33  Bracmat](http://rosettacode.org/wiki/Arrays#Bracmat)
-   [34  Boo](http://rosettacode.org/wiki/Arrays#Boo)
-   [35  Brainf***](http://rosettacode.org/wiki/Arrays#Brainf.2A.2A.2A)
-   [36  C](http://rosettacode.org/wiki/Arrays#C)
-   [37  ChucK](http://rosettacode.org/wiki/Arrays#ChucK)
-   [38  C++](http://rosettacode.org/wiki/Arrays#C.2B.2B)
-   [39  C#](http://rosettacode.org/wiki/Arrays#C.23)
-   [40  Ceylon](http://rosettacode.org/wiki/Arrays#Ceylon)
-   [41  Clean](http://rosettacode.org/wiki/Arrays#Clean)
    -   [41.1  Lazy array](http://rosettacode.org/wiki/Arrays#Lazy_array)
    -   [41.2  Strict array](http://rosettacode.org/wiki/Arrays#Strict_array)
    -   [41.3  Unboxed array](http://rosettacode.org/wiki/Arrays#Unboxed_array)
-   [42  Clipper](http://rosettacode.org/wiki/Arrays#Clipper)
-   [43  Clojure](http://rosettacode.org/wiki/Arrays#Clojure)
-   [44  COBOL](http://rosettacode.org/wiki/Arrays#COBOL)
-   [45  CoffeeScript](http://rosettacode.org/wiki/Arrays#CoffeeScript)
-   [46  ColdFusion](http://rosettacode.org/wiki/Arrays#ColdFusion)
-   [47  Common Lisp](http://rosettacode.org/wiki/Arrays#Common_Lisp)
-   [48  Component Pascal](http://rosettacode.org/wiki/Arrays#Component_Pascal)
-   [49  Computer/zero Assembly](http://rosettacode.org/wiki/Arrays#Computer.2Fzero_Assembly)
    -   [49.1  Fixed-length array](http://rosettacode.org/wiki/Arrays#Fixed-length_array)
    -   [49.2  Zero-terminated array](http://rosettacode.org/wiki/Arrays#Zero-terminated_array)
-   [50  D](http://rosettacode.org/wiki/Arrays#D)
-   [51  Dao](http://rosettacode.org/wiki/Arrays#Dao)
-   [52  Déjà Vu](http://rosettacode.org/wiki/Arrays#D.C3.A9j.C3.A0_Vu)
-   [53  Delphi](http://rosettacode.org/wiki/Arrays#Delphi)
-   [54  Dragon](http://rosettacode.org/wiki/Arrays#Dragon)
-   [55  DWScript](http://rosettacode.org/wiki/Arrays#DWScript)
-   [56  Dyalect](http://rosettacode.org/wiki/Arrays#Dyalect)
-   [57  E](http://rosettacode.org/wiki/Arrays#E)
-   [58  EasyLang](http://rosettacode.org/wiki/Arrays#EasyLang)
-   [59  EGL](http://rosettacode.org/wiki/Arrays#EGL)
-   [60  Eiffel](http://rosettacode.org/wiki/Arrays#Eiffel)
-   [61  Elena](http://rosettacode.org/wiki/Arrays#Elena)
-   [62  Elixir](http://rosettacode.org/wiki/Arrays#Elixir)
-   [63  Erlang](http://rosettacode.org/wiki/Arrays#Erlang)
-   [64  ERRE](http://rosettacode.org/wiki/Arrays#ERRE)
-   [65  Euphoria](http://rosettacode.org/wiki/Arrays#Euphoria)
-   [66  F#](http://rosettacode.org/wiki/Arrays#F.23)
-   [67  Factor](http://rosettacode.org/wiki/Arrays#Factor)
-   [68  FBSL](http://rosettacode.org/wiki/Arrays#FBSL)
-   [69  Forth](http://rosettacode.org/wiki/Arrays#Forth)
-   [70  Fortran](http://rosettacode.org/wiki/Arrays#Fortran)
-   [71  FreeBASIC](http://rosettacode.org/wiki/Arrays#FreeBASIC)
-   [72  Frink](http://rosettacode.org/wiki/Arrays#Frink)
-   [73  Futhark](http://rosettacode.org/wiki/Arrays#Futhark)
-   [74  Gambas](http://rosettacode.org/wiki/Arrays#Gambas)
-   [75  GAP](http://rosettacode.org/wiki/Arrays#GAP)
-   [76  Genie](http://rosettacode.org/wiki/Arrays#Genie)
-   [77  GML](http://rosettacode.org/wiki/Arrays#GML)
    -   [77.1  1-Dimensional Array Examples](http://rosettacode.org/wiki/Arrays#1-Dimensional_Array_Examples)
        -   [77.1.1  Example of Fixed Length Array](http://rosettacode.org/wiki/Arrays#Example_of_Fixed_Length_Array)
        -   [77.1.2  Example of Arbitrary Length Array](http://rosettacode.org/wiki/Arrays#Example_of_Arbitrary_Length_Array)
    -   [77.2  2-Dimensional Array Examples](http://rosettacode.org/wiki/Arrays#2-Dimensional_Array_Examples)
        -   [77.2.1  Example of Fixed Length Array](http://rosettacode.org/wiki/Arrays#Example_of_Fixed_Length_Array_2)
        -   [77.2.2  Example of Arbitrary Length Array](http://rosettacode.org/wiki/Arrays#Example_of_Arbitrary_Length_Array_2)
-   [78  Go](http://rosettacode.org/wiki/Arrays#Go)
-   [79  Golfscript](http://rosettacode.org/wiki/Arrays#Golfscript)
-   [80  Groovy](http://rosettacode.org/wiki/Arrays#Groovy)
-   [81  GUISS](http://rosettacode.org/wiki/Arrays#GUISS)
-   [82  GW-BASIC](http://rosettacode.org/wiki/Arrays#GW-BASIC)
-   [83  Halon](http://rosettacode.org/wiki/Arrays#Halon)
-   [84  Harbour](http://rosettacode.org/wiki/Arrays#Harbour)
-   [85  Haskell](http://rosettacode.org/wiki/Arrays#Haskell)
-   [86  hexiscript](http://rosettacode.org/wiki/Arrays#hexiscript)
-   [87  HicEst](http://rosettacode.org/wiki/Arrays#HicEst)
-   [88  i](http://rosettacode.org/wiki/Arrays#i)
-   [89  HolyC](http://rosettacode.org/wiki/Arrays#HolyC)
-   [90  Icon and Unicon](http://rosettacode.org/wiki/Arrays#Icon_and_Unicon)
    -   [90.1  Icon](http://rosettacode.org/wiki/Arrays#Icon)
    -   [90.2  Unicon](http://rosettacode.org/wiki/Arrays#Unicon)
-   [91  Io](http://rosettacode.org/wiki/Arrays#Io)
-   [92  J](http://rosettacode.org/wiki/Arrays#J)
-   [93  Java](http://rosettacode.org/wiki/Arrays#Java)
-   [94  JavaScript](http://rosettacode.org/wiki/Arrays#JavaScript)
-   [95  jq](http://rosettacode.org/wiki/Arrays#jq)
-   [96  Jsish](http://rosettacode.org/wiki/Arrays#Jsish)
-   [97  Julia](http://rosettacode.org/wiki/Arrays#Julia)
-   [98  KonsolScript](http://rosettacode.org/wiki/Arrays#KonsolScript)
-   [99  Kotlin](http://rosettacode.org/wiki/Arrays#Kotlin)
-   [100  LabVIEW](http://rosettacode.org/wiki/Arrays#LabVIEW)
-   [101  lang5](http://rosettacode.org/wiki/Arrays#lang5)
-   [102  Lasso](http://rosettacode.org/wiki/Arrays#Lasso)
    -   [102.1  Static Arrays](http://rosettacode.org/wiki/Arrays#Static_Arrays)
-   [103  Latitude](http://rosettacode.org/wiki/Arrays#Latitude)
-   [104  LFE](http://rosettacode.org/wiki/Arrays#LFE)
-   [105  Liberty BASIC](http://rosettacode.org/wiki/Arrays#Liberty_BASIC)
-   [106  LIL](http://rosettacode.org/wiki/Arrays#LIL)
-   [107  Lingo](http://rosettacode.org/wiki/Arrays#Lingo)
-   [108  Lisaac](http://rosettacode.org/wiki/Arrays#Lisaac)
-   [109  Little](http://rosettacode.org/wiki/Arrays#Little)
-   [110  Logo](http://rosettacode.org/wiki/Arrays#Logo)
-   [111  LSE64](http://rosettacode.org/wiki/Arrays#LSE64)
-   [112  LSL](http://rosettacode.org/wiki/Arrays#LSL)
-   [113  Lua](http://rosettacode.org/wiki/Arrays#Lua)
-   [114  M2000 Interpreter](http://rosettacode.org/wiki/Arrays#M2000_Interpreter)
    -   [114.1  Passing Arrays By Reference](http://rosettacode.org/wiki/Arrays#Passing_Arrays_By_Reference)
-   [115  Maple](http://rosettacode.org/wiki/Arrays#Maple)
-   [116  Mathematica / Wolfram Language](http://rosettacode.org/wiki/Arrays#Mathematica_.2F_Wolfram_Language)
-   [117  MATLAB / Octave](http://rosettacode.org/wiki/Arrays#MATLAB_.2F_Octave)
-   [118  Maxima](http://rosettacode.org/wiki/Arrays#Maxima)
-   [119  Mercury](http://rosettacode.org/wiki/Arrays#Mercury)
-   [120  MIPS Assembly](http://rosettacode.org/wiki/Arrays#MIPS_Assembly)
-   [121  MiniScript](http://rosettacode.org/wiki/Arrays#MiniScript)
-   [122  Modula-2](http://rosettacode.org/wiki/Arrays#Modula-2)
-   [123  Modula-3](http://rosettacode.org/wiki/Arrays#Modula-3)
-   [124  Monte](http://rosettacode.org/wiki/Arrays#Monte)
-   [125  Neko](http://rosettacode.org/wiki/Arrays#Neko)
-   [126  Nemerle](http://rosettacode.org/wiki/Arrays#Nemerle)
-   [127  NetRexx](http://rosettacode.org/wiki/Arrays#NetRexx)
-   [128  NewLISP](http://rosettacode.org/wiki/Arrays#NewLISP)
-   [129  Nim](http://rosettacode.org/wiki/Arrays#Nim)
-   [130  NS-HUBASIC](http://rosettacode.org/wiki/Arrays#NS-HUBASIC)
-   [131  NSIS](http://rosettacode.org/wiki/Arrays#NSIS)
-   [132  Oberon-2](http://rosettacode.org/wiki/Arrays#Oberon-2)
-   [133  Objeck](http://rosettacode.org/wiki/Arrays#Objeck)
-   [134  Objective-C](http://rosettacode.org/wiki/Arrays#Objective-C)
-   [135  OCaml](http://rosettacode.org/wiki/Arrays#OCaml)
-   [136  Oforth](http://rosettacode.org/wiki/Arrays#Oforth)
-   [137  Ol](http://rosettacode.org/wiki/Arrays#Ol)
-   [138  ooRexx](http://rosettacode.org/wiki/Arrays#ooRexx)
-   [139  OxygenBasic](http://rosettacode.org/wiki/Arrays#OxygenBasic)
-   [140  Oz](http://rosettacode.org/wiki/Arrays#Oz)
-   [141  PARI/GP](http://rosettacode.org/wiki/Arrays#PARI.2FGP)
-   [142  Pascal](http://rosettacode.org/wiki/Arrays#Pascal)
-   [143  Perl](http://rosettacode.org/wiki/Arrays#Perl)
-   [144  Perl 6](http://rosettacode.org/wiki/Arrays#Perl_6)
    -   [144.1  Some further exposition:](http://rosettacode.org/wiki/Arrays#Some_further_exposition:)
-   [145  Phix](http://rosettacode.org/wiki/Arrays#Phix)
-   [146  PHP](http://rosettacode.org/wiki/Arrays#PHP)
    -   [146.1  Writing To An Array](http://rosettacode.org/wiki/Arrays#Writing_To_An_Array)
        -   [146.1.1  Single Dimension](http://rosettacode.org/wiki/Arrays#Single_Dimension)
        -   [146.1.2  Multi-Dimensional](http://rosettacode.org/wiki/Arrays#Multi-Dimensional)
        -   [146.1.3  Array push](http://rosettacode.org/wiki/Arrays#Array_push)
    -   [146.2  Reading From An Array](http://rosettacode.org/wiki/Arrays#Reading_From_An_Array)
        -   [146.2.1  Single Dimension](http://rosettacode.org/wiki/Arrays#Single_Dimension_2)
        -   [146.2.2  Multi-Dimensional](http://rosettacode.org/wiki/Arrays#Multi-Dimensional_2)
    -   [146.3  Print a whole array](http://rosettacode.org/wiki/Arrays#Print_a_whole_array)
    -   [146.4  Set custom keys for values](http://rosettacode.org/wiki/Arrays#Set_custom_keys_for_values)
    -   [146.5  Other Examples](http://rosettacode.org/wiki/Arrays#Other_Examples)
-   [147  PicoLisp](http://rosettacode.org/wiki/Arrays#PicoLisp)
-   [148  Pike](http://rosettacode.org/wiki/Arrays#Pike)
-   [149  PL/I](http://rosettacode.org/wiki/Arrays#PL.2FI)
-   [150  Pony](http://rosettacode.org/wiki/Arrays#Pony)
-   [151  PostScript](http://rosettacode.org/wiki/Arrays#PostScript)
-   [152  PowerShell](http://rosettacode.org/wiki/Arrays#PowerShell)
-   [153  Prolog](http://rosettacode.org/wiki/Arrays#Prolog)
-   [154  PureBasic](http://rosettacode.org/wiki/Arrays#PureBasic)
-   [155  Python](http://rosettacode.org/wiki/Arrays#Python)
-   [156  R](http://rosettacode.org/wiki/Arrays#R)
-   [157  Racket](http://rosettacode.org/wiki/Arrays#Racket)
-   [158  REBOL](http://rosettacode.org/wiki/Arrays#REBOL)
-   [159  Red](http://rosettacode.org/wiki/Arrays#Red)
-   [160  Retro](http://rosettacode.org/wiki/Arrays#Retro)
-   [161  REXX](http://rosettacode.org/wiki/Arrays#REXX)
    -   [161.1  simple arrays](http://rosettacode.org/wiki/Arrays#simple_arrays)
    -   [161.2  simple arrays, mimic other languages](http://rosettacode.org/wiki/Arrays#simple_arrays.2C_mimic_other_languages)
    -   [161.3  simple arrays, assigned default](http://rosettacode.org/wiki/Arrays#simple_arrays.2C_assigned_default)
    -   [161.4  arrays with non-unity index start](http://rosettacode.org/wiki/Arrays#arrays_with_non-unity_index_start)
    -   [161.5  arrays, disjoint](http://rosettacode.org/wiki/Arrays#arrays.2C_disjoint)
    -   [161.6  sparse arrays and special indices](http://rosettacode.org/wiki/Arrays#sparse_arrays_and_special_indices)
-   [162  RLaB](http://rosettacode.org/wiki/Arrays#RLaB)
-   [163  RPG](http://rosettacode.org/wiki/Arrays#RPG)
-   [164  Ring](http://rosettacode.org/wiki/Arrays#Ring)
-   [165  Robotic](http://rosettacode.org/wiki/Arrays#Robotic)
-   [166  Ruby](http://rosettacode.org/wiki/Arrays#Ruby)
-   [167  Run BASIC](http://rosettacode.org/wiki/Arrays#Run_BASIC)
-   [168  Rust](http://rosettacode.org/wiki/Arrays#Rust)
-   [169  Sather](http://rosettacode.org/wiki/Arrays#Sather)
-   [170  Scala](http://rosettacode.org/wiki/Arrays#Scala)
-   [171  Scheme](http://rosettacode.org/wiki/Arrays#Scheme)
-   [172  Scratch](http://rosettacode.org/wiki/Arrays#Scratch)
-   [173  Seed7](http://rosettacode.org/wiki/Arrays#Seed7)
-   [174  Self](http://rosettacode.org/wiki/Arrays#Self)
-   [175  Sidef](http://rosettacode.org/wiki/Arrays#Sidef)
-   [176  Simula](http://rosettacode.org/wiki/Arrays#Simula)
-   [177  Slate](http://rosettacode.org/wiki/Arrays#Slate)
-   [178  Smalltalk](http://rosettacode.org/wiki/Arrays#Smalltalk)
-   [179  SNOBOL4](http://rosettacode.org/wiki/Arrays#SNOBOL4)
-   [180  SPL](http://rosettacode.org/wiki/Arrays#SPL)
-   [181  SSEM](http://rosettacode.org/wiki/Arrays#SSEM)
-   [182  Stata](http://rosettacode.org/wiki/Arrays#Stata)
    -   [182.1  Matrix command](http://rosettacode.org/wiki/Arrays#Matrix_command)
    -   [182.2  Mata](http://rosettacode.org/wiki/Arrays#Mata)
-   [183  Suneido](http://rosettacode.org/wiki/Arrays#Suneido)
-   [184  Swift](http://rosettacode.org/wiki/Arrays#Swift)
-   [185  Tcl](http://rosettacode.org/wiki/Arrays#Tcl)
-   [186  Tern](http://rosettacode.org/wiki/Arrays#Tern)
-   [187  TI-83 BASIC](http://rosettacode.org/wiki/Arrays#TI-83_BASIC)
-   [188  TorqueScript](http://rosettacode.org/wiki/Arrays#TorqueScript)
-   [189  TXR](http://rosettacode.org/wiki/Arrays#TXR)
    -   [189.1  Literals](http://rosettacode.org/wiki/Arrays#Literals)
    -   [189.2  Construction](http://rosettacode.org/wiki/Arrays#Construction)
    -   [189.3  Array Indexing Notation](http://rosettacode.org/wiki/Arrays#Array_Indexing_Notation)
    -   [189.4  Array Range Notation](http://rosettacode.org/wiki/Arrays#Array_Range_Notation)
    -   [189.5  In The Pattern Language](http://rosettacode.org/wiki/Arrays#In_The_Pattern_Language)
    -   [189.6  Other Kinds of Objects](http://rosettacode.org/wiki/Arrays#Other_Kinds_of_Objects)
-   [190  uBasic/4tH](http://rosettacode.org/wiki/Arrays#uBasic.2F4tH)
-   [191  Unicon](http://rosettacode.org/wiki/Arrays#Unicon_2)
-   [192  UNIX Shell](http://rosettacode.org/wiki/Arrays#UNIX_Shell)
-   [193  உயிர்/Uyir](http://rosettacode.org/wiki/Arrays#.E0.AE.89.E0.AE.AF.E0.AE.BF.E0.AE.B0.E0.AF.8D.2FUyir)
-   [194  Vala](http://rosettacode.org/wiki/Arrays#Vala)
-   [195  Vim Script](http://rosettacode.org/wiki/Arrays#Vim_Script)
-   [196  VBA](http://rosettacode.org/wiki/Arrays#VBA)
-   [197  Visual Basic .NET](http://rosettacode.org/wiki/Arrays#Visual_Basic_.NET)
-   [198  VHDL](http://rosettacode.org/wiki/Arrays#VHDL)
-   [199  Wee Basic](http://rosettacode.org/wiki/Arrays#Wee_Basic)
-   [200  Wren](http://rosettacode.org/wiki/Arrays#Wren)
-   [201  X86 Assembly](http://rosettacode.org/wiki/Arrays#X86_Assembly)
-   [202  XLISP](http://rosettacode.org/wiki/Arrays#XLISP)
-   [203  XPL0](http://rosettacode.org/wiki/Arrays#XPL0)
-   [204  Yabasic](http://rosettacode.org/wiki/Arrays#Yabasic)
-   [205  zonnon](http://rosettacode.org/wiki/Arrays#zonnon)
-   [206  zkl](http://rosettacode.org/wiki/Arrays#zkl)
-   [207  ZX Spectrum Basic](http://rosettacode.org/wiki/Arrays#ZX_Spectrum_Basic)

## [360 Assembly](http://rosettacode.org/wiki/Category:360_Assembly "Category:360 Assembly")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=1 "Edit section: 360 Assembly")]

*        Arrays                    04/09/2015  
ARRAYS   PROLOG  
*        we use TA array with 1 as origin. So TA(1) to TA(20)  
*        ta(i)=ta(j)  
         L      R1,J               j  
         BCTR   R1,0               -1  
         SLA    R1,2               r1=(j-1)*4  (*4 by shift left)  
         L      R0,TA(R1)          load r0 with ta(j)  
         L      R1,I               i  
         BCTR   R1,0               -1  
         SLA    R1,2               r1=(i-1)*4  (*4 by shift left)  
         ST     R0,TA(R1)          store r0 to ta(i)  
         EPILOG  
* Array of 20 integers (32 bits) (4 bytes)  
TA       DS     20F  
* Initialized array of 10 integers (32 bits)  
TB       DC     10F'0'  
* Initialized array of 10 integers (32 bits)  
TC       DC     F'1',F'2',F'3',F'4',F'5',F'6',F'7',F'8',F'9',F'10'  
* Array of 10 integers (16 bits)  
TD       DS     10H  
* Array of 10 strings of 8 characters (initialized)  
TE       DC     10CL8' '  
* Array of 10 double precision floating point reals (64 bits)  
TF       DS     10D  
*  
I        DC     F'2'  
J        DC     F'4'  
         YREGS  
         END    ARRAYS

## [8051 Assembly](http://rosettacode.org/wiki/Category:8051_Assembly "Category:8051 Assembly")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=2 "Edit section: 8051 Assembly")]

There are three types of fixed-length arrays:

-   In the code segment - array elements are constant; good for strings, elements are easily indexed
-   In internal RAM - good for small arrays; elements are easily indexed
-   In external RAM - element retrieval/altering is most efficiently done sequentially, necessary for large arrays or peripherals

Dynamic (resizable) arrays are possible to implement, but are error-prone since bounds checking must be done by the programmer.

; constant array (elements are unchangeable) - the array is stored in the CODE segment  
myarray db 'Array' ; db = define bytes - initializes 5 bytes with values 41, 72, 72, etc. (the ascii characters A,r,r,a,y)  
myarray2 dw 'A','r','r','a','y' ; dw = define words - initializes 5 words (1 word = 2 bytes) with values 41 00 , 72 00, 72 00, etc.  
; how to read index a of the array  
        push acc  
        push dph  
        push dpl  
        mov dpl,#low(myarray) ; location of array  
        mov dph,#high(myarray)  
        movc a,@a+dptr	; a = element a  
        mov r0, a	; r0 = element a  
        pop dpl  
        pop dph  
        pop acc		; a = original index again  
   
; array stored in internal RAM (A_START is the first register of the array, A_END is the last)  
; initalise array data (with 0's)  
	push 0  
	mov r0, #A_START  
clear:  
	mov @r0, #0  
	inc r0  
	cjne r0, #A_END, clear  
	pop 0  
; how to read index r1 of array  
	push psw  
	mov a, #A_START  
	add a, r1	; a = memory location of element r1  
	push 0  
	mov r0, a  
	mov a, @r0	; a = element r1  
	pop 0  
	pop psw  
; how to write value of acc into index r1 of array  
	push psw  
	push 0  
	push acc  
	mov a, #A_START  
	add a, r1  
	mov r0, a  
	pop acc  
	mov @r0, a	; element r1 = a  
	pop 0  
	pop psw  
   
; array stored in external RAM (A_START is the first memory location of the array, LEN is the length)  
; initalise array data (with 0's)  
	push dph  
	push dpl  
	push acc  
	push 0  
	mov dptr, #A_START  
	clr a  
	mov r0, #LEN  
clear:  
	movx @dptr, a  
	inc dptr  
	djnz r0, clear  
	pop 0  
	pop acc  
	pop dpl  
	pop dph  
; how to read index r1 of array  
	push dph  
	push dpl  
	push 0  
	mov dptr, #A_START-1  
	mov r0, r1  
	inc r0  
loop:  
	inc dptr  
	djnz r0, loop  
	movx a, @dptr	; a = element r1  
	pop 0  
	pop dpl  
	pop dph  
; how to write value of acc into index r1 of array  
	push dph  
	push dpl  
	push 0  
	mov dptr, #A_START-1  
	mov r0, r1  
	inc r0  
loop:  
	inc dptr  
	djnz r0, loop  
	movx @dptr, a	; element r1 = a  
	pop 0  
	pop dpl  
	pop dph  
   
 

## [8th](http://rosettacode.org/wiki/Category:8th "Category:8th")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=3 "Edit section: 8th")]

Arrays are declared using JSON syntax, and are dynamic (but not sparse)

   
[ 1 , 2  ,3 ] \ an array holding three numbers  
1 a:@       \ this will be '2', the element at index 1  
drop  
1 123 a:@ \ this will store the value '123' at index 1, so now  
.              \ will print [1,123,3]  
   
[1,2,3] 45 a:push  
\ gives us [1,2,3,45]  
\ and empty spots are filled with null:  
[1,2,3] 5 15 a:!  
\ gives [1,2,3,null,15]  
   
\ arrays don't have to be homogenous:  
[1,"one", 2, "two"]  
 

## [ABAP](http://rosettacode.org/wiki/Category:ABAP "Category:ABAP")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=4 "Edit section: ABAP")]

There are no real arrays in ABAP but a construct called internal tables.

   
[TYPES](http://help.sap.com/abapdocu/en/ABAPTYPES.htm): tty_int TYPE STANDARD TABLE OF i  
                    WITH NON-UNIQUE DEFAULT KEY.  
   
[DATA](http://help.sap.com/abapdocu/en/ABAPDATA.htm)(itab) = VALUE tty_int( ( 1 )  
                            ( 2 )  
                            ( 3 ) ).  
   
[INSERT](http://help.sap.com/abapdocu/en/ABAPINSERT.htm) 4 INTO TABLE itab.  
[APPEND](http://help.sap.com/abapdocu/en/ABAPAPPEND.htm) 5 TO itab.  
[DELETE](http://help.sap.com/abapdocu/en/ABAPDELETE.htm) itab INDEX 1.  
   
cl_demo_output=>display( itab ).  
cl_demo_output=>display( itab[ 2 ] ).  
 

Output:

2 
3 
4 
5 

3

## [ACL2](http://rosettacode.org/wiki/Category:ACL2 "Category:ACL2")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=5 "Edit section: ACL2")]

;; Create an array and store it in array-example  
(assign array-example  
        (compress1 'array-example  
                   (list '(:header :dimensions (10)  
                                   :maximum-length 11))))  
   
;; Set a[5] to 22  
(assign array-example  
        (aset1 'array-example  
               (@ array-example)  
               5  
               22))  
   
;; Get a[5]  
(aref1 'array-example (@ array-example) 5)

## [ActionScript](http://rosettacode.org/wiki/Category:ActionScript "Category:ActionScript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=6 "Edit section: ActionScript")]

//creates an array of length 10  
var array1:Array = new Array(10);  
//creates an array with the values 1, 2  
var array2:Array = new Array(1,2);  
//arrays can also be set using array literals  
var array3:Array = ["foo", "bar"];  
//to resize an array, modify the length property  
array2.length = 3;  
//arrays can contain objects of multiple types.  
array2[2] = "Hello";  
//get a value from an array  
trace(array2[2]);  
//append a value to an array  
array2.push(4);  
//get and remove the last element of an array  
trace(array2.pop());

## [Ada](http://rosettacode.org/wiki/Category:Ada "Category:Ada")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=7 "Edit section: Ada")]

procedure Array_Test is  
   
   A, B : array (1..20) of Integer;  
   
   -- Ada array indices may begin at any value, not just 0 or 1  
   C : array (-37..20) of integer  
   
   -- Ada arrays may be indexed by enumerated types, which are   
   -- discrete non-numeric types  
   type Days is (Mon, Tue, Wed, Thu, Fri, Sat, Sun);  
   type Activities is (Work, Fish);  
   type Daily_Activities is array(Days) of Activities;  
   This_Week : Daily_Activities := (Mon..Fri => Work, Others => Fish);  
   
   -- Or any numeric type  
   type Fingers is range 1..4; -- exclude thumb  
   type Fingers_Extended_Type is array(fingers) of Boolean;  
   Fingers_Extended : Fingers_Extended_Type;  
   
   -- Array types may be unconstrained. The variables of the type  
   -- must be constrained  
   type Arr is array (Integer range <>) of Integer;  
   Uninitialized : Arr (1 .. 10);  
   Initialized_1 : Arr (1 .. 20) := (others => 1);  
   Initialized_2 : Arr := (1 .. 30 => 2);  
   Const         : constant Arr := (1 .. 10 => 1, 11 .. 20 => 2, 21 | 22 => 3);  
   Centered      : Arr (-50..50) := (0 => 1, Others => 0);  
   
   Result        : Integer  
begin  
   
   A := (others => 0);     -- Assign whole array  
   B := (1 => 1, 2 => 1, 3 => 2, others => 0);   
                           -- Assign whole array, different values   
   A (1) := -1;            -- Assign individual element  
   A (2..4) := B (1..3);   -- Assign a slice  
   A (3..5) := (2, 4, -1); -- Assign a constant slice  
   A (3..5) := A (4..6);   -- It is OK to overlap slices when assigned  
   
   Fingers_Extended'First := False; -- Set first element of array  
   Fingers_Extended'Last := False;  -- Set last element of array  
   
end Array_Test;

Arrays are first-class objects in  [Ada](http://rosettacode.org/wiki/Ada "Ada"). They can be allocated statically or dynamically as any other object. The number of elements in an array object is always constrained. Variable size arrays are provided by the standard container library. They also can be implemented as user-defined types.

## [Aikido](http://rosettacode.org/wiki/Category:Aikido "Category:Aikido")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=8 "Edit section: Aikido")]

Aikido arrays (or vectors) are dynamic and not fixed in size. They can hold a set of any defined value.

   
var arr1 = [1,2,3,4]   // initialize with array literal  
var arr2 = new [10]   // empty array of 10 elements (each element has value none)  
var arr3 = new int [40]  // array of 40 integers  
var arr4 = new Object (1,2) [10]   // array of 10 instances of Object  
   
arr1.append (5)   // add to array  
var b = 4 in arr1   // check for inclusion  
arr1 <<= 2           // remove first 2 elements from array  
var arrx = arr1[1:3]   // get slice of array  
var s = arr1.size()  // or sizeof(arr1)  
delete arr4[2]     // remove an element from an array  
   
var arr5 = arr1 + arr2   // append arrays  
var arr6 = arr1 | arr2    // union  
var arr7 = arr1 & arr2   // intersection  
   
 

## [Aime](http://rosettacode.org/wiki/Category:Aime "Category:Aime")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=9 "Edit section: Aime")]

The aime  _list_  is a heterogeneous, dynamic sequence. No special creation procedure, only declaration is needed:

list l;

Values (numbers, strings, collections, functions, etc) can be added in a type generic fashion:

l_append(l, 3);  
l_append(l, "arrays");  
l_append(l, pow);

The insertion position can be specified:

l_push(l, 3, .5);  
l_push(l, 4, __type(l));

More aptly, values (of selected types) can be inserted in a type specific fashion:

l_p_integer(l, 5, -1024);  
l_p_real(l, 6, 88);

Similarly, values can be retrieved in a type generic fashion:

l_query(l, 5);

or is type specific fashion:

l_q_real(l, 6);  
l_q_text(l, 1);

## [ALGOL 68](http://rosettacode.org/wiki/Category:ALGOL_68 "Category:ALGOL 68")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=10 "Edit section: ALGOL 68")]

PROC array_test = VOID:  
(  
  [1:20]INT a;  
  a := others;                           # assign whole array #  
  a[1] := -1;                            # assign individual element #  
  a[3:5] := (2, 4, -1);                  # assign a slice #  
  [1:3]INT slice = a[3:5];               # copy a slice #  
   
  REF []INT rslice = a[3:5];             # create a reference to a slice #  
  print((LWB rslice, UPB slice));        # query the bounds of the slice #  
  rslice := (2, 4, -1);                  # assign to the slice, modifying original array #  
   
  [1:3, 1:3]INT matrix;                  # create a two dimensional array #  
  REF []INT hvector = matrix[2,];        # create a reference to a row #  
  REF []INT vvector = matrix[,2];        # create a reference to a column #  
  REF [,]INT block = matrix[1:2, 1:2];   # create a reference to an area of the array #  
   
  FLEX []CHAR string := "Hello, world!"; # create an array with variable bounds #  
  string := "shorter"                    # flexible arrays automatically resize themselves on assignment #  
)

Arrays in ALGOL 68 are first class objects. Slices to any portion of the array can be created and then treated equivalently to arrays, even sections of a multidimensional array; the bounds are queried at run time. References may be made to portions of an array. Flexible arrays are supported, which resize themselves on assignment, but they can't be resized without destroying the data.

## [ALGOL W](http://rosettacode.org/wiki/Category:ALGOL_W "Category:ALGOL W")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=11 "Edit section: ALGOL W")]

begin  
    % declare an array %  
    integer array a ( 1 :: 10 );  
    % set the values %  
    for i := 1 until 10 do a( i ) := i;  
    % change the 3rd element %  
    a( 3 ) := 27;  
    % display the 4th element %  
    write( a( 4 ) ); % would show 4 %  
    % arrays with sizes not known at compile-time must be created in inner-blocks or procedures %  
    begin  
        integer array b ( a( 3 ) - 2 :: a( 3 ) ); % b has bounds 25 :: 27 %  
        for i := a( 3 ) - 2 until a( 3 ) do b( i ) := i  
    end  
    % arrays cannot be part of records and cannot be returned by procecures though they can be passed %  
    % as parameters to procedures                                                                     %  
    % multi-dimension arrays are supported                                                            %  
end.

## [AmigaE](http://rosettacode.org/wiki/Category:AmigaE "Category:AmigaE")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=12 "Edit section: AmigaE")]

DEF ai[100] : ARRAY OF CHAR, -> static  
    da: PTR TO CHAR,  
    la: PTR TO CHAR  
   
PROC main()  
  da := New(100)  
  -> or  
  NEW la[100]  
  IF da <> NIL  
    ai[0] := da[0]    -> first is 0  
    ai[99] := da[99]  -> last is "size"-1  
    Dispose(da)  
  ENDIF  
  -> using NEW, we must specify the size even when  
  -> "deallocating" the array  
  IF la <> NIL THEN END la[100]  
ENDPROC

## [AntLang](http://rosettacode.org/wiki/Category:AntLang "Category:AntLang")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=13 "Edit section: AntLang")]

/ Create an immutable sequence (array)  
arr: <1;2;3>  
   
/ Get the head an tail part  
h: head[arr]  
t: tail[arr]  
   
/ Get everything except the last element and the last element  
nl: first[arr]  
l: last[arr]  
   
/ Get the nth element (index origin = 0)  
nth:arr[n]

## [APL](http://rosettacode.org/wiki/Category:APL "Category:APL")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=14 "Edit section: APL")]

Arrays in APL are one dimensional matrices, defined by seperating variables with spaces. For example:

+/ 1 2 3

Is equivalent to

1 + 2 + 3

We're folding function

+

over the array

1 2 3

## [App Inventor](http://rosettacode.org/wiki/Category:App_Inventor "Category:App Inventor")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=15 "Edit section: App Inventor")]

Arrays in App Inventor are represented with Lists. Lists may be nested to any level and contain other Lists. All supported data types may be stored in a List.  [Basic List blocks](https://lh4.googleusercontent.com/-5y13nsUEj1U/UunGAhuqWEI/AAAAAAAAJ7U/i2IL5v6EQ5I/w631-h658-no/Capture.PNG)

## [Apex](http://rosettacode.org/wiki/Category:Apex "Category:Apex")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=16 "Edit section: Apex")]

Integer[] array = new Integer[10]; // optionally, append a braced list of Integers like "{1, 2, 3}"  
array[0] = 42;  
System.debug(array[0]); // Prints 42

Dynamic arrays can be made using  `List`s.  `List`s and array can be used interchangeably in Apex, e.g. any method that accepts a  `List<String>`  will also accept a  `String[]`

List <Integer> aList = new List <Integer>(); // optionally add an initial size as an argument  
aList.add(5);// appends to the end of the list  
aList.add(1, 6);// assigns the element at index 1  
System.debug(list[0]); // Prints 5, alternatively you can use list.get(0)

## [AppleScript](http://rosettacode.org/wiki/Category:AppleScript "Category:AppleScript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=17 "Edit section: AppleScript")]

AppleScript arrays are called lists:

 set empty to {}  
 set ints to {1, 2, 3}

Lists can contain any objects including other lists:

 set any to {1, "foo", 2.57, missing value, ints}

## [Arendelle](http://rosettacode.org/wiki/Category:Arendelle "Category:Arendelle")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=18 "Edit section: Arendelle")]

// Creating an array as [ 23, 12, 2, 5345, 23 ] 
// with name "space"

   ( space , 23; 12; 2; 5345; 23 )

// Getting the size of an array:

   "Size of array is | @space? |"

// Appending array with 54

   ( space[ @space? ] , 54 )

// Something else fun about arrays in Arendelle
// for example when you have one like this:
//
//    space -> [ 23, 34, 3, 6345 ]
//
// If you do this on the space:

   ( space[ 7 ] , 10 )

// Arendelle will make the size of array into
// 8 by appending zeros and then it will set
// index 7 to 10 and result will be:
//
//    space -> [ 23, 34, 3, 6345, 0, 0, 0, 10 ]

// To remove the array you can use done keyword:

   ( space  , done )

## [Argile](http://rosettacode.org/wiki/Category:Argile "Category:Argile")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=19 "Edit section: Argile")]

**Works with**:  [Argile](http://rosettacode.org/wiki/Argile "Argile")  version 1.0.0

use std, array  
   
(:::::::::::::::::  
 : Static arrays :  
 :::::::::::::::::)  
let the array of 2 text aabbArray be Cdata{"aa";"bb"}  
let raw array of real :my array: = Cdata {1.0 ; 2.0 ; 3.0} (: auto sized :)  
let another_array be an array of 256 byte (: not initialised :)  
let (raw array of (array of 3 real)) foobar = Cdata {  
  {1.0; 2.0; 0.0}  
  {5.0; 1.0; 3.0}  
}  
   
(: macro to get size of static arrays :)  
=: <array>.length := -> nat {size of array / (size of array[0])}  
printf "%lu, %lu\n" foobar.length (another_array.length) (: 2, 256 :)  
   
(: access :)  
another_array[255] = '&'  
printf "`%c'\n" another_array[255]  
   
   
(::::::::::::::::::  
 : Dynamic arrays :  
 ::::::::::::::::::)  
let DynArray = new array of 5 int  
DynArray[0] = -42  
DynArray = (realloc DynArray (6 * size of DynArray[0])) as (type of DynArray)  
DynArray[5] = 243  
prints DynArray[0] DynArray[5]  
del DynArray

**Works with**:  [Argile](http://rosettacode.org/wiki/Argile "Argile")  version 1.1.0

use std, array  
let x = @["foo" "bar" "123"]  
print x[2]  
x[2] = "abc"

## [ARM Assembly](http://rosettacode.org/wiki/Category:ARM_Assembly "Category:ARM Assembly")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=20 "Edit section: ARM Assembly")]

**Works with**:  [as](http://rosettacode.org/mw/index.php?title=As&action=edit&redlink=1 "As (page does not exist)")  version Raspberry Pi

   
/* ARM assembly Raspberry PI  */  
/*  program areaString.s   */  
   
/* Constantes    */  
.equ STDOUT, 1     @ Linux output console  
.equ EXIT,   1     @ Linux syscall  
.equ WRITE,  4     @ Linux syscall  
/* Initialized data */  
.data  
szMessStringsch: .ascii "The string is at item : "  
sZoneconv:		 .fill 12,1,' '  
szCarriageReturn:  .asciz "\n"  
szMessStringNfound: .asciz "The string is not found in this area.\n"  
   
/* areas strings  */  
szString1:  .asciz "Apples"  
szString2:  .asciz "Oranges"  
szString3:  .asciz "Pommes"  
szString4:  .asciz "Raisins"  
szString5:  .asciz "Abricots"  
   
/* pointer items area 1*/  
tablesPoi1:  
pt1_1:	    .int szString1  
pt1_2:	    .int szString2  
pt1_3:	    .int szString3  
pt1_4:	    .int szString4  
ptVoid_1: .int 0  
ptVoid_2: .int 0  
ptVoid_3: .int 0  
ptVoid_4: .int 0  
ptVoid_5: .int 0  
   
szStringSch:	.asciz "Raisins"  
szStringSch1:	.asciz "Ananas"  
   
/* UnInitialized data */  
.bss   
   
/*  code section */  
.text  
.global main   
main:                /* entry of program  */  
    push {fp,lr}    /* saves 2 registers */  
   
    @@@@@@@@@@@@@@@@@@@@@@@@  
    @ add string 5 to area  
   @@@@@@@@@@@@@@@@@@@@@@@@  
    ldr r1,iAdrtablesPoi1  @ begin pointer area 1  
    mov r0,#0    @ counter  
1:   @ search first void pointer  
    ldr r2,[r1,r0,lsl #2]    @ read string pointer address item r0 (4 bytes by pointer)  
    cmp r2,#0                @ is null ?  
    addne r0,#1             @ no increment counter  
    bne 1b                  @ and loop  
   
    @ store pointer string 5 in area  at position r0  
    ldr r2,iAdrszString5  @ address string 5  
    str r2,[r1,r0,lsl #2]    @ store address   
   
    @@@@@@@@@@@@@@@@@@@@@@@@  
    @ display string at item 3  
    @@@@@@@@@@@@@@@@@@@@@@@@  
    mov r2,#2        @ pointers begin in position 0   
    ldr r1,iAdrtablesPoi1  @ begin pointer area 1  
    ldr r0,[r1,r2,lsl #2]  
    bl affichageMess  
    ldr r0,iAdrszCarriageReturn  
    bl affichageMess  
   
    @@@@@@@@@@@@@@@@@@@@@@@@  
    @ search string in area   
    @@@@@@@@@@@@@@@@@@@@@@@@  
    ldr r1,iAdrszStringSch  
    //ldr r1,iAdrszStringSch1  @ uncomment for other search : not found !!  
    ldr r2,iAdrtablesPoi1  @ begin pointer area 1  
    mov r3,#0    
2:   @ search   
    ldr r0,[r2,r3,lsl #2]    @ read string pointer address item r0 (4 bytes by pointer)  
    cmp r0,#0                @ is null ?  
    beq 3f        @ end search  
    bl comparaison  
    cmp r0,#0                @ string = ?  
    addne r3,#1             @ no increment counter  
    bne 2b                  @ and loop  
    mov r0,r3             @ position item string  
    ldr r1,iAdrsZoneconv   @ conversion decimal  
    bl conversion10S  
    ldr r0,iAdrszMessStringsch  
    bl affichageMess  
    b 100f  
3:   @ end search  string not found  
    ldr r0,iAdrszMessStringNfound  
    bl affichageMess  
   
100:   /* standard end of the program */  
    mov r0, #0                  @ return code  
    pop {fp,lr}                 @restaur 2 registers  
    mov r7, #EXIT              @ request to exit program  
    swi 0                       @ perform the system call  
iAdrtablesPoi1:		.int tablesPoi1  
iAdrszMessStringsch:   .int szMessStringsch  
iAdrszString5:		.int szString5  
iAdrszStringSch:	.int szStringSch  
iAdrszStringSch1:   .int szStringSch1  
iAdrsZoneconv:       .int sZoneconv  
iAdrszMessStringNfound:  .int szMessStringNfound  
iAdrszCarriageReturn:  .int  szCarriageReturn  
/******************************************************************/  
/*     display text with size calculation                         */   
/******************************************************************/  
/* r0 contains the address of the message */  
affichageMess:  
    push {fp,lr}    			/* save  registres */   
    push {r0,r1,r2,r7}    		/* save others registers */  
    mov r2,#0   				/* counter length */  
1:      	/* loop length calculation */  
    ldrb r1,[r0,r2]  			/* read octet start position + index */  
    cmp r1,#0       			/* if 0 its over */  
    addne r2,r2,#1   			/* else add 1 in the length */  
    bne 1b          			/* and loop */  
                                /* so here r2 contains the length of the message */  
    mov r1,r0        			/* address message in r1 */  
    mov r0,#STDOUT      		/* code to write to the standard output Linux */  
    mov r7, #WRITE             /* code call system "write" */  
    swi #0                      /* call systeme */  
    pop {r0,r1,r2,r7}     		/* restaur others registers */  
    pop {fp,lr}    				/* restaur des  2 registres */   
    bx lr	        			/* return  */  
/***************************************************/  
/*   conversion register signed décimal     */  
/***************************************************/  
/* r0 contient le registre   */  
/* r1 contient l adresse de la zone de conversion */  
conversion10S:  
    push {r0-r5,lr}    /* save des registres */  
    mov r2,r1       /* debut zone stockage */  
    mov r5,#'+'     /* par defaut le signe est + */  
    cmp r0,#0       /* nombre négatif ? */  
    movlt r5,#'-'     /* oui le signe est - */  
    mvnlt r0,r0       /* et inversion en valeur positive */  
    addlt r0,#1  
    mov r4,#10   /* longueur de la zone */  
1: /* debut de boucle de conversion */  
    bl divisionpar10 /* division  */  
    add r1,#48        /* ajout de 48 au reste pour conversion ascii */	  
    strb r1,[r2,r4]  /* stockage du byte en début de zone r5 + la position r4 */  
    sub r4,r4,#1      /* position précedente */  
    cmp r0,#0       
    bne 1b	       /* boucle si quotient different de zéro */  
    strb r5,[r2,r4]  /* stockage du signe à la position courante */  
    subs r4,r4,#1   /* position précedente */  
    blt  100f         /* si r4 < 0  fin  */  
    /* sinon il faut completer le debut de la zone avec des blancs */  
    mov r3,#' '   /* caractere espace */	  
2:  
    strb r3,[r2,r4]  /* stockage du byte  */  
    subs r4,r4,#1   /* position précedente */  
    bge 2b        /* boucle si r4 plus grand ou egal a zero */  
100:  /* fin standard de la fonction  */  
    pop {r0-r5,lr}   /*restaur desregistres */  
    bx lr     
   
/***************************************************/  
/*   division par 10   signé                       */  
/* Thanks to http://thinkingeek.com/arm-assembler-raspberry-pi/*    
/* and   http://www.hackersdelight.org/            */  
/***************************************************/  
/* r0 contient le dividende   */  
/* r0 retourne le quotient */	  
/* r1 retourne le reste  */  
divisionpar10:	  
  /* r0 contains the argument to be divided by 10 */  
   push {r2-r4}   /* save registers  */  
   mov r4,r0   
   ldr r3, .Ls_magic_number_10 /* r1 <- magic_number */  
   smull r1, r2, r3, r0   /* r1 <- Lower32Bits(r1*r0). r2 <- Upper32Bits(r1*r0) */  
   mov r2, r2, ASR #2     /* r2 <- r2 >> 2 */  
   mov r1, r0, LSR #31    /* r1 <- r0 >> 31 */  
   add r0, r2, r1         /* r0 <- r2 + r1 */  
   add r2,r0,r0, lsl #2   /* r2 <- r0 * 5 */  
   sub r1,r4,r2, lsl #1   /* r1 <- r4 - (r2 * 2)  = r4 - (r0 * 10) */  
   pop {r2-r4}  
   bx lr                  /* leave function */  
   bx lr                  /* leave function */  
.Ls_magic_number_10: .word 0x66666667  
   
 

## [Arturo](http://rosettacode.org/wiki/Category:Arturo "Category:Arturo")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=21 "Edit section: Arturo")]

// empty  array  
arrA #()  
   
// array with initial values  
arrB #("one" "two" "three")  
   
// adding an element to an existing array  
arrB arrB + "four"  
print arrB  
   
// another way to add an element  
add arrB "five"  
print arrB  
   
// retrieve an element at some index  
print arrB.1

Output:

#("one" "two" "three" "four")
#("one" "two" "three" "four" "five")
two

## [AutoHotkey](http://rosettacode.org/wiki/Category:AutoHotkey "Category:AutoHotkey")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=22 "Edit section: AutoHotkey")]

**Works with**:  [AutoHotkey_L](http://rosettacode.org/wiki/AutoHotkey_L "AutoHotkey L")

The current, official build of AutoHotkey is called AutoHotkey_L. In it, arrays are called Objects, and associative/index based work hand-in-hand.

myArray := Object() ; could use JSON-syntax sugar like {key: value}  
myArray[1] := "foo"  
myArray[2] := "bar"  
[MsgBox](http://www.autohotkey.com/docs/commands/MsgBox.htm) % myArray[2]  
   
; Push a value onto the array  
myArray.Insert("baz")

AutoHotkey Basic (deprecated) did not have typical arrays. However, variable names could be concatenated, simulating associative arrays. By convention, based on built-in function stringsplit, indexes are 1-based and "0" index is the length.

arrayX0 = 4      ; length  
arrayX1 = first  
arrayX2 = second  
arrayX3 = foo  
arrayX4 = bar  
[Loop](http://www.autohotkey.com/docs/commands/Loop.htm), %arrayX0%  
  [Msgbox](http://www.autohotkey.com/docs/commands/MsgBox.htm) % arrayX%A_Index%  
source = apple bear cat dog egg fish  
[StringSplit](http://www.autohotkey.com/docs/commands/StringSplit.htm) arrayX, source, %A_Space%    
[Loop](http://www.autohotkey.com/docs/commands/Loop.htm), %arrayX0%  
  [Msgbox](http://www.autohotkey.com/docs/commands/MsgBox.htm) % arrayX%A_Index%

## [AutoIt](http://rosettacode.org/wiki/Category:AutoIt "Category:AutoIt")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=23 "Edit section: AutoIt")]

Create an userdefined array.

   
#include <Array.au3> ;Include extended Array functions (_ArrayDisplay)  
   
[Local](https://www.autoitscript.com/autoit3/docs/keywords.htm) $aInputs[1] ;Create the Array with just 1 element  
   
[While](https://www.autoitscript.com/autoit3/docs/keywords.htm) [True](https://www.autoitscript.com/autoit3/docs/keywords.htm) ;Endless loop  
	$aInputs[[UBound](https://www.autoitscript.com/autoit3/docs/functions/UBound.htm)($aInputs) - 1] = [InputBox](https://www.autoitscript.com/autoit3/docs/functions/InputBox.htm)("Array", "Add one value") ;Save user input to the last element of the Array  
	[If](https://www.autoitscript.com/autoit3/docs/keywords.htm) $aInputs[[UBound](https://www.autoitscript.com/autoit3/docs/functions/UBound.htm)($aInputs) - 1] = "" [Then](https://www.autoitscript.com/autoit3/docs/keywords.htm) ;If an empty string is entered, then...  
		[ReDim](https://www.autoitscript.com/autoit3/docs/keywords.htm) $aInputs[[UBound](https://www.autoitscript.com/autoit3/docs/functions/UBound.htm)($aInputs) - 1] ;...remove them from the Array and...  
		[ExitLoop](https://www.autoitscript.com/autoit3/docs/keywords.htm) ;... exit the loop!  
	[EndIf](https://www.autoitscript.com/autoit3/docs/keywords.htm)  
	[ReDim](https://www.autoitscript.com/autoit3/docs/keywords.htm) $aInputs[[UBound](https://www.autoitscript.com/autoit3/docs/functions/UBound.htm)($aInputs) + 1] ;Add an empty element to the Array  
[WEnd](https://www.autoitscript.com/autoit3/docs/keywords.htm)  
   
_ArrayDisplay($aInputs) ;Display the Array  
 

## [AWK](http://rosettacode.org/wiki/Category:AWK "Category:AWK")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=24 "Edit section: AWK")]

Every array in AWK is an associative array. AWK converts each array subscript to a string, so a[33], a["33"] and a[29 + 4] are the same element.

An ordered array just uses subscripts as integers. Array subscripts can start at 1, or any other integer. The built-in split() function makes arrays that start at 1.

BEGIN {  
  # to make an array, assign elements to it  
  array[1] = "first"  
  array[2] = "second"  
  array[3] = "third"  
  alen = 3  # want the length? store in separate variable  
   
  # or split a string  
  plen = split("2 3 5 7 11 13 17 19 23 29", primes)  
  clen = split("Ottawa;Washington DC;Mexico City", cities, ";")  
   
  # retrieve an element  
  print "The 6th prime number is " primes[6]  
   
  # push an element  
  cities[clen += 1] = "New York"  
   
  dump("An array", array, alen)  
  dump("Some primes", primes, plen)  
  dump("A list of cities", cities, clen)  
}  
   
function dump(what, array, len,    i) {  
  print what;  
   
  # iterate an array in order  
  for (i = 1; i <= len; i++) {  
    print "  " i ": " array[i]  
  }  
}

Output:

The 6th prime number is 13
An array
  1: first
  2: second
  3: third
Some primes
  1: 2
  2: 3
  3: 5
  4: 7
  5: 11
  6: 13
  7: 17
  8: 19
  9: 23
  10: 29
A list of cities
  1: Ottawa
  2: Washington DC
  3: Mexico City
  4: New York

## [Axe](http://rosettacode.org/wiki/Category:Axe "Category:Axe")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=25 "Edit section: Axe")]

1→{L₁}  
2→{L₁+1}  
3→{L₁+2}  
4→{L₁+3}  
Disp {L₁}►Dec,i  
Disp {L₁+1}►Dec,i  
Disp {L₁+2}►Dec,i  
Disp {L₁+3}►Dec,i

## [Babel](http://rosettacode.org/wiki/Category:Babel "Category:Babel")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=26 "Edit section: Babel")]

### Create an array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=27 "Edit section: Create an array")]

There are two kinds of array in Babel: value-arrays and pointer-arrays. A value-array is a flat array of data words. A pointer-array is an array of pointers to other things (including value-arrays). You can create a data-array with plain square-brackets. You can create a value-array with the [ptr ] list form:

[1 2 3]

[ptr 1 2 3]

  

### Get a single array element[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=28 "Edit section: Get a single array element")]

[1 2 3] 1 th ;

Output:

[val 0x2 ]

### Change an array element[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=29 "Edit section: Change an array element")]

Changing a value-array element:

[1 2 3] dup 1 7 set ;  
 

Output:

[val 0x1 0x7 0x3 ]

Changing a pointer-array element:

[ptr 1 2 3] dup 1 [ptr 7] set ;

Output:

[ptr [val 0x1 ] [val 0x7 ] [val 0x3 ] ]

### Select a range of an array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=30 "Edit section: Select a range of an array")]

[ptr 1 2 3 4 5 6] 1 3 slice ;

Output:

[ptr [val 0x2 ] [val 0x3 ] ]

### Add a new element to an array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=31 "Edit section: Add a new element to an array")]

You can concatenate arrays of same type:

[1 2 3] [4] cat

[ptr 1 2 3] [ptr 4] cat

Concatenation creates a new array - it does not add to an array in-place. Instead, Babel provides operators and standard utilities for converting an array to a list in order to manipulate it, and then convert back.

### Convert between arrays and lists[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=32 "Edit section: Convert between arrays and lists")]

Convert a value-array to a list of values:

[1 2 3] ar2ls lsnum !

Output:

( 1 2 3 )

Convert a list of values to a value-array:

(1 2 3) ls2lf ;

Output:

[val 0x1 0x2 0x3 ]

Convert a pointer-array to a list of pointers:

[ptr 'foo' 'bar' 'baz'] ar2ls lsstr !

Output:

( "foo" "bar" "baz" )

Convert a list of pointers to a pointer-array:

(1 2 3) bons ;

Output:

[ptr [val 0x1 ] [val 0x2 ] [val 0x3 ] ]

To learn more about manipulating arrays and lists in Babel, type "help !" (no quotes) and follow the instructions to load the man.sp file.

## [BASIC](http://rosettacode.org/wiki/Category:BASIC "Category:BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=33 "Edit section: BASIC")]

**Works with**:  [QuickBasic](http://rosettacode.org/wiki/QuickBasic "QuickBasic")  version 4.5

**Works with**:  [PB](http://rosettacode.org/wiki/PB "PB")  version 7.1

The default array base (lower bound) can be set with OPTION BASE. If OPTION BASE is not set, the base may be either 0 or 1, depending on implementation. The value given in DIM statement is the upper bound. If the base is 0, then DIM a(100) will create an array containing 101 elements.

 [OPTION](http://www.qbasicnews.com/qboho/qckoption.shtml) [BASE](http://www.qbasicnews.com/qboho/qckbase.shtml) 1  
 [DIM](http://www.qbasicnews.com/qboho/qckdim.shtml) myArray(100) [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [INTEGER](http://www.qbasicnews.com/qboho/qckinteger.shtml) 

Alternatively, the lower and upper bounds can be given while defining the array:

 [DIM](http://www.qbasicnews.com/qboho/qckdim.shtml) myArray(-10 TO 10) [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [INTEGER](http://www.qbasicnews.com/qboho/qckinteger.shtml) 

Dynamic arrays:

 'Specify that the array is dynamic and not static:  
 '$DYNAMIC  
 [DIM](http://www.qbasicnews.com/qboho/qckdim.shtml) [SHARED](http://www.qbasicnews.com/qboho/qckshared.shtml) myArray(-10 TO 10, 10 TO 30) [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [STRING](http://www.qbasicnews.com/qboho/qckstring.shtml)  
 [REDIM](http://www.qbasicnews.com/qboho/qckredim.shtml) [SHARED](http://www.qbasicnews.com/qboho/qckshared.shtml) myArray(20, 20) [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [STRING](http://www.qbasicnews.com/qboho/qckstring.shtml)  
 myArray(1,1) = "Item1"  
 myArray(1,2) = "Item2" 

**Array Initialization**

Arrays are initialized to zero or zero length strings when created. BASIC does not generally have option for initializing arrays to other values, so the initializing is usually done at run time. DATA and READ statements are often used for this purpose:

 [DIM](http://www.qbasicnews.com/qboho/qckdim.shtml) month$(12)  
 [DATA](http://www.qbasicnews.com/qboho/qckdata.shtml) January, February, March, April, May, June, July  
 [DATA](http://www.qbasicnews.com/qboho/qckdata.shtml) August, September, October, November, December  
 FOR m=1 TO 12  
    [READ](http://www.qbasicnews.com/qboho/qckread.shtml) month$(m)  
 NEXT m 

**Works with**:  [FreeBASIC](http://rosettacode.org/wiki/FreeBASIC "FreeBASIC")

FreeBASIC has an option to initialize array while declaring it.

 Dim myArray(1 To 2, 1 To 5) As Integer => {{1, 2, 3, 4, 5}, {1, 2, 3, 4, 5}} 

10 REM TRANSLATION OF QBASIC STATIC VERSION  
20 REM ELEMENT NUMBERS TRADITIONALLY START AT ONE  
30 DIM A%(11): REM ARRAY OF ELEVEN INTEGER ELEMENTS  
40 LET A%(1) = -1  
50 LET A%(11) = 1  
60 PRINT A%(1), A%(11)  
70 END

**Works with**:  [qbasic](http://rosettacode.org/wiki/Qbasic "Qbasic")

### Static[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=34 "Edit section: Static")]

[DIM](http://www.qbasicnews.com/qboho/qckdim.shtml) staticArray(10) [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [INTEGER](http://www.qbasicnews.com/qboho/qckinteger.shtml)  
   
staticArray(0) = -1  
staticArray(10) = 1  
   
[PRINT](http://www.qbasicnews.com/qboho/qckprint.shtml) staticArray(0), staticArray(10)

### Dynamic[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=35 "Edit section: Dynamic")]

Note that BASIC dynamic arrays are not stack-based; instead, their size must be changed in the same manner as their initial declaration -- the only difference between static and dynamic arrays is the keyword used to declare them (`DIM`  vs.  `REDIM`).  [QBasic](http://rosettacode.org/wiki/QBasic "QBasic")  lacks the  `PRESERVE`  keyword found in some modern BASICs; resizing an array without  `PRESERVE`  zeros the values.

[REDIM](http://www.qbasicnews.com/qboho/qckredim.shtml) dynamicArray(10) [AS](http://www.qbasicnews.com/qboho/qckas.shtml) [INTEGER](http://www.qbasicnews.com/qboho/qckinteger.shtml)  
   
dynamicArray(0) = -1  
[PRINT](http://www.qbasicnews.com/qboho/qckprint.shtml) dynamicArray(0)  
   
[REDIM](http://www.qbasicnews.com/qboho/qckredim.shtml) dynamicArray(20)  
   
dynamicArray(20) = 1  
[PRINT](http://www.qbasicnews.com/qboho/qckprint.shtml) dynamicArray(0), dynamicArray(20)

### [Applesoft BASIC](http://rosettacode.org/wiki/Category:Applesoft_BASIC "Category:Applesoft BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=36 "Edit section: Applesoft BASIC")]

10 DIM A%(11): REM ARRAY OF TWELVE INTEGER ELEMENTS  
20 LET A%(0) = -1  
30 LET A%(11) = 1  
40 PRINT A%(0), A%(11)

### [Commodore BASIC](http://rosettacode.org/wiki/Category:Commodore_BASIC "Category:Commodore BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=37 "Edit section: Commodore BASIC")]

same as Applesoft BASIC

## [BASIC256](http://rosettacode.org/wiki/Category:BASIC256 "Category:BASIC256")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=38 "Edit section: BASIC256")]

# numeric array  
dim numbers(10)  
for t = 0 to 9  
  numbers[t] = t + 1  
next t  
   
# string array  
dim words$(10)  
# assigning an array with a list  
words$ = {"one","two","three","four","five","six","seven","eight","nine","ten"}  
   
gosub display  
   
# resize arrays (always preserves values if larger)  
redim numbers(11)  
redim words$(11)  
numbers[10] = 11  
words$[10] = "eleven"  
gosub display  
   
end  
   
display:  
# display arrays  
# using ? to get size of array  
for t = 0 to numbers[?]-1  
  print numbers[t] + "=" + words$[t]  
next t  
return

## [Batch File](http://rosettacode.org/wiki/Category:Batch_File "Category:Batch File")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=39 "Edit section: Batch File")]

Arrays can be approximated, in a style similar to REXX

::arrays.cmd  
@[echo](https://www.ss64.com/nt/echo.html) off  
[setlocal](https://www.ss64.com/nt/setlocal.html) ENABLEDELAYEDEXPANSION  
[set](https://www.ss64.com/nt/set.html) array.1=1  
[set](https://www.ss64.com/nt/set.html) array.2=2  
[set](https://www.ss64.com/nt/set.html) array.3=3  
[set](https://www.ss64.com/nt/set.html) array.4=4  
[for](https://www.ss64.com/nt/for.html) /L %%i [in](https://www.ss64.com/nt/in.html) (1,1,4) [do](https://www.ss64.com/nt/do.html) [call](https://www.ss64.com/nt/call.html) :showit array.%%i !array.%%i!  
[set](https://www.ss64.com/nt/set.html) c=-27  
[call](https://www.ss64.com/nt/call.html) :mkarray marry 5 6 7 8  
[for](https://www.ss64.com/nt/for.html) /L %%i [in](https://www.ss64.com/nt/in.html) (-27,1,-24) [do](https://www.ss64.com/nt/do.html) [call](https://www.ss64.com/nt/call.html) :showit "marry^&%%i" !marry^&%%i!  
[endlocal](https://www.ss64.com/nt/endlocal.html)  
[goto](https://www.ss64.com/nt/goto.html) :eof  
   
:mkarray  
[set](https://www.ss64.com/nt/set.html) %1^&%c%=%2  
[set](https://www.ss64.com/nt/set.html) /a c += 1  
[shift](https://www.ss64.com/nt/shift.html) /2  
[if](https://www.ss64.com/nt/if.html) "%2" [neq](https://www.ss64.com/nt/neq.html) "" [goto](https://www.ss64.com/nt/goto.html) :mkarray  
[goto](https://www.ss64.com/nt/goto.html) :eof  
   
:showit  
[echo](https://www.ss64.com/nt/echo.html) %1 = %2  
[goto](https://www.ss64.com/nt/goto.html) :eof  
 

Output:

array.1 = 1
array.2 = 2
array.3 = 3
array.4 = 4
"marry&-27" = 5
"marry&-26" = 6
"marry&-25" = 7
"marry&-24" = 8

## [BBC BASIC](http://rosettacode.org/wiki/Category:BBC_BASIC "Category:BBC BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=40 "Edit section: BBC BASIC")]

      REM Declare arrays, dimension is maximum index:  
      DIM array(6), array%(6), array$(6)  
   
      REM Entire arrays may be assigned in one statement:  
      array() = 0.1, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7  
      array%() = 0, 1, 2, 3, 4, 5, 6  
      array$() = "Zero", "One", "Two", "Three", "Four", "Five", "Six"  
   
      REM Or individual elements may be assigned:  
      array(2) = PI  
      array%(3) = RND  
      array$(4) = "Hello world!"  
   
      REM Print out sample array elements:  
      PRINT array(2)  TAB(16) array(3)  TAB(32) array(4)  
      PRINT array%(2) TAB(16) array%(3) TAB(32) array%(4)  
      PRINT array$(2) TAB(16) array$(3) TAB(32) array$(4)

## [bc](http://rosettacode.org/wiki/Category:Bc "Category:Bc")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=41 "Edit section: bc")]

There are 26 arrays available (named 'a' to 'z') with all elements initialized to zero and an installation-specific maximum size (in  [GNU bc](http://rosettacode.org/wiki/GNU_bc "GNU bc")  you can find out the limits of your installation (`BC_DIM_MAX`) by invoking the  `limits`  command). Array identifiers are always followed by square brackets ('[', ']') and need not be declared/defined before usage. Indexing starts at zero.

The following is a transcript of an interactive session:

/* Put the value 42 into array g at index 3 */  
g[3] = 42  
/* Look at some other elements in g */  
g[2]  
0  
g[4342]  
0  
/* Look at the elements of another array */  
a[543]  
0  
/* Array names don't conflict with names of ordinary (scalar) identifiers */  
g  
0  
g = 123  
g  
123  
g[3]  
42

## [BML](http://rosettacode.org/wiki/Category:BML "Category:BML")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=42 "Edit section: BML")]

**Note:**  Variables in BML can either be placed in a prefix group($, @, and &) or in the world. Placing variables in the world is not recommended since it can take large sums of memory when using said variable.

   
% Define an array(containing the numbers 1-3) named arr in the group $  
in $ let arr hold 1 2 3  
   
% Replace the value at index 0 in array to "Index 0"  
set $arr index 0 to "Index 0"  
   
% Will display "Index 0"  
display $arr index 0  
   
% There is no automatic garbage collection  
delete $arr  
 

## [Bracmat](http://rosettacode.org/wiki/Category:Bracmat "Category:Bracmat")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=43 "Edit section: Bracmat")]

In Bracmat, an array is not a variable, but a stack of variables. In fact, local variables in functions are elements in arrays. Global variables are the zeroth element in such arrays. You can explicitly create an array of a specific size using the  `tbl`  function. Indexing is done by using the syntax  `_integer_$_name_`. Indexing is modulo the size of the array. A negative integer counts from the end of the array and backwards. The last used index is remembered by the array. Arrays can grow and shrink by calling  `tbl`  with other values. When shrinking, the values of the upper elements are lost. When growing, the current values are kept and the new elements are initialised with  `0`. To delete and array (and therefore the variable with the array's name), call  `tbl`  with a size  `0`.

( tbl$(mytable,100)  
& 5:?(30$mytable)  
& 9:?(31$mytable)  
& out$(!(30$mytable))  
& out$(!(-169$mytable))      { -169 mod 100 == 31 }  
& out$!mytable               { still index 31 }  
& tbl$(mytable,0)  
& (!mytable & out$"mytable still exists"  
  | out$"mytable is gone"  
  )  
);

Output:

5
9
9
mytable is gone

## [Boo](http://rosettacode.org/wiki/Category:Boo "Category:Boo")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=44 "Edit section: Boo")]

   
myArray as (int) = (1, 2, 3) // Size based on initialization  
fixedArray as (int) = array(int, 1) // Given size(1 in this case)  
   
myArray[0] = 10  
   
myArray = myArray + fixedArray // Append arrays  
   
print myArray[0]  
 

## [Brainf***](http://rosettacode.org/wiki/Category:Brainf*** "Category:Brainf***")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=45 "Edit section: Brainf***")]

Note that Brainf*** does not natively support arrays, this example creates something that's pretty close, with access of elements at each index, altering elements, and changing size of list at runtime.

   
===========[  
ARRAY DATA STRUCTURE  
   
AUTHOR: Keith Stellyes  
WRITTEN: June 2016  
   
This is a zero-based indexing array data structure, it assumes the following   
precondition:  
   
>INDEX<|NULL|VALUE|NULL|VALUE|NULL|VALUE|NULL  
   
(Where >< mark pointer position, and | separates addresses)  
   
It relies heavily on [>] and [<] both of which are idioms for  
finding the next left/right null  
   
HOW INDEXING WORKS:  
It runs a loop _index_ number of times, setting that many nulls  
to a positive, so it can be skipped by the mentioned idioms.   
Basically, it places that many "milestones".  
   
EXAMPLE:  
If we seek index 2, and our array is {1 , 2 , 3 , 4 , 5}  
   
FINDING INDEX 2:  
 (loop to find next null, set to positive, as a milestone  
 decrement index)  
   
index  
 2  |0|1|0|2|0|3|0|4|0|5|0  
 1  |0|1|1|2|0|3|0|4|0|5|0  
 0  |0|1|1|2|1|3|0|4|0|5|0  
   
===========]  
   
=======UNIT TEST=======  
 SET ARRAY {48 49 50}  
>>++++++++++++++++++++++++++++++++++++++++++++++++>>  
+++++++++++++++++++++++++++++++++++++++++++++++++>>  
++++++++++++++++++++++++++++++++++++++++++++++++++  
<<<<<<++ Move back to index and set it to 2  
=======================  
   
===RETRIEVE ELEMENT AT INDEX===  
   
=ACCESS INDEX=  
[>>[>]+[<]<-] loop that sets a null to a positive for each iteration  
 First it moves the pointer from index to first value  
 Then it uses a simple loop that finds the next null  
 it sets the null to a positive (1 in this case)  
 Then it uses that same loop reversed to find the first  
 null which will always be one right of our index  
 so we decrement our index  
 Finally we decrement pointer from the null byte to our  
 index and decrement it  
   
>> Move pointer to the first value otherwise we can't loop  
   
[>]< This will find the next right null which will always be right  
 of the desired value; then go one left  
   
   
. Output the value (In the unit test this print "2"  
   
[<[-]<] Reset array  
   
===ASSIGN VALUE AT INDEX===  
   
STILL NEED TO ADJUST UNIT TESTS  
   
NEWVALUE|>INDEX<|NULL|VALUE etc  
   
[>>[>]+[<]<-] Like above logic except it empties the value and doesn't reset  
>>[>]<[-]   
   
[<]< Move pointer to desired value note that where the index was stored  
 is null because of the above loop  
   
[->>[>]+[<]<] If NEWVALUE is GREATER than 0 then decrement it & then find the   
 newly emptied cell and increment it  
   
[>>[>]<+[<]<<-] Move pointer to first value find right null move pointer left   
 then increment where we want our NEWVALUE to be stored then   
 return back by finding leftmost null then decrementing pointer   
 twice then decrement our NEWVALUE cell  
 

## [C](http://rosettacode.org/wiki/Category:C "Category:C")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=46 "Edit section: C")]

Fixed size static array of integers with initialization:

```c

int myArray2[10] = { 1, 2, 0 }; /* the rest of elements get the value 0 */  
float myFloats[] ={1.2, 2.5, 3.333, 4.92, 11.2, 22.0 }; /* automatically sizes */

```

When no size is given, the array is automatically sized. Typically this is how initialized arrays are defined. When this is done, you'll often see a definition that produces the number of elements in the array, as follows.

```c  
#define MYFLOAT_SIZE (sizeof(myFloats)/sizeof(myFloats[0]))
```  

When defining autosized multidimensional arrays, all the dimensions except the first (leftmost) need to be defined. This is required in order for the compiler to generate the proper indexing for the array.

```c
long a2D_Array[3][5];    /* 3 rows, 5 columns. */  
float my2Dfloats[][3] = {   
   1.0, 2.0, 0.0,  
   5.0, 1.0, 3.0 };  
#define FLOAT_ROWS (sizeof(my2Dfloats)/sizeof(my2dFloats[0]))
```

When the size of the array is not known at compile time, arrays may be dynamically allocated to the proper size. The  `malloc()`,  `calloc()`  and  `free()`  functions require the header  `stdlib.h`.

```c
int numElements = 10;  
int *myArray = [malloc](https://www.opengroup.org/onlinepubs/009695399/functions/malloc.html)(sizeof(int) * numElements);  /* array of 10 integers */  
if ( myArray != NULL )   /* check to ensure allocation succeeded. */  
{  
  /* allocation succeeded */  
  /* at the end, we need to free the allocated memory */  
  [free](https://www.opengroup.org/onlinepubs/009695399/functions/free.html)(myArray);  
}  
                    /* calloc() additionally pre-initializes to all zeros */  
short *myShorts = [calloc](https://www.opengroup.org/onlinepubs/009695399/functions/calloc.html)( numElements, sizeof(short)); /* array of 10 */   
if (myShorts != NULL)....

```

Once allocated, myArray can be used as a normal array.

The first element of a C array is indexed with 0. To set a value:

```c
myArray[0] = 1;  
myArray[1] = 3;
```

And to retrieve it (e.g. for printing, provided that the  stdio.h  header was included for the printf function)
[printf](https://www.opengroup.org/onlinepubs/009695399/functions/printf.html)("%d\n", myArray[1]);


The  array[index]  syntax can be considered as a shortcut for  *(index + array)  and thus the square brackets are a commutative binary operator:

```c
*(array + index) = 1;  
[printf](https://www.opengroup.org/onlinepubs/009695399/functions/printf.html)("%d\n", *(array + index));  
3[array] = 5;
```

There's no bounds check on the indexes. Negative indexing can be implemented as in the following.

```c
#define XSIZE 20  
double *kernel = [malloc](https://www.opengroup.org/onlinepubs/009695399/functions/malloc.html)(sizeof(double)*2*XSIZE+1);  
if (kernel) {  
   kernel += XSIZE;  
   for (ix=-XSIZE; ix<=XSIZE; ix++) {  
       kernel[ix] = f(ix);  
   ....  
   [free](https://www.opengroup.org/onlinepubs/009695399/functions/free.html)(kernel-XSIZE);  
   }  
}
```

In C99, it is possible to declare arrays with a size that is only known at runtime (e.g. a number input by the user).

Typically dynamic allocation is used and the allocated array is sized to the maximum that might be needed. A additional variable is declared and used to maintain the current number of elements used. In C, arrays may be dynamically resized if they were allocated:

```c   
int *array = [malloc](https://www.opengroup.org/onlinepubs/009695399/functions/malloc.html) (sizeof(int) * 20);  
....  
array = [realloc](https://www.opengroup.org/onlinepubs/009695399/functions/realloc.html)(array, sizeof(int) * 40);  
 ```


A Linked List for chars may be implemented like this:

 ```c

#include <stdlib.h>  
#include <stdio.h>  
typedef struct node{  
  char value;  
  struct node* next;  
} node;  
typedef struct charList{  
  node* first;  
  int size;  
} charList;  
   
charList createList(){  
  charList foo = {.first = NULL, .size = 0};  
  return foo;  
}  
int addEl(charList* list, char c){  
  if(list != NULL){  
    node* foo = (node*)[malloc](https://www.opengroup.org/onlinepubs/009695399/functions/malloc.html)(sizeof(node));  
    if(foo == NULL) return -1;  
    foo->value = c; foo->next = NULL;  
    if(list->first == NULL){  
      list->first = foo;  
    }else{  
      node* it= list->first;  
      while(it->next != NULL)it = it->next;  
      it->next = foo;  
    }  
    list->size = list->size+1;  
    return 0;  
  }else return -1;  
}  
int removeEl(charList* list, int index){  
    if(list != NULL && list->size > index){  
      node* it = list->first;  
      for(int i = 0; i < index-1; i++) it = it->next;  
      node* el = it->next;  
      it->next = el->next;  
      [free](https://www.opengroup.org/onlinepubs/009695399/functions/free.html)(el);  
      list->size--;  
      return 0;  
    }else return -1;  
}  
char getEl(charList* list, int index){  
    if(list != NULL && list->size > index){  
        node* it = list->first;  
        for(int i = 0; i < index; i++) it = it->next;  
        return it->value;  
    }else return '\0';  
}  
static void cleanHelp(node* el){  
  if(el != NULL){  
    if(el->next != NULL) cleanHelp(el->next);  
    [free](https://www.opengroup.org/onlinepubs/009695399/functions/free.html)(el);  
  }  
}  
void clean(charList* list){  
  cleanHelp(list->first);  
  list->size = 0;  
}  
 
 ```
 
## [ChucK](http://rosettacode.org/wiki/Category:ChucK "Category:ChucK")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=47 "Edit section: ChucK")]

   
int array[0]; // instantiate int array  
array << 1; // append item  
array << 2 << 3; // append items  
4 => array[3]; // assign element(4) to index(3)  
5 => array.size; // resize  
array.clear(); // clear elements  
<<<array.size()>>>; // print in cosole array size  
[1,2,3,4,5,6,7] @=> array;  
array.popBack(); // Pop last element  
 

## [C++](http://rosettacode.org/wiki/Category:C%2B%2B "Category:C++")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=48 "Edit section: C++")]

**Works with**:  [C++11](http://rosettacode.org/wiki/C%2B%2B11 "C++11")

C++ supports several types of array, depending on whether or not the size is known at compile time, and whether the array must be fixed-size or can grow.

`std::array<T, N>`  is a fixed-size array of  `T`  objects. The size (`N`) must be known at compile time. It wraps a C array, and provides additional functionality and safety. Depending on how it is used, it may be dynamically allocated on the stack as needed, placed in read-only program memory at load time, or possibly may only exist during compilation and get optimized away, among other possibilities.

`std::vector<T>`  is a resizable array of  `T`  objects. The memory for the array will be allocated from the heap (unless a custom allocator is used).

#include <array>  
#include <vector>  
   
// These headers are only needed for the demonstration  
#include <algorithm>  
#include <iostream>  
#include <iterator>  
#include <string>  
   
// This is a template function that works for any array-like object  
template <typename Array>  
void demonstrate(Array& array)  
{  
  // Array element access  
  array[2] = "Three";  // Fast, but unsafe - if the index is out of bounds you  
                       // get undefined behaviour  
  array.at(1) = "Two"; // *Slightly* less fast, but safe - if the index is out  
                       // of bounds, an exception is thrown  
   
  // Arrays can be used with standard algorithms  
  std::reverse(begin(array), end(array));  
  std::for_each(begin(array), end(array),  
    [](typename Array::value_type const& element) // in C++14, you can just use auto  
    {  
      std::cout << element << ' ';  
    });  
   
  std::cout << '\n';  
}  
   
int main()  
{  
  // Compile-time sized fixed-size array  
  auto fixed_size_array = std::array<std::string, 3>{ "One", "Four", "Eight" };  
  // If you do not supply enough elements, the remainder are default-initialized  
   
  // Dynamic array  
  auto dynamic_array = std::vector<std::string>{ "One", "Four" };  
  dynamic_array.push_back("Eight"); // Dynamically grows to accept new element  
   
  // All types of arrays can be used more or less interchangeably  
  demonstrate(fixed_size_array);  
  demonstrate(dynamic_array);  
}

## [C#](http://rosettacode.org/wiki/Category:C_sharp "Category:C sharp")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=49 "Edit section: C#")]

Example of array of 10 int types:

 int[] numbers = [new](https://www.google.com/search?q=new+msdn.microsoft.com) int[10];

Example of array of 3 string types:

 string[] words = { "these", "are", "arrays" };

You can also declare the size of the array and initialize the values at the same time:

  int[] more_numbers = [new](https://www.google.com/search?q=new+msdn.microsoft.com) int[3]{ 21, 14 ,63 };

  
For Multi-Dimensional arrays you declare them the same except for a comma in the type declaration.

The following creates a 3x2 int matrix

  int[,] number_matrix = [new](https://www.google.com/search?q=new+msdn.microsoft.com) int[3,2];

As with the previous examples you can also initialize the values of the array, the only difference being each row in the matrix must be enclosed in its own braces.

  string[,] string_matrix = { {"I","swam"}, {"in","the"}, {"freezing","water"} };

or

 string[,] funny_matrix = [new](https://www.google.com/search?q=new+msdn.microsoft.com) string[2,2]{ {"clowns", "are"} , {"not", "funny"} };

int[] array = [new](https://www.google.com/search?q=new+msdn.microsoft.com) int[10];  
   
array[0] = 1;  
array[1] = 3;  
   
Console.WriteLine(array[0]);

Dynamic

using System;  
using System.Collections.Generic;  
   
List<int> list = [new](https://www.google.com/search?q=new+msdn.microsoft.com) List<int>();  
   
list.Add(1);  
list.Add(3);  
   
list[0] = 2;  
   
Console.WriteLine(list[0]);

## [Ceylon](http://rosettacode.org/wiki/Category:Ceylon "Category:Ceylon")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=50 "Edit section: Ceylon")]

**Works with**:  [Ceylon](http://rosettacode.org/wiki/Ceylon "Ceylon")  version 1.3.0

import ceylon.collection {  
   
	ArrayList  
}  
   
shared void run() {  
   
	// you can get an array from the Array.ofSize named constructor  
	value array = Array.ofSize(10, "hello");  
	value a = array[3];  
	print(a);  
	array[4] = "goodbye";  
	print(array);  
   
	// for a dynamic list import ceylon.collection in your module.ceylon file  
	value list = ArrayList<String>();  
	list.push("hello");  
	list.push("hello again");  
	print(list);  
}

## [Clean](http://rosettacode.org/wiki/Category:Clean "Category:Clean")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=51 "Edit section: Clean")]

Array denotations are overloaded in Clean, therefore we explicitly specify the types. There are lazy, strict, and unboxed array.

### Lazy array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=52 "Edit section: Lazy array")]

Create a lazy array of strings using an array denotation.

array :: {String}  
array = {"Hello", "World"}

Create a lazy array of floating point values by sharing a single element.

array :: {Real}  
array = createArray 10 3.1415

Create a lazy array of integers using an array (and also a list) comprehension.

array :: {Int}  
array = {x \\ x <- [1 .. 10]}

### Strict array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=53 "Edit section: Strict array")]

Create a strict array of integers.

array :: {!Int}  
array = {x \\ x <- [1 .. 10]}

### Unboxed array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=54 "Edit section: Unboxed array")]

Create an unboxed array of characters, also known as  String.

array :: {#Char}  
array = {x \\ x <- ['a' .. 'z']}

## [Clipper](http://rosettacode.org/wiki/Category:Clipper "Category:Clipper")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=55 "Edit section: Clipper")]

Clipper arrays aren't divided to fixed-length and dynamic. Even if we declare it with a certain dimensions, it can be resized in the same way as it was created dynamically. The first position in an array is 1, not 0, as in some other languages.

   // Declare and initialize two-dimensional array  
   Local arr1 := { { "NITEM","N",10,0 }, { "CONTENT","C",60,0} }  
   // Create an empty array  
   Local arr2 := {}  
   // Declare three-dimensional array  
   Local arr3[2,100,3]  
   // Create an array  
   Local arr4 := Array(50)  
   
   // Array can be dynamically resized:  
   arr4 := ASize( arr4, 80 )

Items, including nested arrays, can be added to existing array, deleted from it, assigned to it

// Adding new item to array, its size is incremented  
   Aadd( arr1, { "LBASE","L",1,0 } )  
// Delete the first item of arr3, The size of arr3 remains the same,   all items are shifted to one position, the last item is replaced by Nil:  
   ADel( arr1, 1 )  
// Assigning a value to array item  
   arr3[1,1,1] := 11.4

Retrieve items of an array:

   x := arr3[1,10,2]  
// The retrieved item can be nested array, in this case it isn't copied, the pointer to it is assigned  
 

There is a set of functions to manage arrays in Clipper, including the following:

// Fill the 20 items of array with 0, starting from 5-th item:  
   AFill( arr4, 0, 5, 20 )  
//Copy 10 items from arr4 to arr3[2], starting from the first position:  
   ACopy( arr4, arr3[2], 1, 10 )  
//Duplicate the whole or nested array:  
   arr5 := AClone( arr1 )  
   arr6 := AClone( arr1[3] )

## [Clojure](http://rosettacode.org/wiki/Category:Clojure "Category:Clojure")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=56 "Edit section: Clojure")]

;clojure is a language built with immutable/persistent data structures. there is no concept of changing what a vector/list   
;is, instead clojure creates a new array with an added value using (conj...)  
;in the example below the my-list does not change.  
   
   
user=> (def my-list (list 1 2 3 4 5))  
   
user=> my-list  
(1 2 3 4 5)  
   
user=> (first my-list)  
1  
   
user=> (nth my-list 3)  
4  
   
user=> (conj my-list 100) ;adding to a list always adds to the head of the list  
(100 1 2 3 4 5)  
   
user=> my-list ;it is impossible to change the list pointed to by my-list  
(1 2 3 4 5)  
   
user=> (def my-new-list (conj my-list 100))  
   
user=> my-new-list  
(100 1 2 3 4 5)  
   
user=> (cons 200 my-new-list) ;(cons makes a new list, (conj will make a new object of the same type as the one it is given  
(200 100 1 2 3 4 5)  
   
user=> (def my-vec [1 2 3 4 5 6])  
   
user=> (conj my-vec 300) ;adding to a vector always adds to the end of the vector  
[1 2 3 4 5 6 300]

## [COBOL](http://rosettacode.org/wiki/Category:COBOL "Category:COBOL")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=57 "Edit section: COBOL")]

In COBOL, arrays are called  _tables_. Also, indexes begin from 1.

       IDENTIFICATION DIVISION.  
       PROGRAM-ID. arrays.  
   
       DATA DIVISION.  
       WORKING-STORAGE SECTION.  
       01  fixed-length-table.  
           03  fixed-table-elt      PIC X OCCURS 5 TIMES.  
   
       01  table-length             PIC 9(5) VALUE 1.  
       01  variable-length-table.  
           03  variable-table-elt   PIC X OCCURS 1 TO 5 TIMES  
               DEPENDING ON table-length.  
   
       01  initial-value-area.  
           03  initial-values.  
               05  FILLER           PIC X(10) VALUE "One".  
               05  FILLER           PIC X(10) VALUE "Two".  
               05  FILLER           PIC X(10) VALUE "Three".  
           03 initial-value-table REDEFINES initial-values.  
              05  initial-table-elt PIC X(10) OCCURS 3 TIMES.  
   
       01  indexed-table.  
           03  indexed-elt          PIC X OCCURS 5 TIMES  
               INDEXED BY table-index.  
   
       PROCEDURE DIVISION.  
           *> Assigning the contents of an entire table.  
           MOVE "12345" TO fixed-length-table  
   
           *>  Indexing an array (using an index)  
           MOVE 1 TO table-index  
           MOVE "1" TO indexed-elt (table-index)  
   
           *> Pushing a value into a variable-length table.  
           ADD 1 TO table-length  
           MOVE "1" TO variable-table-elt (2)  
   
           GOBACK  
           .

## [CoffeeScript](http://rosettacode.org/wiki/Category:CoffeeScript "Category:CoffeeScript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=58 "Edit section: CoffeeScript")]

array1 = []  
array1[0] = "Dillenidae"  
array1[1] = "animus"  
array1[2] = "Kona"  
alert "Elements of array1: " + array1 # Dillenidae,animus,Kona  
   
array2 = ["Cepphus", "excreta", "Gansu"]  
alert "Value of array2[1]: " + array2[1] # excreta

## [ColdFusion](http://rosettacode.org/wiki/Category:ColdFusion "Category:ColdFusion")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=59 "Edit section: ColdFusion")]

Creating a one-dimensional Array:

<cfset arr1 = ArrayNew(1)>

Creating a two-dimensional Array in CFScript:

<cfscript>  
  arr2 = ArrayNew(2);  
</cfscript>

_ColdFusion Arrays are  **NOT**  zero-based, they begin at index  **1**_

## [Common Lisp](http://rosettacode.org/wiki/Category:Common_Lisp "Category:Common Lisp")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=60 "Edit section: Common Lisp")]

(let ((array (make-array 10)))  
  (setf (aref array 0) 1  
          (aref array 1) 3)  
  (print array))

Dynamic

(let ((array (make-array 0 :adjustable t :fill-pointer 0)))  
  (vector-push-extend 1 array)  
  (vector-push-extend 3 array)  
  (setf (aref array 0) 2)  
  (print array))

Creates a one-dimensional array of length 10. The initial contents are undefined.

(make-array 10)

Creates a two-dimensional array with dimensions 10x20.

(make-array '(10 20))

make-array  may be called with a number of optional arguments.

; Makes an array of 20 objects initialized to nil  
(make-array 20 :initial-element nil)   
; Makes an integer array of 4 elements containing 1 2 3 and 4 initially which can be resized  
(make-array 4 :element-type 'integer :initial-contents '(1 2 3 4) :adjustable t)

## [Component Pascal](http://rosettacode.org/wiki/Category:Component_Pascal "Category:Component Pascal")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=61 "Edit section: Component Pascal")]

An arrays in Component Pascal are started from zero index.

  

   
MODULE TestArray;  
(* Implemented in BlackBox Component Builder *)  
   
	IMPORT Out;  
   
	(* Static array *)  
   
	PROCEDURE DoOneDim*;  
		CONST M = 5;  
		VAR a: ARRAY M OF INTEGER;  
	BEGIN  
		a[0] := 100; (* set first element's value of array a to 100 *)  
		a[M-1] := -100; (* set M-th element's value of array a to -100 *)  
		Out.Int(a[0], 0); Out.Ln;  
		Out.Int(a[M-1], 0); Out.Ln;  
	END DoOneDim;  
   
	PROCEDURE DoTwoDim*;  
		VAR b: ARRAY 5, 4 OF INTEGER;  
	BEGIN  
		b[1, 2] := 100; (* second row, third column element *)  
		b[4, 3] := -100; (* fifth row, fourth column element *)  
		Out.Int(b[1, 2], 0); Out.Ln;  
		Out.Int(b[4, 3], 0); Out.Ln;  
	END DoTwoDim;  
   
END TestArray.

## [Computer/zero Assembly](http://rosettacode.org/wiki/Category:Computer/zero_Assembly "Category:Computer/zero Assembly")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=62 "Edit section: Computer/zero Assembly")]

An array is simply a sequence of memory addresses. If we have an array beginning at address  ary, we can access element  {\displaystyle n}  (zero-indexed) using an instruction of the form  LDA ary+n  (or  STA ary+n,  ADD ary+n,  SUB ary+n). Generating this instruction will often involve the use of self-modifying code: we start with an instruction like  LDA ary, add  {\displaystyle n}  to it, store it back, and execute it.

It is often convenient to be able to iterate through an array—which means knowing where the array ends. There are two easy ways to do this:  _fixed-length arrays_  and  _zero-terminated arrays_. As an illustration, we shall find the sum of an array of the first ten positive integers using each technique.

### Fixed-length array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=63 "Edit section: Fixed-length array")]

We have finished iterating through the array when the next load instruction would be  LDA ary+length(ary).

load:   LDA  ary  
        ADD  sum  
        STA  sum  
   
        LDA  load  
        ADD  one  
        STA  load  
   
        SUB  end  
        BRZ  done  
   
        JMP  load  
   
done:   LDA  sum  
        STP  
   
one:         1  
end:    LDA  ary+10  
   
sum:         0  
   
ary:         1  
             2  
             3  
             4  
             5  
             6  
             7  
             8  
             9  
             10

### Zero-terminated array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=64 "Edit section: Zero-terminated array")]

load:   LDA  ary  
        BRZ  done  
   
        ADD  sum  
        STA  sum  
   
        LDA  load  
        ADD  one  
        STA  load  
   
        JMP  load  
   
done:   LDA  sum  
        STP  
   
one:         1  
   
sum:         0  
   
ary:         1  
             2  
             3  
             4  
             5  
             6  
             7  
             8  
             9  
             10  
             0

## [D](http://rosettacode.org/wiki/Category:D "Category:D")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=65 "Edit section: D")]

// All D arrays are capable of bounds checks.  
   
import std.stdio, core.stdc.stdlib;  
import std.container: Array;  
   
void main() {  
    // GC-managed heap allocated dynamic array:  
    auto array1 = new int[1];  
    array1[0] = 1;  
    array1 ~= 3; // append a second item  
    // array1[10] = 4; // run-time error  
    writeln("A) Element 0: ", array1[0]);  
    writeln("A) Element 1: ", array1[1]);  
   
    // Stack-allocated fixed-size array:  
    int[5] array2;  
    array2[0] = 1;  
    array2[1] = 3;  
    // array2[2] = 4; // compile-time error  
    writeln("B) Element 0: ", array2[0]);  
    writeln("B) Element 1: ", array2[1]);  
   
    // Stack-allocated dynamic fixed-sized array,  
    // length known only at run-time:  
    int n = 2;  
    int[] array3 = (cast(int*)alloca(n * int.sizeof))[0 .. n];  
    array3[0] = 1;  
    array3[1] = 3;  
    // array3[10] = 4; // run-time error  
    writeln("C) Element 0: ", array3[0]);  
    writeln("C) Element 1: ", array3[1]);  
   
    // Phobos-defined  heap allocated not GC-managed array:  
    Array!int array4;  
    array4.length = 2;  
    array4[0] = 1;  
    array4[1] = 3;  
    // array4[10] = 4; // run-time exception  
    writeln("D) Element 0: ", array4[0]);  
    writeln("D) Element 1: ", array4[1]);  
}

Output:

A) Element 0: 1
A) Element 1: 3
B) Element 0: 1
B) Element 1: 3
C) Element 0: 1
C) Element 1: 3
D) Element 0: 1
D) Element 1: 3

One more kind of built-in array:

import std.stdio, core.simd;  
   
void main() {  
    // Stack-allocated vector for SIMD registers:  
    ubyte16 vector5;  
    vector5.array[0] = 1;  
    vector5.array[1] = 3;  
    // vector5.array[17] = 4; // Compile-time error.  
    writeln("E) Element 0: ", vector5.array[0]);  
    writeln("E) Element 1: ", vector5.array[1]);  
}

Output:

E) Element 0: 1
E) Element 1: 3

## [Dao](http://rosettacode.org/wiki/Category:Dao "Category:Dao")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=66 "Edit section: Dao")]

# use [] to create numeric arrays of int, float, double or complex types:  
a = [ 1, 2, 3 ]  # a vector  
b = [ 1, 2; 3, 4 ] # a 2X2 matrix  
   
# use {} to create normal arrays of any types:  
c = { 1, 2, 'abc' }  
   
d = a[1]  
e = b[0,1] # first row, second column  
f = c[1]

## [Déjà Vu](http://rosettacode.org/wiki/Category:D%C3%A9j%C3%A0_Vu "Category:Déjà Vu")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=67 "Edit section: Déjà Vu")]

In Déjà Vu, the relevant datatype is called list, which is basically a stack with random element access for getting and setting values.

#create a new list  
local :l []  
   
#add something to it  
push-to l "Hi"  
   
#add something else to it  
push-to l "Boo"  
   
#the list could also have been built up this way:  
local :l2 [ "Hi" "Boo" ]  
   
#this prints 2  
!print len l  
   
#this prints Hi  
!print get-from l 0  
   
#this prints Boo  
!print pop-from l  
 

## [Delphi](http://rosettacode.org/wiki/Category:Delphi "Category:Delphi")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=68 "Edit section: Delphi")]

This example creates a static and dynamic array, asks for a series of numbers storing them in the static one, puts in the dynamic one the numbers in reverse order, concatenates the number in two single string variables and display those strings in a popup window.

   
procedure TForm1.Button1Click(Sender: TObject);  
var  
  StaticArray: array[1..10] of Integer; // static arrays can start at any index  
  DynamicArray: array of Integer; // dynamic arrays always start at 0  
  StaticArrayText,  
  DynamicArrayText: string;  
  ixS, ixD: Integer;  
begin  
  // Setting the length of the dynamic array the same as the static one  
  SetLength(DynamicArray, Length(StaticArray));  
  // Asking random numbers storing into the static array  
  for ixS := Low(StaticArray) to High(StaticArray) do  
  begin  
    StaticArray[ixS] := StrToInt(  
      InputBox('Random number',  
               'Enter a random number for position',  
               IntToStr(ixS)));  
  end;  
  // Storing entered numbers of the static array in reverse order into the dynamic  
  ixD := High(DynamicArray);  
  for ixS := Low(StaticArray) to High(StaticArray) do  
  begin  
    DynamicArray[ixD] := StaticArray[ixS];  
    Dec(ixD);  
  end;  
  // Concatenating the static and dynamic array into a single string variable  
  StaticArrayText := '';  
  for ixS := Low(StaticArray) to High(StaticArray) do  
    StaticArrayText := StaticArrayText + IntToStr(StaticArray[ixS]);  
  DynamicArrayText := '';  
  for ixD := Low(DynamicArray) to High(DynamicArray) do  
    DynamicArrayText := DynamicArrayText + IntToStr(DynamicArray[ixD]);  
  end;  
  // Displaying both arrays (#13#10 = Carriage Return/Line Feed)  
  ShowMessage(StaticArrayText + #13#10 + DynamicArrayText);  
end;

## [Dragon](http://rosettacode.org/wiki/Category:Dragon "Category:Dragon")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=69 "Edit section: Dragon")]

array = newarray(3) //optionally, replace "newarray(5)" with a brackets list of values like "[1, 2, 3]"  
array[0] = 42  
showln array[2] 

## [DWScript](http://rosettacode.org/wiki/Category:DWScript "Category:DWScript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=70 "Edit section: DWScript")]

   
// dynamic array, extensible, this a reference type  
var d : array of Integer;  
d.Add(1);     // has various methods to add, delete, etc.  
d.Add(2, 3);  
   
// read and write elements by index  
item := d[5];  
d[6] := item+1;  
   
// static, fixed-size array, arbitrary lower-bound, this is a value type  
var s : array [2..4] of Integer;  
   
// inline array constructor, works for both static and dynamic arrays  
s := [1, 2, 3];  
 

## [Dyalect](http://rosettacode.org/wiki/Category:Dyalect "Category:Dyalect")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=71 "Edit section: Dyalect")]

//Dyalect supports dynamic arrays  
var empty = []  
var xs = [1, 2, 3]  
   
//Add elements to an array  
empty.add(0)  
empty.addRange(xs)  
   
//Access array elements  
var x = xs[2]  
xs[2] = x * x

## [E](http://rosettacode.org/wiki/Category:E "Category:E")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=72 "Edit section: E")]

E's collection library emphasizes providing both mutable and immutable collections. The relevant array-like types are  `ConstList`  and  `FlexList`.

Literal lists are  `ConstList`s.

? [def](http://wiki.erights.org/wiki/def) empty := []  
# value: []  
   
? [def](http://wiki.erights.org/wiki/def) numbers := [1,2,3,4,5]  
# value: [1, 2, 3, 4, 5]  
   
? numbers.with(6)  
# value: [1, 2, 3, 4, 5, 6]  
   
? numbers + [4,3,2,1]  
# value: [1, 2, 3, 4, 5, 4, 3, 2, 1]

Note that each of these operations returns a different list object rather than modifying the original. You can, for example, collect values:

? [var](http://wiki.erights.org/wiki/var) numbers := []  
# value: []  
   
? numbers := numbers.with(1)  
# value: [1]  
   
? numbers with= 2            # shorthand for same  
# value: [1, 2]

FlexLists can be created explicitly, but are typically created by  _diverging_  another list. A ConstList can be gotten from a FlexList by  _snapshot_.

? [def](http://wiki.erights.org/wiki/def) flex := numbers.diverge()  
# value: [1, 2].diverge()  
   
? flex.push(-3)  
? flex  
# value: [1, 2, -3].diverge()  
   
? numbers  
# value: [1, 2]  
   
? flex.snapshot()  
# value: [1, 2, -3]

Creating a FlexList with a specific size, generic initial data, and a type restriction:

([0] * 100).diverge([int](http://wiki.erights.org/wiki/int))    # contains 100 zeroes, can only contain integers

Note that this puts the same value in every element; if you want a collection of some distinct mutable objects, see  [N distinct objects#E](http://rosettacode.org/wiki/N_distinct_objects#E "N distinct objects").

In accordance with its guarantees of determinism, you can never have an  _uninitialized_  FlexList in E.

## [EasyLang](http://rosettacode.org/wiki/Category:EasyLang "Category:EasyLang")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=73 "Edit section: EasyLang")]

len f[] 3  
for i range len f[]  
  f[i] = i  
.  
f[] &= 3  
for i range len f[]  
  print f[i]  
.

## [EGL](http://rosettacode.org/wiki/Category:EGL "Category:EGL")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=74 "Edit section: EGL")]

Arrays in EGL are 1-based, so the first element of an array is placed in element [1].

**Fixed-length array**

   
array int[10]; //optionally, add a braced list of values. E.g. array int[10]{1, 2, 3};  
array[1] = 42;  
SysLib.writeStdout(array[1]);  
 

Output:

42

**Dynamic array**

array int[0]; // Array declared without elements.
array.appendElement(11); // Add an element to the array and provide a value at the samen time.
array.appendElement(new int{}); // Add an element with the correct type, but without a value.
array[2] = 18; // Set the value of the added element.
SysLib.writeStdout(array[1]);
SysLib.writeStdout(array[2]);

Output:

11
18

## [Eiffel](http://rosettacode.org/wiki/Category:Eiffel "Category:Eiffel")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=75 "Edit section: Eiffel")]

   
class  
	APPLICATION  
   
inherit  
	[ARGUMENTS](https://www.google.com/search?q=site%3Ahttp%3A%2F%2Fdocs.eiffel.com%2Feiffelstudio%2Flibraries+arguments&btnI=I%27m+Feeling+Lucky)  
   
create  
	make  
   
feature {[NONE](https://www.google.com/search?q=site%3Ahttp%3A%2F%2Fdocs.eiffel.com%2Feiffelstudio%2Flibraries+none&btnI=I%27m+Feeling+Lucky)} -- Initialization  
	make  
			-- Run application.  
		do  
			-- initialize the array, index starts at 1 (not zero) and prefill everything with the letter z  
			create my_static_array.make_filled ("z", 1, 50)  
   
			my_static_array.put ("a", 1)  
			my_static_array.put ("b", 2)  
			my_static_array [3] := "c"  
   
			-- access to array fields  
			print (my_static_array.at(1) + "%N")  
			print (my_static_array.at(2) + "%N")  
			print (my_static_array [3] + "%N")  
   
			-- in Eiffel static arrays can be resized in three ways  
			my_static_array.force ("c", 51) -- forces 'c' in position 51 and resizes the array to that size (now 51 places)  
			my_static_array.automatic_grow -- adds 50% more indices (having now 76 places)  
			my_static_array.grow (100) -- resizes the array to 100 places  
		end  
   
	my_static_array: [ARRAY](https://www.google.com/search?q=site%3Ahttp%3A%2F%2Fdocs.eiffel.com%2Feiffelstudio%2Flibraries+array&btnI=I%27m+Feeling+Lucky) [[STRING](https://www.google.com/search?q=site%3Ahttp%3A%2F%2Fdocs.eiffel.com%2Feiffelstudio%2Flibraries+string&btnI=I%27m+Feeling+Lucky)]  
end  
 

## [Elena](http://rosettacode.org/wiki/Category:Elena "Category:Elena")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=76 "Edit section: Elena")]

ELENA 4.1:

Static array

    var staticArray := new int[]::(1, 2, 3);

Generic array

    var array := system'Array.allocate:3;  
    array[0] := 1;  
    array[1] := 2;  
    array[2] := 3;

Stack allocated array

    int stackAllocatedArray[3];  
    stackAllocatedArray[0] := 1;  
    stackAllocatedArray[1] := 2;  
    stackAllocatedArray[2] := 3;

Dynamic array

    var dynamicArray := new system'collections'ArrayList();  
    dynamicArray.append:1;  
    dynamicArray.append:2;  
    dynamicArray.append:4;  
   
    dynamicArray[2] := 3;

Printing an element

    system'console.writeLine(array[0]);  
    system'console.writeLine(stackAllocatedArray[1]);  
    system'console.writeLine(dynamicArray[2]);

## [Elixir](http://rosettacode.org/wiki/Category:Elixir "Category:Elixir")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=77 "Edit section: Elixir")]

The elixir language has array-like structures called  _tuples_. The values of tuples occur sequentially in memory, and can be of any type. Tuples are represented with curly braces:

ret = {:ok, "fun", 3.1415}

Elements of tuples are indexed numerically, starting with zero.

elem(ret, 1) == "fun"  
elem(ret, 0) == :ok  
put_elem(ret, 2, "pi")               # => {:ok, "fun", "pi"}  
ret == {:ok, "fun", 3.1415}

Elements can be appended to tuples with  Tuple.append/2, which returns a new tuple, without having modified the tuple given as an argument.

Tuple.append(ret, 3.1415)            # => {:ok, "fun", "pie", 3.1415}

New tuple elements can be inserted with  Tuple.insert/3, which returns a new tuple with the given value inserted at the indicated position in the tuple argument.

Tuple.insert_at(ret, 1, "new stuff") # => {:ok, "new stuff", "fun", "pie"}

Elixir also has structures called  _lists_, which can contain values of any type, and are implemented as linked lists. Lists are represented with square brackets:

[ 1, 2, 3 ]

Lists can be indexed, appended, added, subtracted, and list elements can be replaced, updated, and deleted. In all cases, new lists are returned without affecting the list being operated on.

my_list = [1, :two, "three"]  
my_list ++ [4, :five]              # => [1, :two, "three", 4, :five]  
   
List.insert_at(my_list, 0, :cool)  # => [:cool, 1, :two, "three"]  
List.replace_at(my_list, 1, :cool) # => [1, :cool, "three"]  
List.delete(my_list, :two)         # => [1, "three"]  
my_list -- ["three", 1]            # => [:two]  
my_list                            # => [1, :two, "three"]

Lists have a  _head_, being the first element, and a  _tail_, which are all the elements of the list following the head.

iex(1)> fruit = [:apple, :banana, :cherry]  
[:apple, :banana, :cherry]  
iex(2)> hd(fruit)  
:apple  
iex(3)> tl(fruit)  
[:banana, :cherry]  
iex(4)> hd(fruit) == :apple  
true  
iex(5)> tl(fruit) == [:banana, :cherry]  
true

## [Erlang](http://rosettacode.org/wiki/Category:Erlang "Category:Erlang")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=78 "Edit section: Erlang")]

   
  %% Create a fixed-size array with entries 0-9 set to 'undefined'  
  A0 = [array](http://erlang.org/doc/man/array.html):new(10).  
  10 = [array](http://erlang.org/doc/man/array.html):size(A0).  
   
  %% Create an extendible array and set entry 17 to 'true',  
  %% causing the array to grow automatically  
  A1 = [array](http://erlang.org/doc/man/array.html):set(17, true, [array](http://erlang.org/doc/man/array.html):new()).  
  18 = [array](http://erlang.org/doc/man/array.html):size(A1).  
   
  %% Read back a stored value  
  true = [array](http://erlang.org/doc/man/array.html):get(17, A1).  
   
  %% Accessing an unset entry returns the default value  
  undefined = [array](http://erlang.org/doc/man/array.html):get(3, A1).  
   
  %% Accessing an entry beyond the last set entry also returns the  
  %% default value, if the array does not have fixed size  
  undefined = [array](http://erlang.org/doc/man/array.html):get(18, A1).  
   
  %% "sparse" functions ignore default-valued entries  
  A2 = [array](http://erlang.org/doc/man/array.html):set(4, false, A1).  
  [{4, false}, {17, true}] = [array](http://erlang.org/doc/man/array.html):sparse_to_orddict(A2).  
   
  %% An extendible array can be made fixed-size later  
  A3 = [array](http://erlang.org/doc/man/array.html):fix(A2).  
   
  %% A fixed-size array does not grow automatically and does not  
  %% allow accesses beyond the last set entry  
  {'EXIT',{badarg,_}} = (catch [array](http://erlang.org/doc/man/array.html):set(18, true, A3)).  
  {'EXIT',{badarg,_}} = (catch [array](http://erlang.org/doc/man/array.html):get(18, A3)).  
 

## [ERRE](http://rosettacode.org/wiki/Category:ERRE "Category:ERRE")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=79 "Edit section: ERRE")]

To declare array variables (with their associated type):

 DIM A%[100]   ! integer array
 DIM S$[50]    ! string array
 DIM R[50]     ! real array
 DIM R#[70]    ! long real array

Index starts from 0: you can start from 1 by using a pragma directive

 !$BASE=1 

Subscripts can be a constant like:

 CONST MX=100 
 .......
 DIM A%[MX]

ERRE arrays are static (known at compile-time) but you can declare dynamic arrays (subscripts depends from a user' input):

 !$DYNAMIC
 DIM A%[0]   ! dummy declaration
 .......
 BEGIN
    INPUT(NUM)
    !$DIM A%[NUM]
 .......

You can also redimensioning arrays with ERASE clause:

    !$RCODE="ERASE A%"
    INPUT(NUM2)
    !$DIM A%[NUM2]

Unfortunately there is no PRESERVE clause, so after an ERASE all array values are lost.

Values can be assigned to an array by a DATA..READ statements, by an INPUT or by normal assignment:

   DATA(0,1,2,3,4,5,6,7,8,9,10)
   FOR I%=0 TO 10 DO
     READ(A%[I%])
   END FOR

It's possible to assign an array to another (same type and dimensions) with

   B%[]=A%[]

Arrays are global object in an ERRE module: in the next revision there will be a LOCAL DIM statement for "local arrays".

## [Euphoria](http://rosettacode.org/wiki/Category:Euphoria "Category:Euphoria")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=80 "Edit section: Euphoria")]

   
--Arrays task for Rosetta Code wiki  
--User:Lnettnay  
   
atom dummy  
--Arrays are sequences  
-- single dimensioned array of 10 elements  
sequence xarray = repeat(0,10)  
xarray[5] = 5  
dummy = xarray[5]  
? dummy  
   
--2 dimensional array  
--5 sequences of 10 elements each  
sequence xyarray = repeat(repeat(0,10),5)  
xyarray[3][6] = 12  
dummy = xyarray[3][6]  
? dummy  
   
--dynamic array use (all sequences can be modified at any time)  
sequence dynarray = {}  
for x = 1 to 10 do    
        dynarray = append(dynarray, x * x)  
end for  
? dynarray    
   
for x = 1 to 10 do  
        dynarray = prepend(dynarray, x)  
end for  
? dynarray  
 

Output:

5
12
{1,4,9,16,25,36,49,64,81,100}
{10,9,8,7,6,5,4,3,2,1,1,4,9,16,25,36,49,64,81,100}

## [F#](http://rosettacode.org/wiki/Category:F_Sharp "Category:F Sharp")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=81 "Edit section: F#")]

**Fixed-length arrays:**

> [Array](http://research.microsoft.com/en-us/um/cambridge/projects/fsharp/manual/namespaces.html).create 6 'A';;  
val it : char [] = [|'A'; 'A'; 'A'; 'A'; 'A'; 'A'|]  
> [Array](http://research.microsoft.com/en-us/um/cambridge/projects/fsharp/manual/namespaces.html).init 8 (fun i -> i * 10) ;;  
val it : int [] = [|0; 10; 20; 30; 40; 50; 60; 70|]  
> let arr = [|0; 1; 2; 3; 4; 5; 6 |] ;;  
val arr : int [] = [|0; 1; 2; 3; 4; 5; 6|]  
> arr.[4];;  
val it : int = 4  
> arr.[4] <- 65 ;;  
val it : unit = ()  
> arr;;  
val it : int [] = [|0; 1; 2; 3; 65; 5; 6|]

**Dynamic arrays:**

If dynamic arrays are needed, it is possible to use the .NET class  `System.Collections.Generic.List<'T>`  which is aliased as  `Microsoft.FSharp.Collections.ResizeArray<'T>`:

> let arr = new ResizeArray<int>();;  
val arr : ResizeArray<int>  
> arr.Add(42);;  
val it : unit = ()  
> arr.[0];;  
val it : int = 42  
> arr.[0] <- 13;;  
val it : unit = ()  
> arr.[0];;  
val it : int = 13  
> arr.[1];;  
> System.ArgumentOutOfRangeException: Index was out of range. Must be non-negative and less than the size of the collection.  
Parameter name: index ...  
> arr;;  
val it : ResizeArray<int> = seq [13]

## [Factor](http://rosettacode.org/wiki/Category:Factor "Category:Factor")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=82 "Edit section: Factor")]

(cleave applies all the quotations to the initial argument (the array)) This demonstrates array litterals and writing/reading to the array

Directly in the listener :

{ 1 2 3 }  
{   
  [ "The initial array: " write . ]  
  [ [ 42 1 ] dip set-nth ]  
  [ "Modified array: " write . ]  
  [ "The element we modified: " write [ 1 ] dip nth . ]  
} cleave

   The initial array: { 1 2 3 }
   Modified array: { 1 42 3 }
   The element we modified: 42

Arrays of arbitrary length can be created with the <array> word :

   ( scratchpad - auto ) 10 42 <array> .
   { 42 42 42 42 42 42 42 42 42 42 }

Arrays can contain different types :

   { 1 "coucou" f [ ] }

Arrays of growable length are called Vectors.

V{ 1 2 3 }  
{   
  [ "The initial vector: " write . ]  
  [ [ 42 ] dip push ]  
  [ "Modified vector: " write . ]  
} cleave

   The initial vector: V{ 1 2 3 }
   Modified vector: V{ 1 2 3 42 }

Vectors can also be used with set-nth and nth.

   ( scratchpad - auto ) V{ } [ [ 5 5 ] dip set-nth ] [ . ] bi
   V{ 0 0 0 0 0 5 }

## [FBSL](http://rosettacode.org/wiki/Category:FBSL "Category:FBSL")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=83 "Edit section: FBSL")]

Various types of FBSL's BASIC arrays are listed below:

#APPTYPE CONSOLE

DIM  v[-1  TO  1]  AS VARIANT  ' static Variant  
v[-1] = -1  
v[0] =  "zero"  
v[1] =  !1.0  
FOR EACH  DIM  e  IN  v  

PRINT  e,  " ";  

NEXT  
PRINT

DIM  i[-1  TO  1]  AS INTEGER  ' static strong-type Integer/Quad/Single/Double/String  
i[-1] = -1  
i[0] =  "zero"  
i[1] =  !1  
FOR EACH  e  IN  i  

PRINT  e,  " ";  

NEXT  
PRINT

DIM  d[]  AS INTEGER  ' dynamic growable strong-type Integer/Quad/Single/Double/String  
d[] = -1  
d[] =  "zero"  
d[] =  !1  
FOR EACH  e  IN  d  

PRINT  e,  " ";  

NEXT  
PRINT

DIM  a[]  AS VARIANT  = {-1,  "zero",  !1}  ' dynamic growable Variant w/ anonymous array initialization  
FOR EACH  e  IN  a  

PRINT  e,  " ";  

NEXT  
PRINT

FOR EACH  e  IN  {-1,  "zero",  !1}  ' anonymous Variant  

PRINT  e,  " ";  

NEXT  
PRINT

PAUSE

Output:

-1 zero 1.000000  
-1 0 1  
-1 0 1  
-1 zero 1.000000  
-1 zero 1.000000  
  
Press any key to continue...

  
FBSL's Dynamic C supports static and dynamic initialized arrays. Dynamic variable-length arrays are not currently supported.

## [Forth](http://rosettacode.org/wiki/Category:Forth "Category:Forth")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=84 "Edit section: Forth")]

Forth has a variety of ways to allocate arrays of data as contiguous blocks of memory, though it has no built-in array handling words, favoring pointer arithmetic.

For example, a static array of 10 cells in the dictionary, 5 initialized and 5 uninitialized:

create MyArray 1 , 2 , 3 , 4 , 5 ,  5 cells allot  
here constant MyArrayEnd  
   
30 MyArray 7 cells + !  
MyArray 7 cells + @ .    \ 30  
   
: .array  MyArrayEnd MyArray do I @ .  cell +loop ;

   
: array ( n -- )  
  create   
     dup ,                           \ remember size at offset 0  
     dup cells here swap 0 fill      \ fill cells with zero  
     cells allot                     \ allocate memory  
  does> ( i addr -- )   
     swap 1+ cells + ;               \ hide offset=0 to index [0..n-1]  
: [size] -1 ;   
   
10 array MyArray   
   
30 7 MyArray !   
7 MyArray @ .                        \ 30  
   
: 5fillMyArray  5  0 do  I  I MyArray  !  loop ;  
: .MyArray     [size] MyArray @  0 do  I MyArray  @ .  loop ;  
   
.MyArray                             \ 0 0 0 0 0 0 30 0 0 0   
5fillMyArray   
.MyArray                             \ 1 2 3 4 5 0 30 0 0 0   
 

   
: array  create  dup ,  dup cells here swap 0 fill  cells allot ;  
: [size] @ ;  
: [cell] 1+ cells  + ;               \ hide offset=0 to index [0..n-1]  
   
10 array MyArray    
   
30 MyArray 7 [cell] !   
MyArray 7 [cell] @ .                 \ 30  
   
: 5fillMyArray  5  0 do  I  MyArray I [cell]  !  loop ;  
: .MyArray      MyArray [size]  0 do  MyArray I [cell]  @ .  loop ;  
   
.MyArray                             \ 0 0 0 0 0 0 30 0 0 0   
5fillMyArray   
.MyArray                             \ 1 2 3 4 5 0 30 0 0 0   
 

## [Fortran](http://rosettacode.org/wiki/Category:Fortran "Category:Fortran")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=85 "Edit section: Fortran")]

**Works with**:  [Fortran](http://rosettacode.org/wiki/Fortran "Fortran")  version 90 and later

Basic array declaration:

integer a (10)

integer :: a (10)

integer, dimension (10) :: a

Arrays are one-based. These declarations are equivalent:

integer, dimension (10) :: a

integer, dimension (1 : 10) :: a

Other bases can be used:

integer, dimension (0 : 9) :: a

Arrays can have any type (intrinsic or user-defined), e.g.:

real, dimension (10) :: a

type (my_type), dimension (10) :: a

Multidimensional array declaration:

integer, dimension (10, 10) :: a

integer, dimension (10, 10, 10) :: a

Allocatable array declaration:

integer, dimension (:), allocatable :: a

integer, dimension (:, :), allocatable :: a

Array allocation:

allocate (a (10))

allocate (a (10, 10))

Array deallocation:

deallocate (a)

Array initialisation:

integer, dimension (10) :: a = (/1, 2, 3, 4, 5, 6, 7, 8, 9, 10/)

integer :: i  
integer, dimension (10) :: a = (/(i * i, i = 1, 10)/)

integer, dimension (10) :: a = 0

integer :: i  
integer, dimension (10, 10) :: a = reshape ((/(i * i, i = 1, 100)/), (/10, 10/))

Constant array declaration:

integer :: i  
integer, dimension (10), parameter :: a = (/(i * i, i = 1, 10)/)

Element assignment:

a (1) = 1

a (1, 1) = 1

Array assignment:

a = (/1, 2, 3, 4, 5, 6, 7, 8, 9, 10/)

a = (/(i * i, i = 1, 10)/)

a = reshape ((/(i * i, i = 1, 100)/), (/10, 10/))

a = 0

Array section assignment:

a (:) = (/1, 2, 3, 4, 5, 6, 7, 8, 9, 10/)

a (1 : 5) = (/1, 2, 3, 4, 5/)

a (: 5) = (/1, 2, 3, 4, 5/)

a (6 :) = (/1, 2, 3, 4, 5/)

a (1 : 5) = (/(i * i, i = 1, 10)/)

a (1 : 5)= 0

a (1, :)= (/(i * i, i = 1, 10)/)

a (1 : 5, 1)= (/(i * i, i = 1, 5)/)

Element retrieval:

i = a (1)

Array section retrieval:

a = b (1 : 10)

Size retrieval:

i = size (a)

Size along a single dimension retrieval:

i = size (a, 1)

Bounds retrieval:

i_min = lbound (a)

i_max = ubound (a)

Bounds of a multidimensional array retrieval:

a = ubound (b)

## [FreeBASIC](http://rosettacode.org/wiki/Category:FreeBASIC "Category:FreeBASIC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=86 "Edit section: FreeBASIC")]

This info only applies for the default setting  **fb**. For the other modes  **[fblite, qb, deprecated]**  other keywords and restrictions apply. Consult the  [FreeBASIC manual](http://www.freebasic.net/wiki/wikka.php?wakka=DocToc)  for those modes.

Parts of the info was taken from the FreeBASIC manual.

Arrays limits Maximum Subscript Range [-2147483648, +2147483647] [*] Maximum Elements per Dimension +2147483647 [*] Minimum/Maximum Dimensions 1/9 Maximum Size (in bytes) +2147483647 [*]

[*] All runtime library array procedures take and produce Integer values for subscripts and indexes. The actual limits will vary (smaller) with the number of dimensions, element size, storage location and/or platform.

Every Data Type that is allowed in FreeBASIC can be used for an array. (Integer, Double, String, UDT etc.)

**Static**  Specifies static storage arrays; they are allocated at program startup and deallocated upon exit.  **Shared**  makes module-level array's visible inside Subs and Functions.  **Dim**  fixed length.  **ReDim**  variable length.  **Preserve**  can only be used With ReDim. If the array is resized, data is not reset but is preserved.  **Erase**  statement to erase arrays, clear the elements.

Fixed length array are created in the stack Space, if this space is to small the compiler will issue a warning. "Array too large for stack, consider making it var-len or Shared" You can make the array var-len by using Redim or use Dim Shared instead of Dim.

By default the bounds check is off, you can add the checks by adding the command line option  **-exx**.(will slow the program down)

**The default lower bound is always 0**

' compile with: FBC -s console.   
' compile with: FBC -s console -exx to have boundary checks.  
   
Dim As Integer a(5)  ' from s(0) to s(5)  
Dim As Integer num = 1  
Dim As String s(-num To num) ' s1(-1), s1(0) and s1(1)  
   
Static As UByte c(5) ' create a  array with 6 elements (0 to 5)  
   
'dimension array and initializing it with Data  
Dim d(1 To 2, 1 To 5) As Integer => {{1, 2, 3, 4, 5}, {1, 2, 3, 4, 5}}  
Print "  The first dimension has a lower bound of"; LBound(d);_  
                           " and a upper bound of"; UBound(d)  
Print " The second dimension has a lower bound of"; LBound(d,2);_  
                           " and a upper bound of"; UBound(d,2)  
Print : Print  
   
Dim Shared As UByte u(0 To 3) ' make a shared array of UByte with 4 elements  
   
Dim As UInteger pow() ' make a variable length array  
' you must Dim the array before you can use ReDim  
ReDim pow(num) ' pow now has 1 element  
pow(num) = 10  ' lets fill it with 10 and print it  
Print " The value of pow(num) = "; pow(num)  
   
ReDim pow(10)  ' make pow a 10 element array  
Print  
Print " Pow now has"; UBound(pow) - LBound(pow) +1; " elements"  
' the value of pow(num) is gone now  
Print " The value of pow(num) = "; pow(num); ", should be 0"  
   
Print  
For i As Integer = LBound(pow) To UBound(pow)  
    pow(i) = i * i  
    Print pow(i),  
Next  
Print:Print  
   
ReDim Preserve pow(3 To 7)  
' the first five elements will be preserved, not elements 3 to 7  
Print  
Print " The lower bound is now"; LBound(pow);_  
      " and the upper bound is"; UBound(pow)  
Print " Pow now has"; UBound(pow) - LBound(pow) +1; " elements"  
Print  
For i As Integer = LBound(pow) To UBound(pow)  
    Print pow(i),  
Next  
Print : Print  
   
'erase the variable length array  
Erase pow  
Print " The lower bound is now"; LBound(pow);_  
     " and the upper bound is "; UBound(pow)  
Print " If the lower bound is 0 and the upper bound is -1 it means,"  
Print " that the array has no elements, it's completely removed"  
Print : Print  
   
'erase the fixed length array  
Print " Display the contents of the array d"  
For i As Integer = 1 To 2 : For j As Integer = 1 To 5  
    Print d(i,j);" ";  
Next : Next : Print : Print   
   
Erase d  
Print " We have erased array d"  
Print "  The first dimension has a lower bound of"; LBound(d);_  
                           " and a upper bound of"; UBound(d)  
Print " The second dimension has a lower bound of"; LBound(d,2);_  
                           " and a upper bound of"; UBound(d,2)  
Print  
For i As Integer = 1 To 2 : For j As Integer = 1 To 5  
    Print d(i,j);" ";  
Next : Next  
Print  
Print " The elements self are left untouched but there content is set to 0"  
   
' empty keyboard buffer   
While InKey <> "" : Wend  
Print : Print "hit any key to end program"  
Sleep  
End

Output:

  The first dimension has a lower bound of 1 and a upper bound of 2
 The second dimension has a lower bound of 1 and a upper bound of 5

 The value of pow(num) = 10

 Pow now has 11 elements
 The value of pow(num) = 0, should be 0

0             1             4             9             16
25            36            49            64            81
100           

 The lower bound is now 3 and the upper bound is 7
 Pow now has 5 elements

0             1             4             9             16

 The lower bound is now 0 and the upper bound is -1
 If the lower bound is 0 and the upper bound is -1 it means,
 that the array has no elements, it completely removed

 Display the contents of the array d
 1  2  3  4  5  1  2  3  4  5 

 We have erased array d
  The first dimension has a lower bound of 1 and a upper bound of 2
 The second dimension has a lower bound of 1 and a upper bound of 5

 0  0  0  0  0  0  0  0  0  0 
 The elements self are left untouched but there content is set to 0

## [Frink](http://rosettacode.org/wiki/Category:Frink "Category:Frink")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=87 "Edit section: Frink")]

In Frink, all arrays are dynamically resizable. Arrays can be created as literals or using  `new array`

   
a = new array  
a@0 = 10  
a@1 = 20  
println[a@1]  
   
b = [1, 2, 3]  
 

## [Futhark](http://rosettacode.org/wiki/Category:Futhark "Category:Futhark")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=88 "Edit section: Futhark")]

This example is  **incorrect**. Please fix the code and remove this message.

> _**Details:**_  The language's syntax has changed, so "fun" should be "let"

Multidimensional regular arrays are a built-in datatype in Futhark. They can be written as array literals:

   
[1, 2, 3]  
 

Or created by an assortment of built-in functions:

   
replicate 5 3 == [3,3,3,3,3]  
iota 5 = [0,1,2,3,4]  
 

Uniqueness types are used to permit in-place updates without violating referential transparency. For example, we can write a function that writes an element to a specific index of an array as such:

   
fun update(as: *[]int, i: int, x: int): []int =  
  let as[i] = x  
  in x  
 

Semantically the  `update`  function returns a new array, but the compiler is at liberty to re-use the memory where array  `as`  is stored, rather than create a copy as is normally needed in pure languages. Whenever the compiler encounters a call  `update(as,i,x)`, it checks that the  `as`  is not used again. This prevents the in-place update from being observable, except through the return value of  `modify`.

## [Gambas](http://rosettacode.org/wiki/Category:Gambas "Category:Gambas")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=89 "Edit section: Gambas")]

In Gambas, there is no need to dimension arrays. The first element of an array is numbered zero, and the DIM statement is optional and can be omitted:

   
[DIM](http://gambasdoc.org/help/lang/dim) mynumbers [AS](http://gambasdoc.org/help/lang/as) [INTEGER](http://gambasdoc.org/help/lang/type/integer)[]  
myfruits [AS](http://gambasdoc.org/help/lang/as) [STRING](http://gambasdoc.org/help/lang/type/string)[]  
   
mynumbers[0] = 1.5  
mynumbers[1] = 2.3  
   
myfruits[0] = "apple"  
myfruits[1] = "banana"  
 

  
In Gambas, you DO need to dimension arrays. The first element of an array is numbered zero. The DIM statement is optional and can be omitted ONLY if defined as a Global variable.

**[Click this link to run this code](https://gambas-playground.proko.eu/?gist=5061d7f882a4768d212080e416c25e27)**

[Public](http://gambasdoc.org/help/lang/public) [Sub](http://gambasdoc.org/help/lang/sub) Main()  
[Dim](http://gambasdoc.org/help/lang/dim) sFixedArray [As](http://gambasdoc.org/help/lang/as) [String](http://gambasdoc.org/help/lang/type/string)[] = ["Rosetta", "code", "is", "a", "programming", "chrestomathy", "site"]  
[Dim](http://gambasdoc.org/help/lang/dim) sFixedArray1 [As](http://gambasdoc.org/help/lang/as) [New](http://gambasdoc.org/help/lang/new) [String](http://gambasdoc.org/help/lang/type/string)[10]  
[Dim](http://gambasdoc.org/help/lang/dim) iDynamicArray [As](http://gambasdoc.org/help/lang/as) [New](http://gambasdoc.org/help/lang/new) [Integer](http://gambasdoc.org/help/lang/type/integer)[]  
[Dim](http://gambasdoc.org/help/lang/dim) siCount [As](http://gambasdoc.org/help/lang/as) [Short](http://gambasdoc.org/help/lang/type/short)  
   
[For](http://gambasdoc.org/help/lang/for) siCount = 1 [To](http://gambasdoc.org/help/lang/to) 10  
  iDynamicArray.Add(siCount)  
[Next](http://gambasdoc.org/help/lang/next)  
   
sFixedArray1[5] = "Hello"  
sFixedArray1[6] = " world!"  
   
[Print](http://gambasdoc.org/help/lang/print) sFixedArray.Join(" ")  
[Print](http://gambasdoc.org/help/lang/print) iDynamicArray[5]  
   
[Print](http://gambasdoc.org/help/lang/print) sFixedArray1[5] & sFixedArray1[6]  
   
[End](http://gambasdoc.org/help/lang/end)

Output:

Rosetta code is a programming chrestomathy site
6
Hello world!

## [GAP](http://rosettacode.org/wiki/Category:GAP "Category:GAP")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=90 "Edit section: GAP")]

# Arrays are better called lists in GAP. Lists may have elements of mixed types, e$  
v := [ 10, 7, "bob", true, [ "inner", 5 ] ];  
# [ 10, 7, "bob", true, [ "inner", 5 ] ]  
   
# List index runs from 1 to Size(v)  
v[1];  
# 10  
   
v[0];  
# error  
   
v[5];  
# [ "inner", 5 ]  
   
v[6];  
# error  
   
# One can assign a value to an undefined element  
v[6] := 100;  
   
# Even if it's not after the last: a list may have undefined elements  
v[10] := 1000;  
v;  
# [ 10, 7, "bob", true, [ "inner", 5 ], 100,,,, 1000 ]  
   
# And one can check for defined values  
IsBound(v[10]);  
# true  
   
IsBound(v[9]);  
# false  
   
# Size of the list  
Size(v);  
# 10  
   
# Appending a list to the end of another  
Append(v, [ 8, 9]);  
v;  
# [ 10, 7, "bob", true, [ "inner", 5 ], 100,,,, 1000, 8, 9 ]  
   
# Adding an element at the end  
Add(v, "added");  
v;  
# [ 10, 7, "bob", true, [ "inner", 5 ], 100,,,, 1000, 8, 9, "added" ]

## [Genie](http://rosettacode.org/wiki/Category:Genie "Category:Genie")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=91 "Edit section: Genie")]

[indent=4]  
/*   
   Arrays, in Genie  
   
   valac --pkg=gee-0.8 arrays.gs  
   ./arrays   
*/  
   
uses  
    Gee  
   
init  
    /* allocate a fixed array */  
    var arr = new array of int[10]  
   
    /* initialized array of strings */  
    initialized:array of string = {"This", "is", "Genie"}  
   
    /* length is an array property */  
    stdout.printf("%d\n", arr.length)  
   
    /* read/write access via index */  
    arr[1] = 1  
    arr[9] = arr[1] + 8  
    stdout.printf("%d\n", arr[9])  
   
    print initialized[2]  
   
    /* Dynamic arrays are lists in Genie */  
    var dyn = new list of int  
    dyn.add(1)  
    dyn.add(8)  
    dyn.add(dyn[0]+dyn[1])  
    stdout.printf("dyn size: %d\n", dyn.size)  
    stdout.printf("dyn[2]  : %d\n", dyn[2])

Output:

prompt$ valac --pkg=gee-0.8 arrays.gs && ./arrays
10
9
Genie
dyn size: 3
dyn[2]  : 9

## [GML](http://rosettacode.org/wiki/Category:GML "Category:GML")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=92 "Edit section: GML")]

### 1-Dimensional Array Examples[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=93 "Edit section: 1-Dimensional Array Examples")]

#### Example of Fixed Length Array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=94 "Edit section: Example of Fixed Length Array")]

Array containing a space (" "), "A", "B", and "C":

array[0] = ' '  
array[1] = 'A'  
array[2] = 'B'  
array[3] = 'C'

#### Example of Arbitrary Length Array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=95 "Edit section: Example of Arbitrary Length Array")]

Array containing the set of all natural numbers from 1 through k:

for(i = 0; i < k; i += 1)  
    array[i] = i + 1

### 2-Dimensional Array Examples[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=96 "Edit section: 2-Dimensional Array Examples")]

#### Example of Fixed Length Array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=97 "Edit section: Example of Fixed Length Array")]

Array containing the multiplication table of 1 through 4 by 1 through 3:

array[1,1] = 1  
array[1,2] = 2  
array[1,3] = 3  
array[1,4] = 4  
array[2,1] = 2  
array[2,2] = 4  
array[2,3] = 6  
array[2,4] = 8  
array[3,1] = 3  
array[3,2] = 6  
array[3,3] = 9  
array[3,4] = 12

#### Example of Arbitrary Length Array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=98 "Edit section: Example of Arbitrary Length Array")]

Array containing the multiplication table of 1 through k by 1 through h:

for(i = 1; i <= k; i += 1)  
    for(j = 1; j <= h; j += 1)  
        array[i,j] = i * j

## [Go](http://rosettacode.org/wiki/Category:Go "Category:Go")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=99 "Edit section: Go")]

```go  
package main  
   
import (  
    "fmt"  
)  
   
func main() {  
    // creates an array of five ints.  
    // specified length must be a compile-time constant expression.  
    // this allows compiler to do efficient bounds checking.  
    var a [5]int  
   
    // since length is compile-time constant, len() is a compile time constant  
    // and does not have the overhead of a function call.  
    fmt.Println("len(a) =", len(a))  
   
    // elements are always initialized to 0  
    fmt.Println("a =", a)  
   
    // assign a value to an element.  indexing is 0 based.  
    a[0] = 3  
    fmt.Println("a =", a)  
   
    // retrieve element value with same syntax  
    fmt.Println("a[0] =", a[0])  
   
    // a slice references an underlying array  
    s := a[:4] // this does not allocate new array space.  
    fmt.Println("s =", s)  
   
    // slices have runtime established length and capacity, but len() and  
    // cap() are built in to the compiler and have overhead more like  
    // variable access than function call.  
    fmt.Println("len(s) =", len(s), " cap(s) =", cap(s))  
   
    // slices can be resliced, as long as there is space  
    // in the underlying array.  
    s = s[:5]  
    fmt.Println("s =", s)  
   
    // s still based on a  
    a[0] = 22  
    fmt.Println("a =", a)  
    fmt.Println("s =", s)  
   
    // append will automatically allocate a larger underlying array as needed.  
    s = append(s, 4, 5, 6)  
    fmt.Println("s =", s)  
    fmt.Println("len(s) =", len(s), " cap(s) =", cap(s))  
   
    // s no longer based on a  
    a[4] = -1  
    fmt.Println("a =", a)  
    fmt.Println("s =", s)  
   
    // make creates a slice and allocates a new underlying array  
    s = make([]int, 8)  
    fmt.Println("s =", s)  
    fmt.Println("len(s) =", len(s), " cap(s) =", cap(s))  
   
    // the cap()=10 array is no longer referenced  
    // and would be garbage collected eventually.  
}
```

Output:

len(a) = 5
a = [0 0 0 0 0]
a = [3 0 0 0 0]
a[0] = 3
s = [3 0 0 0]
len(s) = 4  cap(s) = 5
s = [3 0 0 0 0]
a = [22 0 0 0 0]
s = [22 0 0 0 0]
s = [22 0 0 0 0 4 5 6]
len(s) = 8  cap(s) = 10
a = [22 0 0 0 -1]
s = [22 0 0 0 0 4 5 6]
s = [0 0 0 0 0 0 0 0]
len(s) = 8  cap(s) = 8

## [Golfscript](http://rosettacode.org/wiki/Category:Golfscript "Category:Golfscript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=100 "Edit section: Golfscript")]

In Golfscript, arrays are created writing their elements between []. Arrays can contain any kind of object. Once created, they are pushed on the stack, as any other object.

[1 2 3]:a; # numeric only array, assigned to a and then dropped  
10,:a;     # assign to a [0 1 2 3 4 5 6 7 8 9]  
a 0= puts  # pick element at index 0 (stack: 0)  
a 10+puts  # append 10 to the end of a  
10 a+puts  # prepend 10 to a

Append and prepend works for integers or arrays only, since only in these cases the result is coerced to an array.

## [Groovy](http://rosettacode.org/wiki/Category:Groovy "Category:Groovy")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=101 "Edit section: Groovy")]

Arrays and lists are synonymous in Groovy. They can be initialized with a wide range of operations and Groovy enhancements to the Collection and List classes.

[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) aa = [ 1, 25, 31, -3 ]           // list  
[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) a = [0] * 100                    // list of 100 zeroes  
[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) b = 1..9                         // range notation  
[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) c = (1..10).[collect](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20collect) { 2.0**it }  // each output element is 2**(corresponding invoking list element)  
   
// There are no true "multi-dimensional" arrays in Groovy (as in most C-derived languages).  
// Use lists of lists in natural ("row major") order as a stand in.  
[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) d = (0..1).[collect](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20collect) { i -> (1..5).[collect](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20collect) { j -> 2**(5*i+j) [as](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20as) [double](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20double) } }  
[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) e = [ [  1.0,  2.0,  3.0,  4.0 ],  
          [  5.0,  6.0,  7.0,  8.0 ],  
          [  9.0, 10.0, 11.0, 12.0 ],  
          [ 13.0, 14.0, 15.0, 16.0 ] ]  
   
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) aa  
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) b  
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) c  
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println)()  
d.[each](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20each) { [print](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20print) "["; it.[each](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20each) { elt -> printf "%7.1f ", elt }; [println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) "]" }  
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println)()  
e.[each](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20each) { [print](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20print) "["; it.[each](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20each) { elt -> printf "%7.1f ", elt }; [println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) "]" }

Output:

[1, 25, 31, -3]
[1, 2, 3, 4, 5, 6, 7, 8, 9]
[2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]

[    2.0     4.0     8.0    16.0    32.0 ]
[   64.0   128.0   256.0   512.0  1024.0 ]

[    1.0     2.0     3.0     4.0 ]
[    5.0     6.0     7.0     8.0 ]
[    9.0    10.0    11.0    12.0 ]
[   13.0    14.0    15.0    16.0 ]

Here is a more interesting example showing a function that creates and returns a square identity matrix of order N:

[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) identity = { n ->  
   (1..n).[collect](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20collect) { i -> (1..n).[collect](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20collect) { j -> i==j ? 1.0 : 0.0 } }  
}

Test program:

[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) i2 = identity(2)  
[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) i15 = identity(15)  
   
   
i2.[each](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20each) { [print](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20print) "["; it.[each](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20each) { elt -> printf "%4.1f ", elt }; [println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) "]" }  
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println)()  
i15.[each](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20each) { [print](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20print) "["; it.[each](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20each) { elt -> printf "%4.1f ", elt }; [println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) "]" }

Output:

[ 1.0  0.0 ]
[ 0.0  1.0 ]

[ 1.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  1.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  1.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  1.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  1.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  1.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  0.0  1.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  0.0  0.0  1.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  1.0  0.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  1.0  0.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  1.0  0.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  1.0  0.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  1.0  0.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  1.0  0.0 ]
[ 0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  0.0  1.0 ]

Groovy, like every other C-derived language in the known universe, uses ZERO-based array/list indexing.

[def](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20def) strings = ['Mary', 'had', 'a', 'little', 'lamb', ". It's", 'fleece', 'was', 'white', 'as', 'snow']  
   
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) strings  
   
strings[0] = 'Arthur'  
strings[4] = 'towel'  
strings[6] = 'stain'  
strings[8] = 'ripe'  
strings[10] = 'strawberries'  
   
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) strings

Output:

["Mary", "had", "a", "little", "lamb", ". It's", "fleece", "was", "white", "as", "snow"]
["Arthur", "had", "a", "little", "towel", ". It's", "stain", "was", "ripe", "as", "strawberries"]

Negative indices are valid. They indicate indexing from the end of the list towards the start.

[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) strings[-1]

Output:

strawberries

Groovy lists can be resequenced and subsequenced by providing lists or ranges of indices in place of a single index.

[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) strings[0, 7, 2, 3, 8]  
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) strings[0..4]  
[println](https://www.google.de/search?q=site%3Agroovy.codehaus.org/%20println) strings[0..3, -5]

Output:

["Arthur", "was", "a", "little", "ripe"]
["Arthur", "had", "a", "little", "towel"]
["Arthur", "had", "a", "little", "stain"]

## [GUISS](http://rosettacode.org/wiki/Category:GUISS "Category:GUISS")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=102 "Edit section: GUISS")]

Graphical User Interface Support Script does not have variables or array storage of its own. However, it can make use of installed applications, so it is possible to utilize an installed spreadsheet application to create and manipulate arrays. Here we assume that a spreadsheet is installed and create an array containing three names:

Start,Programs,Lotus 123,Type:Bob[downarrow],Kat[downarrow],Sarah[downarrow]

## [GW-BASIC](http://rosettacode.org/wiki/Category:GW-BASIC "Category:GW-BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=103 "Edit section: GW-BASIC")]

"An array, once dimensioned, cannot be re-dimensioned within the program without first executing a CLEAR or ERASE statement." (GW-BASIC User's Guide)

10 [DATA](http://www.qbasicnews.com/qboho/qckdata.shtml) 0, 1, 2, 3, 4, 5, 6, 7, 8, 9  
20 [DIM](http://www.qbasicnews.com/qboho/qckdim.shtml) A(9)        ' Array with size 10 (9 is maximum subscript), all elements are set to 0  
30 FOR I = 0 TO 9  
40     [READ](http://www.qbasicnews.com/qboho/qckread.shtml) A(I)   ' Initialize by reading data  
50 NEXT I  
60 [PRINT](http://www.qbasicnews.com/qboho/qckprint.shtml) A(4)      ' Get 4th element of array  
70 A(4) = 400      ' Set 4th element of array  
80 [PRINT](http://www.qbasicnews.com/qboho/qckprint.shtml) A(4)  
 

## [Halon](http://rosettacode.org/wiki/Category:Halon "Category:Halon")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=104 "Edit section: Halon")]

$array = [];  
   
$array[] = 1;  
$array["key"] = 3;  
   
$array[0] = 2;  
   
echo $array[0];  
echo $array["key"];

## [Harbour](http://rosettacode.org/wiki/Category:Harbour "Category:Harbour")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=105 "Edit section: Harbour")]

Harbour arrays aren't divided to fixed-length and dynamic. Even if we declare it with a certain dimensions, it can be resized in the same way as it was created dynamically. The first position in an array is 1, not 0, as in some other languages.

   // Declare and initialize two-dimensional array  
   local arr1 := { { "NITEM", "N", 10, 0 }, { "CONTENT", "C", 60, 0 } }  
   // Create an empty array  
   local arr2 := {}  
   // Declare three-dimensional array  
   local arr3[ 2, 100, 3 ]  
   // Create an array  
   local arr4 := Array( 50 )  
   
   // Array can be dynamically resized:  
   arr4 := ASize( arr4, 80 )

Items, including nested arrays, can be added to existing array, deleted from it, assigned to it

// Adding new item to array, its size is incremented  
   AAdd( arr1, { "LBASE", "L", 1, 0 } )  
// Delete the first item of arr3, The size of arr3 remains the same, all items are shifted to one position, the last item is replaced by Nil:  
   ADel( arr1, 1 )  
// Assigning a value to array item  
   arr3[ 1, 1, 1 ] := 11.4

Retrieve items of an array:

   x := arr3[ 1, 10, 2 ]  
// The retrieved item can be nested array, in this case it isn't copied, the pointer to it is assigned  
 

There is a set of functions to manage arrays in Clipper, including the following:

// Fill the 20 items of array with 0, starting from 5-th item:  
   AFill( arr4, 0, 5, 20 )  
// Copy 10 items from arr4 to arr3[ 2 ], starting from the first position:  
   ACopy( arr4, arr3[ 2 ], 1, 10 )  
// Duplicate the whole or nested array:  
   arr5 := AClone( arr1 )  
   arr6 := AClone( arr1[ 3 ] )

## [Haskell](http://rosettacode.org/wiki/Category:Haskell "Category:Haskell")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=106 "Edit section: Haskell")]

You can read all about Haskell arrays  [here](https://haskell.org/haskellwiki/Arrays). The following example is taken from that page:

import Data.Array.[IO](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:IO)  
   
main = do arr <- newArray (1,10) 37 :: [IO](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:IO) (IOArray [Int](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:Int) [Int](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#t:Int))  
          a <- readArray arr 1  
          writeArray arr 1 64  
          b <- readArray arr 1   
          [print](https://haskell.org/ghc/docs/latest/html/libraries/base/Prelude.html#v:print) (a,b)

## [hexiscript](http://rosettacode.org/wiki/Category:Hexiscript "Category:Hexiscript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=107 "Edit section: hexiscript")]

let a    arr 2  # fixed size  
let a[0] 123    # index starting at 0  
let a[1] "test" # can hold different types  
   
println a[1]

## [HicEst](http://rosettacode.org/wiki/Category:HicEst "Category:HicEst")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=108 "Edit section: HicEst")]

REAL :: n = 3, Astat(n), Bdyn(1, 1)  
   
Astat(2) = 2.22222222  
WRITE(Messagebox, Name) Astat(2)  
   
ALLOCATE(Bdyn, 2*n, 3*n)  
Bdyn(n-1, n) = -123  
WRITE(Row=27) Bdyn(n-1, n)  
   
ALIAS(Astat, n-1,   last2ofAstat, 2)  
WRITE(ClipBoard) last2ofAstat      ! 2.22222222 0 

## [i](http://rosettacode.org/wiki/Category:I "Category:I")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=109 "Edit section: i")]

main  
	//Fixed-length arrays.  
	f $= array.integer[1]()  
	f[0] $= 2  
	print(f[0])  
   
	//Dynamic arrays.  
	d $= list.integer()  
	d[+] $= 2  
	print(d[1])  
}

## [HolyC](http://rosettacode.org/wiki/Category:HolyC "Category:HolyC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=110 "Edit section: HolyC")]

// Create an array of fixed size  
U8 array[10] = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10;  
   
// The first element of a HolyC array is indexed at 0. To set a value:  
array[0] = 123;  
   
// Access an element  
Print("%d\n", array[0]);

## Icon and Unicon[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=111 "Edit section: Icon and Unicon")]

### [Icon](http://rosettacode.org/wiki/Category:Icon "Category:Icon")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=112 "Edit section: Icon")]

record aThing(a, b, c)       # arbitrary object (record or class) for illustration  
   
procedure main()  
    A0 := []                 # empty list  
    A0 := list()             # empty list (default size 0)  
    A0 := list(0)            # empty list (literal size 0)  
   
    A1 := list(10)           # 10 elements, default initializer &null  
    A2 := list(10, 1)        # 10 elements, initialized to 1  
   
    # literal array construction - arbitrary dynamically typed members  
    A3 := [1, 2, 3, ["foo", "bar", "baz"], aThing(1, 2, 3), "the end"]  
   
    # left-end workers  
    # NOTE: get() is a synonym for pop() which allows nicely-worded use of put() and get() to implement queues  
    #  
    Q := [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]  
    x := pop(A0)        # x is 1  
    x := get(A0)        # x is 2  
    push(Q,0)  
    # Q is now [0,3, 4, 5, 6, 7, 8, 9, 10]  
   
    # right-end workers  
    x := pull(Q)        # x is 10  
    put(Q, 100)         # Q is now [0, 3, 4, 5, 6, 7, 8, 9, 100]  
   
    # push and put return the list they are building  
    # they also can have multiple arguments which work like repeated calls  
   
    Q2 := put([],1,2,3)    # Q2 is [1,2,3]  
    Q3 := push([],1,2,3)   # Q3 is [3,2,1]  
    Q4 := push(put(Q2),4),0] # Q4 is [0,1,2,3,4] and so is Q2  
   
    # array access follows with A as the sample array  
    A := [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]  
   
    # get element indexed from left  
    x := A[1]           # x is 10  
    x := A[2]           # x is 20  
    x := A[10]          # x is 100  
   
    # get element indexed from right  
    x := A[-1]          # x is 100  
    x := A[-2]          # x is 90  
    x := A[-10]         # x is 10  
   
    # copy array to show assignment to elements  
    B := copy(A)  
   
    # assign element indexed from left  
    B[1] := 11  
    B[2] := 21  
    B[10] := 101  
    # B is now [11, 21, 30, 50, 60, 60, 70, 80, 90, 101]  
   
    # assign element indexed from right - see below  
    B[-1] := 102  
    B[-2] := 92  
    B[-10] := 12  
    # B is now [12, 21, 30, 50, 60, 60, 70, 80, 92, 102]  
   
    # list slicing  
    # the unusual nature of the slice - returning 1 less element than might be expected  
    # in many languages - is best understood if you imagine indexes as pointing to BEFORE  
    # the item of interest. When a slice is made, the elements between the two points are  
    # collected. eg in the A[3 : 6] sample, it will get the elements between the [ ] marks  
    #  
    # sample list:              10  20 [30  40  50] 60  70  80  90  100  
    # positive indexes:        1   2   3   4   5   6   7   8   9   10  11  
    # non-positive indexes:  -10  -9  -8  -7  -6  -5  -4  -3  -2  -1   0  
    #  
    # I have deliberately drawn the indexes between the positions of the values.  
    # The nature of this indexing brings simplicity to string operations  
    #  
    # list slicing can also use non-positive indexes to access values from the right.  
    # The final index of 0 shown above shows how the end of the list can be nominated  
    # without having to know it's length  
    #  
    # NOTE: list slices are distinct lists, so assigning to the slice  
    # or a member of the slice does not change the values in A  
    #  
    # Another key fact to understand: once the non-positive indexes and length-offsets are  
    # resolved to a simple positive index, the index pair (if two are given) are swapped  
    # if necessary to yield the elements between the two.  
    #  
    S := A[3 : 6]       # S is [30, 40, 50]  
    S := A[6 : 3]       # S is [30, 40, 50]   not illegal or erroneous  
    S := A[-5 : -8]     # S is [30, 40, 50]  
    S := A[-8 : -5]     # S is [30, 40, 50]   also legal and meaningful  
   
    # list slicing with length request  
    S := A[3 +: 3]      # S is [30, 40, 50]  
    S := A[6 -: 3]      # S is [30, 40, 50]  
    S := A[-8 +: 3]     # S is [30, 40, 50]  
    S := A[-5 -: 3]     # S is [30, 40, 50]  
    S := A[-8 -: -3]    # S is [30, 40, 50]  
    S := A[-5 +: -3]    # S is [30, 40, 50]  
end

### [Unicon](http://rosettacode.org/wiki/Category:Unicon "Category:Unicon")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=113 "Edit section: Unicon")]

This Icon solution works in Unicon.

# Unicon provides a number of extensions  
# insert and delete work on lists allowing changes in the middle  
# possibly others  
 

This example is  **in need of improvement:**

> Need code examples for these extensions

## [Io](http://rosettacode.org/wiki/Category:Io "Category:Io")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=114 "Edit section: Io")]

foo := list("foo", "bar", "baz")  
foo at(1) println // bar  
foo append("Foobarbaz")  
foo println  
foo atPut(2, "barbaz") // baz becomes barbaz

Io> foo := list("foo", "bar", "baz")
==> list(foo, bar, baz)
Io> foo at(1) println // bar
bar
==> bar
Io> foo append("Foobarbaz")
==> list(foo, bar, baz, Foobarbaz)
Io> foo println
list(foo, bar, baz, Foobarbaz)
==> list(foo, bar, baz, Foobarbaz)
Io> foo atPut(2, "barbaz") // baz becomes barbaz
==> list(foo, bar, barbaz, Foobarbaz)
Io> 

## [J](http://rosettacode.org/wiki/Category:J "Category:J")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=115 "Edit section: J")]

In J, all data occurs in the form of rectangular (or generally  [orthotopic](https://en.wikipedia.org/wiki/Hyperrectangle "wp:Hyperrectangle")) arrays. This is true for both named and anonymous data.

   1                          NB. a stand-alone scalar value is an array without any axis  
1  
   NB. invoking any array produces that array as the result  
   {. array=: 1 3, 6#0        NB. create, name, then get head item of the array: 1 3 0 0 0 0 0 0  
1  
   0 { array                  NB. another way to get the head item  
1  
   aword=: 'there'            NB. a literal array  
   0 1 3 2 2 { aword          NB. multiple items can be drawn in a single action  
three  
   ]twoD=: 3 5 $ 'abcdefghijklmnopqrstuvwxyz'  
abcde  
fghij  
klmno  
   1 { twoD                   NB. item 1 from twoD - a list of three items  
fghij  
   1 {"1 twoD                 NB. item 1 from each rank-1 item of twoD (i.e. column 1)  
bgl  
   (<2 2){ twoD               NB. bracket indexing is not used in J  
m  
   'X' 1} aword               NB. amend item 1  
tXere  
   aword=: 'X' 1 4} aword     NB. in-place amend of items 1 and 4  
tXerX  
   'X' (0 0;1 1;2 2)} twoD    NB. amend specified items  
Xbcde  
fXhij  
klXno

Because arrays are so important in J, a large portion of the language applies to this topic.

## [Java](http://rosettacode.org/wiki/Category:Java "Category:Java")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=116 "Edit section: Java")]

int[] array = new int[10]; //optionally, replace "new int[10]" with a braced list of ints like "{1, 2, 3}"  
array[0] = 42;  
[System](https://www.google.com/search?hl=en&q=allinurl%3Asystem+java.sun.com&btnI=I%27m%20Feeling%20Lucky).out.println(array[3]);

Dynamic arrays can be made using  `List`s:

[List](https://www.google.com/search?sitesearch=java.sun.com&q=allinurl%3Aj2se%2F1+5+0%2Fdocs%2Fapi+List)<[Integer](http://java.sun.com/j2se/1.5.0/docs/api/java/lang/Integer.html)> list = new [ArrayList](http://java.sun.com/j2se/1.5.0/docs/api/java/util/ArrayList.html)<[Integer](http://java.sun.com/j2se/1.5.0/docs/api/java/lang/Integer.html)>();   // optionally add an initial size as an argument  
list.add(5);   // appends to the end of the list  
list.add(1, 6);   // inserts an element at index 1  
[System](http://java.sun.com/j2se/1.5.0/docs/api/java/lang/System.html).out.println(list.get(0));

## [JavaScript](http://rosettacode.org/wiki/Category:JavaScript "Category:JavaScript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=117 "Edit section: JavaScript")]

JavaScript arrays are Objects that inherit from Array prototype and have a special length property that is always one higher than the highest non–negative integer index. Methods inherited from Array.prototype are mostly generic and can be applied to other objects with a suitable length property and numeric property names. Note that if the Array constructor is provided with one argument, it is treated as specifying the length of the new array, if more than one argument is supplied, they are treated as members of the new array.

// Create a new array with length 0  
var myArray = new Array();  
   
// Create a new array with length 5  
var myArray1 = new Array(5);  
   
// Create an array with 2 members (length is 2)   
var myArray2 = new Array("Item1","Item2");  
   
// Create an array with 2 members using an array literal  
var myArray3 = ["Item1", "Item2"];  
   
// Assign a value to member [2] (length is now 3)  
myArray3[2] = 5;  
   
var x = myArray[2] + myArray.length;   // 8  
   
// You can also add a member to an array with the push function (length is now 4)  
myArray3.push('Test');  
   
// Elisions are supported, but are buggy in some implementations  
var y = [0,1,,];  // length 3, or 4 in buggy implementations   
 

## [jq](http://rosettacode.org/wiki/Category:Jq "Category:Jq")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=118 "Edit section: jq")]

jq arrays have the same syntax as JSON arrays, and there are similarities with Javascript arrays. For example, the index origin is 0; and if a is an array and if n is an integer less than the array's length, then a[n] is the n-th element. The length of any array, a, can be ascertained using the length filter: a|length.

There are, however, some interesting extensions, e.g.  [][4] = null  creates an array of length 5 as explained below.

# Create a new array with length 0  
[]  
   
# Create a new array of 5 nulls  
[][4] = null   # setting the element at offset 4 expands the array  
   
# Create an array having the elements 1 and 2 in that order   
[1,2]  
   
# Create an array of integers from 0 to 10 inclusive  
[ range(0; 11) ]  
   
# If a is an array (of any length), update it so that a[2] is 5  
a[2] = 5;  
   
# Append arrays a and b  
a + b  
   
# Append an element, e, to an array a  
a + [e]  
   
##################################################  
# In the following, a is assumed to be [0,1,2,3,4]  
   
# It is not an error to use an out-of-range index:  
a[10]  # => null  
   
# Negative indices count backwards from after the last element:  
a[-1]  # => 4  
   
# jq supports simple slice operations but  
# only in the forward direction:  
a[:1]  # => [0]  
a[1:]  # => [1,2,3,4]  
a[2:4] # => [2,3]  
a[4:2] # null

## [Jsish](http://rosettacode.org/wiki/Category:Jsish "Category:Jsish")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=119 "Edit section: Jsish")]

From Javascript, with the differences that Jsi treats  _typeof [elements]_  as "array", not "object".

/* Arrays in Jsi */  
// Create a new array with length 0  
var myArray = new Array();  
;myArray;  
   
// In Jsi, typeof [] is "array".  In ECMAScript, typeof [] is "object"  
;typeof [];  
   
// Create a new array with length 5  
var myArray1 = new Array(5);  
;myArray1;  
   
// Create an array with 2 members (length is 2)   
var myArray2 = new Array("Item1","Item2");  
;myArray2;  
;myArray2.length;  
   
// Create an array with 2 members using an array literal  
var myArray3 = ["Item1", "Item2"];  
;myArray3;  
   
// Assign a value to member [2] (length is now 3)  
myArray3[2] = 5;  
;myArray3;  
;myArray3.length;  
   
var x = myArray3[2] + myArray3.length;   // 8  
;x;  
   
// You can also add a member to an array with the push function (length is now 4)  
myArray3.push('Test');  
;myArray3;  
;myArray3.length;  
   
// Empty array entries in a literal is a syntax error, elisions not allowed  
//var badSyntax = [1,2,,4];  
   
   
/*  
=!EXPECTSTART!=  
myArray ==> []  
typeof [] ==> array  
myArray1 ==> [ undefined, undefined, undefined, undefined, undefined ]  
myArray2 ==> [ "Item1", "Item2" ]  
myArray2.length ==> 2  
myArray3 ==> [ "Item1", "Item2" ]  
myArray3 ==> [ "Item1", "Item2", 5 ]  
myArray3.length ==> 3  
x ==> 8  
myArray3 ==> [ "Item1", "Item2", 5, "Test" ]  
myArray3.length ==> 4  
=!EXPECTEND!=  
*/

Output:

prompt$ jsish -u arrays.jsi
PASS] arrays.jsi

## [Julia](http://rosettacode.org/wiki/Category:Julia "Category:Julia")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=120 "Edit section: Julia")]

Julia has both heterogeneous arrays and typed arrays.

julia> A = cell(3)   # create an heterogeneous array of length 3
3-element Array{Any,1}:
 #undef
 #undef
 #undef

julia> A[1] = 4.5 ; A[3] =  "some string" ; show(A)
{4.5,#undef,"some string"}

julia> A[1]          # access a value. Arrays are 1-indexed
4.5

julia> push!(A, :symbol) ; show(A)    # append an element
{4.5,#undef,"some string",:symbol}

julia> A[10]         # error if the index is out of range
ERROR: BoundsError()

For typed arrays, the type can be specified explicitely or infered from its elements.

julia> B = Array(String, 3) ; B[1]="first" ; push!(B, "fourth") ; show(B)
["first",#undef,#undef,"fourth"]

julia> push!(B, 3)   # type error
ERROR: no method convert(Type{String}, Int64)
 in push! at array.jl:488

julia> ['a':'c']     # type inference
3-element Array{Char,1}:
 'a'
 'b'
 'c'

## [KonsolScript](http://rosettacode.org/wiki/Category:KonsolScript "Category:KonsolScript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=121 "Edit section: KonsolScript")]

//creates an array of length 3  
Array:New array[3]:Number;  
   
function main() {  
  Var:Number length;  
  Array:GetLength(array, length)  //retrieve length of array  
  Konsol:Log(length)  
   
  array[0] = 5;                   //assign value  
  Konsol:Log(array[0])            //retrieve value and display  
}

## [Kotlin](http://rosettacode.org/wiki/Category:Kotlin "Category:Kotlin")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=122 "Edit section: Kotlin")]

fun main(x: Array<String>) {  
    [var](https://scala-lang.org/) a = arrayOf(1, 2, 3, 4)  
    println(a.asList())  
    a += 5  
    println(a.asList())  
    println(a.reversedArray().asList())  
}

Output:

[1, 2, 3, 4]
[1, 2, 3, 4, 5]
[5, 4, 3, 2, 1]

## [LabVIEW](http://rosettacode.org/wiki/Category:LabVIEW "Category:LabVIEW")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=123 "Edit section: LabVIEW")]

This image is a  [VI Snippet](http://zone.ni.com/devzone/cda/tut/p/id/9330), an executable image of  [LabVIEW](http://rosettacode.org/wiki/LabVIEW "LabVIEW")  code. The LabVIEW version is shown on the top-right hand corner. You can download it, then drag-and-drop it onto the LabVIEW block diagram from a file browser, and it will appear as runnable, editable code.  
[![LabVIEW Arrays.png](http://rosettacode.org/mw/images/5/50/LabVIEW_Arrays.png)](http://rosettacode.org/wiki/File:LabVIEW_Arrays.png)

## [lang5](http://rosettacode.org/wiki/Category:Lang5 "Category:Lang5")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=124 "Edit section: lang5")]

[]  
1 append  
['foo 'bar] append  
2 reshape  
0 remove 2 swap 2 compress collapse .

## [Lasso](http://rosettacode.org/wiki/Category:Lasso "Category:Lasso")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=125 "Edit section: Lasso")]

Lasso Array  [[1]](http://lassoguide.com/operations/containers.html?#array)  objects store zero or more elements and provide random access to those elements by position. Positions are 1-based integers. Lasso Arrays will grow as needed to accommodate new elements. Elements can be inserted and removed from arrays at any position. However, inserting an element anywhere but at the end of an array results in all subsequent elements being moved down.

// Create a new empty array  
local(array1) = array  
   
// Create an array with 2 members (#myarray->size is 2)   
local(array1) = array('ItemA','ItemB')  
   
// Assign a value to member [2]   
#array1->get(2) = 5  
   
// Retrieve a value from an array  
#array1->get(2) + #array1->size // 8  
   
// Merge arrays  
local(  
    array1 = array('a','b','c'),  
    array2 = array('a','b','c')  
)  
#array1->merge(#array2) // a, b, c, a, b, c  
   
// Sort an array  
#array1->sort // a, a, b, b, c, c  
   
// Remove value by index  
#array1->remove(2) // a, b, b, c, c  
   
// Remove matching items  
#array1->removeall('b') // a, c, c  
   
// Insert item  
#array1->insert('z')  // a, c, c, z  
   
// Insert item at specific position  
#array1->insert('0',1)  // 0, a, c, c, z

### Static Arrays[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=126 "Edit section: Static Arrays")]

Lasso also supports Static Arrays[[2]](http://lassoguide.com/operations/containers.html#staticarray). A Lasso staticarray is a container object that is not resizable. Staticarrays are created with a fixed size. Objects can be reassigned within the staticarray, but new positions cannot be added or removed.

// Create a staticarray containing 5 items  
local(mystaticArray) = staticarray('a','b','c','d','e')  
   
// Retreive an item  
#mystaticArray->get(3) // c  
   
// Set an item  
#mystaticArray->get(3) = 'changed' // a, b, changed, d, e  
   
// Create an empty static array with a length of 32  
local(mystaticArray) = staticarray_join(32,void)

## [Latitude](http://rosettacode.org/wiki/Category:Latitude "Category:Latitude")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=127 "Edit section: Latitude")]

Like everything in Latitude, arrays are simply objects. In particular, arrays store their elements in numerical slots rather than traditional symbolic ones. The translation scheme used to store them enables constant-time push and pop operations on either side of the array.

;; Construct an array.  
foo := [1, 2, 3].  
   
;; Arrays can also be constructed explicitly.  
bar := Array clone.  
bar pushBack (1).  
bar pushBack (2).  
bar pushBack (3).  
   
;; Accessing values.  
println: foo nth (2). ;; 3  
   
;; Mutating values.  
foo nth (1) = 99.  
println: foo. ;; [1, 99, 3]  
   
;; Appending to either the front or the back of the array.  
foo pushBack ("back").  
foo pushFront ("front").  
println: foo. ;; ["front", 1, 99, 3, "back"]  
   
;; Popping from the front or back.  
println: foo popBack. ;; "back"  
println: foo popBack. ;; 3  
println: foo popFront. ;; "front"  
println: foo. ;; [1, 99]

## [LFE](http://rosettacode.org/wiki/Category:LFE "Category:LFE")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=128 "Edit section: LFE")]

Using the LFE REPL, you can explore arrays in the following manner:

   
; Create a fixed-size array with entries 0-9 set to 'undefined'  
> (set a0 (: array new 10))  
#(array 10 0 undefined 10)  
> (: array size a0)  
10  
   
; Create an extendible array and set entry 17 to 'true',  
; causing the array to grow automatically  
> (set a1 (: array set 17 'true (: array new)))  
#(array  
  18  
  ...  
(: array size a1)  
18  
   
; Read back a stored value  
> (: array get 17 a1)  
true  
   
; Accessing an unset entry returns the default value  
> (: array get 3 a1)  
undefined  
   
; Accessing an entry beyond the last set entry also returns the  
; default value, if the array does not have fixed size  
> (: array get 18 a1)  
undefined  
   
; "sparse" functions ignore default-valued entries  
> (set a2 (: array set 4 'false a1))  
#(array  
  18  
  ...  
> (: array sparse_to_orddict a2)  
(#(4 false) #(17 true))  
   
; An extendible array can be made fixed-size later  
> (set a3 (: array fix a2))  
#(array  
  18  
  ...  
   
; A fixed-size array does not grow automatically and does not  
; allow accesses beyond the last set entry  
> (: array set 18 'true a3)  
exception error: badarg  
  in (array set 3)  
   
> (: array get 18 a3)  
exception error: badarg  
  in (array get 2)  
   
 

  

## [Liberty BASIC](http://rosettacode.org/wiki/Category:Liberty_BASIC "Category:Liberty BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=129 "Edit section: Liberty BASIC")]

Arrays of less than 10 terms need not be dimensioned.  
Arrays may only be 1D or 2D.  
An empty numeric array term returns '0'. Empty string array terms ="".  
'redim'ming allows the array size to be extended, but all existing values are lost.  
DATA is READ into variables. It cannot be READ directly into arrays.  
To fill arrays with DATA items, first READ the item into a variable, then use that variable to fill an index of the array.

   
dim Array(10)  
   
Array(0) = -1  
Array(10) =  1  
   
print Array( 0), Array( 10)  
   
REDIM Array( 100)  
   
print Array( 0), Array( 10)  
   
Array( 0) = -1  
print Array( 0), Array( 10)  
   
 

## [LIL](http://rosettacode.org/wiki/Category:LIL "Category:LIL")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=130 "Edit section: LIL")]

LIL, like Tcl, doesn't manage arrays as such. Indexed lists are used in LIL. The  **list**  command creates a list from the remaining arguments in the statement. The  **index LIST NUM**  command returns the NUM'th item in the list, starting from zero. Lists are copied on assignment. The array-ish functions and operators would be

-   index LIST NUM, returning the NUM'th item
-   count LIST, returning the number of items in the list
-   indexof LIST VAL, returning the offset from zero position of where VAL is found in LIST, or an empty string
-   filter VARNAME LIST EXPRESSION, returning a new list of filtered items matching EXPRESSION, with the value under test in VARNAME.
-   list ..., creating a list from remaining word tokens in the statement.
-   append LIST VAL (list VAL values are appended as single items to the given LIST)
-   slice LIST FROM-NUM TO-NUM
-   foreach VARNAME LIST CODE
-   charat STRING NUM, indexing a string for characters
-   codeat STRING NUM, indexing a string for the character byte code
-   lmap LIST VARNAME..., maps the list items to the given variable names, in the order given.

For filter and foreach, the VARNAME fields are optional, LIL creates defaults inside the code block of  **x**  for filter and  **i**  for foreach if user names are not given.

# (not) Arrays, in LIL  
set a [list abc def ghi]  
set b [list 4 5 6]  
print [index $a 0]  
print [index $b 1]  
print [count $a]  
append b [list 7 8 9]  
print [count $b]  
print $b

Output:

prompt$ lil arrays.lil
abc
5
3
4
4 5 6 {7 8 9}

By and large, LIL is NOT an array processing language; LIL is a Little Interpreted Language built to deal with strings, commands, and substitutions.

If need arose for tight array processing, LIL is very easy to embed in C applications and extend with new functions that run at speed. If need arises. LIL is small enough, under 4K of source lines, total, that adding extra commands for LIL scripting using C code is quite approachable. If a developer is more comfortable in Pascal, fplil.pas is only 86K characters of source.

## [Lingo](http://rosettacode.org/wiki/Category:Lingo "Category:Lingo")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=131 "Edit section: Lingo")]

a = [1,2] -- or: a = list(1,2)  
put a[2] -- or: put a.getAt(2)  
-- 2  
a.append(3)  
put a  
-- [1, 2, 3]  
a.deleteAt(2)  
put a  
-- [1, 3]  
a[1] = 5 -- or: a.setAt(1, 5)  
put a  
-- [5, 3]  
a.sort()  
put a  
-- [3, 5]

In addition to the 'list' type shown above, for arrays of bytes (i.e. integers between 0 and 255) there is also the bytearray data type:

ba = bytearray(2, 255) -- initialized with size 2 and filled with 0xff  
put ba  
-- <ByteArrayObject length = 2 ByteArray = 0xff, 0xff >  
ba[1] = 1  
ba[2] = 2  
ba[ba.length+1] = 3 -- dynamically increases size  
put ba  
-- <ByteArrayObject length = 3 ByteArray = 0x1, 0x2, 0x3 >  
ba[1] = 5  
put ba  
-- <ByteArrayObject length = 3 ByteArray = 0x5, 0x2, 0x3 >

## [Lisaac](http://rosettacode.org/wiki/Category:Lisaac "Category:Lisaac")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=132 "Edit section: Lisaac")]

+ a : ARRAY(INTEGER);  
a := ARRAY(INTEGER).create 0 to 9;  
a.put 1 to 0;  
a.put 3 to 1;  
a.item(1).print;

## [Little](http://rosettacode.org/wiki/Category:Little "Category:Little")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=133 "Edit section: Little")]

Arrays in Little are list of values of the same type and they grow dynamically.

String fruit[] = {"apple", "orange", "Pear"}

They are zero-indexed. You can use END to get the last element of an array:

   
[puts](https://www.opengroup.org/onlinepubs/009695399/functions/puts.html)(fruit[0]);  
[puts](https://www.opengroup.org/onlinepubs/009695399/functions/puts.html)(fruit[1]);  
[puts](https://www.opengroup.org/onlinepubs/009695399/functions/puts.html)(fruit[END]);  
fruit[END+1] = "banana";

## [Logo](http://rosettacode.org/wiki/Category:Logo "Category:Logo")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=134 "Edit section: Logo")]

array 5      ; default origin is 1, every item is empty  
(array 5 0)  ; custom origin  
make "a {1 2 3 4 5}  ; array literal  
setitem 1 :a "ten       ; Logo is dynamic; arrays can contain different types  
print item 1 :a   ; ten

## [LSE64](http://rosettacode.org/wiki/Category:LSE64 "Category:LSE64")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=135 "Edit section: LSE64")]

10 myArray :array  
0 array 5 [] !      # store 0 at the sixth cell in the array  
array 5 [] @     # contents of sixth cell in array

## [LSL](http://rosettacode.org/wiki/Category:LSL "Category:LSL")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=136 "Edit section: LSL")]

LSL does not have Arrays, but it does have  [lists](http://wiki.secondlife.com/wiki/List)  which can function similar to a one dimensional ArrayList in Java or C#.

   
default {  
    state_entry() {  
        list lst = ["1", "2", "3"];  
        llSay(0, "Create and Initialize a List\nList=["+llList2CSV(lst)+"]\n");  
   
        lst += ["A", "B", "C"];  
        llSay(0, "Append to List\nList=["+llList2CSV(lst)+"]\n");  
   
        lst = llListInsertList(lst, ["4", "5", "6"], 3);  
        llSay(0, "List Insertion\nList=["+llList2CSV(lst)+"]\n");  
   
        lst = llListReplaceList(lst, ["a", "b", "c"], 3, 5);  
        llSay(0, "Replace a portion of a list\nList=["+llList2CSV(lst)+"]\n");  
   
        lst = llListRandomize(lst, 1);  
        llSay(0, "Randomize a List\nList=["+llList2CSV(lst)+"]\n");  
   
        lst = llListSort(lst, 1, TRUE);  
        llSay(0, "Sort a List\nList=["+llList2CSV(lst)+"]\n");  
   
        lst = [1, 2.0, "string", (key)NULL_KEY, ZERO_VECTOR, ZERO_ROTATION];  
        string sCSV = llList2CSV(lst);  
        llSay(0, "Serialize a List of different datatypes to a string\n(integer, float, string, key, vector, rotation)\nCSV=\""+sCSV+"\"\n");  
   
        lst = llCSV2List(sCSV);  
        llSay(0, "Deserialize a string CSV List\n(note that all elements are now string datatype)\nList=["+llList2CSV(lst)+"]\n");  
    }  
}

Output:

Create and Initialize a List
List=[1, 2, 3]

Append to List
List=[1, 2, 3, A, B, C]

List Insertion
List=[1, 2, 3, 4, 5, 6, A, B, C]

Replace a portion of a list
List=[1, 2, 3, a, b, c, A, B, C]

Randomize a List
List=[2, 3, B, a, A, b, C, c, 1]

Sort a List
List=[1, 2, 3, a, A, b, B, c, C]

Serialize a List of different datatypes to a string
(integer, float, string, key, vector, rotation)
CSV="1, 2.000000, string, 00000000-0000-0000-0000-000000000000, <0.00000, 0.00000, 0.00000>, <0.00000, 0.00000, 0.00000, 1.00000>"

Deserialize a string CSV List
(note that all elements are now string datatype)
List=[1, 2.000000, string, 00000000-0000-0000-0000-000000000000, <0.00000, 0.00000, 0.00000>, <0.00000, 0.00000, 0.00000, 1.00000>]

## [Lua](http://rosettacode.org/wiki/Category:Lua "Category:Lua")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=137 "Edit section: Lua")]

Lua does not differentiate between arrays, lists, sets, dictionaries, maps, etc. It supports only one container: Table. Using Lua's simple yet powerful syntax, any of these containers can be emulated. All tables are dynamic. If a static array is necessary, that behavior can be created.

l = {}  
l[1] = 1      -- Index starts with 1, not 0.  
l[0] = 'zero' -- But you can use 0 if you want  
l[10] = 2     -- Indexes need not be continuous  
l.a = 3       -- Treated as l['a']. Any object can be used as index  
l[l] = l      -- Again, any object can be used as an index. Even other tables  
for i,v in next,l do print (i,v) end

## [M2000 Interpreter](http://rosettacode.org/wiki/Category:M2000_Interpreter "Category:M2000 Interpreter")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=138 "Edit section: M2000 Interpreter")]

Here present Arrays of type variant (can be any type, object, pointer to object), and arrays of structures (unsigned numbers plus double and single, and strings including pointers to BSTR). We can copy multiple items from an array to another array (ore the same) with statement Stock. We can copy from memory to strings and place them to other address.

   
Module CheckArray {  
      \\ Array with parenthesis in name  
      Dim A(10)=1  
      Global B(10)=1  
      For This {  
            Local A(10)=5  
            Print A(4)=5  
      }  
      Print A(4)=1  
   
      \\ Auto Array  
      M=(1,2,3,4,5)  
      Link M to M()  
      Print M(2)=3  
      Return M, 0:=100, 5-4:=300  
   
      \\ Retrieve an Element of an Array  
      k=Each(M, 1, 2)  
      \\ print 100 300  
      While k { Print Array(k),}  
      Print   
      Print Array(M, 2)=3  
      Print Array("M", 2)=3  
      Print Array(B(), 1)=1  
      \\ arrays are containers for every value/object/pointer  
      B(0):="Hello",100,"Good Morning", 200  
      \\ using set to make B$() global too  
      Set Link B() to B$()  
      Print B$(0), B(1), B$(2), B(3)  
      Swap B(0), B(2)  
      Swap B(1), B(3)  
      Print B$(0), B(1), B$(2), B(3)  
      Print B()  
      \\ Reduce B() to 4 elements - and change dimensions  
      \\ we have to redim the global array, using set to send line to console  
      \\ all globals are part of level 0, at console input.  
      Set Dim B(4)  
      Module CheckGlobal {  
            Print B$(0), B(1), B$(2), B(3)  
      }  
      CheckGlobal  
      Print B()  
      Dim BB(4)  
      \\ Copy 4 items from B() to BB(), from B(0), to BB(0)  
      Stock B(0) keep 4, BB(0)  
      Link BB() to BB$()  
      Print BB$(0), BB(1), BB$(2), BB(3)  
      \\ Arrays of structures in Buffers  
   
      Structure TwoByte {  
            {  
                  ab as integer  
            }  
            a as byte  
            b as byte  
      }  
      Print Len(TwoByte) = 2  
      \ Use clear to clear memory  
      \\ Mem is a pointer to a Buffer object  
      Buffer Clear Mem as TwoByte*20  
      Print Len(Mem)=40  
      Return Mem, 0!ab:=0xFFAA  
      Print Eval(Mem, 0!a)=0xAA,  Eval(Mem, 0!b)=0xFF  
      Return Mem, 0!b:=0xF2  
      Hex Eval(Mem,0!ab)   ' print 0xF2AA  
      \\ Redim with preserve  
      Buffer Mem as TwoByte*40  
      \\ copy 40 bytes  at index 20 (40 bytes from start)  
      Return Mem, 20:=Eval$(Mem, 0, 20*2)  
      Hex Eval(Mem,20!ab)   ' print 0xF2AA  
      A(3)=Mem  
      Hex Eval(A(3),20!ab)   ' print 0xF2AA  
      \\ now Mem change pointer  
      Clear Mem  
      Print Len(Mem)  
      \\ old Mem is in A(3)  
      Hex Eval(A(3),20!ab)   ' print 0xF2AA  
      \\ we can change   
      Buffer Clear Mem as Integer * 200  
      Print Len(Mem)=400  
      Return Mem, 0:=Eval$(A(3), 0, 80)  
      Hex Eval(Mem,20)   ' print 0xF2AA  
      \\ change type without use of clear  
      Buffer Mem as TwoByte * 200  
      Hex Eval(Mem,20!ab)   ' print 0xF2AA  
}  
CheckArray  
 

### Passing Arrays By Reference[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=139 "Edit section: Passing Arrays By Reference")]

By default arrays passed by value. Here in make() we read reference in a variable A, which interpreter put then pointer to array, so it is a kind of reference (like in C). Using & we have normal reference. A ++ operator in a pointer of array add one to each element.

   
Dim a(10)=1  
Print a()  ' 1 1 1 1 1 1 1 1 1 1  
make(a())  
Print a()  ' 2 2 2 2 2 2 2 2 2 2  
make2(&a())  
Print a()  ' 3 3 3 3 3 3 3 3 3 3  
Sub make(A)  
      A++  
End Sub  
Sub make2(&a())  
      A=A()  
      A++  
End Sub  
 

## [Maple](http://rosettacode.org/wiki/Category:Maple "Category:Maple")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=140 "Edit section: Maple")]

#defining an array of a certain length  
a := Array (1..5);  
                                 a := [ 0 0 0 0 0 ]  
#can also define with a list of entries  
a := Array ([1, 2, 3, 4, 5]);  
                                a := [ 1 2 3 4 5 ]  
a[1] := 9;  
a  
                                a[1] := 9  
                                [ 9 2 3 4 5 ]  
a[5];  
                                5  
#can only grow arrays using ()  
a(6) := 6;  
                                a := [ 9 2 3 4 5 6 ]  
a[7] := 7;  
Error, Array index out of range

## [Mathematica](http://rosettacode.org/wiki/Category:Mathematica "Category:Mathematica")  /  [Wolfram Language](http://rosettacode.org/wiki/Category:Wolfram_Language "Category:Wolfram Language")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=141 "Edit section: Mathematica / Wolfram Language")]

a = Array[Sin, 10]  
a[[1]]  
Delete[a, 2]

Output:

{Sin[1],Sin[2],Sin[3],Sin[4],Sin[5],Sin[6],Sin[7],Sin[8],Sin[9],Sin[10]}
Sin[1]
{Sin[1],Sin[3],Sin[4],Sin[5],Sin[6],Sin[7],Sin[8],Sin[9],Sin[10]}

## [MATLAB](http://rosettacode.org/wiki/Category:MATLAB "Category:MATLAB")  /  [Octave](http://rosettacode.org/wiki/Category:Octave "Category:Octave")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=142 "Edit section: MATLAB / Octave")]

Variables are not typed until they are initialized. So, if you want to create an array you simply assign a variable name the value of an array. Also, memory is managed by MATLAB so an array can be expanded, resized, and have elements deleted without the user dealing with memory. Array elements can be retrieved in two ways. The first way is to input the row and column indicies of the desired elements. The second way is to input the subscript of the array elements.

>> a = [1 2 35] %Declaring a vector (i.e. one-dimensional array)  
   
a =  
   
     1     2    35  
   
>> a = [1 2 35;5 7 9] % Declaring a matrix (i.e. two-dimensional array)  
   
a =  
   
     1     2    35  
     5     7     9  
   
>> a3 = [reshape](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/reshape.html)(1:2*3*4,[2,3,4]);   % declaring a three-dimensional array of size 2x3x4  
   
a3 =  
   
[ans](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/ans.html)(:,:,1) =  
   
   1   3   5  
   2   4   6  
   
[ans](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/ans.html)(:,:,2) =  
   
    7    9   11  
    8   10   12  
   
[ans](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/ans.html)(:,:,3) =  
   
   13   15   17  
   14   16   18  
   
[ans](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/ans.html)(:,:,4) =  
   
   19   21   23  
   20   22   24  
   
   
>> a(2,3) %Retrieving value using row and column indicies  
   
     9  
   
>> a(6) %Retrieving value using array subscript  
   
[ans](https://www.mathworks.com/access/helpdesk/help/techdoc/ref/ans.html) =  
   
     9  
   
>> a = [a [10;42]] %Added a column vector to the array  
   
a =  
   
     1     2    35    10  
     5     7     9    42  
   
>> a(:,1) = [] %Deleting array elements  
   
a =  
   
     2    35    10  
     7     9    42

## [Maxima](http://rosettacode.org/wiki/Category:Maxima "Category:Maxima")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=143 "Edit section: Maxima")]

/* Declare an array, subscripts run from 0 to max value */  
array(a, flonum, 20, 20, 3)$  
   
arrayinfo(a);  
/* [complete, 3, [20, 20, 3]] */  
   
a[0, 0]: 1.0;  
   
listarray(a);  
/* [1.0, 0.0, 0.0, ..., 0.0] */  
   
/* Show all declared arrays */  
arrays;  
/* [a] */  
   
   
/* One may also use an array without declaring it, it's a hashed array */  
b[1]: 1000;  
b['x]: 3/4; /* hashed array may have any subscript */  
   
arrayinfo(b);  
/* [hashed, 1, [1], [x]] */  
   
listarray(b);  
/* [1000, 3/4] */

## [Mercury](http://rosettacode.org/wiki/Category:Mercury "Category:Mercury")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=144 "Edit section: Mercury")]

Mercury's arrays are a mutable non-functional type, and therefore are slightly more troublesome than functional types to A) accept as parameters to predicates, and B) involve in higher-order code, and C) include as a member of a composite data type. All of this is still very possible, but it requires an understanding of Mercury's variable instantiation system, as you can't just have 'in' and 'out' modes for parameters that involve arrays. Mercury has a 'bt_array' module with performance characteristics very similar to that of arrays, but which is a functional type and therefore is easier to work with. Especially if you're just starting out with Mercury, going with bt_array can be a big win for 'whippitupitude'.

:- module array_example.  
:- interface.  
:- import_module io.  
:- pred main(io::di, io::uo) is det.  
:- implementation.  
:- import_module array, int.  
:- use_module exception.  
   
:- type example_error ---> impossible.  
   
main(!IO) :-  
    some [!A] ( % needed to introduce a state variable not present in the head  
        % Create an array(int) of length 10, with initial values of 0  
        array.init(10, 0, !:A),  
   
        % create an empty array (with no initial value)  
        % since the created array is never used, type inference can't tell what  
        % kind of array it is, and there's an unresolved polymorphism warning.  
        array.make_empty_array(_Empty),  
   
        % resize our first array, so that we can then set its 17th member  
        % new values are set to -1  
        array.resize(20, -1, !A),  
        !A ^ elem(17) := 5,  
   
        % Mercury data structures tend to have deterministic (exception thrown  
        % on error), semideterministic (logical failure on error), and unsafe  
        % (undefined behavior on error) access methods.  
        array.lookup(!.A, 5, _), % det  
        ( if array.semidet_lookup(!.A, 100, _) then  % semidet  
            exception.throw(impossible)  
        else  
            true  
        ),  
        array.unsafe_lookup(!.A, 5, _), % could cause a segfault on a smaller array  
   
        % output: array([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, 5, -1, -1])  
        io.print_line(!.A, !IO),  
   
        plusminus(2, 0, !A),  
   
        % output: array([2, -2, 2, -2, 2, -2, 2, -2, 2, -2, 1, -3, 1, -3, 1, -3, 1, 3, 1, -3])  
        io.print_line(!.A, !IO)  
    ).  
   
    % Sample predicate operating on an array.  
    % Note the array_* modes instead of in/out.  
:- pred plusminus(int, int, array(int), array(int)).  
:- mode plusminus(in, in, array_di, array_uo) is det.  
plusminus(N, I, !A) :-  
    ( if array.semidet_lookup(!.A, I, X) then  
        !A ^ unsafe_elem(I) := X + N,  
        plusminus(-N, I+1, !A)  
    else  
        true  
    ).

## [MIPS Assembly](http://rosettacode.org/wiki/Category:MIPS_Assembly "Category:MIPS Assembly")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=145 "Edit section: MIPS Assembly")]

   
	.data  
array:	.word	1, 2, 3, 4, 5, 6, 7, 8, 9 # creates an array of 9 32 Bit words.   
   
	.text  
main:	la 	$s0, array  
	li	$s1, 25  
	sw	$s1, 4($s0)	# writes $s1 (25) in the second array element  
# the four counts thi bytes after the beginning of the address. 1 word = 4 bytes, so 4 acesses the second element   
   
	lw	$s2, 20($s0)	# $s2 now contains 6	  
   
	li	$v0, 10		# end program  
	syscall  
 

## [MiniScript](http://rosettacode.org/wiki/Category:MiniScript "Category:MiniScript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=146 "Edit section: MiniScript")]

Lists and arrays are synonymous in MiniScript.

Operations:

+	list concatenation
*	replication (i.e. repeat the list some number of times)
/	division (get some fraction of a list)
==, !=	comparison (for equality)
[i]	get/set item i (first item is 0)
[i:j]	get sublist ("slice") from i up to j

Slicing:

x = ["a", 42, 3.14, 7, "hike"]
x[0]	"a" (first item)
x[1]	42 (second item)
x[-1]	"hike" (last item)
x[-2]	7 (next-to-last item)
x[1:-1]	[42, 3.14, 7] (everything from the second up to the last item)
x[1:]	[42, 3.14, 7, "hike"] (everything from the second item to the end)
x[:-1]	["a", 42, 3.14, 7] (everything up to the last item)

Example:

arr = ["a", 1, 3]  
print arr[0]  
   
arr.push "x"  
print arr.pop

## [Modula-2](http://rosettacode.org/wiki/Category:Modula-2 "Category:Modula-2")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=147 "Edit section: Modula-2")]

Same as described for Modula-3

## [Modula-3](http://rosettacode.org/wiki/Category:Modula-3 "Category:Modula-3")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=148 "Edit section: Modula-3")]

VAR a: ARRAY [1..10] OF INTEGER;

Defines an array of 10 elements, indexed 1 through 10.

Arrays can also be given initial values:

VAR a := ARRAY [1..10] OF INTEGER {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};  
VAR arr1 := ARRAY [1..10] OF INTEGER {1, ..} (* Initialize all elements to 1. *)

To retrieve an element:

VAR arr := ARRAY [1..3] OF INTEGER {1, 2, 3};  
VAR myVar := a[2];

To assign a value to an element:

VAR arr := ARRAY [1..3] OF INTEGER;  
arr[1] := 10;

## [Monte](http://rosettacode.org/wiki/Category:Monte "Category:Monte")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=149 "Edit section: Monte")]

var myArray := ['a', 'b', 'c','d']

To retrieve a value:

traceln(myArray[0])

To change a value:

myArray := myArray.with(3, 'z')

Now myArray is ['a','b','c','z'].

## [Neko](http://rosettacode.org/wiki/Category:Neko "Category:Neko")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=150 "Edit section: Neko")]

var myArray = $array(1);  
   
$print(myArray[0]);

Output:

1

## [Nemerle](http://rosettacode.org/wiki/Category:Nemerle "Category:Nemerle")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=151 "Edit section: Nemerle")]

using System;  
using System.Console;  
using System.Collections;  
   
module ArrayOps  
{  
    Main() : void  
    {  
        def fives = array(10);  
        foreach (i in [1 .. 10]) fives[i - 1] = i * 5;  
        def ten = fives[1];  
        WriteLine($"Ten: $ten");  
   
        def dynamic = ArrayList();  
        dynamic.Add(1);  
        dynamic.Add(3);  
        dynamic[1] = 2;  
        foreach (i in dynamic) Write($"$i\t"); // Nemerle isn't great about displaying arrays, it's better with lists though  
    }  
}

## [NetRexx](http://rosettacode.org/wiki/Category:NetRexx "Category:NetRexx")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=152 "Edit section: NetRexx")]

**Note:**  Dynamic arrays can be simulated via the  [Java](http://rosettacode.org/wiki/Java "Java")  [Collections Framework](https://download.oracle.com/javase/6/docs/technotes/guides/collections/index.html)  or by using  [NetRexx](http://rosettacode.org/wiki/NetRexx "NetRexx")  _indexed strings_  (AKA:  [associative arrays](http://rosettacode.org/wiki/Creating_an_Associative_Array "Creating an Associative Array")).

/* NetRexx */  
options replace format comments java crossref symbols nobinary  
   
array = int[10]  
array[0] = 42  
   
say array[0] array[3]  
say  
   
words = ['Ogof', 'Ffynnon', 'Ddu']  
   
say words[0] words[1] words[2]  
say  
   
-- Dynamic arrays can be simulated via the Java Collections package  
splk = ArrayList()  
splk.add(words[0])  
splk.add(words[1])  
splk.add(words[2])  
splk.add('Draenen')  
   
say splk.get(0) splk.get(3)  
say splk.get(0) splk.get(1) splk.get(2)  
say  
   
-- or by using NetRexx "indexed strings" (associative arrays)  
cymru = ''  
cymru[0] = 0  
cymru[0] = cymru[0] + 1; cymru[cymru[0]] = splk.get(0) splk.get(1) splk.get(2)  
cymru[0] = cymru[0] + 1; cymru[cymru[0]] = splk.get(0) splk.get(3)  
   
loop x_ = 1 to cymru[0] by 1  
  say x_':' cymru[x_]  
  end x_

Output:

42 0

Ogof Ffynnon Ddu

Ogof Draenen
Ogof Ffynnon Ddu

1: Ogof Ffynnon Ddu
2: Ogof Draenen

## [NewLISP](http://rosettacode.org/wiki/Category:NewLISP "Category:NewLISP")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=153 "Edit section: NewLISP")]

This creates an array of 5 elements, initialized to  `nil`:

(array 5)  
→ (nil nil nil nil nil)

The example below creates a multi-dimensional array (a 3-element array of 4-element arrays), initialized using the values returned by the function sequence (a list containing whole numbers from 1 to 12) and stores the newly created array in a variable called myarray. The return value of the set function is the array.

(set 'myarray (array 3 4 (sequence 1 12)))  
→ ((1 2 3 4) (5 6 7 8) (9 10 11 12))

## [Nim](http://rosettacode.org/wiki/Category:Nim "Category:Nim")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=154 "Edit section: Nim")]

var # fixed size arrays  
  x = [1,2,3,4,5,6,7,8,9,10] # type and size automatically inferred  
  y: array[1..5, int] = [1,2,3,4,5] # starts at 1 instead of 0  
  z: array['a'..'z', int] # indexed using characters  
   
x[0] = x[1] + 1  
echo x[0]  
echo z['d']  
   
x[7..9] = y[3..5] # copy part of array  
   
var # variable size sequences  
  a = @[1,2,3,4,5,6,7,8,9,10]  
  b: seq[int] = @[1,2,3,4,5]  
   
a[0] = a[1] + 1  
echo a[0]  
   
a.add(b) # append another sequence  
a.add(200) # append another element  
echo a.pop() # pop last item, removing and returning it  
echo a

## [NS-HUBASIC](http://rosettacode.org/wiki/Category:NS-HUBASIC "Category:NS-HUBASIC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=155 "Edit section: NS-HUBASIC")]

10 DIM A(1)  
20 A(1)=10  
30 PRINT A(1)

## [NSIS](http://rosettacode.org/wiki/Category:NSIS "Category:NSIS")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=156 "Edit section: NSIS")]

**Library:**  [NSISArray](http://rosettacode.org/mw/index.php?title=Category:NSISArray&action=edit&redlink=1 "Category:NSISArray (page does not exist)")

NSIS does not have native support for arrays. Array support is provided by the  [NSISArray](http://nsis.sourceforge.net/Arrays_in_NSIS)  plugin.

   
!include NSISArray.nsh  
Function ArrayTest  
	Push $0  
	; Declaring an array  
	NSISArray::New TestArray 1 2  
	NSISArray::Push TestArray "Hello"  
	; NSISArray arrays are dynamic by default.  
	NSISArray::Push TestArray "World"  
	NSISArray::Read TestArray 1  
	Pop $0  
	DetailPrint $0  
	Pop $0  
FunctionEnd  
 

## [Oberon-2](http://rosettacode.org/wiki/Category:Oberon-2 "Category:Oberon-2")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=157 "Edit section: Oberon-2")]

   
MODULE Arrays;  
IMPORT   
  Out;  
   
  PROCEDURE Static;  
  VAR  
    x: ARRAY 5 OF LONGINT;  
  BEGIN  
    x[0] := 10;  
    x[1] := 11;  
    x[2] := 12;  
    x[3] := 13;  
    x[4] := x[0];  
   
    Out.String("Static at 4: ");Out.LongInt(x[4],0);Out.Ln;  
  END Static;  
   
  PROCEDURE Dynamic;  
  VAR  
    x: POINTER TO ARRAY OF LONGINT;  
  BEGIN  
    NEW(x,5);  
   
    x[0] := 10;  
    x[1] := 11;  
    x[2] := 12;  
    x[3] := 13;  
    x[4] := x[0];  
   
    Out.String("Dynamic at 4: ");Out.LongInt(x[4],0);Out.Ln;  
  END Dynamic;  
   
BEGIN  
  Static;  
  Dynamic  
END Arrays.  
 

## [Objeck](http://rosettacode.org/wiki/Category:Objeck "Category:Objeck")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=158 "Edit section: Objeck")]

   
bundle Default {  
  class Arithmetic {  
    function : Main(args : System.String[]), Nil {  
      array := Int->New[2];  
      array[0] := 13;  
      array[1] := 7;  
      (array[0] + array[1])->PrintLine();  
    }  
  }  
}  
 

## [Objective-C](http://rosettacode.org/wiki/Category:Objective-C "Category:Objective-C")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=159 "Edit section: Objective-C")]

// NSArrays are ordered collections of NSObject subclasses only.  
   
// Create an array of NSString objects.  
[NSArray](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSArray_Class/) *firstArray = [[[NSArray](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSArray_Class/) alloc] initWithObjects:@"Hewey", @"Louie", @"Dewey", nil];  
   
// NSArrays are immutable; it does have a mutable subclass, however - NSMutableArray.  
// Let's instantiate one with a mutable copy of our array.  
// We can do this by sending our first array a -mutableCopy message.  
[NSMutableArray](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSMutableArray_Class/) *secondArray = [firstArray mutableCopy];  
   
// Replace Louie with Launchpad McQuack.  
[secondArray replaceObjectAtIndex:1 withObject:@"Launchpad"];  
   
// Display the first object in the array.  
NSLog(@"%@", [secondArray objectAtIndex:0]);  
   
// In non-ARC or non-GC environments, retained objects must be released later.  
[firstArray release];  
[secondArray release];  
   
// There is also a modern syntax which allows convenient creation of autoreleased immutable arrays.  
// No nil termination is then needed.  
[NSArray](https://developer.apple.com/documentation/Cocoa/Reference/Foundation/Classes/NSArray_Class/) *thirdArray = @[ @"Hewey", @"Louie", @"Dewey", @1, @2, @3 ];  
 

## [OCaml](http://rosettacode.org/wiki/Category:OCaml "Category:OCaml")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=160 "Edit section: OCaml")]

in the toplevel:

# [Array](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Array.html).make 6 'A' ;;  
- : [char](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEchar) [array](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEarray) = [|'A'; 'A'; 'A'; 'A'; 'A'; 'A'|]  
   
# [Array](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Array.html).init 8 (fun i -> i * 10) ;;  
- : [int](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEint) [array](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEarray) = [|0; 10; 20; 30; 40; 50; 60; 70|]  
   
# let arr = [|0; 1; 2; 3; 4; 5; 6 |] ;;  
val arr : [int](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEint) [array](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEarray) = [|0; 1; 2; 3; 4; 5; 6|]  
   
# arr.(4) ;;  
- : [int](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEint) = 4  
   
# arr.(4) <- 65 ;;  
- : [unit](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEunit) = ()  
   
# arr ;;  
- : [int](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEint) [array](http://caml.inria.fr/pub/docs/manual-ocaml/libref/Pervasives.html#TYPEarray) = [|0; 1; 2; 3; 65; 5; 6|]

## [Oforth](http://rosettacode.org/wiki/Category:Oforth "Category:Oforth")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=161 "Edit section: Oforth")]

Array created with [ ... ] are immutable array. To create a mutable array, #new is used.

[ "abd", "def", "ghi" ] at( 3 ) .  
   
Array new dup addAll( [1, 2, 3] ) dup put( 2, 8.1 ) .  
 

Output:

ghi
[1, 8.1, 3]

## [Ol](http://rosettacode.org/wiki/Category:Ol "Category:Ol")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=162 "Edit section: Ol")]

Ol provides arrays in the form of smart objects named vectors.

Vectors are heterogeneous structures whose elements are indexed by integers. A vector typically occupies less space than a list of the same length, and the average time needed to access a randomly chosen element is typically less for the vector than for the list.

The length of a vector is the number of elements that it contains. This number is a non-negative integer that is fixed when the vector is created. The valid indexes of a vector are the exact non-negative integers less than the length of the vector. The first element in a vector is indexed by one, and the last element is indexed by length of the vector.

   
; making an array  
#(1 2 3 4 5)  
   
; making an empty array  
#()  
#0  
   
; making n-length array with undefined values (actually, #false)  
(make-array 5)  
   
; making n-length array with default value  
(make-array 5 0)  
   
; getting n-th element of array  
(ref array 1)  
 

## [ooRexx](http://rosettacode.org/wiki/Category:OoRexx "Category:OoRexx")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=163 "Edit section: ooRexx")]

ooRexx arrays hold object references. Arrays will automatically increase in size if needed.

   a = .array~new       -- create a zero element array   
   b = .array~new(10)   -- create an array with initial size of 10   
   c = .array~of(1, 2, 3)  -- creates a 3 element array holding objects 1, 2, and 3  
   a[3] = "Fred"        -- assign an item   
   b[2] = a[3]          -- retrieve an item from the array   
   c~append(4)          -- adds to end.  c[4] == 4 now

The above Array class supports only one-dimensional arrays (vectors) with positive integer indexes. Much more powerful are stems such as a.i.j where i and j can be any string value. See category REXX for details. ooRexx introduces a notation a.[x,y] where x and y can actually be expressions. This way one can implement one- and multidimensional (associative) arrays. The indexes can be strings containing any characters including blanks. The total length of the stemmed variable (stem and index values separated by periods) must not be longer than 250.

## [OxygenBasic](http://rosettacode.org/wiki/Category:OxygenBasic "Category:OxygenBasic")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=164 "Edit section: OxygenBasic")]

   
   
'CREATING AN ARRAY  
   
float f[100]  
   
'SETTING INDEX BASE  
   
indexbase 1 'default  
   
'FILLING PART OF AN ARRAY  
   
f[20]<=1,2,3,4,5,1.25  
   
'MAPPING AN ARRAY TO ANOTHER  
   
float *g  
@g=@f[20]  
print g[6] 'result 1.25  
   
 

## [Oz](http://rosettacode.org/wiki/Category:Oz "Category:Oz")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=165 "Edit section: Oz")]

declare  
  Arr = {Array.new 1   %% lowest index  
                   10  %% highest index  
                   37} %% all 10 fields initialized to 37  
in  
  {Show Arr.1}  
  Arr.1 := 64  
  {Show Arr.1}

## [PARI/GP](http://rosettacode.org/wiki/Category:PARI/GP "Category:PARI/GP")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=166 "Edit section: PARI/GP")]

v=[];  
v=concat(v,7);  
v[1]

## [Pascal](http://rosettacode.org/wiki/Category:Pascal "Category:Pascal")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=167 "Edit section: Pascal")]

A modification of the Delphi example:

   
Program ArrayDemo;  
uses  
  SysUtils;  
var  
  StaticArray: array[0..9] of Integer;  
  DynamicArray: array of Integer;  
  StaticArrayText,  
  DynamicArrayText: string;  
  lcv: Integer;  
begin  
  // Setting the length of the dynamic array the same as the static one  
  SetLength(DynamicArray, Length(StaticArray));  
  // Asking random numbers storing into the static array  
  for lcv := 0 to Pred(Length(StaticArray)) do  
  begin  
    write('Enter a integer random number for position ', Succ(lcv), ': ');  
    readln(StaticArray[lcv]);  
  end;  
  // Storing entered numbers of the static array in reverse order into the dynamic  
  for lcv := 0 to Pred(Length(StaticArray)) do  
    DynamicArray[Pred(Length(DynamicArray)) - lcv] := StaticArray[lcv];  
  // Concatenating the static and dynamic array into a single string variable  
  StaticArrayText := '';  
  DynamicArrayText := '';  
  for lcv := 0 to Pred(Length(StaticArray)) do  
  begin  
    StaticArrayText := StaticArrayText + IntToStr(StaticArray[lcv]) + ' ';  
    DynamicArrayText := DynamicArrayText + IntToStr(DynamicArray[lcv]) + ' ';  
  end;  
  // Displaying both arrays  
  writeln(StaticArrayText);  
  writeln(DynamicArrayText);  
end.  
 

## [Perl](http://rosettacode.org/wiki/Category:Perl "Category:Perl")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=168 "Edit section: Perl")]

In-line

 my @empty;  
 my @empty_too = ();  
   
 my @populated   = ('This', 'That', 'And', 'The', 'Other');  
 [print](https://perldoc.perl.org/functions/print.html) $populated[2];  # And  
   
 my $aref = ['This', 'That', 'And', 'The', 'Other'];  
 [print](https://perldoc.perl.org/functions/print.html) $aref->[2];  # And  
 

Dynamic

my @arr;  
   
[push](https://perldoc.perl.org/functions/push.html) @arr, 1;  
[push](https://perldoc.perl.org/functions/push.html) @arr, 3;  
   
$arr[0] = 2;  
   
[print](https://perldoc.perl.org/functions/print.html) $arr[0];

Two-dimensional

 my @multi_dimensional = (  
     [0, 1, 2, 3],  
     [[qw](https://perldoc.perl.org/functions/qw.html)(a b c d e f g)],  
     [[qw](https://perldoc.perl.org/functions/qw.html)(! $ % & *)],  
 );  
 

## [Perl 6](http://rosettacode.org/wiki/Category:Perl_6 "Category:Perl 6")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=169 "Edit section: Perl 6")]

At its most basic, an array in Perl 6 is quite similar to an array in Perl 5.

my @arr;  
   
push @arr, 1;  
push @arr, 3;  
   
@arr[0] = 2;  
   
say @arr[0];

### Some further exposition:[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=170 "Edit section: Some further exposition:")]

In Perl 6, arrays have a very specific definition: "A collection of Scalar containers that do the Positional Role." Scalar container means it is mutable and may contain any object; an Integer, a Rational number, a String, another Array, whatever... literally any other object that can be instantiated in Perl 6. The Positional Role means that it uses integer indexing for access. The index must be a positive integer, an expression that evaluates to a positive integer, or something that can be coerced to a positive integer. Arrays are always indexed from 0. The starting index can not be changed.

Arrays are unconstrained by default. They may hold any number of any type of object up to available memory. They do not need to be pre-allocated. Simply assigning (or even referring in some cases) to an index slot is enough to autovivify the container and allocate enough memory to hold the assigned object. Memory will automatically be allocated and will grow and shrink as necessary to hold the values assigned.

Values may be pushed onto the end of an array, popped off of the end, shifted off of the front or unshifted onto the front, and spliced into or out of the interior.

   @array.push: 'value';
   my $value = @array.pop;
   @array.unshift: 'value';
   my $value = @array.shift;
   @array.splice(2,3, <some arbitrary string values>);

Arrays may be constrained to only accept a certain number of objects or only a certain type of object.

   my Int @array; # can only hold Integer objects. Assigning any other type will cause an exception.
   my @array[9];  # can only 10 objects (zero indexed). Trying to assign to an index greater than 9 with cause an exception. 

Arrays are constructed with square brackets, an explicit constructor, or by coercing some other object either explicitly using a coercer or implicitly by simply assigning to an array variable. These are all arrays:

   [1, 2, 3, 4]
   ['a', 'b', 'c', 'd']
   Array.new<this is an array of words>
   ('as', 'is', 'this').Array
   my @implicit = <yep, this too>

Array variables in Perl 6 are variables whose names bear the @ sigil, and are expected to contain some sort of list-like object. Of course, other variables may also contain these objects, but @-sigiled variables always do, and are expected to act the part. Array storage slots are accessed through postcircumfix square bracket notation. Unlike Perl 5, @-sigiled variables are invariant on access, whether you are accessing one slot, many slots, or all of the slots. The first slot in @array is @array[0] not $array[0]. @array and $array are two different unconnected variables.

   @array[1]      # a single value in the 2nd slot
   @array[*-1]    # a single value in the last slot
   @array[1..5]   # an array slice, 2nd through 6th slots
   @array[1,3,7]  # slice, 2nd, 4th and 8th slot
   @array[8,5,2]  # can be in any order
   @array[*]      # all the slots
   @array[]       # all the slots (zen slice)
   @array[^10]    # first 10 slots (upto 10 or 0..9)
   @array.head(5) # first 5 slots
   @array.tail(2) # last two

Multi-dimensioned arrays also use postcircumfix square brackets for access. If the array is not ragged, (every sub array is the same size) you may use semicolon subscripting.

   @array[1][1]   # 2nd item in the second slot
   @array[1;1]    # same thing, implies rectangular (non-ragged) arrays

There are several objects that have an Iterable Role and a PositionalBindFailover Role which makes them act similar to arrays and allows them to be used nearly interchangeably in read-only applications. (Perl 6 is big on duck typing. "If it looks like a duck and quacks like a duck and waddles like a duck, it's a duck.") These constructs are ordered and use integer indexing and are often used in similar circumstances as arrays, however,  **they are immutable**. Values in slots can not be changed. They can not be pushed to, popped from or spliced. They can easily converted to arrays by simply assigning them to an array variable.

**List**: A fixed Iterable collection of immutable values. Lists are constructed similarly to arrays:

   (1, 2, 3, 4)
   ('a', 'b', 'c', 'd')
   List.new(<this is a list of words>)
   ('as', 'is', 'this').List
   my @not-a-list = (<oops, this isn't>)
   my @implicit := (<but this is>) # note the values are bound := not assigned =

**Range**: Iterable list of consecutive numbers or strings with a lower and an upper boundary. (That boundary may be infinite.) Reified on demand.

   2..20    # integers two through twenty
   1..Inf   # natural numbers
   'a'..'z' # lowercase latin letters
   '⁰'..'⁹'  # superscript digits

**Sequence**: Iterable list of objects with some method to determine the next (or previous) item in the list. Reified on demand. Will try to automatically deduce simple arithmetic or geometric sequences. Pass in a code object to calculate more complex sequences.

  0,2,4 ... 64   # even numbers up to 64
  1,2,4 ... 64   # geometric increase
  1,1, *+* ... * # infinite Fibonacci sequence
  1,1,{$^n2 + $^n1 + 1} ... * # infinite Leonardo numbers

Postcircumfix indexing works for any object that has a Positional (or PositionalBindFailover) role, it need not be in a @-sigiled variable, or indeed, in a variable at all.

   [2,4,6,8,10][1]                             # 4 - anonymous array
   <my dog has fleas>[*-2]                     # 'has' - anonymous list
   sub a {(^Inf).grep: *.is-prime}; a()[99];   # 541 - (100th prime) subroutine returning a sequence
   my $lol = ((1,2), (3,4), (5,6)); $lol[1];   # (3 4) - list of lists in a Scalar variable

## [Phix](http://rosettacode.org/wiki/Category:Phix "Category:Phix")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=171 "Edit section: Phix")]

In Phix, sequences are  **it**  - there are no other data structures to learn.

Arrays, multidimensional arrays, lists, stacks, queues, trees, etc. and even character strings can all be easily represented in Phix with sequences. They can grow or shrink without any need to worry about memory management issues.

-- simple one-dimensional arrays:  
sequence s1 = {0.5, 1, 4.7, 9}, -- length(s1) is now 4  
         s2 = repeat(0,6),     -- s2 is {0,0,0,0,0,0}  
         s3 = tagset(5)         -- s3 is {1,2,3,4,5}  
   
    ?s1[3]      -- displays 4.7 (nb 1-based indexing)  
    s1[3] = 0   -- replace that 4.7  
    s1 &= {5,6} -- length(s1) is now 6 ({0.5,1,0,9,5,6})  
    s1 = s1[2..5]   -- length(s1) is now 4 ({1,0,9,5})  
    s1[2..3] = {2,3,4} -- length(s1) is now 5 ({1,2,3,4,5})  
    s1 = append(s1,6)   -- length(s1) is now 6 ({1,2,3,4,5,6})  
    s1 = prepend(s1,0)  -- length(s1) is now 7 ({0,1,2,3,4,5,6})  
   
-- negative subscripts can also be used, counting from the other end, eg  
    s2[-2..-1] = {-2,-1}    -- s2 is now {0,0,0,0,-2,-1}  
   
-- multi dimensional arrays:  
sequence y = {{{1,1},{3,3},{5,5}},  
              {{0,0},{0,1},{9,1}},  
              {{1,7},{1,1},{2,2}}}  
    -- y[2][3][1] is 9  
   
         y = repeat(repeat(repeat(0,2),3),3)  
    -- same structure, but all 0s  
   
-- Array of strings:  
sequence s = {"Hello", "World", "Phix", "", "Last One"}  
    -- s[3] is "Phix"  
    -- s[3][2] is 'h'  
   
-- A Structure:  
sequence employee = {{"John","Smith"},  
                     45000,  
                     27,  
                     185.5}  
   
-- To simplify access to elements within a structure it is good programming style to define constants that name the various fields, eg:  
    constant SALARY = 2  
   
-- Array of structures:  
sequence employees = {  
                      {{"Jane","Adams"}, 47000, 34, 135.5},  -- a[1]  
                      {{"Bill","Jones"}, 57000, 48, 177.2},  -- a[2]  
                      -- .... etc.  
                     }  
    -- employees[2][SALARY] is 57000  
   
-- A tree can be represented easily, for example after adding "b","c","a" to it you might have:  
sequence tree = {{"b",3,2},  
                 {"c",0,0},  
                 {"a",0,0}}  
   
-- ie assuming   
constant ROOT=1, VALUE=1, LEFT=2, RIGHT=3 -- then  
--  tree[ROOT][VALUE] is "b"  
--  tree[ROOT][LEFT] is 3, and tree[3] is the "a"  
--  tree[ROOT][RIGHT] is 2, and tree[2] is the "c"  
   
-- The operations you might use to build such a tree (tests/loops/etc omitted) could be:  
    tree = {}  
    tree = append(tree,{"b",0,0})  
    tree = append(tree,{"c",0,0})  
    tree[1][RIGHT] = length(tree)  
    tree = append(tree,{"a",0,0})  
    tree[1][LEFT] = length(tree)  
   
-- Finally, some tests (recall that we have already output a 4.7):  
?s[3]  
?tree  
?tree[ROOT][VALUE]  
employees = append(employees, employee)  
?employees[3][SALARY]  
?s1  
?s2

Output:

4.7
"Phix"
{{"b",3,2},{"c",0,0},{"a",0,0}}
"b"
45000
{0,1,2,3,4,5,6}
{0,0,0,0,-2,-1}

## [PHP](http://rosettacode.org/wiki/Category:PHP "Category:PHP")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=172 "Edit section: PHP")]

### Writing To An Array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=173 "Edit section: Writing To An Array")]

#### Single Dimension[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=174 "Edit section: Single Dimension")]

$NumberArray = [array](http://www.php.net/array)(0, 1, 2, 3, 4, 5, 6);  
$LetterArray = [array](http://www.php.net/array)("a", "b", "c", "d", "e", "f");  
$simpleForm = ['apple', 'orange'];

#### Multi-Dimensional[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=175 "Edit section: Multi-Dimensional")]

$MultiArray = [array](http://www.php.net/array)(  
                [array](http://www.php.net/array)(0, 0, 0, 0, 0, 0),  
                [array](http://www.php.net/array)(1, 1, 1, 1, 1, 1),  
                [array](http://www.php.net/array)(2, 2, 2, 2, 2, 2),  
                [array](http://www.php.net/array)(3, 3, 3, 3, 3, 3)  
          );

#### Array push[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=176 "Edit section: Array push")]

$arr = ['apple', 'orange'];  
[array_push](http://www.php.net/array_push)($arr, 'pear');  
print [implode](http://www.php.net/implode)(',', $arr); // Returns apple,orange,pear

### Reading From An Array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=177 "Edit section: Reading From An Array")]

#### Single Dimension[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=178 "Edit section: Single Dimension")]

Read the 5th value in the array:

echo $NumberArray[5]; // Returns 5  
echo $LetterArray[5]; // Returns f

#### Multi-Dimensional[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=179 "Edit section: Multi-Dimensional")]

Read the 2nd line, column 5

echo $MultiArray[1][5]; // 2

### Print a whole array[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=180 "Edit section: Print a whole array")]

This is useful while developing to view the contents of an array:

[print_r](http://www.php.net/print_r)($MultiArray);

Which would give us:

[Array](http://www.php.net/array)(  
    0 => [array](http://www.php.net/array)(  
            0 => 0  
            1 => 0  
            2 => 0  
            3 => 0  
            4 => 0  
            5 => 0  
         )  
    1 => [array](http://www.php.net/array)(  
            0 => 1  
            1 => 1  
            2 => 1  
            3 => 1  
            4 => 1  
            5 => 1  
         )  
    2 => [array](http://www.php.net/array)(  
            0 => 2  
            1 => 2  
            2 => 2  
            3 => 2  
            4 => 2  
            5 => 2  
         )  
    3 => [array](http://www.php.net/array)(  
            0 => 3  
            1 => 3  
            2 => 3  
            3 => 3  
            4 => 3  
            5 => 3  
         )  
)

### Set custom keys for values[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=181 "Edit section: Set custom keys for values")]

This example starts the indexing from 1 instead of 0

$StartIndexAtOne = [array](http://www.php.net/array)(1 => "A", "B", "C", "D");

This example shows how you can apply any key you want

$CustomKeyArray = [array](http://www.php.net/array)("d" => "A", "c" => "B", "b" =>"C", "a" =>"D");

To read the 3rd value of the second array:

echo $CustomKeyArray["b"]; // Returns C

### Other Examples[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=182 "Edit section: Other Examples")]

Create a blank array:

$BlankArray = [array](http://www.php.net/array)();

Set a value for the next key in the array:

$BlankArray[] = "Not Blank Anymore";

Assign a value to a certain key:

$AssignArray["CertainKey"] = "Value";

## [PicoLisp](http://rosettacode.org/wiki/Category:PicoLisp "Category:PicoLisp")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=183 "Edit section: PicoLisp")]

PicoLisp has no built-in array data type. Lists are used instead.

(setq A '((1 2 3) (a b c) ((d e) NIL 777)))  # Create a 3x3 structure  
(mapc println A)  # Show it

Output:

(1 2 3)
(a b c)
((d e) NIL 777)

Replace 'b' with 'B' in middle row:

(set (nth A 2 2) 'B)  
(mapc println A)

Output:

(1 2 3)
(a B c)
((d e) NIL 777)

Insert '1' in front of the middle row:

(push (cdr A) 1)  
(mapc println A)

Output:

(1 2 3)
(1 a B c)
((d e) NIL 777)

Append '9' to the middle row:

(queue (cdr A) 9)  
(mapc println A)

Output:

(1 2 3)
(1 a B c 9)
((d e) NIL 777)

## [Pike](http://rosettacode.org/wiki/Category:Pike "Category:Pike")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=184 "Edit section: Pike")]

int main(){  
   // Initial array, few random elements.  
   array arr = ({3,"hi",84.2});  
   
   arr += ({"adding","to","the","array"}); // Lets add some elements.  
   
   write(arr[5] + "\n"); // And finally print element 5.  
}

## [PL/I](http://rosettacode.org/wiki/Category:PL/I "Category:PL/I")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=185 "Edit section: PL/I")]

/* Example of an array having fixed dimensions */  
declare A(10) float initial (1, 9, 4, 6, 7, 2, 5, 8, 3, 10);  
   
A(6) = -45;  
   
/* Example of an array having dynamic bounds. */  
get list (N);  
   
begin;  
   declare B(N) float initial (9, 4, 7, 3, 8, 11, 0, 5, 15, 6);  
   B(3) = -11;  
   put (B(2));  
end;  
   
/* Example of a dynamic array. */  
   declare C(N) float controlled;  
   get list (N);  
   allocate C;  
   C = 0;  
   c(7) = 12;  
   put (C(9));

## [Pony](http://rosettacode.org/wiki/Category:Pony "Category:Pony")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=186 "Edit section: Pony")]

Arrays are homogenous.

use "assert"      // due to the use of Fact  
   
- - -  
   
var numbers = Array[I32](16) // creating array of 32-bit ints with initial allocation for 16 elements  
numbers.push(10) // add value 10 to the end of array, extending the underlying memory if needed  
try  
  let x = numbers(0) // fetch the first element of array. index starts at 0  
  Fact(x == 10)      // try block is needed, because both lines inside it can throw exception  
end  
   
var other: Array[U64] = [10, 20, 30] // array literal  
let s = other.size() // return the number of elements in array  
try  
  Fact(s == 3)  // size of array 'other' is 3  
  other(1) = 40 // 'other' now is [10, 40, 30]  
end

## [PostScript](http://rosettacode.org/wiki/Category:PostScript "Category:PostScript")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=187 "Edit section: PostScript")]

   
%Declaring array  
   
/x [0 1] def  
   
%Assigning value to an element, PostScript arrays are 0 based.  
   
x 0 3 put  
   
%Print array  
   
x pstack  
[3 1]  
   
%Get an element  
   
x 1 get  
 

## [PowerShell](http://rosettacode.org/wiki/Category:PowerShell "Category:PowerShell")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=188 "Edit section: PowerShell")]

Empty array:

$a = @()

Array initialized with only one member:

$a = ,2  
$a = @(2)  # alternative

Longer arrays can simply be created by separating the values with commas:

$a = 1,2,3

A value can be appended to an array using the  `+=`  operator:

$a += 5

Since arrays are immutable this simply creates a new array containing one more member.

Values can be retrieved using a fairly standard indexing syntax:

$a[1]

Similarly, those values can also be replaced:

$a[1] = 42

The range operator  `..`  can be used to create contiguous ranges of integers as arrays:

$r = 1..100

Indexing for retrieval allows for arrays as well, the following shows a fairly complex example combining two ranges and an arbitrary array in the indexer:

$r[0..9+25..27+80,85,90]

Indexing from the end of the array can be done with negative numbers:

$r[-1]  # last index

## [Prolog](http://rosettacode.org/wiki/Category:Prolog "Category:Prolog")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=189 "Edit section: Prolog")]

**Works with**:  [SWI Prolog](http://rosettacode.org/wiki/SWI_Prolog "SWI Prolog")

Prolog Terms can be abused as array structure. Using functor/3 to create arrays and arg/3 to nondestructively retrieve and set elements.

   
singleassignment:-                     
    [functor](http://pauillac.inria.fr/~deransar/prolog/bips.html)(Array,array,100), % create a term with 100 free Variables as arguments  
                              % index of arguments start at 1  
    [arg](http://pauillac.inria.fr/~deransar/prolog/bips.html)(1 ,Array,a),          % put an a at position 1   
    [arg](http://pauillac.inria.fr/~deransar/prolog/bips.html)(12,Array,b),          % put an b at position 12  
    [arg](http://pauillac.inria.fr/~deransar/prolog/bips.html)(1 ,Array,Value1),     % get the value at position 1  
    print(Value1),[nl](http://pauillac.inria.fr/~deransar/prolog/bips.html),         % will print Value1 and therefore a followed by a newline   
    [arg](http://pauillac.inria.fr/~deransar/prolog/bips.html)(4 ,Array,Value2),     % get the value at position 4 which is a free Variable  
    print(Value2),[nl](http://pauillac.inria.fr/~deransar/prolog/bips.html).         % will print that it is a free Variable followed by a newline  
 

To destructively set an array element, which is the "normal" way to set an element in most other programming languages, setarg/3 can be used.

   
destructive:-                     
    [functor](http://pauillac.inria.fr/~deransar/prolog/bips.html)(Array,array,100), % create a term with 100 free Variables as arguments  
                              % index of arguments start at 1  
    setarg(1 ,Array,a),       % put an a at position 1   
    setarg(12,Array,b),       % put an b at position 12  
    setarg(1, Array,c),       % overwrite value at position 1 with c  
    [arg](http://pauillac.inria.fr/~deransar/prolog/bips.html)(1 ,Array,Value1),     % get the value at position 1  
    print(Value1),[nl](http://pauillac.inria.fr/~deransar/prolog/bips.html).         % will print Value1 and therefore c followed by a newline   
 

Lists can be used as arrays.

   
listvariant:-  
    length(List,100),          % create a list of length 100  
    nth1(1 ,List,a),           % put an a at position 1 , nth1/3 uses indexing from 1, nth0/3 from 0  
    nth1(12,List,b),           % put an b at position 3  
    append(List,[d],List2),    % append an d at the end , List2 has 101 elements  
    length(Add,10),            % create a new list of length 10  
    append(List2,Add,List3),   % append 10 free variables to List2 , List3 now has 111 elements  
    nth1(1 ,List3,Value),      % get the value at position 1  
    print(Value),[nl](http://pauillac.inria.fr/~deransar/prolog/bips.html).           % will print out a  
 

## [PureBasic](http://rosettacode.org/wiki/Category:PureBasic "Category:PureBasic")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=190 "Edit section: PureBasic")]

**Dim**  is used to create new arrays and initiate each element will be zero. An array in PureBasic can be of any types, including structured, and user defined types. Once an array is defined it can be resized with  **ReDim**. Arrays are dynamically allocated which means than a variable or an expression can be used to size them.

  ;Set up an Array of 23 cells, e.g. 0-22  
Dim MyArray.i(22)  
  MyArray(0) = 7  
  MyArray(1) = 11   
  MyArray(7) = 23

**ReDim**  is used to 'resize' an already declared array while preserving its content. The new size can be both larger or smaller, but the number of dimension of the array can not be changed after initial creation.

  ;Extend the Array above to 56 items without affecting the already stored data  
ReDim MyArray(55)  
  MyArray(22) = 7  
  MyArray(33) = 11   
  MyArray(44) = 23

  ;Find all 6 non-zero cells from the Array above  
For i=0 To ArraySize(MyArray())  
  If MyArray(i)  
    PrintN(Str(i)+" differs from zero.")  
  EndIf  
Next

  ; Now, set up a multi dimensional Array   
Dim MultiArray.i(800, 600)  
  MultiArray(100, 200) = 640  
  MultiArray(130,  40) = 120

Dim MultiArray2.i(64, 128, 32)  
  PrintN( Str(ArraySize(MultiArray2(), 2)) ; Will tell that second dimension size is '128'

## [Python](http://rosettacode.org/wiki/Category:Python "Category:Python")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=191 "Edit section: Python")]

```python

Python lists are dynamically resizeable.

array = []  
   
array.append(1)  
array.append(3)  
   
array[0] = 2  
   
print array[0]

A simple, single-dimensional array can also be initialized thus:

myArray = [0] * size

However this will not work as intended if one tries to generalize from the syntax:

myArray = [[0]* width] * height] # DOES NOT WORK AS INTENDED!!!

```

This creates a list of "height" number of references to one list object ... which is a list of width instances of the number zero. Due to the differing semantics of immutables (strings, numbers) and mutables (dictionaries, lists), a change to any one of the "rows" will affect the values in all of them. Thus we need to ensure that we initialize each row with a newly generated list.

To initialize a list of lists one could use a pair of nested list comprehensions like so:

```python

myArray = [[0 for x in range(width)] for y in range(height)]

That is equivalent to:

myArray = list()  
for x in range(height):  
   myArray.append([0] * width)

```

To retrieve an element in an array, use any of the following methods:

```python

# Retrieve an element directly from the array.  

item = array[index]  
   
# Use the array like a stack.  Note that using the pop() method removes the element.  
array.pop()  # Pop last item in a list  
array.pop(0) # Pop first item in a list  
   
# Using a negative element counts from the end of the list.  
item = array[-1] # Retrieve last element in a list.  

``` 

Python produces an IndexError when accessing elements out of range:

 ```python    
try:  
    # This will cause an exception, which will then be caught.  
    print array[len(array)]  
except IndexError as e:  
    # Print the exception.   
    print e  
 ```

## [R](http://rosettacode.org/wiki/Category:R "Category:R")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=192 "Edit section: R")]

Dynamic

arr <- array(1)  
   
arr <- append(arr,3)  
   
arr[1] <- 2  
   
print(arr[1])

## [Racket](http://rosettacode.org/wiki/Category:Racket "Category:Racket")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=193 "Edit section: Racket")]

#lang racket  
   
;; import dynamic arrays  
(require data/gvector)  
   
(define v (vector 1 2 3 4))   ; array  
(vector-ref v 0)              ; 1  
(vector-set! v 1 4)           ; 2 -> 4  
   
(define gv (gvector 1 2 3 4)) ; dynamic array  
(gvector-ref gv 0)            ; 1  
(gvector-add! gv 5)           ; increase size  
 

## [REBOL](http://rosettacode.org/wiki/Category:REBOL "Category:REBOL")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=194 "Edit section: REBOL")]

   
a: []      ; Empty.  
b: ["foo"] ; Pre-initialized.  
 

Inserting and appending.

   
append a ["up" "down"] ; -> ["up" "down"]  
insert a [left right]  ; -> [left right "up" "down"]  
 

Getting specific values.

   
first a ; -> left  
third a ; -> "up"  
last a  ; -> "down"  
a/2     ; -> right (Note: REBOL is 1-based.)  
 

Getting subsequences. REBOL allows relative motion through a block (list). The list variable returns the current position to the end of the list, you can even assign to it without destroying the list.

   
a         ; -> [left right "up" "down"]  
next a    ; -> [right "up" "down"]  
skip a 2  ; -> ["up" "down"]  
   
a: next a ; -> [right "up" "down"]  
head a    ; -> [left right "up" "down"]  
   
copy a                 ; -> [left right "up" "down"]  
copy/part a 2          ; -> [left right]  
copy/part  skip a 2  2 ; -> ["up" "down"]   
 

## [Red](http://rosettacode.org/wiki/Category:Red "Category:Red")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=195 "Edit section: Red")]

arr1: []      ;create empty array  
arr2: ["apple" "orange" 1 2 3]    ;create an array with data  
>> insert arr1 "blue"  
>> arr1  
== ["blue"]  
append append arr1 "black" "green"  
>> arr1  
== ["blue" "black" "green"]  
>> arr1/2  
== "black"  
>> second arr1  
== "black"  
>> pick arr1 2  
== "black"

A vector! is a high-performance series! of items. The items in a vector! must all have the same type. The allowable item types are: integer! float! char! percent! Vectors of string! are not allowed.

>> vec1: make vector! [ 20 30 70]  
== make vector! [20 30 70]  
>> vec1/2  
== 30  
>> second vec1  
== 30  
>> append vec1 90  
== make vector! [20 30 70 90]  
>> append vec1 "string"  
*** Script Error: invalid argument: "string"  
*** Where: append  
*** Stack:   
>> append vec1 3.0  
*** Script Error: invalid argument: 3.0  
*** Where: append  
*** Stack:

## [Retro](http://rosettacode.org/wiki/Category:Retro "Category:Retro")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=196 "Edit section: Retro")]

Retro has a vocabulary for creating and working with arrays.

   
needs array'  
   
( Create an array with four elements )  
^array'new{ 1 2 3 4 } constant a  
   
( Add 10 to each element in an array and update the array with the results )  
a [ 10 + ] ^array'map  
   
( Apply a quote to each element in an array; leaves the contents alone )  
a [ 10 + putn cr ] ^array'apply  
   
( Display an array )  
a ^array'display  
   
( Look for a value in an array )  
3 a ^array'in?  
6 a ^array'in?  
   
( Look for a string in an array )  
"hello" a ^array'stringIn?  
   
( Reverse the order of items in an array )  
a ^array'reverse  
   
( Append two arrays and return a new one )  
^array'new{ 1 2 3 } constant a  
^array'new{ 4 5 6 } constant b  
a b ^array'append constant c  
   
( Create an array from the values returned by a quote )  
[ 1 2 "hello" "world" ] ^array'fromQuote constant d  
   
( Create a quote from the values in an array )  
d ^array'toQuote  
 

## [REXX](http://rosettacode.org/wiki/Category:REXX "Category:REXX")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=197 "Edit section: REXX")]

Strictly speaking, REXX doesn't have arrays, but it does have something that looks, feels, and tastes like arrays;  
they're called  _stemmed arrays_.

### simple arrays[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=198 "Edit section: simple arrays")]

/*REXX program  demonstrates a  simple  array usage.                    */  
a.='not found'                         /*value for all  a.xxx  (so far).*/  
                do j=1  to 100         /*start at 1, define 100 elements*/  
                a.j=-j*1000            /*define as negative J thousand. */  
                end   /*j*/            /*the above defines 100 elements.*/  
   
say 'element 50 is:'   a.50  
say 'element 3000 is:' a.3000  
                                       /*stick a fork in it, we're done.*/

Output:

element 50 is: -50000
element 3000 is: not found

### simple arrays, mimic other languages[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=199 "Edit section: simple arrays, mimic other languages")]

/*REXX program  demonstrates  array usage  with mimicry.                */  
a. = 'not found'                       /*value for all a.xxx  (so far). */  
                  do j=1  to 100       /*start at 1, define 100 elements*/  
                  a.j = -j * 100       /*define element as  -J hundred. */  
                  end   /*j*/          /*the above defines 100 elements.*/  
   
say 'element 50 is:'    a(50)  
say 'element 3000 is:'  a(3000)  
exit                                   /*stick a fork in it, we're done.*/  
/*──────────────────────────────────A subroutine────────────────────────*/  
a:   _a_ = arg(1);          return  a._a_

element 50 is: -5000
element 3000 is: not found

### simple arrays, assigned default[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=200 "Edit section: simple arrays, assigned default")]

/*REXX program  demonstrates  array usage  with mimicry.                */  
a. = 00                                /*value for all a.xxx  (so far). */  
                  do j=1  to 100       /*start at 1, define 100 elements*/  
                  a.j = -j * 100       /*define element as  -J hundred. */  
                  end   /*j*/          /*the above defines 100 elements.*/  
   
say 'element 50 is:'    a(50)  
say 'element 3000 is:'  a(3000)  
exit                                   /*stick a fork in it, we're done.*/  
/*──────────────────────────────────A subroutine────────────────────────*/  
a:   _a_ = arg(1);          return  a._a_

Output:

element 50 is: -5000
element 3000 is: 00

### arrays with non-unity index start[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=201 "Edit section: arrays with non-unity index start")]

/*REXX program  demonstrates  array usage  (with elements out-of-range).*/  
array. = 'out of range'                /*define  ALL  elements to this. */  
   
              do j=-3000  to 3000      /*start at -3k,  going up to +3k.*/  
              array.j=j**2             /*define element as its square.  */  
              end   /*j*/              /* [↑]   defines 6,001 elements. */  
g=-7  
say g      "squared is:"   array.g  
say 7000   "squared is:"   array.7000  
                                       /*stick a fork in it, we're done.*/

Output:

-7 squared is: 49
7000 squared is: out of range

### arrays, disjoint[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=202 "Edit section: arrays, disjoint")]

/*REXX program  demonstrates  disjointed array usage.                   */  
yr. = 'year not supported'             /*value for all yr.xxx  (so far).*/  
   
                do k=600  to 1100      /*a bunch of years prior to 1800.*/  
                yr.k=k "AD"            /*Kth element as the year itself.*/  
                end   /*k*/            /* [↑]      defines 501 elements.*/  
   
            do j=1800  to 2100         /*start at 1800, define a bunch. */  
            yr.j=j 'AD'                /*Jth element as the year itself.*/  
            end   /*j*/                /* [↑]      defines 301 elements.*/  
   
year=1946  
say 'DOB' year "is:" yr.year  
   
year=1744  
say 'DOB' year "is:" yr.year  
                                       /*stick a fork in it, we're done.*/

Output:

DOB 1946 is: 1946 AD
DOB 1744 is: year not supported

### sparse arrays and special indices[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=203 "Edit section: sparse arrays and special indices")]

/*REXX program  demonstrates  array usage:   sparse and disjointed.     */  
  yyy = -55                            /*REXX must use this mechanism···*/  
a.yyy = 1e9                            /*··· when assigning neg indices.*/  
   
a.1 = 1000  
a.2 = 2000.0001  
a.7 = 7000  
a.2012 = 'out here in left field.'  
a.cat = 'civet, but not a true cat ─── belonging to the family Viverridae'  
a.civet = "A.K.A.: toddycats"  
/*┌────────────────────────────────────────────────────────────────────┐  
  │ Array elements need not be continuous (nor even defined).   They   │  
  │ can hold any manner of numbers,  or strings (which can include any │  
  │ characters,  including    null    or    '00'x   characters).       │  
  │                                                                    │  
  │ Array elements need not be numeric, as the above code demonstrates.│  
  │ Indeed, the element "name" can be ANYTHING,  even non-displayable  │  
  │ characters.    To illustrate  [↓]:                                 │  
  └────────────────────────────────────────────────────────────────────┘*/  
stuff=')g.u.t.s(  or  ½ of an intestine!'  
a.stuff=44  
/*┌────────────────────────────────────────────────────────────────────┐  
  │ where the element name has special characters:  blanks,  and the   │  
  │ glyph of  one-half (½),  as well as the symbol used in REXX to     │  
  │ identify stemmed arrays (the period).                              │  
  └────────────────────────────────────────────────────────────────────┘*/  
                                       /*stick a fork in it, we're done.*/

## [RLaB](http://rosettacode.org/wiki/Category:RLaB "Category:RLaB")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=204 "Edit section: RLaB")]

   
// 1-D (row- or column-vectors)  
// Static:  
// row-vector  
x = [1:3];    
x = zeros(1,3); x[1]=1; x[2]=2; x[3]=3;  
// column-vector  
x = [1:3]';  // or  
x = [1;2;3]; // or  
x = zeros(3,1); x[1]=1; x[2]=2; x[3]=3;  
// Dynamic:  
x = [];           // create an empty array  
x = [x; 1, 2];    // add a row to 'x' containing [1, 2], or  
x = [x, [1; 2]];  // add a column to 'x' containing [1; 2]  
   
// 2-D array  
// Static:  
x = zeros(3,5);        // create an zero-filed matrix of size 3x5  
x[1;1] = 1;            // set the x(1,1) element to 1  
x[2;]  = [1,2,3,4,5];  // set the second row x(2,) to a row vector  
x[3;4:5] = [2,3];      // set x(3,4) to 2 and x(3,5) to 3  
// Dynamic  
x = [1:5];               // create an row-vector x(1,1)=1, x(1,2)=2, ... x(1,5)=5  
x = [x; 2, 3, 4, 6, 7];  // add to 'x' a row.  
   
// Accessing an element of arrays:  
// to retrieve/print element of matrix 'x' just put this in a single line in the script  
i=1;  
j=2;  
x[i;j]  
   
   
 

## [RPG](http://rosettacode.org/wiki/Category:RPG "Category:RPG")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=205 "Edit section: RPG")]

**Works with**:  [ILE RPG](http://rosettacode.org/mw/index.php?title=ILE_RPG&action=edit&redlink=1 "ILE RPG (page does not exist)")

   
      //-Static array  
      //--def of 10 el array of integers, initialised to zeros  
     D array...   
     D                 s             10i 0 dim(10)  
     D                                     inz  
      //--def an el  
     D el_1...   
     D                 s             10i 0 inz  
   
      /free  
   
       //-assign first el  
       //--first element of RPG array is indexed with 1  
       array(1) = 111;  
   
       //-get first el of array  
       el_1 = array(1);  
   
       //--display it  
       dsply ('First el of array='+%char(el_1));  
       //--displays: First el of array=111  
   
       //---or shorter, without "el_1"  
       dsply ('First el of array='+%char(array(1)));  
       //--displays: First el of array=111  
   
      /end-free  
 

  

## [Ring](http://rosettacode.org/wiki/Category:Ring "Category:Ring")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=206 "Edit section: Ring")]

Dynamic

# create an array with one string in it  
a = ['foo']  
   
# add items  
a + 1         # ["foo", 1]  
   
# set the value at a specific index in the array  
a[1] = 2       # [2, 1]  
   
# retrieve an element  
see a[1]

## [Robotic](http://rosettacode.org/wiki/Category:Robotic "Category:Robotic")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=207 "Edit section: Robotic")]

Robotic does not natively support arrays of any kind. However, using  [Counter Interpolation](https://www.digitalmzx.net/wiki/index.php?title=Counter_interpolation), we can create a simple (faux) array.

   
set "index" to 0  
. "Assign random values to array"  
: "loop"  
set "array&index&" to random 0 to 99  
inc "index" by 1  
if "index" < 100 then "loop"  
   
* "Value of index 50 is ('array('50')')."  
end  
 

You can even create multi-dimensional arrays using the Counter Interpolation method.

   
set "xx" to 0  
set "yy" to 0  
. "Assign random values to array"  
: "loopX"  
set "array&xx&,&yy&" to random 0 to 99  
inc "xx" by 1  
if "xx" < 32 then "loopX"  
set "xx" to 0  
inc "yy" by 1  
if "yy" < 32 then "loopX"  
   
* "Value of 16,16 is ('array('16'),('16')')."  
end  
 

Because arrays aren't built in, there are no functions that allow you to manipulate the data you create within an array. You would have to create your own function when, for example, you want to sort numbers from least to greatest.

## [Ruby](http://rosettacode.org/wiki/Category:Ruby "Category:Ruby")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=208 "Edit section: Ruby")]

Dynamic

# create an array with one object in it  
a = ['foo']  
   
# the Array#new method allows several additional ways to create arrays  
   
# push objects into the array  
a << 1         # ["foo", 1]  
a.push(3,4,5)  # ["foo", 1, 3, 4, 5]  
   
# set the value at a specific index in the array  
a[0] = 2       # [2, 1, 3, 4, 5]  
   
# a couple of ways to set a slice of the array  
a[0,3] = 'bar'    # ["bar", 4, 5]  
a[1..-1] = 'baz'  # ["bar", "baz"]  
a[0] = nil        # [nil, "baz"]  
a[0,1] = nil      # ["baz"]  
   
# retrieve an element  
puts a[0]

## [Run BASIC](http://rosettacode.org/wiki/Category:Run_BASIC "Category:Run BASIC")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=209 "Edit section: Run BASIC")]

print "Enter array 1 greater than 0"; : input a1  
print "Enter array 2 greater than 0"; : input a2  
   
dim chrArray$(max(a1,1),max(a2,1))  
dim numArray(max(a1,1),max(a2,1))  
   
chrArray$(1,1) = "Hello"  
numArray(1,1) = 987.2  
print chrArray$(1,1);" ";numArray(1,1)

## [Rust](http://rosettacode.org/wiki/Category:Rust "Category:Rust")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=210 "Edit section: Rust")]

The Rust book has a  [tutorial on arrays](https://doc.rust-lang.org/1.0.0-beta/book/arrays-vectors-and-slices.html).

By default, arrays are immutable unless defined otherwise.

```rust  
let a = [1, 2, 3]; // immutable array  
let mut m = [1, 2, 3]; // mutable array  
let zeroes = [0; 200]; // creates an array of 200 zeroes
```

To get the length and iterate,

```rust
let a = [1, 2, 3];  
a.len();  
for e in a.iter() {  
    e;  
}
```

Accessing a particular element uses subscript notation, starting from 0.

```rust
let names = ["Graydon", "Brian", "Niko"];  
names[1]; // second element
```

Dynamic arrays in Rust are called vectors.

```rust
let v = vec![1, 2, 3];
```

However, this defines an immutable vector. To add elements to a vector, we need to define v to be mutable.

```rust
let mut v = vec![1, 2, 3];  
v.push(4);  
v.len(); // 4
```

## [Sather](http://rosettacode.org/wiki/Category:Sather "Category:Sather")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=211 "Edit section: Sather")]

-- a is an array of INTs  
a :ARRAY{INT};  
-- create an array of five "void" elements  
a := #ARRAY{INT}(5);  
-- static creation of an array with three elements  
b :ARRAY{FLT} := |1.2, 1.3, 1.4|;  
-- accessing an array element  
c ::= b[0]; -- syntactic sugar for b.aget(0)  
-- set an array element  
b[1] := c; -- syntactic sugar for b.aset(1, c)  
-- append another array  
b := b.append(|5.5|);

## [Scala](http://rosettacode.org/wiki/Category:Scala "Category:Scala")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=212 "Edit section: Scala")]

Arrays are not used often in Scala, since they are mutable and act differently to other collections with respect to type erasure, but are necessary for interoperability with Java. Alternatives such as List, Seq, and Vector are more commonly used.

// Create a new integer array with capacity 10  
[val](https://scala-lang.org/) a = [new](https://scala-lang.org/) Array[Int](10)  
   
// Create a new array containing specified items  
[val](https://scala-lang.org/) b = Array("foo", "bar", "baz")  
   
// Assign a value to element zero  
a(0) = 42  
   
// Retrieve item at element 2  
[val](https://scala-lang.org/) c = b(2)

Dynamic arrays can be made using  `ArrayBuffer`s:

[val](https://scala-lang.org/) a = [new](https://scala-lang.org/) collection.mutable.ArrayBuffer[Int]   
a += 5   // Append value 5 to the end of the list  
a(0) = 6 // Assign value 6 to element 0

## [Scheme](http://rosettacode.org/wiki/Category:Scheme "Category:Scheme")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=213 "Edit section: Scheme")]

Lists are more often used in Scheme than vectors.

(let ((array #(1 2 3 4 5))     ; vector literal  
      (array2 (make-vector 5))  ; default is unspecified  
      (array3 (make-vector 5 0))) ; default 0  
 (vector-set! array 0 3)  
 (vector-ref array 0))    ; 3

## [Scratch](http://rosettacode.org/wiki/Category:Scratch "Category:Scratch")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=214 "Edit section: Scratch")]

[![Scratch Arrays.png](http://rosettacode.org/mw/images/5/55/Scratch_Arrays.png)](http://rosettacode.org/wiki/File:Scratch_Arrays.png)

## [Seed7](http://rosettacode.org/wiki/Category:Seed7 "Category:Seed7")[[edit](http://rosettacode.org/mw/index.php?title=Arrays&action=edit&section=215 "Edit section: Seed7")]

By default array indices have the type integer and start from 1. Other index types and start values are also possible. E.g.: The famous arrays with indices starting from 0 are possible. Every type, which can be mapped to integer, can be used as index type.

$ include "seed7_05.s7i";  
   
const type: charArray is array [char] string;  # Define an array type for arrays with char index.  
const type: twoDim is array array char;        # Define an array type for a two dimensional array.  
   
const proc: main is func  
  local  
    var array integer: array1   is 10 times 0;           # Array with 10 elements of 0.  
    var array boolean: array2   is [0 .. 4] times TRUE;  # Array with 5 elements of TRUE.  
    var array integer: array3   is [] (1, 2, 3, 4);      # Array with the elements 1, 2, 3, 4.  
    var array string: array4    is [] ("foo", "bar");    # Array with 