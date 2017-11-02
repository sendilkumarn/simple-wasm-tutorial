fn main() {
    println!("Hello, world!");
}

// Functions that you wish to access from Javascript
// must be marked as no_mangle
#[no_mangle]
pub fn fibonacci(n: i32) -> f64 {
    let mut a: f64 = 1.0;
    let mut b: f64 = 0.0;
    let mut temp: f64 = 0.0;
    let mut i = n;

    while i >= 0 {
      temp = a;
      a = a + b;
      b = temp;
      i -= 1;
    }

    b
}

// initial implementation.
// if n < 0 {
// 	panic!("{} is negative!", n);
// } else if n == 0 {
// 	panic!("zero is not a right argument to fibonacci()!");
// } else if n == 1 {
// 	return 1.0;
// }
// let mut i = 0;
// let mut sum: f64 = 0.0;
// let mut last: f64 = 0.0;
// let mut curr: f64 = 1.0;
// while i < n - 1 {
// 	sum = last + curr;
// 	last = curr;
// 	curr = sum;
// 	i += 1;
// }
// sum

#[no_mangle]
pub fn fibonacci_recursive(n: i32) -> u64 {
	if n < 0 {
		panic!("{} is negative!", n);
	}
	match n {
		0     => panic!("zero is not a right argument to fibonacci_reccursive()!"),
		1 | 2 => 1,
		3     => 2,
		/*
		50    => 12586269025,
		*/
		_     => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
	}
}
