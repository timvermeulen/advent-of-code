use parser::prelude::*;

pub fn rubbish<'a>() -> impl Parser<&'a str, Output = ()> + Copy + Clone {
    satisfy(|c: char| !c.is_digit(10) && c != '\n').skip_many()
}

pub fn newline<'a>() -> impl Parser<&'a str, Output = ()> + Copy + Clone {
    token('\n').map(|_| ())
}

pub fn comma<'a>() -> impl Parser<&'a str, Output = ()> + Copy + Clone {
    token(',').map(|_| ())
}
