struct Foo<T> {
    v: Vec<T>,
}

impl<A> std::iter::FromIterator<A> for Foo<A> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=A> {
        let mut foo = Foo{ v: vec![] };
        for i in iter {
            foo.v.push(i);
        }
        foo
    }
}

 
fn main(){
let foo: Foo<_> = (0..10).collect();
// or
// let foo = (0..10).collect::<Foo<_>>();
println!("{:?}", foo.v);

}
