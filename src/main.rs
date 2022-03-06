fn add_numbers(a: u16, b: u16, c: u16) -> u16 {
    a + b + c
}

fn main() {
    let a = 3;
    let b = 6;
    let c = 9;

    println!(
        "Result of adding {}, {} and {} is {}",
        a,
        b,
        c,
        add_numbers(a, b, c)
    );
}

#[test]
fn test_add_numbers() {
    let a = 1;
    let b = 3;
    let c = 7;

    assert_eq!(11, add_numbers(a,b,c));
}
