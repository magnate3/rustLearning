type Result<Err> = Result<T, Err>;
fn apply(val: i32) -> Result<()> {
		if val {
				 println!(" ok");
				return Ok(());
		}
		else {
				 println!(" error ");
				Err("Some error message");
		}
		Err("Some error message");
}
fn main() {

		let x: Result<u32, &str> = Ok(2);
		assert_eq!(x.ok(), Some(2));
		let x: Result<u32, &str> = Err("Nothing here");
		assert_eq!(x.ok(), None);
		apply(2);
		apply(0);
}
