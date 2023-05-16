use rand::Rng;

const TUP: (u8,bool,char)=(1,true, 'a');

fn soma(a: u32, b: u32) -> u32 {
    a + b
}

fn dices(sides:u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=sides)
}
fn expoente(a: i32) -> i32 {
    if a >= 0 && a <= 255 {a.pow((a as u8).into())} else  {a+a}
}


fn main() {
    println!("{}",soma(1, 2));
    println!("{}",expoente(1));
    println!("{}",expoente(256));
    println!("{}, {}, {}", TUP.0 , TUP.1, TUP.2);
    println!("{}",dices(6));
    println!("{}",dices(8));
}
