fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f_arg = if args.len() < 2 { "" } else { &args[1] };
    let f: f32 = f_arg.parse().unwrap_or(0.0);
    println!("Temperature in degree celsius: {}", (5. / 9.) * (f - 32.));
}