//mod projectile;

//use rustrace::projectile::*;

fn main() {
    println!("this is the particle tracker");
    let tup = rustrace::projectile::ITuple{x:0,y:0,z:0, w:0};
    println!("tup:{:?}", tup);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup() {}
}
