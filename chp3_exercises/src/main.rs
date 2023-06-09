fn main() {
    exercise_1();
    println!("{}", exercise_2a(10));
    exercise_3();
}

fn christmas_day(mut n: i32) {
    n = if n < 1 { 1 } else if n > 12 { 12 } else { n };
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    println!("On the {} day of Christmas my true love sent to me", days[(n - 1) as usize]);
}


fn exercise_3() {
    println!();

    let items = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies a-dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    let mut day = 1;
    while day <= 12 {
        christmas_day(day);

        let mut count_down = day;
        while count_down > 1 {

            println!("{} {}", count_down, items[(count_down-1) as usize]);
            count_down -= 1;
        }

        println!("{} {}", "And a", items[(count_down-1) as usize]);
        println!();
        day += 1;
    }
}

fn exercise_2a(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return exercise_2a(n - 1) + exercise_2a(n - 2);
    }
}

fn exercise_2(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut fib_n_minus_2 = 0;
    let mut fib_n_minus_1 = 1;
    let mut fib_n = 1;

    let mut i = 2;
    while i <= n {
        fib_n = fib_n_minus_2 + fib_n_minus_1;
        fib_n_minus_2 = fib_n_minus_1;
        fib_n_minus_1 = fib_n;
        i += 1;
    }

    fib_n
}

fn exercise_1() {
    let fahrenheit: i32 = 32;
    let celsius: i32 = 0;

    let converted_celsius = fahr_to_cel(fahrenheit);
    let converted_fahrenheit = cel_to_fahr(celsius);

    println!("{} Fahrenheit is {} Celsius", fahrenheit, converted_celsius);
    println!("{} Celsius is {} Fahrenheit", celsius, converted_fahrenheit);
}

fn fahr_to_cel(degrees: i32) -> f32 {
    (((degrees - 32) as f32) * 5.0) / 9.0
}

fn cel_to_fahr(degrees: i32) -> f32 {
    ((degrees as f32) * 9.0) / 5.0 + 32.0
}
