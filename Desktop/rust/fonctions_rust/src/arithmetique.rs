// Nombre primaire

fn primalite(x: i32) -> bool {
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}

// PGCD

fn euclide(mut a: u32, mut b: u32) -> u32 {
    while a != b {
        if a > b {
            a = a - b
        } else {
            b = b - a
        }
    }
    return a;
}

// Fibonacci

fn fibonacci(valeur_arrive: i32) {
    let mut a = 0;
    let mut b = 1;

    println!("{}, {}", a, b);

    for i in 2..valeur_arrive {
        let mut n = a + b;
        println!("{}", n);

        a = b;
        b = n;
    }
}

// Conversion

fn convertir_en_celsius(n: i32) -> i32 {
    let celsius = (n - 32) * 5/9;
    celsius
}

fn convertir_en_fahrenheit(n: i32) -> i32 {
    let fahrenheit = (n * 9/5) + 32;
    fahrenheit
}


