use std::{num::Wrapping, ops::AddAssign, io};

fn main() {
	/*
	 * Scalar Types(primitif)
	 */
	println!("\n* * * Scalar Types * * *");
	{

		/*
		 * integer
		 * To optimize(economize) memory,
		 * It's recommmanded to use correct int range
		 * Not put a type too big or small(overflow)
		 */
		{
			const DECIMAL: u32 = 98_222; //98222, to use u32(0 - 4294967295)
			println!("DECIMAL:\t{DECIMAL}");
			const HEX: u8 = 0xff; //255, able to use u8(0 - 255)
			println!("HEX:\t\t{HEX}");
			const OCTAL: u8 = 0o77; //63
			println!("OCTAL:\t\t{OCTAL}");
			const BINARY: u8 = 0b1111_0000; //240
			println!("BINARY:\t\t{BINARY}");
			/*
			 * `const` is more for hard coding data
			 * In those case, `let` is better but I used `const` to test
			 */
			let hex32: u32 = HEX.into(); // convert from const to let
			let mut wrapped: Wrapping<u32> = Wrapping(hex32); // Wrapping to avoid overflow
			println!("wrapped:\t{wrapped}");
			wrapped.add_assign(2);
			println!("wrapped:\t{wrapped}");
		}
	
		/*
		 * float 
		 */

		{
			let x = 2.0;// f64
			let y:f32 = 3.2; //f32
			/*
			 * Numeric operation 
			 */
			println!("\n[Numeric operation]");
			println!("< x:{} y:{} >", x,y);
			println!("addition:\t{}", x + y);
			println!("substraction:\t{}", x - y);
			println!("mutiplication:\t{}", x * y);
			println!("division:\t{}", x / y);
			println!("remainder:\t{}", x % y);
		
		}
	
		/*
		 * boolean 
		 */
		{

			let t = true;
			let f:bool = false;
			println!("\n[Boolean]");
			println!("t: {} f: {}", t, f);
		}
	
		/*
		 * char 
		 */
		{
			let c = 'z';
			let z: char = 'ℤ'; // with explicit type annotation
			let heart_eyed_cat = '😻';
			println!("\n[char]");
			println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);
		}
	
		/*
		 * cf/ String 
		 */
		{
			let s1 = String::from("Hello, ");
			let s2 = String::from("world!");
			let s3 = s1.clone() + &s2; // without .clone(), s1's value moved, we cannot use s1 after this line
		
			println!("\n[String]");
			println!("s1: {s1}");
			println!("s2: {s2}");
			println!("s3: {s3}");
		}

	}		
	/*
	 * Compound Types
	 */
	println!("\n* * * Compound Types * * *");
	{
		/*
		 * Tuple
		 */
		{
			let tup: (i32, f64, u8) = (500, 64.0, 1);
			println!("\n[Tuple]");
			println!("Tuple's value => `tup.0`:{}, `tup.1`:{}, `tup.2`:{}", tup.0, tup.1, tup.2);
			// by default key is 0,1,2... but we can assign thoses things in new variable
			// `a little bit like` destructuring in javsScript
			let (x, y, z) = tup;
			println!("Tuple's value => x={x}, y={y}, z={z}");
		}

		/*
		 * Array 
		 */
		{
			println!("\n[Array]");
			let a = [1, 2, 3, 4, 5];
			//by default, loop takes ownership of the element.
			//BUT, adding `&` to array(iterates each ref) is more efficient
			println!("a = [1,2,3,4,5]");
			for el in &a {
				println!("el: {el}");
			}
			let b:[i32;5] = [1,2,3,4,5];
			println!("b:[i32;5] = [1,2,3,4,5]");
			for el in &b {
				println!("el: {el}");
			}
			let c = [3;5];
			println!("c = [3;5]");
			for el in &c {
				println!("el: {el}");
			}
			
			/*
			 * Invalid index access on runtime
			 */
			{
				// let a = [1, 2, 3, 4, 5];
			
				// println!("Please enter an array index.");
			
				// let mut index = String::new();
			
				// io::stdin()
				// 	.read_line(&mut index)
				// 	.expect("Failed to read line");
			
				// let index: usize = index
				// 	.trim()
				// 	.parse()
				// 	.expect("Index entered was not a number");
			
				// let element = a[index];
			
				// println!("The value of the element at index {index} is: {element}");
			}
		}
 	}
}
