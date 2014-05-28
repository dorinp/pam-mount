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
	i().push((~"bye", ~"kitty"));	
	hi();
	println!("{}", *i());

	let o = i().iter().find(|& &(ref a, ref b)| { 
		println!("{}", a); 
		a == &~"bye"
	});
	println!("{}", o);
}

fn hi() {
	i().push((~"hello", ~"kitty"));	
}
