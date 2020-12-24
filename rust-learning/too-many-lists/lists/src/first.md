
# 一个糟糕的单向链表栈

这将是_到此为止_最长的教程，因为我们需要基本上介绍Rust的所有内容，并且需要通过“强硬”的构建一些东西来更好的了解这门语言。

我们会把第一个列表放在  `src/first.rs`  中。我们需要告诉Rust  `first.rs`是我们的库使用的文件。只需要把下述代码放在  `src/lib.rs`  即可（Cargo已经为我们创建了）:

```
// in lib.rs
pub mod first;
```

## 基本布局

好吧，所以链表是什么东西呢？大致上说，它是一大片相互指向的数据，以顺序连接起来（嘘，Linux内核！）。链表是过程式程序员应该用一切代价避免的东西，而函数程序员则在所有情况下使用它。那么，看起来向函数式程序员询问链表的定义是很公平的。他们可能会给你类似这样的定义：

```
List a = Empty | Elem a (List a)

```

它可以大致读成“一个列表要么是空的，要么是一个元素接着一个列表”。这是用一个复合类型（sum type）表达的递归定义，而复合类型只是“一个可以拥有多种类型的值的类型”的酷炫叫法。Rust把复合类型称作`enum`！如果你是从一个C系语言过来的，那么这正是你所熟知并热爱的枚举类型，但是强大了许多。让我们把这个函数式的链表定义转录到Rust吧！

现在为了保持说明简单，我们会避开泛型。我们暂时只支持有符号的32位整数：

```
// 在 first.rs 中

// pub 表明我们想让这个模块外之外的人使用这个 List
pub enum List {
    Empty,
    Elem(i32, List),
}

```

呼，这一点也不麻烦嘛。我们继续下去编译它吧：

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/lists)
src/first.rs:1:1: 4:2 error: illegal recursive enum type; wrap the inner value in a box to make it representable [E0072]
src/first.rs:1 pub enum List {
src/first.rs:2    Empty,
src/first.rs:3    Elem(T, List),
src/first.rs:4 }
error: aborting due to previous error
Could not compile `lists`.

```

不！！！！函数式程序员欺骗了我们！它让我们做了_不合法_的东西！这是圈套！

……

我冷静下来了。你冷静了么？如果我们真正去检查错误消息（而不是像我们中的某些人一样，准备逃出这个国家），我们就会发现rustc实际上在告诉我们如何解决这个问题：

> illegal recursive enum type; wrap the inner value in a box to make it representable 不合法的递归枚举类型；把值包装在一个box里让它变得可表示 好吧，`box`。那是什么东西？让我们 google  `rust box`……
> 
> [std::boxed::Box - Rust](https://doc.rust-lang.org/std/boxed/struct.Box.html)

看看接下来是啥……

> `pub struct Box<T>(_);`
> 
> A pointer type for heap allocation. See the  [module-level documentation](https://doc.rust-lang.org/std/boxed/)  for more.

_点击链接_

> `Box<T>`，或被随意的称为`box`，提供了Rust中最简单的堆内存分配的形式。Box提供了当次内存分配的所有权，并在退出作用域时销毁存放的内容。
> 
> 示例
> 
> 创建一个box
> 
> `let x = Box::new(5);`
> 
> 创建一个递归数据结构：
> 
> ```
> #[derive(Debug)]
> enum List<T> {
>     Cons(T, Box<List<T>>),
>     Nil,
> }
> 
> ```
> 
> ```
> fn main() {
>     let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
>     println!("{:?}", list);
> }
> 
> ```
> 
> 这会打印  `Cons(1, Box(Cons(2, Box(Nil))))`.
> 
> 递归的结构必须使用box包装，因为如果Cons的定义如下这样：
> 
> `Cons(T, List<T>),`
> 
> 是不会工作的。这是因为List的大小由其中的元素数量所决定，所以我们无法决定为一个Cons分配多少内存。通过引入一个具有固定大小的Box，我们才知道Cons需要占用多少内存。

哇哦。这或许是我见过的最相关最有帮助的文档了。在里面的第一个东西就是_我们正在尝试写的东西，为什么它不能工作，以及如何修复它_。好耶，文档。

OK，我们来完成它：

```
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

```

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/lists)

```

嘿，它成功编译了！

……但这实际上是一个非常蠢的List的定义，出于以下的一些原因。

考虑一个拥有两个元素的列表：

```
[] = 栈
() = 堆

[Elem A, ptr] -> (Elem B, ptr) -> (Empty *垃圾*)

```

这里有两个关键问题：

-   我们创建了一个“实际上不是个节点”的节点
-   其中的一个节点根本没分配在堆里

在表面上，这两个问题似乎相互抵消。我们分配了一个多余的节点，但有一个节点完全无需在堆里分配。然而，考虑我们链表的一个潜在的内存布局：

```
[ptr] -> (元素A, ptr) -> (元素B, *null*)

```

在这个布局里，我们在堆里分配所有的元素。和第一个布局相比，核心的区别是多余的_垃圾_的消失。这个垃圾到底是什么？为了理解它，我们需要看一看enum是如何在内存中布局的。

通常的，如果我们有像这样的一个enum：

```
enum Foo {
    D1(T1),
    D2(T2),
    ...
    Dn(Tn),
}

```

一个Foo需要保存一个整数，来指出它实际表示的是哪一个_变体_（`D1`,  `D2`, ..  `Dn`）。这是enum的_标签_（tag）。它也需要足够大的空间，来存储`T1`,  `T2`, ..  `Tn`中的最大元素（以及用来满足内存对齐要求的附加空间）。

一个很大的缺陷是，尽管`Empty`只存储了一位的信息，它却消耗了一个指针和一个元素的内存空间，因为它要随时准备成为一个`Elem`。因此第一种布局在堆里分配了一个充满垃圾的多余元素，比第二种布局消耗更多的空间。

让我们的一个元素不在堆中分配，或许也比所有元素都在堆中分配更糟。这是因为它给了我们一个_不一致的_节点内存布局。在推入和弹出节点时这并无问题，但在分割和合并列表时确实会有影响。

考虑在两种布局里分别分割一个列表：

```
布局 1：

[Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr) -> (Empty *junk*)

在C点分割：

[Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)
[Elem C, ptr] -> (Empty *junk*)

```

```
布局 2：

[ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)

在C点分割：

[ptr] -> (Elem A, ptr) -> (Elem B, *null*)
[ptr] -> (Elem C, *null*)

```

布局2的分割仅仅涉及将B的指针拷贝到栈上，并把原值设置为null。布局1最终还是做了同一件事，但是还得把C从堆中拷贝到栈中。反过来执行上述操作，就是合并列表。

链表的优点之一就是可以在节点中构建元素，然后在列表中随意调换它的位置而不需移动它的内存。你只需要调整指针，元素就被“移动了”。第一个布局毁掉了这个特点。

好吧，我现在很确信布局1是糟糕的。我们要怎么重写List呢？可以这么做：

```
pub enum List {
    Empty,
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<List>),
}

```

或许你觉得这看起来更糟了。一个问题是，这让逻辑变得更复杂了。具体地说，现在出现了一个完全无效的状态：`ElemThenNotEmpty(0, Box(Empty))`。它也_仍_被内存分配模式不一致的问题所困扰。

不过它确实有_一个_有趣的特性：它完全避免了在堆里分配Empty，让堆内存分配的数量减少了1。不幸的是，这么做反而浪费了_更多空间_！因为之前的布局利用了_空指针优化_。

我们之前了解到每个enum需要存储一个_标签_，来指明它代表哪一个enum的_变体_。然而，如果我们有如下特殊类型的enum：

```
enum Foo {
    A,
    B(ContainsANonNullPtr),
}

```

空指针优化就会发挥作用，_消除标签所占用的内存空间_。如果变体是A，那整个enum就被设置为0。否则，变体是B。这可以工作，因为B存放了一个非空指针，永远不可能为0。真聪明！

你还能想到能进行这种优化的enum和类型么？实际上有很多！这就是为什么Rust没有详细描述enum的内存布局。悲伤的是，现在实现的优化只有空指针优化——尽管它很重要！这意味着`&`,  `&mut`,  `Box`,  `Rc`,  `Arc`,  `Vec`，以及其他一些Rust中的重要类型，在放到一个  `Option`  中时没有多余开销！（上面这些概念的大部分，我们在适当的时候都会接触到）

所以我们要如何避免多余垃圾，统一的分配内存，_并且_从空指针优化中获益呢？我们需要更好的将存储元素和分配列表这两个想法分开。要做到它，我们该像C语言看齐：struct！

enum让我们定义了一种可以存放多个变体中的一个的类型，而struct则让我们定义可以同时存放多种元素的类型。让我们把List分成两个类型吧：一个List，和一个Node。

和之前一样，一个List要么是Empty，要么是一个元素跟着一个List。不过，要通过另外一种类型来表示“一个元素跟着一个List”，我们可以将Box提升到一个更理想的位置：

```
struct Node {
    elem: i32,
    next: List,
}

pub enum List {
    Empty,
    More(Box<Node>),
}

```

让我们检查各个条目：

-   列表末尾不分配多余垃圾：通过！
-   `enum`  享受美妙的空指针优化：通过！
-   所有元素的内存分配一致：通过！

好的！我们创建的正是用来指明第一种内存布局（官方Rust文档中所建议的那种）有问题的第二种内存布局。

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/lists)
src/first.rs:8:11: 8:18 error: private type in exported type signature
src/first.rs:8    More(Box<Node>),
                           ^~~~~~~
error: aborting due to previous error
Could not compile `lists`.

```

:(

Rust又对我们发飙了。我们将`List`标记为public（因为我们想让其他人使用它），却没有公开`Node`。问题在于，`enum`的内部是完全公开的，所以在其中包含内部类型是不允许的。我们可以让整个`Node`都成为公开的，但是通常在Rust中，我们倾向于让实现细节私有化。让我们把`List`改造成一个struct，这样我们就可以隐藏实现细节：

```
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

```

因为`List`是一个单值的struct，它的大小和该值完全相同。零代价抽象超赞！

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/lists)
src/first.rs:2:2: 2:15 warning: struct field is never used: `head`, #[warn(dead_code)] on by default
src/first.rs:2    head: Link,
                  ^~~~~~~~~~~~~
src/first.rs:6:2: 6:7 warning: variant is never used: `Empty`, #[warn(dead_code)] on by default
src/first.rs:6    Empty,
                  ^~~~~
src/first.rs:7:2: 7:20 warning: variant is never used: `More`, #[warn(dead_code)] on by default
src/first.rs:7    More(Box<Node>),
                  ^~~~~~~~~~~~~~~~~~
src/first.rs:11:2: 11:9 warning: struct field is never used: `elem`, #[warn(dead_code)] on by default
src/first.rs:11   elem: i32,
                  ^~~~~~~
src/first.rs:12:2: 12:15 warning: struct field is never used: `next`, #[warn(dead_code)] on by default
src/first.rs:12   next: Link,
                  ^~~~~~~~~~~~~

```

好吧，终于编译了！Rust非常生气，因为我们现在写的东西完全无用：我们从不使用`head`，并且因为它是私有的；使用我们库的人也无法使用它。进而Link和Node也毫无用处。让我们来解决它吧！为我们的List实现一些代码！


## 创建

我们使用`impl`语句块把实际代码关联到一个类型上：

```
impl List {
    // TODO：填充代码
}

```

现在我们只需要了解实际编写代码的方法。在Rust我们像这样声明一个函数：

```
fn foo(arg1: Type1, arg2: Type2) -> ReturnType {
    // body
}

```

我们想要的第一件事是_构造_一个列表的方法。由于我们需要隐藏实现细节，需要以函数的形式提供它。在Rust中，创建新对象的通常方法是实现一个`impl`块中的普通静态函数：

```
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
}

```

一些关键点：

-   Self是“我写在impl右侧的那个类型”的别名。不用重复真是太好了！
-   创建一个struct的实例的语法和声明struct的语法基本相同，只是我们在每个字段的后面提供的是值而非它的类型。
-   我们使用命名空间运算符`::`来访问enum的变体。
-   函数的最后一个表达式被隐式的返回。这让简单的函数看起来更简洁。你仍然可以像其他C系语言一样，用`return`来提前返回。


## 所有权入门

在我们能够创建一个列表之后，自然会想用它进行实际操作。我们使用“普通”（非静态）方法来实现这一点。方法在Rust中是一种特殊的函数，它的第一个参数是self，并且没有类型声明：

```
fn foo(self, arg2: Type2) -> ReturnType {
    // body
}

```

主要存在3种self可以采用的形式：`self`,  `&mut self`  和  `&self`。它们代表了Rust中所有权的三种主要形式：

-   `self`  - 值（value）
-   `&mut self`  - 可变引用（mutable reference）
-   `&self`  - 共享引用（shared reference）

一个值代表了_真正的_所有权。你可以对值做你想做的任何事：移动它，销毁它，改变它的内容，或者通过一个引用借出它。当你通过值传递东西时，它就被_移动_到了新的位置。这个新位置现在拥有了这个值，并且老位置不能再访问该值。因此，对于大部分函数我们都不想使用`self`——如果调用函数让我们无法再访问它，那还真是很糟糕啊。

一个可变引用代表了对你不拥有的一个值的临时_唯一访问_权。当你拥有一个可变引用时，你被允许做几乎任何想做的事，只要满足该引用过期时，被借用者仍然可以看见合法的值。这意味着你可以完全覆盖这个值。一个有用的特殊情况是把这个值和另外一个做交换——我们会经常使用这一技巧。唯一不能对`&mut`做的一件事是不加替换的将它的值移出。对于要对`self`加以修改的方法，`&mut self`是极好的。

一个共享引用代表对你不拥有的值的临时_共享访问_。由于访问是共享的，通常改变任何内容是不被允许的。可以把`&`想作把值丢到博物馆里用于展览。如果我们只想观察`self`的值，`&`是很好用的。

晚些我们会见到在一些特殊情况下，允许进行值的修改的规则。这就是为什么共享引用不被称为_不可变引用_。实际上，可变引用还可以称为_唯一引用_，但是我们发现在99%的情况下，将可变性和所有权联系起来可以带来正确的直觉。


## Push

我们来实现将一个值推入列表的功能吧。`push`_改变了_列表，因此我们需要`&mut self`。还需要提供一个推入的i32参数：

```
impl List {
    pub fn push(&mut self, elem: i32) {
        // TODO
    }
}

```

首先，我们需要创建一个节点来存储我们的元素：

```
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: ?????
        };
    }

```

`next`那里存储的是什么呢？呃，是整个原先的列表！我们...可以这么做么？

```
impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: self.head,
        };
    }
}

```

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/lists)
src/first.rs:19:10: 19:14 error: cannot move out of borrowed content
src/first.rs:19           next: self.head,
                                ^~~~
error: aborting due to previous error
Could not compile `lists`.

```

不不不。Rust在告诉我们正确的事，但是它的实际意义和修复它的方法并不明了：

> cannot move out of borrowed content （无法移动出租借的内容）

我们尝试把`self.head`字段移动到`next`中，但是Rust不想让我们做这件事。这会导致在我们的租借结束，值返回到其所有者的时候，`self`只是部分初始化的。正如我们之前所说，这是你不能通过`&mut`做的_唯一一件事_：这样做是非常粗野的，而Rust则打算尽量保持礼貌（同时这也是极其危险的，但是并不是Rust关心它的原因）。

如果我们把东西放回去呢？具体的说，就是我们创建的节点：

```
pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
        elem: elem,
        next: self.head,
    });

    self.head = Link::More(new_node);
}

```

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/lists)
src/first.rs:19:10: 19:14 error: cannot move out of borrowed content
src/first.rs:19           next: self.head,
                                ^~~~
error: aborting due to previous error
Could not compile `lists`.

```

不行。在原则上，Rust是可以接受这样的行为的，但是它不会（出于数个原因——最重要的是安全性）我们需要某种方法得到head，而不让Rust发现它已经消失了。我们转而向声名狼藉的Rust黑客Indiana Jones寻求建议：

![Indy准备进行mem::replace](https://weathfold.gitbooks.io/rust-too-many-lists-zhcn/content/indy.gif)

噢没错，Indy建议采用`mem::replace`这一招。这个极其有用的函数让我们通过_替换_将一个值从租借中偷出来。让我们先在文件顶部把`std::mem`拉进来，这样`mem`就在局部作用域了：

```
use std::mem;

```

然后恰当的使用它：

```
pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
        elem: elem,
        next: mem::replace(&mut self.head, Link::Empty),
    });

    self.head = Link::More(new_node);
}

```

在将self.head替换为列表的新头部之前，我们临时的将它`replace`为Link::Empty。我不会撒谎：非要这么去做真是很糟糕。悲伤的是，我们必须这样（至少现在）。

不过，这样子`push`就完成了！或许吧。说真的，我们应该测试一下它。现在来说，最简单的测试方法是实现`pop`，然后确认它给出了正确的结果。



## Pop

和`push`一样，`pop`想要改变列表；除此之外，我们还想返回结果。然而`pop`还得处理一个特殊的边界情况：如果列表是空的呢？为了表示这个情况，我们使用可靠的`Option`类型：

```
pub fn pop(&mut self) -> Option<i32> {
    //TODO
}

```

`Option<T>`是一个表示一个值可能存在也可能不存在的enum。它要么是`Some(T)`，要么是`None`。我们也可以像Link一样创建一个自己的enum，但是我们想让用户了解我们的返回类型到底是什么，而Option是如此的无处不在，每个人都知道它。实际上，因为它是如此的基本，它被隐式的导入到了每一个源文件的作用域中，也包括它的两个变体：`Some`和`None`（这样我们就不用写`Option::None`）。

在`Option<T>`尖括号里的部分指出Option实际上是一个泛型，它的泛型参数是T。这意味着你可以创建一个任何类型的Option！

所以，我们有这个`Link`了，我们怎么检查它是Empty还是More呢？使用`match`进行模式匹配：

```
pub fn pop(&mut self) -> Option<i32> {
    match self.head {
        Link::Empty => {
            // TODO
        }
        Link::More(node) => {
            // TODO
        }
    };
}

```

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/lists)
src/first.rs:27:2: 36:3 error: not all control paths return a value [E0269]
src/first.rs:27   pub fn pop(&mut self) -> Option<i32> {
src/first.rs:28       match self.head {
src/first.rs:29           Link::Empty => {
src/first.rs:30               // TODO
src/first.rs:31           }
src/first.rs:32           Link::More(node) => {
              ...
error: aborting due to previous error
Could not compile `lists`.

```

啊，`pop`必须返回一个值，我们还没做这件事。我们_可以_直接返回`None`，但是在这情况下，返回`unimplemented!`来指出我们没有完成该函数的实现会更好。`unimplemented!`是一个宏（`!代表一个宏`），它会在被调用的时候让整个程序panic（基本上也就是以可控的方式崩溃）。

```
pub fn pop(&mut self) -> Option<i32> {
    match self.head {
        Link::Empty => {
            // TODO
        }
        Link::More(node) => {
            // TODO
        }
    };
    unimplemented!()
}

```

无条件panic是一个[发散函数(diverging function)](http://doc.rust-lang.org/nightly/book/functions.html#diverging-functions)的例子。发散函数永远不会返回到调用者，所以无论一个地方期待何种类型的返回值，它的返回值都能拿来用。在这里，`unimplemented!`被使用在期待一个`Option<T>`的地方。

注意到我们不需要在程序里写`return`。函数中的最后一个表达式也就隐式的成为它的返回值。这让我们可以更精炼的表达简单的逻辑。你也可以像C系语言一样，显式的`return`返回。

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/lists)
src/first.rs:28:9: 28:13 error: cannot move out of borrowed content
src/first.rs:28       match self.head {
                            ^~~~
src/first.rs:32:15: 32:19 note: attempting to move value to here
src/first.rs:32           Link::More(node) => {
                                     ^~~~
src/first.rs:32:15: 32:19 help: to prevent the move, use `ref node` or `ref mut node` to capture value by reference
error: aborting due to previous error
Could not compile `lists`.

```

噢，Rust，别纠缠不休了！和往常一样，Rust对我们非常生气。值得感谢的是，这次它还给出了深入的信息！

```
src/first.rs:17:9: 17:13 error: cannot move out of borrowed content
src/first.rs:17       match self.head {
                            ^~~~
src/first.rs:21:15: 21:19 note: attempting to move value to here
src/first.rs:21           Link::More(node) => {
                                     ^~~~

```

模式匹配默认会移动匹配的值，所以这就是它如此生气的原因。

```
help: to prevent the move, use `ref node` or `ref mut node` to capture value by reference
帮助：要避免移动，使用`ref node`或`ref mut node`来引用捕获该值

```

为了避免移动，我们使用`ref`关键词来指明我们想要把`node`进行引用绑定。来试试吧：

```
pub fn pop(&mut self) -> Option<i32> {
    match self.head {
        Link::Empty => {
            // TODO
        }
        Link::More(ref node) => {
            // TODO
        }
    };
    unimplemented!()
}

```

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/lists)
src/first.rs:13:2: 13:9 warning: struct field is never used: `elem`, #[warn(dead_code)] on by default
src/first.rs:13   elem: i32,
                  ^~~~~~~
src/first.rs:14:2: 14:15 warning: struct field is never used: `next`, #[warn(dead_code)] on by default
src/first.rs:14   next: Link<T>,
                  ^~~~~~~~~~~~~
src/first.rs:32:15: 32:23 warning: unused variable: `node`, #[warn(unused_variables)] on by default
src/first.rs:32           Link::More(ref node) => {
                                     ^~~~~~~~

```

好耶，又编译了！现在让我们搞清楚实现逻辑。我们要创建一个Option，所以要为这个预留一个变量。在Empty情况下要返回None，在More情况下需要返回`Some(i32)`，并且改变列表的head。来吧：

```
pub fn pop(&mut self) -> Option<i32> {
    let result;
    match self.head {
        Link::Empty => {
            result = None;
        }
        Link::More(ref node) => {
            result = Some(node.elem);
            self.head = node.next;
        }
    };
    result
}

```

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/too-many-lists/lists)
src/first.rs:39:29: 39:33 error: cannot move out of borrowed content
src/first.rs:39                 self.head = node.next;
                                            ^~~~
src/first.rs:39:17: 39:38 error: cannot assign to `self.head` because it is borrowed
src/first.rs:39                 self.head = node.next;
                                ^~~~~~~~~~~~~~~~~~~~~
src/first.rs:37:24: 37:32 note: borrow of `self.head` occurs here
src/first.rs:37             Link::More(ref node) => {
                                       ^~~~~~~~
error: aborting due to 2 previous errors
Could not compile `lists`.

```

_头_

_桌_

我们现在有了两个不同的的错误。。首先，我们在仅仅拥有一个共享引用的情况下就尝试把值移动出`node`。其次，在我们已经租借了`node`的引用的时候，还在尝试改变`self.head`的值！

真是一堆纠缠不清的乱东西。

我们应该后退一步，思考我们要做什么。我们想要：

-   检查列表是否为空。
-   如果是空的，返回None
-   如果是非空
    -   移除list头部
    -   移除该头部的`elem`
    -   将列表的head替换为`next`
    -   返回`Some(elem)`

重要的一点事我们想要_删除_东西，这意味着我们需要_按值_获取list的head。我们肯定不能通过由`ref node`获取的共享引用来做这件事。我们也“只”拥有一个可变引用，所以能移动东西的唯一方法就是_替换它_。看来我们又在做Empty替换那一套了！来试试吧：

```
pub fn pop(&mut self) -> Option<i32> {
    let result;
    match mem::replace(&mut self.head, Link::Empty) {
        Link::Empty => {
            result = None;
        }
        Link::More(node) => {
            result = Some(node.elem);
            self.head = node.next;
        }
    };
    result
}

```

```
cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/too-many-lists/lists)

```

我 的 天 哪

它编译了，一个警告都没有！！！！！

这里我要给出我的优化提示了：我们现在返回的是result变量的值，但实际上根本不用这么做！就像一个函数的结果是它的最后一个表达式，每个代码块的结果也是它的最后一个表达式。通常我们使用分号来阻止这一行为，这会让代码块的值变成空元组（tuple）`()`。这实际上也是不声明返回值的函数——例如`push`——返回的。

因此，我们可以将`pop`修改为：

```
pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Link::Empty) {
        Link::Empty => None,
        Link::More(node) => {
            self.head = node.next;
            Some(node.elem)
        }
    }
}

```

这更简洁，也更符合语言惯例。注意到Link::Empty分支只需要求值一个表达式，所以我们把大括号也去掉了。这是对于简单情况的简便处理。

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/too-many-lists/lists)
src/first.rs:36:22: 36:31 error: use of moved value: `node` [E0382]
src/first.rs:36                 Some(node.elem)
                                     ^~~~~~~~~
src/first.rs:35:29: 35:38 note: `node` moved here (through moving `node.next`) because it has type `first::Link`, which is non-copyable
src/first.rs:35                 self.head = node.next;
                                            ^~~~~~~~~
error: aborting due to previous error

```

啥？别这样啊。

为什么我们的代码不工作了？！

实际上，我们之前的代码只是侥幸通过了编译，借助Copy的魔法。当我们介绍[所有权](https://weathfold.gitbooks.io/rust-too-many-lists-zhcn/content/first-ownership.html)的时候说过，当你移动值的时候，就无法再使用它。对于某些类型，这是完全合理的。我们的好朋友Box为我们管理堆中的内存分配，而我们显然不想让两段代码认为它们应该释放相同的一块内存。

但是对某些类型这简直_糟透了_。整数可没有所有权语义：它们只是毫无意义的数字！这也正是为什么整数被标记为Copy。Copy类型可以通过按位复制进行完整的拷贝。因此，它们拥有一个超能力：当被移动的时候，老的值仍然是可用的。作为结果，你可以将一个Copy类型从引用移出而不需替换！

所有rust中的基本数字类型（i32, u64, bool, f32, char, etc...)都是Copy。同时，共享引用也是Copy，这很有用！只要一个自定类型的所有字段都是Copy，你也可以将该类型声明为Copy。

总之，回到代码：出了什么错？在第一次迭代过程中，我们对result进行赋值时在进行_拷贝_，因此node没被改变，可以被继续用于下一个操作。现在我们在移动`next`（它不是Copy），而这在我们能碰到elem之前消耗掉了整个Box里的值。

现在，我们可以重新调整代码来只拿到`elem`，但我们只是使用i32作为某种数据的占位符。晚些时候我们会处理非Copy类型的数据，所以最好现在就研究研究怎么做。

正确的答案是将_整个_节点从Box中取出来，这样就可以安全的将它拆开了。我们通过显式解引用操作来做这件事：

```
pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Link::Empty) {
        Link::Empty => None,
        Link::More(boxed_node) => {
            let node = *boxed_node;
            self.head = node.next;
            Some(node.elem)
        }
    }
}

```

在这之后，Rust就可以足够好的理解一个栈上的值，来让你一步步分解它了。

```
> cargo build
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/too-many-lists/lists)

```

不错。

Box在Rust里真的很特殊，因为它是固定于语言里的一部分，编译器可以让你对它做一些其它任何类型都不能做的事。我们实际上一直在做这件事：`DerefMove`。当你拥有一个指针类型时，可以通过`*`或`.`来获得它的内容。通常你可以获得一个`Deref`或者一个`DerefMut`，分别对应共享和可变引用。

但是因为Box完全拥有它的内容，你可以通过解引用_将内容移出_。这是完完全全的魔法，因为其他任何类型都无法实现这个操作。编译器还知道如何在Box上实现很多很多的酷炫技巧，只因为它_就是_Box，但是它们都被阻挡在了1.0版本的实现目标之外，等待进一步的设计。理想的，Box会在未来完全可自定义化。


## 测试

我们现在已经实现了`push`和`pop`，就可以测试我们的栈了！Rust和cargo把测试作为一个一级特性来实现，所以写起测试来会很轻松。我们需要做的只是写一个函数，然后用`#[test]`标记它。

通常在Rust社区中，我们会把测试代码放在它所测试的部分的附近。不过我们通常会为测试创建单独的命名空间，来让它不与“真正的”代码产生冲突。就像我们用`mod`来表明`first.rs`应该被包含在`lib.rs`中，可以使用`mod`来_内联的_创建一个新文件：

```
// in first.rs

mod test {
    #[test]
    fn basics() {
        // TODO
    }
}

```

之后，我们通过`cargo test`调用它。

```
> cargo test
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/too-many-lists/lists)
     Running target/debug/lists-5c71138492ad4b4a

running 1 test
test first::test::basics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests lists

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

```

好的，我们的什么都不做的测试通过了！我们来让它实际做些事吧。我们会使用`assert_eq!`宏来进行测试。这不是什么特殊的测试魔法。它所做的仅仅是比较你给它的两个值，并且让在它们不相等的情况下让程序panic。没错，你通过崩溃来指出测试中的失败！

```
mod test {
    #[test]
    fn basics() {
        let mut list = List::new();

        // 检查空列表行为正确
        assert_eq!(list.pop(), None);

        // 填充列表
        list.push(1);
        list.push(2);
        list.push(3);

        // 检查通常的移除
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // 推入更多元素来确认没有问题
        list.push(4);
        list.push(5);

        // 检查通常的移除
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // 检查完全移除
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

```

```
> cargo test
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/too-many-lists/lists)
src/first.rs:47:24: 47:33 error: failed to resolve. Use of undeclared type or module `List` [E0433]
src/first.rs:47         let mut list = List::new();
                                       ^~~~~~~~~
src/first.rs:47:24: 47:33 error: unresolved name `List::new` [E0425]
src/first.rs:47         let mut list = List::new();
                                       ^~~~~~~~~
error: aborting due to 2 previous errors

```

噢！因为我们做了一个新的模块，所以需要把List显式的导入进来才能使用它。

```
mod test {
    use super::List;
    // 其他不变
}

```

```
> cargo test
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/too-many-lists/lists)
src/first.rs:45:9: 45:20 warning: unused import, #[warn(unused_imports)] on by default
src/first.rs:45     use super::List;
                        ^~~~~~~~~~~
     Running target/debug/lists-5c71138492ad4b4a

running 1 test
test first::test::basics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests lists

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

```

好极了！

那个警告又是怎么回事呢……？我们清楚的在测试里用了List！

……但仅仅在测试的过程中！为了平息编译器（以及对库的使用者友好），我们应该指明test模块只会在运行测试的过程中编译。

```
#[cfg(test)]
mod test {
    use super::List;
    // everything else the same
}

```

这就是关于测试的所有要点了！


## Drop

我们现在可以创建一个栈，推入元素，弹出元素，甚至确认了一切都可以正常的工作！

我们需要担心列表元素的清理么？严格的说，根本不用！就像C++，Rust使用析构器来自动的处理使用完毕的资源。如果一个类型实现了叫做 Drop 的_特性（Trait）_，它就拥有一个析构器。特性是Rust对接口的特别术语。Drop特性有如下的接口：

```
pub trait Drop {
    fn drop(&mut self);
}

```

基本上是这个意思：“当对象退出作用域的时候，我会给你清理事务的第二次机会”。

如果你的类型里存放有实现了Drop的其他类型，而你想要调用它们的析构器，是不需要实际实现Drop的。对于List来说，我们想做的不过是把列表头丢弃，之后或许会接着丢弃一个`Box<Node>`。所有这些都会自动在一瞬间处理完成。

自动处理会很糟糕。

让我们考虑这个简单的列表。

```
list -> A -> B -> C

```

当列表被丢弃时，它会先丢弃A，然后尝试丢弃B，然后会尝试丢弃C。现在你可能已经紧张起来了。这是递归代码，而递归代码会把栈爆掉！

```
impl Drop for List {
    fn drop(&mut self) {
        // 注意：在实际Rust代码中你不能显式调用`drop`，
        // 我们假装自己是编译器！
        list.head.drop(); // 尾递归——好！
    }
}

impl Drop for Link {
    fn drop(&mut self) {
        match list.head {
            Link::Empty => {} // 完成！
            Link::More(ref mut boxed_node) => {
                boxed_node.drop(); // 尾递归——好！
            }
        }
    }
}

impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop(); // 糟糕，不是尾递归！
        deallocate(self.ptr);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
    }
}

```

我们不能在释放内存之后再丢弃Box的内容，所以没有办法以尾递归的形式进行drop！作为替代，我们必须为`List`手动编写一个迭代drop，来把节点从box中拿出来。

```
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == “在这个模式不匹配之前持续循环”
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node在这里退出作用域然后被丢弃；
            // 但是其节点的`next`字段被设置为 Link::Empty
            // 所以没有多层递归产生。
        }
    }
}

```

```
> cargo test
   Compiling lists v0.1.0 (file:///Users/ABeingessner/dev/too-many-lists/lists)
     Running target/debug/lists-5c71138492ad4b4a

running 1 test
test first::test::basics ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests lists

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

```

棒极了！

## 最终代码

好吧，在6000词之后，这是我们实际写成的所有代码：

```
use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                let node = *node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

```

天。80行，而且其中的一半都是测试！嘛，我确实说过这第一个教程会花一些时间的！