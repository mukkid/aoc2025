fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn proof_of_concept() {
        let current = 0b0000_0010_0010_0110;
        let blocker = 0b0100_0010_0000_0010;
        let final_n = 0b0000_0101_0010_0101;

        let split_b = current & blocker;
        let after_s = (split_b << 1) | (split_b >> 1);
        let unsplit = !split_b & current;
        let final_b = after_s | unsplit;
        assert_eq!(final_b, final_n);
    }
}
