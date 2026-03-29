fn main() {
    
    let mut x = 10;
    println!("Original value of x: {}", x);

    {
        let y = &mut x; 
        *y = 20;
    } 

    println!("Modified value of x via reference: {}", x);
}