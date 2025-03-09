fn main() {
	let args: Vec<String> = std::env::args().collect();
	println!(">>> wasi-example: Hello, world!");
	println!(">>> wasi-example: args = {args:?}");
}
