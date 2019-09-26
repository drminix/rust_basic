//bring two traits in the current scope
use std::io::Write;
use std::str::FromStr;

fn main() {
	//create a new vector
	let mut numbers = Vec::new();

	//read in the commandline argument expcet the first one(executable name)
	for arg in std::env::args().skip(1) {
		numbers.push(u64::from_str(&arg).expect("error parsing argument"));
	}

	//if no numbers are given
	if numbers.len() == 0 {
		writeln!(std::io::stderr(), "Usage: gcd ").unwrap();
		std::process::exit(1);
	}

	let mut d = numbers[0];
	for m in &numbers[1..] {
		d = gcd(d, *m);
	}

	println!("The great common divisor of {:?} os {}", numbers,d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n !=0 && m !=0);
    while m !=0 {
        if m<n {
            let t = m;
            m = n;
            n=t;
        }
        m = m%n;
    }
    n
}    

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}