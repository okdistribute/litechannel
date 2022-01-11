extern crate spake2;
use rand::Rng;
use spake2::{Ed25519Group, Identity, Password, SPAKE2};

fn create_code() -> ([u8; 1], [u8; 2]) {
    return (
		rand::thread_rng().gen::<[u8; 1]>(),
		rand::thread_rng().gen::<[u8; 2]>()
	)
}

fn get_index(val: &u8) -> String {
	if val < &10 {
		return format!("00{}", val.to_string()) 
	} 
	if val < &100 {
		return format!("0{}", val.to_string()) 
	}
	return val.to_string();
}

fn as_number (mailbox: &[u8], password: &[u8])  -> String {
	let str_repr = format!("{} {} {}", 
		get_index(&mailbox[0]), 
		get_index(&password[0]),
		get_index(&password[1])
	);
	return str_repr
}


fn open (mailbox: &[u8], password: &[u8]) -> (SPAKE2<Ed25519Group>, std::vec::Vec<u8>) {
	return SPAKE2::<Ed25519Group>::start_symmetric(
		&Password::new(password),
		&Identity::new(mailbox)
	)
}

fn finish (s1: SPAKE2<Ed25519Group>, inbound_msg: &[u8]) -> Vec<u8> {
	let key = s1.finish(&inbound_msg).unwrap();
	return key
}


#[cfg(test)]
mod tests {
    #[test]
    fn generate_and_redeem_code() {
		let (mailbox, password) = crate::create_code();

 	    println!("{:?}", password);
		let (state_a, msg_a) = crate::open(&mailbox, &password);
		let (state_b, msg_b) = crate::open(&mailbox, &password);

		let key = crate::finish(state_a, &msg_b);
		let key2 = crate::finish(state_b, &msg_a);

		assert_eq!(key, key2)

    }

    #[test]
	fn representation_string() {
		let (mailbox, password) = crate::create_code();
		let str_repr = crate::as_number(&mailbox, &password);

 	    println!("{:?}", str_repr);
	}

}
