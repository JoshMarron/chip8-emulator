pub fn get_digits(mut val: u16, out: &mut Vec<u8>) {
    while val > 0 {
        let digit = val % 10;
        out.push(digit as u8);
        val /= 10;
    }

    while out.len() < 3 {
        out.push(0);
    }

    out.reverse();
}