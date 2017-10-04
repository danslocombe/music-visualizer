mod match_keywords;
mod match_visualizer;

use common::*;
use mapper::Mapper;
use graphics::Visualization;
use self::match_keywords::{check_garg_name, check_audio_name};
use self::match_visualizer::new_visualizer;
use nom::IResult;
use nom::{multispace, alpha, double};

use std::str;
use std::fs::File;
use std::io::Read;

use graphics::geom_visuals::*;

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

named!(p_arg_list<&[u8], Vec<(AudioOption,GArg)> >,
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

named!(p_arg<&[u8], (AudioOption,GArg)>,
    do_parse!(
        g: p_garg_name      >>
        opt!(multispace)    >>
        tag!("=")           >>
        opt!(multispace)    >>
        a: p_audio_name     >>
        (a,g)
    )
);

named!(p_audio_name<&[u8], AudioOption>,
    alt!(
        p_audio_const |
        p_audio_id
    )
);

named!(p_audio_const<&[u8], AudioOption>,
    do_parse!(
        f: double   >>
        (AudioOption::Const(f))
    )
);

named!(p_audio_id<&[u8], AudioOption>,
    map_res!(alpha, check_audio_name)
);

named!(p_garg_name<&[u8], GArg>,
    map_res!(alpha, check_garg_name)
);



pub fn parse_from_file(file_name: &str) -> (Vec<Box<Visualization>>, Vec<Mapper>) {
    let mut input_file = File::open(file_name).unwrap();
    let mut file_contents = String::new();
    input_file.read_to_string(&mut file_contents);

    parse_from_string(file_contents.as_str())
}

pub fn parse_from_string(text: &str) -> (Vec<Box<Visualization>>, Vec<Mapper>) {
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

    (boxes, maps)
}

fn output_visualizer(vis_name: &[u8],
                     args: Option<Vec<(AudioOption,GArg)>>,
                     final_arg: Option<(AudioOption,GArg)>)
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
