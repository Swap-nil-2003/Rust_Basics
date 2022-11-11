//
fn main() {
    println!("Hello, world{}");
}

fn calc_weight_on_mars(mass: i32)->f32{
    let mut gravity:i32=10;
    
    gravity =9.81;
    (mass*gravity) as f32
}
