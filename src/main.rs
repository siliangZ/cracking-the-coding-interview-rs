fn main() {
    let f = 0.42f32;
    let f_b: u32 = unsafe { std::mem::transmute(f) };
    println!("{:b}", f_b);
}
