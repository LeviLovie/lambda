use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
}

impl Var {
    pub fn new(name: &str) -> Var {
        Var {
            name: name.to_string(),
        }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Abs {
    pub arg: Var,
    pub body: Box<Token>,
}

impl Abs {
    pub fn new(arg: Var, body: Box<Token>) -> Abs {
        Abs { arg, body }
    }
}

impl std::fmt::Display for Abs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}",
            "(".bright_black(),
            "λ".yellow(),
            format!("{}", self.arg).green().bold(),
            ".".bright_black(),
            format!("{}", self.body).blue().bold(),
            ")".bright_black()
        )
    }
}

#[derive(Debug, Clone)]
pub struct Apl {
    pub func: Abs,
    pub arg: Box<Token>,
}

impl Apl {
    pub fn new(func: Abs, arg: Box<Token>) -> Apl {
        Apl { func, arg }
    }
}

impl std::fmt::Display for Apl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            "(".bright_black(),
            format!("{}", self.func),
            ":".bright_black(),
            format!("{}", self.arg).blue().bold(),
            ")".bright_black()
        )
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Var(Var),
    Abs(Abs),
    Apl(Apl),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Var(v) => write!(f, "{}", v),
            Token::Abs(a) => write!(f, "{}", a),
            Token::Apl(a) => write!(f, "{}", a),
        }
    }
}
