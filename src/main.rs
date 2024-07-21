use rand::{thread_rng, Rng};
fn main(){print!("\x1b[38;5;46m");print!("\x1b[48;5;16m");loop{let mut v=Vec::new();let mut k=0;for _i in 0..252{let n=thread_rng().gen::<u8>();v.push(n)}for c in v{print!("{c}");z(9);k+=1;if c<=9{print!
("  ");}else if c<=99{print!(" ")};if k%12==1{print!("\n");}}println!("");}}
fn z(q:u8){for _i in 0..q {print!(" ")}}
