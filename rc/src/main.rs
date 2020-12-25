use std::rc::Rc;

struct Owner {
    name: String
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>
}

fn main() {
    // Create a reference counted Owner.
    let gadget_owner : Rc<Owner> = Rc::new(
        Owner { name: String::from("Gadget Man") }
    );

    // Create Gadgets belonging to gadget_owner. To increment the reference
    // count we clone the `Rc<T>` object.
    let gadget1 = Gadget { id: 1, owner: gadget_owner.clone() };
    let gadget2 = Gadget { id: 2, owner: gadget_owner.clone() };

    drop(gadget_owner);

    // Despite dropping gadget_owner, we're still able to print out the name
    // of the Owner of the Gadgets. This is because we've only dropped the
    // reference count object, not the Owner it wraps. As long as there are
    // other `Rc<T>` objects pointing at the same Owner, it will remain
    // allocated. Notice that the `Rc<T>` wrapper around Gadget.owner gets
    // automatically dereferenced for us.
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // At the end of the method, gadget1 and gadget2 get destroyed, and with
    // them the last counted references to our Owner. Gadget Man now gets
    // destroyed as well.
}