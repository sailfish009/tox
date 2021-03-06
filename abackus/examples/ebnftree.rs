#![deny(warnings)]

extern crate lexers;
extern crate abackus;

struct Tokenizer(lexers::Scanner<char>);

impl Iterator for Tokenizer {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.ignore_ws();
        lexers::scan_math_op(&mut self.0)
            .or_else(|| lexers::scan_number(&mut self.0))
            .or_else(|| lexers::scan_identifier(&mut self.0))
    }
}

impl Tokenizer {
    fn scanner(input: &str) -> lexers::Scanner<String> {
        lexers::Scanner::new(
            Box::new(Tokenizer(lexers::Scanner::from_buf(input.chars()))))
    }
}

fn main() {
    let grammar = r#"
        expr   := expr ('+'|'-') term | term ;
        term   := term ('*'|'/') factor | factor ;
        factor := '-' factor | power ;
        power  := ufact '^' factor | ufact ;
        ufact  := ufact '!' | group ;
        group  := num | '(' expr ')' ;
    "#;

    let input = std::env::args().skip(1).
        collect::<Vec<String>>().join(" ");

    use std::str::FromStr;
    let trificator = abackus::ParserBuilder::default()
        .plug_terminal("num", |n| f64::from_str(n).is_ok())
        .sexprificator(&grammar, "expr");

    match trificator(&mut Tokenizer::scanner(&input)) {
        Ok(trees) => for t in trees { t.print(); },
        Err(e) => println!("{:?}", e)
    }
}
