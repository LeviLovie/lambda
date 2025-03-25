pub use lambda::token::*;

fn main() {
    let tree: Token = Token::Apl(Apl::new(
        Abs::new(Var::new("a"), Box::new(Token::Var(Var::new("a")))),
        Box::new(Token::Var(Var::new("b"))),
    ));
    println!("{}", tree);
}
