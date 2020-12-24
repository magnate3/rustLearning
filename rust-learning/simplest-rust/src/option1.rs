#![feature(option_expect_none)]
fn main() {
    let mut outtext: String = String::new();
    let text: Option<String> = Some("Text-as_ref()".to_string());
    println!("{:?}", text);
    let what = text.as_ref();
    let mut text_length: Option<usize> = text.as_ref().map(|s| {
        println!("In closure: {:?}", s);
        outtext = s.to_string();
        s.len()
    });
    println!(
        "outtext: {}; from text_length: Option<usize>: {:?}",
        outtext, text_length
    );
    println!("what=text.as_ref(): {:?}", what);
    println!("{:?}", text_length);
    println!("after text: {:?}", text);
    println!("");

    match text_length.as_mut() {
        Some(x) => *x = 0,
        None => {}
    }
    println!("text_length after as_mut {:?}", text_length);

    use std::collections::HashMap;
    let mut squares = HashMap::new();
    for i in -10..=10 {
        // This will not panic, since all keys are unique.
        squares.insert(i, i * i).expect_none("duplicate key");
    }
    let mut i: u32 = 0;
    for (one, two) in squares {
        println!("{}: {} {} ", i, one, two);
        i = i + 1;
    }
}
