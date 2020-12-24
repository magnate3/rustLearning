
# [How to include a module from another file from the same project?](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project)

[Ask Question](https://stackoverflow.com/questions/ask)

Asked  5 years, 5 months ago

Active  [1 month ago](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project?lastactivity "2020-03-02 21:04:27Z")

Viewed  64k times

107

18

[](https://stackoverflow.com/posts/26388861/timeline "Timeline")

By following  [this guide](https://doc.rust-lang.org/0.12.0/guide.html#crates-and-modules)  I created a Cargo project.

**`src/main.rs`**

```
fn main() {
    hello::print_hello();
}

mod hello {
    pub fn print_hello() {
        println!("Hello, world!");
    }
}

```

which I run using

```
cargo build && cargo run

```

and it compiles without errors. Now I'm trying to split the main module in two but cannot figure out how to include a module from another file.

My project tree looks like this

```
├── src
    ├── hello.rs
    └── main.rs

```

and the content of the files:

**`src/main.rs`**

```
use hello;

fn main() {
    hello::print_hello();
}

```

**`src/hello.rs`**

```
mod hello {
    pub fn print_hello() {
        println!("Hello, world!");
    }
}

```

When I compile it with  `cargo build`  I get

```
error[E0432]: unresolved import `hello`
 --> src/main.rs:1:5
  |
1 | use hello;
  |     ^^^^^ no `hello` external crate

```

I tried to follow the compiler's suggestions and modified  `main.rs`  to:

```
#![feature(globs)]

extern crate hello;

use hello::*;

fn main() {
    hello::print_hello();
}

```

But this still doesn't help much, now I get this:

```
error[E0463]: can't find crate for `hello`
 --> src/main.rs:3:1
  |
3 | extern crate hello;
  | ^^^^^^^^^^^^^^^^^^^ can't find crate

```

Is there a trivial example of how to include one module from the current project into the project's main file?

[rust](https://stackoverflow.com/questions/tagged/rust "show questions tagged 'rust'")

[share](https://stackoverflow.com/q/26388861/8146671 "short permalink to this question")[edit](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project# "revise and improve this post")follow

[edited  Mar 2 at 21:04](https://stackoverflow.com/posts/26388861/revisions "show all edits to this post")

[](https://stackoverflow.com/users/155423/shepmaster)

![](https://www.gravatar.com/avatar/419218774d04a581476ea1887a0921e0?s=32&d=identicon&r=PG)

[Shepmaster](https://stackoverflow.com/users/155423/shepmaster)

211k3131 gold badges512512 silver badges690690 bronze badges

asked  Oct 15 '14 at 17:47

[](https://stackoverflow.com/users/1509733/ave)

![](https://www.gravatar.com/avatar/b999a449f3dcbf15ff4278738bdbd47a?s=32&d=identicon&r=PG&f=1)

[ave](https://stackoverflow.com/users/1509733/ave)

11.2k66 gold badges1919 silver badges3333 bronze badges

-   1
    
    possible duplicate of  [Rust basic imports (includes)](http://stackoverflow.com/questions/26224947/rust-basic-imports-includes)  – [Levans](https://stackoverflow.com/users/2536143/levans "10,358 reputation")  Oct 15 '14 at 18:52
    
-   Related to  [stackoverflow.com/questions/22596920/…](http://stackoverflow.com/questions/22596920/split-a-module-across-several-files "split a module across several files")  – [Kelvin](https://stackoverflow.com/users/498594/kelvin "16,130 reputation")  Jul 31 '15 at 15:50
    

[add a comment](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project# "Use comments to ask for more information or suggest improvements. Avoid answering questions in comments.")

## 3 Answers

[Active](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project?answertab=active#tab-top "Answers with the latest activity first")[Oldest](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project?answertab=oldest#tab-top "Answers in the order they were provided")[Votes](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project?answertab=votes#tab-top "Answers with the highest score first")

199

[](https://stackoverflow.com/posts/26390046/timeline "Timeline")

You don't need the  `mod hello`  in your  `hello.rs`  file. Code in any file but the crate root (`main.rs`  for executables,  `lib.rs`  for libraries) is automatically namespaced in a module.

To include the code from  `hello.rs`  in your  `main.rs`, use  `mod hello;`. It gets expanded to the code that is in  `hello.rs`  (exactly as you had before). Your file structure continues the same, and your code needs to be slightly changed:

**`main.rs`**:

```
mod hello;

fn main() {
    hello::print_hello();
}

```

**`hello.rs`**:

```
pub fn print_hello() {
    println!("Hello, world!");
}

```

[share](https://stackoverflow.com/a/26390046/8146671 "short permalink to this answer")[edit](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project# "revise and improve this post")follow

[edited  Mar 2 at 21:04](https://stackoverflow.com/posts/26390046/revisions "show all edits to this post")

[](https://stackoverflow.com/users/155423/shepmaster)

![](https://www.gravatar.com/avatar/419218774d04a581476ea1887a0921e0?s=32&d=identicon&r=PG)

[Shepmaster](https://stackoverflow.com/users/155423/shepmaster)

211k3131 gold badges512512 silver badges690690 bronze badges

answered  Oct 15 '14 at 18:57

[](https://stackoverflow.com/users/667984/renato-zannon)

![](https://www.gravatar.com/avatar/0425d6234a21e166e8cc37878d0e87cd?s=32&d=identicon&r=PG)

[Renato Zannon](https://stackoverflow.com/users/667984/renato-zannon)

21.4k66 gold badges2828 silver badges3535 bronze badges

-   1
    
    Late Question wouldn't it also work if I specify it with use hello instead of mod hello?!  – [Christian Schmitt](https://stackoverflow.com/users/2250209/christian-schmitt "595 reputation")  Sep 16 '15 at 17:23
    
-   15
    
    @ChristianSchmitt No, they are different things.  `use`  is just a namespace thing, while  `mod`  pulls in the file. You would use  `use`, for example, to be able to call the  `print_hello`  function without having to prefix with the namespace  – [Renato Zannon](https://stackoverflow.com/users/667984/renato-zannon "21,387 reputation")  Sep 16 '15 at 19:08
    

[add a comment](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project# "Use comments to ask for more information or suggest improvements. Avoid comments like “+1” or “thanks”.")

12

[](https://stackoverflow.com/posts/49476448/timeline "Timeline")

If you wish to have nested modules...

# Rust 2018

It's  [no longer  _required_](https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html#no-more-modrs)  to have the file  `mod.rs`  (although it is still supported). The idiomatic alternative is to name the file the name of the module:

```
$ tree src
src
├── main.rs
├── my
│   ├── inaccessible.rs
│   └── nested.rs
└── my.rs

```

**`main.rs`**

```
mod my;

fn main() {
    my::function();
}

```

**`my.rs`**

```
pub mod nested; // if you need to include other modules

pub fn function() {
    println!("called `my::function()`");
}

```

# Rust 2015

You need to put a  `mod.rs`  file inside your folder of the same name as your module.  [Rust by Example](https://doc.rust-lang.org/1.41.1/rust-by-example/mod/split.html)  explains it better.

```
$ tree src
src
├── main.rs
└── my
    ├── inaccessible.rs
    ├── mod.rs
    └── nested.rs

```

**`main.rs`**

```
mod my;

fn main() {
    my::function();
}

```

**`mod.rs`**

```
pub mod nested; // if you need to include other modules

pub fn function() {
    println!("called `my::function()`");
}

```

[share](https://stackoverflow.com/a/49476448/8146671 "short permalink to this answer")[edit](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project# "revise and improve this post")follow

[edited  Mar 2 at 20:59](https://stackoverflow.com/posts/49476448/revisions "show all edits to this post")

[](https://stackoverflow.com/users/155423/shepmaster)

![](https://www.gravatar.com/avatar/419218774d04a581476ea1887a0921e0?s=32&d=identicon&r=PG)

[Shepmaster](https://stackoverflow.com/users/155423/shepmaster)

211k3131 gold badges512512 silver badges690690 bronze badges

answered  Mar 25 '18 at 13:33

[](https://stackoverflow.com/users/6520591/amxa)

![](https://www.gravatar.com/avatar/f0c4110bc86b48dcccce53430301914b?s=32&d=identicon&r=PG&f=1)

[amxa](https://stackoverflow.com/users/6520591/amxa)

19133 silver badges55 bronze badges

-   3
    
    Suppose I wanted to use something from  `inaccessible.rs`  in  `nested.rs`... how would I do that?  – [Heman Gandhi](https://stackoverflow.com/users/5292630/heman-gandhi "1,155 reputation")  Jun 30 '19 at 20:00
    
-   To access a sibling .rs file from a file other than main.rs, use the path attribute. So, at the top of nested.rs, add the following:  `#[path = "inaccessible.rs"]`  and on the next line:  `mod inaccessible;`  – [Gardener](https://stackoverflow.com/users/4983398/gardener "2,250 reputation")  Jul 24 '19 at 11:26
    
-   @Gandhi See  [The path attribute](https://doc.rust-lang.org/reference/items/modules.html)  – [Gardener](https://stackoverflow.com/users/4983398/gardener "2,250 reputation")  Jul 24 '19 at 11:41
    
-   2
    
    @HemanGandhi add  `mod inaccessible;`  to  `my/mod.rs`  to make it submodule of  `my`, then access sibling module from  `nested.rs`  by relative path  `super::inaccessible::function()`. you dont need  `path`  attribute here.  – [artin](https://stackoverflow.com/users/1064570/artin "1,115 reputation")  Nov 14 '19 at 15:59
    

[add a comment](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project# "Use comments to ask for more information or suggest improvements. Avoid comments like “+1” or “thanks”.")

0

[](https://stackoverflow.com/posts/59692102/timeline "Timeline")

I really like Gardener's response. I've been using the suggestion for my module declarations. Someone please chime in if there is a technical issue with this.

```
./src
├── main.rs
├── other_utils
│   └── other_thing.rs
└── utils
    └── thing.rs

```

main.rs

```
#[path = "utils/thing.rs"] mod thing;
#[path = "other_utils/other_thing.rs"] mod other_thing;

fn main() {
  thing::foo();
  other_thing::bar();
}

```

utils/thing.rs

```
pub fn foo() {
  println!("foo");
}

```

other_utils/other_thing.rs

```
#[path = "../utils/thing.rs"] mod thing;

pub fn bar() {
  println!("bar");
  thing::foo();
}

```

[share](https://stackoverflow.com/a/59692102/8146671 "short permalink to this answer")[edit](https://stackoverflow.com/questions/26388861/how-to-include-a-module-from-another-file-from-the-same-project# "revise and improve this post")follow

---

```
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        // 让我们从这个作用域中访问所有名为 `function` 的函数！
        print!("called `my::indirect_call()`, that\n> ");
        
        // `self` 关键字表示当前的模块作用域——在这个例子是 `my`。
        // 调用 `self::function()` 和直接调用 `function()` 都得到相同的结果，
        // 因为他们表示相同的函数。
        self::function();
        function();
        
        // 我们也可以使用 `self` 来访问 `my` 内部的另一个模块：
        self::cool::function();
        
        // `super` 关键字表示父作用域（在 `my` 模块外面）。
        super::function();
        
        // 这将在 *crate* 作用域内绑定 `cool::function` 。
        // 在这个例子中，crate 作用域是最外面的作用域。
        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}

```