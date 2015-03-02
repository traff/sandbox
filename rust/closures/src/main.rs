fn twice<F: Fn(u32)->u32> (val: u32, f:F)->u32 {
   f(val) + f(val)
}

fn main() {
   println!("Answer {}", twice(5, |x|{x*4}));
    
}
