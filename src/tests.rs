#[cfg(test)]

mod tests {
    use crate::huff_helper::map_to_reconstruct;

    #[test]
    fn test_map() {
        let p = vec![3, 9, 20, 15, 7];   
        let i = vec![9, 3, 15, 20, 7];
        let (pre_mapped, in_mapped) = map_to_reconstruct(p, i);
        println!("Pre = {:?}\nIn = {:?}", pre_mapped, in_mapped);   
        assert!(false)
    }
}