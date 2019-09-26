# rust_basic

## why use rust?
1. type-safety(memory type-safety): ensures that a variable can't access memory location that are not assigned to it. This prevents undefined behaviours. type-safety in this context doesn't mean type-checking.
2. thread-safe. concurrency

## components of rust:
1. cargo: Rust's compilation maanger, package manager, and general-purpose tool

```
cargo new --bin hello --vcs none
```
This creates a new project which produces an executable(--bin) without any version control system(--vcs none)

2. rustc: Rust's compiler

3. rustdoc: Rust documentation tool

## simple overview
```rust
fn gcd(mut n: u64, mut m: u64) -> u64 {
  assert!(n !=0 && m != 0);
  while m !=0 {
    if m < n {
      let t: u64 =m;
      m=n;
      n=t;
    }
    m =m % n;
  }
  n
}
```
1. function definition: _fn_ introduces a function
2. macro: ! indicates it's a macro. eg) assert!
3. i32: signed 32-bit integer, isize: pointer-sized integer
4. mut: it's mutable. In rust, variable is mutable by default.
5. let declares a local variable
6. return value: function body ends with an expression, that is not followed by a semicolon(";"), that's the functino's return value.
7. return value: you can use return statement only if for explicit early returns from the midst of a function
8. simple testing is bulit into the language. To test fn gcd:
```rust
#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
```
and type cargo test
9. attribute: #[test] is an exmple of an attribute. 

#another example
```rust
extern crate iron;
#[macro_use] extern crate mine;

use iron::prelude::*;
use iron::status;

fn main() {
	println!("Serving on http://localhost:3000...");
	Iron::new(get_form).http("localhost:3000").unwrap();
}


fn get_form(_request_request:&mut Request) -> IronResult<Respose> {
	//response
	let mut response = Reponse::new();

	response.set_mut(Status::Ok);
	Response.set_mut(mine!(Text/Html;Charset=Utf8));
	Response.set_mut(r#"
		<title>GCD Calculator</title>
		<form action="/gcd" method="post">
		<input type="text" name="n"/>
		<input type="text" name="n"/>
		<button type="submit">Compute GCD</button>
		</form>
	"#);

	Ok(response)

}
```
1. importing other crates: extern crate makes cargos that we cited in our cargo.toml file available to our program.
#[macro_use] attribute before the extern crate mime alerts Rust that we plan to use macros exported by this crate

2. Make public names visible in our current code: eg) use iron::prelude::* 

3. \_request: tell Rust that it will be unused so don't warn us

4. 




