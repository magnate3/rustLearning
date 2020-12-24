# Learning

### Some

```
fn main() {
    let mut res = 42;
    let option = Some(12);
    /*for x in option {
        res += x;
    }*/
    if let Some(x)=option{res+=x}; //Using `option{...}`
    println!("{}", res);
}

```

### bytes count or chars count

```
fn byte_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}


fn main() {
    let s = "Café au lait";
    println!("{}", char_counter(s));
    println!("{}", byte_counter(s));
}


```

### str to number

```
let nstr: String = ss.chars().filter(|x| x.is_digit(10)).collect();
```

###     complite enums

```
enum Message {
    // TODO: implement the message variant types based on their usage below
    Quit,
    Echo(String),
    Move{x:u8,y:u8},
    ChangeColor(u8,u8,u8),
}

struct Point {
    x: u8,
    y: u8
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        match message {
                        Message::ChangeColor(x,y,z) => {let c=(x,y,z); self.change_color(c);},
                        Message::Move{x,y} => {let p =Point{x,y}; self.move_position(p);},
                        Message::Echo(s) => {self.echo(s);},
                        Message::Quit => self.quit(),
        }

    }
}

```

### errors

### iterator

```
// iterators3.rs
// This is a bigger exercise than most of the others! You can do it!
// Here is your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass
// 2. Uncomment the last two tests and get them to pass by filling in
//    values for `x` using `division_results`.
// Execute `rustlings hint iterators3` to get some hints!
// Have fun :-)

// It is very good example.

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// This function should calculate `a` divided by `b` if `a` is
// evenly divisible by b.
// Otherwise, it should return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if a==0 {return  Ok(0)};
    match b {
        b if b >0 =>{
            if a%b==0 {return Ok(a/b);
            }else{
                return  Err(DivisionError:: NotDivisible(NotDivisibleError{ dividend: a, divisor: b,}));
            }
        }
        0 => return Err(DivisionError::DivideByZero) ,
        _ => return Err(DivisionError::NotDivisible(NotDivisibleError{ dividend: b, divisor: a,})),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests that verify your `divide` function implementation
    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    // Iterator exercises using your `divide` function
 
    #[test]
    fn result_with_list() {
        let numbers = vec![27, 297, 38502, 81];
        let division_results = numbers.into_iter().map(|n| divide(n, 27));
        let x: Result<Vec<i32>, DivisionError> = division_results.collect();
         //let x =
        //... Fill in here!
        assert_eq!(format!("{:?}", x), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn list_of_results() {
        let numbers = vec![27, 297, 38502, 81];
        let division_results = numbers.into_iter().map(|n| divide(n, 27));
//        let x: Result<Vec<i32>, DivisionError> = division_results.collect();
        let x: Vec<Result<i32, DivisionError>> = division_results.collect();
 //       let x: Result<Vec<i32>, DivisionError> = division_results.collect();
        //let x //... Fill in here!
        assert_eq!(format!("{:?}", x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");
    }
   
}

```