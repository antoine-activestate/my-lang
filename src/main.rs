use my_lang::parse;

fn main() {
    let input = "()";
    match parse(input) {
        Err(err) => println!("err: {:?}", err),
        Ok(expr) => println!("ok: {:?}", expr),
    }
}
