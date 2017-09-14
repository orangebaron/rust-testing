mod abc;
use abc::Traitb;
pub fn dostuff<T: abc::Traitb>(asdf: &mut T) {
	asdf.geta().print();
	asdf.geta().seta(65);
	asdf.geta().print();
	asdf.setb(10);
}
#[allow(unused_variables)]
fn main() {
	let mut asdf = abc::maketraitb();
	dostuff(&mut asdf);
    let mut b = asdf.c();
    *b = 66;
}
