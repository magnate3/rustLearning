# learning-rust

***To learning can be start from "rustup doc" is good idea.***

## rustlings

Thanks for [Rustlings](https://github.com/fmoko/rustlings)

All is ok!

✓ Successfully compiled exercises/variables/variables1.rs!
✓ Successfully compiled exercises/variables/variables2.rs!
✓ Successfully compiled exercises/variables/variables3.rs!
✓ Successfully compiled exercises/variables/variables4.rs!
✓ Successfully compiled exercises/variables/variables5.rs!
✓ Successfully compiled exercises/functions/functions1.rs!
✓ Successfully compiled exercises/functions/functions2.rs!
✓ Successfully compiled exercises/functions/functions3.rs!
✓ Successfully compiled exercises/functions/functions4.rs!
✓ Successfully compiled exercises/functions/functions5.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types1.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types2.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types3.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types5.rs!
✓ Successfully compiled exercises/primitive_types/primitive_types6.rs!
✓ Successfully compiled exercises/strings/strings1.rs!
✓ Successfully compiled exercises/strings/strings2.rs!
✓ Successfully compiled exercises/test2.rs!
✓ Successfully compiled exercises/enums/enums1.rs!
✓ Successfully compiled exercises/enums/enums2.rs!
✓ Successfully compiled exercises/modules/modules1.rs!
✓ Successfully compiled exercises/modules/modules2.rs!
✓ Successfully compiled exercises/macros/macros1.rs!
✓ Successfully compiled exercises/macros/macros2.rs!
✓ Successfully compiled exercises/macros/macros3.rs!
✓ Successfully compiled exercises/macros/macros4.rs!
✓ Successfully compiled exercises/move_semantics/move_semantics1.rs!
✓ Successfully compiled exercises/move_semantics/move_semantics2.rs!
✓ Successfully compiled exercises/move_semantics/move_semantics3.rs!
✓ Successfully compiled exercises/move_semantics/move_semantics4.rs!
✓ Successfully compiled exercises/clippy/clippy1.rs!
✓ Successfully compiled exercises/clippy/clippy2.rs!
✓ Successfully compiled exercises/standard_library_types/arc1.rs!
✓ Successfully compiled exercises/threads/threads1.rs!
✓ Successfully compiled exercises/conversions/using_as.rs!

Enjoy!

- rustlings:enum
	exercises/enums/enums3.rs
- rustlings:modules
	exercises/modules/modules2.rs
- rustlings: macros	
	exercises/macros/macros3.rs
	exercises/test4.rs
- rustlings: Error handling
	exercises/error_handling/errorsn.rs
- rustlings: Error option
        exercises/error_handling/option1.rs
-rustlings: Error result
	exercises/error_handling/result1.rs

-rustlings: if let and Some
        exercises/clippy/clippy2.rs
-rustlings: std and Arc
	exercises/standard_library_types/arc1.rs
-rustlings: iterators
	exercises/standard_library_types/iterators2.rs
```
/*
Step 1
You need to call something on `first` before it can be collected
Currently its type is `char`. Have a look at the methods that are available on that type:
https://doc.rust-lang.org/std/primitive.char.html


Step 2
First you'll need to turn the Vec into an iterator
Then you'll need to apply your function unto each item in the vector
P.s. Don't forget to collect() at the end!


Step 3.
This is very similar to the previous test. The only real change is that you will need to
alter the type that collect is coerced into. For a bonus you could try doing this with a
turbofish

*/
```
-rustlings: iterators and Error - Result
	exercises/standard_library_types/iterators3.rs

```
Minor hint: In each of the two cases in the match in main, you can create x with either
a 'turbofish' or by hinting the type of x to the compiler. You may try both.

Major hint: Have a look at the Iter trait and at the explanation of its collect function.
Especially the part about Result is interesting.
```

-rustlings: closure fold 
	exercises/standard_library_types/iterators4.rs
```
( 1..=num).fold(1,|r,x| r*x)
```

-rustlings: taril
	exercises/traits/traits2.rs

```
trait AppendBar {
    fn append_bar(&mut self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(&mut self) -> Self {
        self.push(String::from("Bar"));
        self.to_vec()
    }
}

```

-rustlings: threads 
	exercises/threads/threads1.rs
```
// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// I AM NOT DONE

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

//use std::sync::Arc;
//use std::thread;
//use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    /*
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = Arc::clone(&status);
    //let status = Arc::new(JobStatus { jobs_completed: 0 });
    //let mut child_numbers=Arc::clone(&shared_numbers);
   // let mut status_shared = status_shared.lock().unwrap();
    //let status_shared = Arc::clone(&status);
    thread::spawn(move || {
        
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut status_shared = status_shared.lock().unwrap();
            (*status_shared).jobs_completed +=1;
            //status_shared.jobs_completed += 1;
        }
    });
    while status.jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
*/

let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
let status_shared = status.clone();
thread::spawn(move || {
    for _ in 0..10 {
        thread::sleep(Duration::from_millis(250));
        status_shared.lock().unwrap().jobs_completed += 1;
    }
});
while status.lock().unwrap().jobs_completed < 10 {
    println!("waiting... ");
    thread::sleep(Duration::from_millis(500));
}
}


// Do you now have an `Arc` `Mutex` `JobStatus` at the beginning of main? Like:
// `let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));`
// Similar to the code in the example in the book that happens after the text
// that says "We can use Arc<T> to fix this.". If not, give that a try! If you
// do and would like more hints, keep scrolling!!












// Make sure neither of your threads are holding onto the lock of the mutex
// while they are sleeping, since this will prevent the other thread from
// being allowed to get the lock. Locks are automatically released when
// they go out of scope.

// Ok, so, real talk, this was actually tricky for *me* to do too. And
// I could see a lot of different problems you might run into, so at this
// point I'm not sure which one you've hit :) Please see a few possible
// answers on https://github.com/carols10cents/rustlings/issues/3 --
// mine is a little more complicated because I decided I wanted to see
// the number of jobs currently done when I was checking the status.

// Please open an issue if you're still running into a problem that
// these hints are not helping you with, or if you've looked at the sample
// answers and don't understand why they work and yours doesn't.

// If you've learned from the sample solutions, I encourage you to come
// back to this exercise and try it again in a few days to reinforce
// what you've learned :)
```

---

-rustlings:  as_ref
	exercises/conversions/as_ref_mut.rs

```
//from: fn byte_counter<T>(arg: T) -> usize {
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
//from: fn char_counter<T>(arg: T) -> usize {	
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
```	
---
***Result***

	exercises/standard_library_types/iterators3.rs
```

        let numbers = vec![27, 297, 38502, 81];
        let division_results = numbers.into_iter().map(|n| divide(n, 27));
//        let x: Result<Vec<i32>, DivisionError> = division_results.collect();
        let x: Vec<Result<i32, DivisionError>> = division_results.collect();
 //       let x: Result<Vec<i32>, DivisionError> = division_results.collect();
        //let x //... Fill in here!
        assert_eq!(format!("{:?}", x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");

```
***clorure***
errors:(0..=num)....
right:
(1..=num).fold(1,|r,x|r*x)
---


# [How to determine the resulting type of a Rust iterator?](https://stackoverflow.com/questions/56499241/how-to-determine-the-resulting-type-of-a-rust-iterator)

[Ask Question](https://stackoverflow.com/questions/ask)

Asked  10 months ago

Active  [10 months ago](https://stackoverflow.com/questions/56499241/how-to-determine-the-resulting-type-of-a-rust-iterator?lastactivity "2019-06-07 19:06:41Z")

Viewed  97 times

1

[](https://stackoverflow.com/posts/56499241/timeline "Timeline")

I am working through the  [rustlings](https://github.com/rust-lang/rustlings)  exercises to learn Rust. I have reached the  [iterator3.rs](https://github.com/rust-lang/rustlings/blob/master/exercises/standard_library_types/iterator3.rs)  exercise and am stuck. This exercise asks me to provide a line of code that will map results from one type to another as part of the operation; I need to fill in the line x= with the correct operation. There are two parts - the first reads in part:

```
let numbers = vec![27, 297, 38502, 81];
let division_results = numbers.into_iter().map(|n| divide(n, 27));
let x = ???
assert_eq!(format!("{:?}", x), "Ok([1, 11, 1426, 3])");

```

The next is the same with a slightly different format for the assertion of the output:

```
assert_eq!(format!("{:?}", x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");

```

I believe I understand that the first instance needs to return a Result that contains either a Vector of i32 or some error type. The second needs to return a Vector of Results that each have a i32 or an error type.

I am, however, generally having difficulty understanding how to determine what type is returned by the combinations of into_iter, map, and collect. I could use some help in learning how to reason about it or in getting compiler assistance.

Here is where I am so far:

I don't understand what the resultant type of division_results is. I have tried to use compiler error messages as well as  [the answer of this question](https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust/29168659#29168659)  to find out, but the results are opaque to me, perhaps because of lazy evaluation. For example, just replacing x with division_results so that assert will show the types, like this:

```
assert_eq!(format!("{:?}", division_results), "Ok([1, 11, 1426, 3])");

```

Gives me this result:

```
left: `"Map { iter: IntoIter([27, 297, 38502, 81]) }"`,
right: `"Ok([1, 11, 1426, 3])"`', exercises/standard_library_types/iterator3.rs:75:9

```

And it isn't clear what the left side results are as the iteration and map have not happened. The other various things I have tried provide similar results.

Thinking that the problem is lazy evaluation, I have also tried using collect to see if that will force evaluation. For example, calling collect at the end of the division_results line like this:

```
division_results = numbers.into_iter().map(|n| divide(n, 27)).collect();

```

Provides an error:

```
cannot infer type consider giving `division_results` a type

```

When I modify collect to say:

```
let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect::<i32>();

```

I get an error that I thought gave me a hint at the right type:

```
let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect::<i32>();
   |                                                                           ^^^^^^^ a collection of type `i32` cannot be built from `std::iter::Iterator<Item=std::result::Result<i32, DivisionError>>`

```

So I tried with the type shown in the error message:

```
let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect::<Result<i32, DivisionError>>();

```

Only to get this error:

```
let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect::<Result<i32, DivisionError>>();
   |                                                                           ^^^^^^^ a collection of type `i32` cannot be built from `std::iter::Iterator<Item=i32>`

```

Clearly I am missing something. Perhaps you can tell me what?

[rust](https://stackoverflow.com/questions/tagged/rust "show questions tagged 'rust'")

[share](https://stackoverflow.com/q/56499241/8146671 "short permalink to this question")[edit](https://stackoverflow.com/posts/56499241/edit "revise and improve this post")follow

[edited  Jun 7 '19 at 18:28](https://stackoverflow.com/posts/56499241/revisions "show all edits to this post")

asked  Jun 7 '19 at 18:01

[](https://stackoverflow.com/users/11610674/rustybob)

![](https://www.gravatar.com/avatar/eff6513da0880379c0d3d391feccb713?s=32&d=identicon&r=PG&f=1)

[RustyBob](https://stackoverflow.com/users/11610674/rustybob)

1333 bronze badges

[add a comment](https://stackoverflow.com/questions/56499241/how-to-determine-the-resulting-type-of-a-rust-iterator# "Use comments to ask for more information or suggest improvements. Avoid answering questions in comments.")

## 1 Answer

[Active](https://stackoverflow.com/questions/56499241/how-to-determine-the-resulting-type-of-a-rust-iterator?answertab=active#tab-top "Answers with the latest activity first")[Oldest](https://stackoverflow.com/questions/56499241/how-to-determine-the-resulting-type-of-a-rust-iterator?answertab=oldest#tab-top "Answers in the order they were provided")[Votes](https://stackoverflow.com/questions/56499241/how-to-determine-the-resulting-type-of-a-rust-iterator?answertab=votes#tab-top "Answers with the highest score first")

3

[](https://stackoverflow.com/posts/56499987/timeline "Timeline")

The  `map()`  method on an  _iterator adapter_; it takes an iterator, and returns another iterator, but it does not consume any items from the original iterator by itself. The return type  `Map`  is a wrapper around the original iterator that applies the provided closure to every item when they are consumed.

If you want the  `Map`  to actually do something, you need to  _consume_  the iterator. The most common ways to do so are  `for`  loops and the  `collect()`  method (but there are many other methods that consume the iterator, like  `sum()`,  `count()`,  `fold()`,  `max()`, …). In this particular case, calling the  `collect()`  method is most appropriate, since you want to collect the results in a vector.

You already figured out that the desired type for  `x`  is a  `Result`  wrapping a vector of  `i32`  or an error, or  `Result<Vec<i32>, DivisionError>`  in Rust syntax. Since  `collect()`  can produce many different return types, we need to tell the compiler which one we want. One way to do so is to explicitly specify the type of  `x`:

```
let x: Result<Vec<i32>, DivisionError> = division_results.collect();

```

This uses an  [implementation of the  `FromIterator`  trait that allows to collect an iterable of  `Result`s into a  `Result`  wrapping a collection of values](https://doc.rust-lang.org/std/iter/trait.FromIterator.html#impl-FromIterator%3CResult%3CA%2C%20E%3E%3E).

The other case you mentioned is very similar. This time, the target type is a vector of  `Result`  instances, so all you need to do is specify the different type. This will automatically select the right implementation of  `FromIterator`  for you:

```
let x: Vec<Result<i32, DivisionError>> = division_results.collect();

```

[share](https://stackoverflow.com/a/56499987/8146671 "short permalink to this answer")[edit](https://stackoverflow.com/posts/56499987/edit "revise and improve this post")follow

answered  Jun 7 '19 at 19:06

[](https://stackoverflow.com/users/279627/sven-marnach)

![](https://www.gravatar.com/avatar/2dceea858ad8f1577bec6ddaa0485d15?s=32&d=identicon&r=PG)

[Sven Marnach](https://stackoverflow.com/users/279627/sven-marnach)

417k9090 gold badges803803 silver badges732732 bronze badges

-   Thanks! I am more used to C and keep underestimating how much Rust will do for me. I'll try these out.  – [RustyBob](https://stackoverflow.com/users/11610674/rustybob "13 reputation")  Jun 7 '19 at 19:43
    

[add a comment](https://stackoverflow.com/questions/56499241/how-to-determine-the-resulting-type-of-a-rust-iterator# "Use comments to ask for more information or suggest improvements. Avoid comments like “+1” or “thanks”.")
