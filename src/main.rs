fn main() {

    let x = 0;

    if x >= 60 {
        println!("x більше за 60")
    } else if x >= 55 {
        println!("x більше за 55 але менше за 60")
    } else if x >= 45 {
        println!("x більше за 45 але менше за 60")
    } else {
        println!("Ні одна із умов не підходить") // Спрацює ця умова
    }

    no1();
}

fn no1() {
    let time = 10;
    let greeting = if time < 18 {
        "Тайм менше 18"
    } else {
        "Тайм більше 18" // Спрацює ця умова
    };
    println!("{}", greeting);
}