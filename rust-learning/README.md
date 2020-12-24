# learning-rust

- study study study  
- coding coding and coding  
- question question and question.  

## Revice

***Again to check rust-lang***

### Start form print 

[print](./refresher/print.md)

## cookbook

### workspace

**warning:**
 
-  workspace inclued members director must be existed.

-  The director name is not same as dependencies name.

### execute

   cargo run -p reg

## rustlings
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