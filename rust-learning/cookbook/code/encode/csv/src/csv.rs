// Reads standard CSV records into csv::StringRecord — a weakly typed data representation which expects valid UTF-8 rows. Alternatively, csv::ByteRecord makes no assumptions about UTF-8.
extern crate csv;
use csv::Error;

fn main() -> Result<(), Error> {
    let csv = "year,make,model,description
        1948,Porsche,356,Luxury sports car
        1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0],
            &record[1],
            &record[2],
            &record[3]
        );
    }

    Ok(())
}

