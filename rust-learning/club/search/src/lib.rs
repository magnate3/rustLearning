//! Learning from [rust-algo.club](https://rust-algo.club/index.html)
//! And source of it [source](https://github.com/weihanglo/rust-algorithm-club)

/*
pub fn linear_search<T>(arr: &[T], target: &T) -> Option<usize>
    where T: PartialEq
{
    for (index, item) in arr.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}
*/


pub fn liner_search<T>(arr: &[T], obj:&T) -> Option<usize>
	where T:PartialOrd,
{

 arr.iter().position(|x| x == obj) 

}


pub fn binary_search<T>(arr:&[T],target:&T) -> Result<usize,usize>
	where T:PartialOrd,
{
    let mut size = arr.len();
	if size == 0 {
	return  Err(0);
	}
	
	let mut base = 0_usize;
	
	while size > 1 {
		let half = size / 2;
		let mid = base + half;
		if arr[mid] <= *target {
 		base = mid;
		}
		size-=half;
	}
	if arr[base] == *target {
	Ok(base)
	} else {
	Err(base + (arr[base] < *target) as usize)    
	}
}

pub fn exponential_search<T>(arr: &[T], target: &T) -> Result<usize, usize>
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