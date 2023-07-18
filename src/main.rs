fn main() {
    println!("Fizz Buzz");
    for x in 1..100 {
        let f = fizz_buzz(x);
        println!("{x}: {f}");
    }
}

fn fizz_buzz<'life>( n: i32) -> String {
    if (n % 3) == 0 && (n % 5) == 0 {    
        return "FizzBuzz".to_string();
    }
    if (n % 3) == 0 {
        return "Fizz".to_string();
    }
    if (n % 5) == 0 {
        return "Buzz".to_string();
    }

    format!("{}", n)
    
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! fizz_buzz_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, fizz_buzz(input));
            }
        )*
        }
    }
    
    fizz_buzz_tests! {
        fizz_buzz_01: (1, "1"),
        fizz_buzz_02: (2, "2"),
        fizz_buzz_03: (3, "Fizz"),
        fizz_buzz_04: (4, "4"),
        fizz_buzz_05: (5, "Buzz"),
        fizz_buzz_06: (6, "Fizz"),
        fizz_buzz_07: (7, "7"),
        fizz_buzz_08: (8, "8"),
        fizz_buzz_09: (9, "Fizz"),
        fizz_buzz_10: (10, "Buzz"),
        fizz_buzz_11: (11, "11"),
        fizz_buzz_12: (12, "Fizz"),
        fizz_buzz_13: (13, "13"),
        fizz_buzz_14: (14, "14"),
        fizz_buzz_15: (15, "FizzBuzz"),        
        fizz_buzz_16: (16, "16"),
        fizz_buzz_17: (17, "17"),
        fizz_buzz_18: (18, "Fizz"),
        fizz_buzz_19: (19, "19"),
        fizz_buzz_20: (20, "Buzz"),
        fizz_buzz_21: (21, "Fizz"),
        fizz_buzz_22: (22, "22"),
        fizz_buzz_23: (23, "23"),
        fizz_buzz_24: (24, "Fizz"),
        fizz_buzz_25: (25, "Buzz"),        
        fizz_buzz_26: (26, "26"),
        fizz_buzz_27: (27, "Fizz"),
        fizz_buzz_28: (28, "28"),
        fizz_buzz_29: (29, "29"),
        fizz_buzz_30: (30, "FizzBuzz"),

    }
    
    // #[test]
    // fn check_1_returns_1() {
    //     assert_eq!("1", fizz_buzz(1));
    // }

    // #[test]
    // fn check_2_returns_2() {
    //     assert_eq!("2", fizz_buzz(2));
    // }

    // #[test]
    // fn check_4_returns_4() {
    //     assert_eq!("4", fizz_buzz(4));
    // }
}
