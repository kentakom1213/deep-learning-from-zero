
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + x.exp())
}

fn main() {
    for i in -50..50 {
        let x = (i as f64) / 10.0;
        println!("{:3.3} : {}", x, sigmoid(-x));
    }
}
