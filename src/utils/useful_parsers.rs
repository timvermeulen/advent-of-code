use parser::prelude::*;

pub fn rubbish<'a>() -> impl Parser<&'a str, Output = ()> + Copy + Clone {
    satisfy(|c: char| !c.is_digit(10) && c != '\n' && c != '-' && c != '+').skip_many()
}

pub fn newline<'a>() -> impl Parser<&'a str, Output = ()> + Copy + Clone {
    token('\n').map(|_| ())
}

pub fn comma<'a>() -> impl Parser<&'a str, Output = ()> + Copy + Clone {
    token(',').map(|_| ())
}

pub fn alphabetic<'a>() -> impl Parser<&'a str, Output = char> + Copy + Clone {
    satisfy(char::is_alphabetic)
}

pub fn alphanumeric<'a>() -> impl Parser<&'a str, Output = char> + Copy + Clone {
    satisfy(char::is_alphanumeric)
}
