#![allow(unused_variables)]
fn main() {
    let a: [u32; 7] = [3, 2, 5, 6, 7, 8, 0];
    let k: u32 = 7;
    let mut u = "".to_string();
    if let Some(z) = linear_search::<u32>(&a, &k) {
        u = z.to_string();
    };

    println!("{}", u);
}

fn linear_search<T>(arr: &[T], obj: &T) -> Option<usize>
where
    T: PartialEq,
{
    arr.iter().position(|x| x == obj)
}


fn binary_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
{
    let mut size = arr.len();       // 1
    if size == 0 {
        return Err(0);
    }
    let mut base = 0_usize;

    while size > 1 {                // 2
        // mid: [base..size)
        let half = size / 2;        // 2.1
        let mid = base + half;
        if arr[mid] <= *target {    // 2.2
            base = mid
        }
        size -= half;               // 2.3
    }

    if arr[base] == *target {       // 3
        Ok(base)
    } else {
        Err(base + (arr[base] < *target) as usize)
    }
}


fn interpolation_search(
    arr: &[i32],
    target: &i32,
) -> Result<usize, usize> {
    // 1. Handle empty sequence.
    if arr.is_empty() {
        return Err(0)
    }

    // 2. Setup variable storing iteration informaion.
    let mut hi = arr.len() - 1;
    let mut lo = 0_usize;

    let mut interpolant = 0_usize;

    // 3. Main loop to calculate the interpolant.
    loop {
        let lo_val = arr[lo];
        let hi_val = arr[hi];

        // 3.1. Three condition to exit the loop
        if hi <= lo || *target < lo_val || *target > hi_val {
            break
        }

        // 3.2. The linear interpolation part
        let offset = (*target - lo_val) * (hi - lo) as i32 / (hi_val - lo_val);
        interpolant = lo + offset as usize;

        let mid_val = arr[interpolant];

        // 3.3. Comparison between the interpolant and targert value.
        if mid_val > *target {
            hi = interpolant - 1;
        } else if mid_val < *target {
            lo = interpolant + 1;
        } else {
            break
        }
    }

    // 4. Determine whether the returning interpolant are equal to target value.
    if *target > arr[hi] {
        Err(hi + 1)
    } else if *target < arr[lo] {
        Err(lo)
    } else {
        Ok(interpolant)
    }

}



fn exponential_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
{
    // 1. Handle empty scenario.
    let size = arr.len();
    if size == 0 {
        return Err(0);
    }

    // 2. Determine searching boundaries.
    let mut hi = 1_usize; // Upper bound.
    while hi < size && arr[hi] < *target {
        hi <<= 1;
    }
    let lo = hi >> 1; // Lower bound.

    // 3. Do binary search.
    binary_search(&arr[lo..size.min(hi + 1)], target)
        .map(|index| lo + index)
        .map_err(|index| lo + index)
}

