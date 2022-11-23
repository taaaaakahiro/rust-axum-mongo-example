fn main() {
    println!("Hello, world!");
    let num = math(3,5);
    let str_num = &num.to_string();
    println!("{str_num}");
}

fn math(n: i64, m: i64) -> i64 {
    return n + m;  
}

#[test]
fn test_gcd(){
    assert_eq!(math(20, 30), 50)
}