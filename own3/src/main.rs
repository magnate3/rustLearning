fn push(vec: mut Vec<u32>) {
		    vec.push(1);
}

fn main(){
    let mut vec = vec![0, 1, 3, 5];
    push(mut vec);
    println!("{:?}", vec);
}
