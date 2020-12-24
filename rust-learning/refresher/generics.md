# trait

```
trait Foo {
    fn run(&mut self) {
        println!("Foo::run()");
    }
}

fn run_base(foo: &mut Foo) {
    println!("run_base()");
}

struct Bar;

impl Foo for Bar {
    fn run(&mut self) {
        run_base(self);
        println!("Bar::run()");
    }
}

fn main() {
    let mut bar = Bar;
    bar.run();
}
```









## PhantomData 