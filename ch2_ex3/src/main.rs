extern crate iron;
#[macro_use] extern crate mime;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
	let mut router = Router::new();

	router.get("/", get_form, "root");
	router.post("/gcd", post_gcd, "gcd");
	
	println!("Serving on http://localhost:3000...");
	Iron::new(router).http("localhost:3000").unwrap();
}


fn get_form(_request_request:&mut Request) -> IronResult<Response> {

	//response
	let mut response = Response::new();

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html;Charset=Utf8));
	response.set_mut(r#"
		<title>GCD Calculator</title>
		<form action="/gcd" method="post">
		<input type="text" name="n"/>
		<input type="text" name="n"/>
		<button type="submit">Compute GCD</button>
		</form>
	"#);

	Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
	let mut response = Response::new();

	/* match res {
		Condition1 => {  }
		Condtion2 =>  {  }
	
	-	match is a conditional like an if statement or switch statement
	- 	It forces the programmer to check the type of return values before they can be accessed.
	-   In C/C++, error code or null results are often neglected and cause error when used later
	}*/


	//(1) to parse the request's body as a table mapping query parameters names to arrays of values
	let form_data = match request.get_ref::<UrlEncodedBody>() {
		Err(e) => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("Error parsing form data: {:?}\n",e));
			return Ok(response); //explicit return
		}
		Ok(map) => map
	};

	//(2) extract numbers from the form data
	let unparsed_numbers = match form_data.get("n") {
		None => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("form data has no n parameters\n"));
			return Ok(response);
		}
		Some(nums) => nums
	};

	//(3) convert strings to numbers
	let mut numbers = Vec::new(); //vector
	for unparsed in unparsed_numbers {
		match u64::from_str(&unparsed) {
			Err(_) => {
				response.set_mut(status::BadRequest);
				response.set_mut(format!("Value for 'n' parameter not a number: {:?}\n", unparsed));
				return Ok(response);
			}

			Ok(n) => {numbers.push(n);}
		}
	}

	//run gcd
	let mut d = numbers[0];
	for m in &numbers[1..] {
		d = gcd(d,*m);
	}
	
	//return the response back
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html;Charset=Utf8));
	response.set_mut(format!("Gcd of the numbers {:?} is <b>{}</b>\n", numbers,d));

	Ok(response) //implicit return
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
