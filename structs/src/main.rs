#[derive(Debug)]
struct Eq {
    x2_coff: isize,
    x1_coff: isize,
    x0_coff: isize,
    test_string: String,
}

impl Eq {
    fn order(&self) -> i8 {
        if self.x2_coff != 0 {
            return 2;
        } else if self.x1_coff != 0 {
            return 1;
        }
        0
    }

	fn nonsense(&self, other: &Eq) -> bool{
		if self.x2_coff >=  other.x2_coff {true} else {false}
	}
}

impl Eq {
	fn linear(x1: isize, x0: isize) -> Self {
		Self{
			x2_coff: 0,
			x1_coff: x1,
			x0_coff: x0,
			test_string: String::from("Green"),
		}
	}
}

fn main() {
    println!("Hello, world!");
    let mut quadratic = Eq {
        x2_coff: 1,
        x1_coff: 0,
        x0_coff: -1,
        test_string: String::from("HAHAHAH"),
    };
    let quadratic1 = Eq {
        test_string: quadratic.test_string.clone(),
        ..quadratic
    };
	let quadratic2 = Eq::linear(1,0);
    quadratic.test_string.push_str(" NANANAN");
    quadratic.x2_coff = 0;
    println!("the fisrt quadratic {:#?}", quadratic);
    println!("the second quadratic {:?}", quadratic1);
    println!("the third quadratic {:?}", quadratic2);
    println!("The order of quadratic is {}", quadratic.order());
    println!("The order of quadratic1 is {}", quadratic1.order());
    println!("The order of quadratic2 is {}", quadratic2.order());
}
