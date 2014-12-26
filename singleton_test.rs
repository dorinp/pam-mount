use singleton::Singleton;
mod singleton;

type MyVec = Vec<(String, String)>;

fn i<'r>() -> &'r mut MyVec {
	let z: &mut MyVec = Singleton::instance();
	z
}
static mut stack: Vec<String> = Vec::new();

fn main() {
	println!("{}", *i());
	hi();
	i().push(("bye".to_string(), "kitty".to_string()));	
	hi();
	println!("{}", *i());

	let o = i().iter().find(|& &(ref a, ref b)| { 
		println!("{}", a); 
		a == &"bye"
	});
	println!("{}", o);
}

fn hi() {
	i().push(("hello".to_string(), "kitty".to_string()));	
}
