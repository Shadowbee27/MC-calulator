use std::io;
pub fn portal_input() -> [f32; 2] {
    let mut cords: [f32; 2] = [0.0, 0.0];
    println!("Please enter the x coordinate");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    println!("Please enter the z coordinate");
    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("Failed to read line");
    cords[1] = z.trim().parse().unwrap();
    cords[0] = x.trim().parse().unwrap();
    cords
}
pub fn storage_input() -> f32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().unwrap()
}
pub fn menu_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}
