fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = &a[1.. 4];

        assert_eq!([2, 3, 4], nice_slice); // 因为 Rust 自动为 [T; N] 和 &[T] 实现了相互比较，所以 assert_eq! 可以直接拿 &[i32] 用，不用手动解引用。
    }
}

