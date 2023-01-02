pub fn add_two(x : i32) -> i32{
    x + 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_two() {
        assert_eq!(add_two(2), 4);
    }
}