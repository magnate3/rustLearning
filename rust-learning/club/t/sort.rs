
#![allow(unused_variables)]
fn main() {
fn sift_down(arr: &mut [i32], start: usize, end: usize) {
    let mut root = start;
    loop {
        let mut child = root * 2 + 1; // Get the left child   // 1
        if child > end {
            break;
        }
        if child + 1 <= end && arr[child] < arr[child + 1] {  // 2
            // Right child exists and is greater.
            child += 1;
        }

        if arr[root] < arr[child] {                           // 3
            // If child is greater than root, swap'em!
            arr.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}
}
