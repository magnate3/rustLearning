use std::rc::Rc;

struct Owner {
    name: String,
    // ...other fields
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
    // ...other fields
}

fn main() {
    let gadget_owner: Rc<Owner> = Rc::new(  //堆分配Owner, 产生一个Rc<Owner>计数
        Owner {
            name: "Gadget Man".to_string(),
        }
    );  //现在有一个Owner，在堆中，一个Rc<Owner>, 指针

    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),  //获得一个指向堆中Owner的Rc<Owner>，计数加一
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner), //获得指针，计数加一
    };  //现在有一个Owner, 三个Rc<Owner>

    drop(gadget_owner);  //std::mem::drop，销毁一个Rc<Owner>，内存Owner还在

   //剩余两个Rc<Owner>仍然指向Owner
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name); 
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

}  //gadget1, gadget2出局，Rc<Owner>归零，Owner内存释放