mod test {
    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}

fn main() {
    let s = "abc a A ABC abC_def \"string\" ";
    //let s = "abc !".to_string();  // unmatch
    //let s = "\" \n \"".to_string();  // unmatch
    let mut lex = test::Lexer::new(&s, test::SpaceCounter::new());
    loop {
        let res = lex.yylex();
        println!("{:?}", res);
        if res.is_err() {
            break;
        }
        println!("remain '{}' characters", lex.remain());
    }
    println!("space count: {}", lex.get_space_counter().count());
}
