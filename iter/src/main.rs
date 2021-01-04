fn main() {

    // Basic Range (exclusive on the right)
    for i in 1..11 {
        print!("{} ", i);
    }
    println!("");

    // Inclusive range
    for i in 1..=10 {
        print!("{} ", i);
    }
    println!("");

    // use of discard "_" pattern
    let mut n: i32 = 0;
    for _ in 0..10 {
        n += 1;
    }
    println!("num = {}", n);

    // count()
    println!("num = {}", (0..10).count());

    // Range with step using a filter
    for i in (0..21).filter(|x| (x % 2 == 0)) {
      print!("{} ", i);
    }
    println!("");


    // Reverse range
    for i in (0..11).rev() {
        print!("{} ", i);
    }
    println!("");

    // map()
    for i in (1..11).map(|x| x * x) {
        print!("{} ", i);
    }
    println!("");

    // fold()
    let result = (1..=5).fold(0, |acc, x| acc + x * x);
    println!("result = {}", result);
}