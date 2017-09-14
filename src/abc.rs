use std::string::String;
pub struct Thinga {
	a: i64,
	b: i64,
	c: String
}
#[allow(dead_code)]
impl Thinga {
	pub fn geta(&self) -> i64 {
		self.a
	}
	pub fn getb(&self) -> i64 {
		self.b
	}
	pub fn getc(&self) -> &String {
		&self.c
	}
	pub fn seta(&mut self,x:i64) {
		self.a = x;
	}
	pub fn print(&self) {
		println!("{} {} {}",self.a,self.b,self.c);
	}
}
pub trait Traitb {
	fn geta(&mut self) -> &mut Thinga;
	fn setb(&mut self,x:i64);
	fn c(&mut self) -> &mut i64;
}
pub struct Thingb {
	a: Thinga,
	b: i64
}
impl Traitb for Thingb {
	fn geta(&mut self) -> &mut Thinga {
		&mut self.a
	}
	fn setb(&mut self,x:i64) {
		self.b = x;
	}
	fn c(&mut self) -> &mut i64 {
		&mut self.b
	}
}
pub fn maketraitb() -> Thingb {
	Thingb { a: Thinga {a:1,b:2,c:String::from("abc")}, b: 65 }
}
