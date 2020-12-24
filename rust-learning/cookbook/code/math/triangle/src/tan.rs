//Verifies tan(x) is equal to sin(x)/cos(x) for x = 6.
fn main() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();

    assert_eq!(a, b);
}

