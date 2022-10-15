// https://eugene-babichenko.github.io/blog/2019/11/13/rust-popcount-intrinsics/
#[cfg_attr(target_arch = "x86_64", target_feature(enable = "popcnt"))]
pub fn popcnt(n: u64) -> u32 {
    n.count_ones()
}

const ALPHABET_SIZE: u64 = 64;

pub fn is_set(bitmap: u64, n: u64) -> bool {
    let shift = ALPHABET_SIZE - n - 1;
    let mask = 1 << shift; // Shift 0b1 n places to the left
    bitmap & mask != 0
}

pub fn bitmap_index_of(bitmap: u64, key: u64) -> u32 {
    let shift = ALPHABET_SIZE - key;
    popcnt(bitmap >> shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_set_test() {
        let bitmap = 0b10100000 << (ALPHABET_SIZE - 8);
        assert_eq!(true, is_set(bitmap, 0));
        assert_eq!(false, is_set(bitmap, 1));
        assert_eq!(true, is_set(bitmap, 2));
        assert_eq!(false, is_set(bitmap, 3));
    }

    #[test]
    fn bitmap_index_of_test() {
	let bitmap = 0b01011000 << (ALPHABET_SIZE - 8);
        assert_eq!(0, bitmap_index_of(bitmap, 1));
        assert_eq!(1, bitmap_index_of(bitmap, 3));
        assert_eq!(2, bitmap_index_of(bitmap, 4));
    }
}

// #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
// struct Tree {
//     bitmap: u64,
//     children: Vec<Node>,
// }

// #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
// struct Leaf {}

// #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
// enum Node {
//     Leaf(Leaf),
//     Tree(Tree),
// }

// impl Tree {
//     pub fn index_of(&self, key: u64) -> u64 {
//         key
//     }
// }
