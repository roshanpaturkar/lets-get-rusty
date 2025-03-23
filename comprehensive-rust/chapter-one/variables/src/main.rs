fn main() {
    // integers
    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");

    let y: i32 = 10;
    println!("y: {y}");

    // floats
    let z: f64 = 3.14;
    println!("z: {z}");

    // boolean
    let a: bool = true;
    println!("a: {a}");

    // characters
    let b: char = 'a';
    println!("b: {b}");

    // tuples
    let c: (i32, f64, bool) = (10, 3.14, true);
    // println!("c: {c}");

    // arrays
    let d: [i32; 3] = [1, 2, 3];
    // println!("d: {d}");

    // slices
    let e: &[i32] = &d;
    // println!("e: {e}");

    // strings
    let f: &str = "Hello, World!";
    println!("f: {f}");

    // string literals
    let g: String = String::from("Hello, World!");
    println!("g: {g}");

    // string concatenation
    let h: String = format!("{f} {g}");
    println!("h: {h}");

    // string interpolation
    let i: String = format!("{f} {g}");
    println!("i: {i}");

    // string slicing
    let j: &str = &f[0..5];
    println!("j: {j}");

    // // string splitting
    // let k: Vec<&str> = f.split(",").collect();
    // println!("k: {k}");

    // string joining
    // let l: String = k.join(" ");
    // println!("l: {l}");


    // string formatting
    // let m: String = format!("{:?}", k);
    // println!("m: {m}");

    // string escaping
    let n: String = "Hello, \"World!\"".to_string();
    println!("n: {n}");

    // string escaping
    let o: String = "Hello, \\World!".to_string();
    println!("o: {o}");
}