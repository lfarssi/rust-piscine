pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
	let mut res= Vec::new();
	let nums = s.split_whitespace();
	for num in nums {
		if num.ends_with("k"){

			
			let  num2=&num[..num.len()-1];
			if let Ok(mut n)=num2.parse::<f32>(){

				n*=1000.0;
				res.push(Box::new(n as u32));		
			}
		}	else{
			if let Ok( n)=num.parse::<u32>(){

				res.push(Box::new(n));		
			}
}
}    
res
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
	let mut res = Vec::new();
	for i in a {
		res.push(*i);
	}
	res
}
