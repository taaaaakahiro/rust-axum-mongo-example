fn main() {
    println!("Hello, world!");
}

fn math(mut n: u64, mut m: u64) -> u64 {
    return n + m;  
}

#[test]
fn test_gcd(){
    assert_eq!(math(20, 30), 50)
}