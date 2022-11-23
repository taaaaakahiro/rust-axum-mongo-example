fn main() {
    println!("Hello, world!");
}

fn math(n: u64, m: u64) -> u64 {
    return n + m;  
}

#[test]
fn test_gcd(){
    assert_eq!(math(20, 30), 50)
}