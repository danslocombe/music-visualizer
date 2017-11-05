use parser::p_add_sub;
use nom::IResult;
use nom::multispace;
use expression::Expr;

pub fn check_func(i: &[u8]) -> IResult<&[u8], Expr> {
    p_func(i)
}

named!(p_func<&[u8], Expr>,
    alt!(
        p_cond_f    |
        p_sin_f     |
        p_cos_f     |
        p_floor_f   |
        p_ceil_f
    )
);

// Function macros
named!(p_cond_f<&[u8], Expr>,
    do_parse!(
        tag!("cond(")       >>
        opt!(multispace)    >>
        c: p_add_sub        >>
        opt!(multispace)    >>
        tag!(",")           >>
        opt!(multispace)    >>
        a: p_add_sub        >>
        opt!(multispace)    >>
        tag!(",")           >>
        opt!(multispace)    >>
        b: p_add_sub        >>
        opt!(multispace)    >>
        tag!(")")           >>
        (Expr::Cond(Box::new(c),Box::new(a),Box::new(b)))
    )
);

named!(p_sin_f<&[u8], Expr>,
    do_parse!(
        tag!("sin(")        >>
        opt!(multispace)    >>
        v: p_add_sub        >>
        opt!(multispace)    >>
        tag!(")")           >>
        (Expr::Sin(Box::new(v)))
    )
);

named!(p_cos_f<&[u8], Expr>,
    do_parse!(
        tag!("cos(")        >>
        opt!(multispace)    >>
        v: p_add_sub        >>
        opt!(multispace)    >>
        tag!(")")           >>
        (Expr::Cos(Box::new(v)))
    )
);

named!(p_floor_f<&[u8], Expr>,
    do_parse!(
        tag!("floor(")        >>
        opt!(multispace)    >>
        v: p_add_sub        >>
        opt!(multispace)    >>
        tag!(")")           >>
        (Expr::Floor(Box::new(v)))
    )
);

named!(p_ceil_f<&[u8], Expr>,
    do_parse!(
        tag!("ceil(")        >>
        opt!(multispace)    >>
        v: p_add_sub        >>
        opt!(multispace)    >>
        tag!(")")           >>
        (Expr::Ceil(Box::new(v)))
    )
);
