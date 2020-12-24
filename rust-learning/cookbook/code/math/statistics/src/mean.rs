//These examples calculate measures of central tendency for a data set contained within a Rust array. There may be no mean, median or mode to calculate for an empty set of data, so each function returns an [Option] to be handled by the caller.

//The first example calculates the mean (the sum of all measurements divided by the number of measurements in the set) by producing an iterator of references over the data, and using [sum] and [len] to determine the total value and count of values respectively.

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    let mean = match count {
       positive if positive > 0 => Some(sum  / count as f32),
       _ => None
    };

    println!("Mean of the data is {:?}", mean);
}

