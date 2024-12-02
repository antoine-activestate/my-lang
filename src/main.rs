use my_lang::parse;

fn main() -> Result<(), i32> {
    let input = "()";
    let output = parse(input);
    println!("{:?}", output);
    match output {
        Ok(_) => Ok(()),
        Err(_) => Err(1),
    }
}
