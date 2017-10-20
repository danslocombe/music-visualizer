mod match_keywords;
mod match_visualizer;

use common::*;
use mapper::Mapper;
use graphics::{Visualization, ActiveEffects};
use self::match_keywords::{check_garg_name, check_audio_name};
use self::match_visualizer::new_visualizer;
use nom::IResult;
use nom::{multispace, alpha, double, digit};

use std::str;
use std::fs::File;
use std::io::Read;

// Parser macros

named!(parse_script<&[u8], Vec<(Box<Visualization>,Mapper)> >,
    many1!(
        do_parse!(
            v: p_visualizer     >>
            opt!(multispace)    >>
            (v)
        )
    )
);

named!(p_visualizer<&[u8], (Box<Visualization>,Mapper)>,
    do_parse!(
        vis: alpha              >>
        opt!(multispace)        >>
        tag!("{")               >>
        opt!(multispace)        >>
        args: opt!(p_arg_list)  >>
        final_arg: opt!(p_arg)  >>
        opt!(multispace)        >>
        tag!("}")               >>
        (output_visualizer(vis, args, final_arg))
    )
);

named!(p_arg_list<&[u8], Vec<(Expr,GArg)> >,
    do_parse!(
        list: many1!(do_parse!(
            arg: p_arg          >>
            opt!(multispace)    >>
            tag!(",")           >>
            opt!(multispace)    >>
            (arg)))         >>
        (list)
    )
);

named!(p_arg<&[u8], (Expr,GArg)>,
    do_parse!(
        g: p_garg_name      >>
        opt!(multispace)    >>
        tag!("=")           >>
        opt!(multispace)    >>
        a: p_add_sub        >>
        (a,g)
    )
);

named!(p_add_sub<&[u8], Expr>,
    alt!(
        p_add       |
        p_sub       |
        p_mul_div
    )
);

named!(p_add<&[u8], Expr>,
    do_parse!(
        a: p_mul_div        >>
        opt!(multispace)    >>
        tag!("+")           >>
        opt!(multispace)    >>
        b: p_add_sub        >>
        (Expr::Add(Box::new(a), Box::new(b)))
    )
);
  
named!(p_sub<&[u8], Expr>,
    do_parse!(
        a: p_mul_div        >>
        opt!(multispace)    >>
        tag!("-")           >>
        opt!(multispace)    >>
        b: p_add_sub        >>
        (Expr::Sub(Box::new(a), Box::new(b)))
    )
);

named!(p_mul_div<&[u8], Expr>,
    alt!(
        p_mul       |
        p_div       |
        p_prim_expr
    )
);

named!(p_mul<&[u8], Expr>,
    do_parse!(
        a: p_prim_expr      >>
        opt!(multispace)    >>
        tag!("*")           >>
        opt!(multispace)    >>
        b: p_mul_div        >>
        (Expr::Mul(Box::new(a), Box::new(b)))
    )
);
  
named!(p_div<&[u8], Expr>,
    do_parse!(
        a: p_prim_expr      >>
        opt!(multispace)    >>
        tag!("/")           >>
        opt!(multispace)    >>
        b: p_mul_div        >>
        (Expr::Div(Box::new(a), Box::new(b)))
    )
);

named!(p_prim_expr<&[u8], Expr>,
    alt!(
        p_audio_id   |
        p_expr_const |
        do_parse!(
            tag!("(")           >>
            opt!(multispace)    >>
            e: p_add_sub        >>
            opt!(multispace)    >>
            tag!(")")           >>
            (e)
        )
    )
);

named!(p_expr_const<&[u8], Expr>,
    alt!(
        p_float |
        p_int
    )
);

named!(p_int<&[u8], Expr>,
    do_parse!(
        i: digit   >>
        (Expr::Const(str_to_int(i).unwrap() as f64))
    )
);

named!(p_float<&[u8], Expr>,
    do_parse!(
        f: double   >>
        (Expr::Const(f))
    )
);

named!(p_audio_id<&[u8], Expr>,
    map_res!(alpha, check_audio_name)
);

named!(p_garg_name<&[u8], GArg>,
    map_res!(alpha, check_garg_name)
);



pub fn parse_from_file(file_name: &str) -> (ActiveEffects, Vec<Mapper>) {
    let mut input_file = File::open(file_name).unwrap();
    let mut file_contents = String::new();
    let _ = input_file.read_to_string(&mut file_contents);

    parse_from_string(file_contents.as_str())
}

fn parse_from_string(text: &str) -> (ActiveEffects, Vec<Mapper>) {
    let mut output = match parse_script(text.as_bytes()) {
        IResult::Done(_,o) => o,
        IResult::Incomplete(i) => panic!("Incomplete: {:?}", i),
        IResult::Error(e) => panic!("Error: {:?}", e)
    };

    let mut boxes = Vec::new();
    let mut maps = Vec::new();
    
    while let Some((v,m)) = output.pop() {
        boxes.push(v);
        maps.push(m);
    };

    boxes.reverse();
    maps.reverse();

    let effects = ActiveEffects {effects: boxes};

    (effects, maps)
}

fn output_visualizer(vis_name: &[u8],
                     args: Option<Vec<(Expr,GArg)>>,
                     final_arg: Option<(Expr,GArg)>)
                     -> (Box<Visualization>, Mapper) {
    // combine arg lists
    let mut arg_list = match args {
        Some(l) => l,
        None => Vec::new()
    };

    match final_arg {
        Some(a) => arg_list.push(a),
        None => {}
    };

    let map = Mapper::new(arg_list);
    // optional: check args vs visualizer possible args

    // optional: extract const args for use in constructing visualizer.
    // this will mean we don't need to send them over from the mapper every time.

    // construct visualizer
    let vis = new_visualizer(str::from_utf8(vis_name).unwrap());

    (vis, map)
}

fn str_to_int(s: &[u8]) -> Result<i32, String> {
    match str::from_utf8(s) {
        Ok(i_str) => match i_str.parse() {
            Ok(i) => Ok(i),
            Err(_) => Err(format!("Not an integer: {}", i_str))
        },
        Err(_) => Err(format!("Incorrectly parsed input string."))
    }
}
