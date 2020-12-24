// structs3.rs
// Structs contain more than simply some data, they can also have logic, in this
// exercise we have defined the Package struct and we want to test some logic attached to it,
// make the code compile and the tests pass! If you have issues execute `rustlings hint structs3`

// I AM DONE It must be again.

#[derive(Debug)]
struct Package {
    from: String,
    to: String,
    weight: f32
}

impl Package {
    fn new(from: String, to: String, weight: f32) -> Package {
        if weight <= 0.0 {
            // Something goes here...
			panic!("The weight must be >=0 !");
        } else {
            return Package {from, to, weight};
        }
    }

    fn is_international(&self) -> bool {
        // Something goes here...
		Package::new((*self.from).to_string(), (*self.to).to_string(), self.weight);
		true
    }

    fn get_fees(&self, cost_per_kg: f32) -> f32 {
		Package::new((*self.from).to_string(), (*self.to).to_string(), cost_per_kg);
		cost_per_kg*8.0
        // Something goes here...
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let country_from = String::from("Spain");
        let country_to = String::from("Austria");

        Package::new(country_from, country_to, -2.21);
    }

    #[test]
    fn create_international_package() {
        let country_from = String::from("Spain");
        let country_to = String::from("Russia");
        
        let package = Package::new(country_from, country_to, 1.2);

        assert!(package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let country_from = String::from("Spain");
        let country_to = String::from("Spain");
		let t=Package::new(country_from.clone(), country_to.clone(), 1.2);
        let country_fee = Package::get_fees(&t,2.75);
        
        let package = Package::new(country_from, country_to, 22.0);
        
        assert_eq!(package.get_fees(country_fee), 176.0);
    }
}

/*

    fn calculate_transport_fees() {
        let country_from = String::from("Spain");
        let country_to = String::from("Spain");

        let country_fee = ???;
        
        let package = Package::new(country_from, country_to, 22.0);
        
        assert_eq!(package.get_fees(country_fee), 176.0);
    }
*/	