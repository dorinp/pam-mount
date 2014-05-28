use singleton::Singleton;
mod singleton;

type MyVec = Vec<(~str, ~str)>;

fn i() -> &mut MyVec {
	let z: &mut MyVec = Singleton::instance();
	z
}

fn main() {
	println!("{}", *i());
	hi();
	hi();
	hi();
	println!("{}", *i());
}

fn hi() {
	i().push((~"hello", ~"kitty"));	
}
