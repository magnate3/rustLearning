struct Counter {
    max: i32,
    // `count` tracks the state of this iterator.
    count: i32,
}

impl Counter {
    fn new(max: i32) -> Counter {
        Counter { count: -1, max: max }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.max {
            Some(self.count)
        } else {
            None
        }
    }
}
fn main(){
for i in Counter::new(10) {
    println!("{}", i);
}
}