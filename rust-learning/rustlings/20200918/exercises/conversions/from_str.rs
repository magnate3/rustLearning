// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}


impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// I AM  DONE
// Steps:
// 1. If the length of the provided string is 0, then return an error
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. If the name is empty, then return an error
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`.
// If while parsing the age, something goes wrong, then return an error
// Otherwise, then return a Result of a Person object
impl FromStr for Person {
	
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
		let mut ss=s;
		if (ss.find(',')==None)||(ss.len()==0) {
			return Err((r#"Input is """#).to_string());
		}
		let v:Vec<&str>=ss.split(',').collect();
		if v.len()!=2 {
			return Ok(Person::default());
		}		
		let nstr=v[0];
		let num=v[1];
		if nstr.len()==0||num.len()==0{
			return Err((r#"Input is """#).to_string());
		}
		let rstr: String = num.chars().filter(|c| c.is_digit(10)).collect();
		if rstr.len()==0{
			return Err((r#"Input is """#).to_string());
		}
		let num: usize= rstr.parse().unwrap();
		//let agef=v[1];
		let p=        Person {
            name: String::from(v[0]),
            age: num,
        };
		Ok(p)
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John,".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_age() {
        "John,twenty".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_comma_and_age() {
        "John".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name() {
        ",1".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_age() {
        ",".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_invalid_age() {
        ",one".parse::<Person>().unwrap();
    }

}
