// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Execute `rustlings hint option1` for hints :)

// Learning using Some(..)= 

//pub fn pop_too_much() -> bool {
    pub fn pop_too_much() ->bool{
    //Result
    let mut list = vec![3];
    //let last = Some(list.pop().unwrap();
    if let Some(last )= list.pop(){
    println!("The last item in the list is {:?}", last);
    } else{
        {}
    }


    if let Some(second_to_last) = list.pop(){
        println!(
            "The second-to-last item in the list is {:?}",
            second_to_last
        );
    } else {
        {}
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}
