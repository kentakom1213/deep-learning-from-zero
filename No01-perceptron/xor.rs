
// fn xor(x1: f64, x2: f64) -> f64 {
//     let (w1, w2, b) = (0.5, 0.5, 0.7);
//     let tmp = x1*w1 + x2*w2;

//     // xorは線形に分離できない！！
    
// }

fn and(x1: f64, x2: f64) -> f64 {
    let (w1, w2, b) = (0.5, 0.5, -0.7);
    let tmp = x1*w1 + x2*w2 + b;
    if tmp <= 0.0 {
        0.0
    } else {
        1.0
    }
}

fn or(x1: f64, x2: f64) -> f64 {
    let (w1, w2, b) = (0.5, 0.5, -0.2);
    let tmp = x1*w1 + x2*w2 + b;
    if tmp <= 0.0 {
        0.0
    } else {
        1.0
    }
}

fn nand(x1: f64, x2: f64) -> f64 {
    let (w1, w2, b) = (-0.5, -0.5, 0.7);
    let tmp = x1*w1 + x2*w2 + b;
    if tmp <= 0.0 {
        0.0
    } else {
        1.0
    }
}

fn xor(x1: f64, x2: f64) -> f64 {
    let s1 = nand(x1, x2);
    let s2 = or(x1, x2);
    and(s1, s2)
}

fn main() {
    println!("{}", xor(0.0, 0.0));
    println!("{}", xor(0.0, 1.0));
    println!("{}", xor(1.0, 0.0));
    println!("{}", xor(1.0, 1.0));
}
