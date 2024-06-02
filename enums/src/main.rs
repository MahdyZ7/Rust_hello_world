
enum IpAddrKind {
	v4,
	v6,
}

struct IpAdder {
	kind:		IpAddrKind,
	address:	String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[derive(Debug)]
enum State {
	Alabama,
	Alaska,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Qurater(State),

}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Qurater(state) => {
			println!("State {:?}", state);
			25
		}
	}
}


fn main() {
	impl Message {
        fn call(&self) {
            println!("Hello fro there")
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

	let x = Coin::Qurater(State::Alabama);
	println!{"{}", value_in_cents(x)};



}
