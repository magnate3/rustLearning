# collections


## hash

```
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

macro_rules! seq_to_map {
    ($seq:expr) => {{
        let mut map = HashMap::<_, _, RandomState>::default();
        let mut iter = $seq.into_iter();
        while let Some(e) = iter.next() {
            map.entry(e).or_insert(iter.next().unwrap_or_default());
        }
        map
    }};
}

fn seq_to_map<T: IntoIterator>(vec: T) -> HashMap<T::Item, T::Item>
where
    T::Item: Hash + Default + Eq,
{
    let mut map = HashMap::default();
    let mut iter = vec.into_iter();
    while let Some(e) = iter.next() {
        map.entry(e).or_insert(iter.next().unwrap_or_default());
    }
    map
}

fn main() {

	let mut v=Vec::new();
	for i in "a1b2c3d4e5f6g7h8i9j".chars(){
		v.push(i)
	}
	let h=seq_to_map(v);
	println!("{:?}",h);
	//vec.chunks_exact(2).map(|chunk| (chunk[0], chunk[1])).collect::<HashMap::<_, _>>()

}

```

```
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}

let person1 = Person {
    id: 5,
    name: "Janet".to_string(),
    phone: 555_666_7777,
};
let person2 = Person {
    id: 5,
    name: "Bob".to_string(),
    phone: 555_666_7777,
};

assert!(calculate_hash(&person1) != calculate_hash(&person2));

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
```

```
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Person {
    id: u32,
    name: String,
    phone: u64,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.phone.hash(state);
    }
}

let person1 = Person {
    id: 5,
    name: "Janet".to_string(),
    phone: 555_666_7777,
};
let person2 = Person {
    id: 5,
    name: "Bob".to_string(),
    phone: 555_666_7777,
};

assert_eq!(calculate_hash(&person1), calculate_hash(&person2));

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
```