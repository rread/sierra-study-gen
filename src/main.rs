use clap::Parser;
use std::convert::Infallible;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

mod study;

use study::*;

fn indent(depth: i8) -> String {
    let tab_width = 4;
    let mut spaces = String::new();
    for _ in 0..(depth * tab_width) {
        spaces.push(' ');
    }
    spaces
}

fn escape_str(s: &str) -> String {
    let mut e = String::new();

    for c in s.chars() {
        if c == '"' {
            e.push('\\');
        }
        e.push(c);
    }
    e
}

fn gen_main(config: &Study) -> String {
    format!(
        r#"
#include "{}.h"

void {}::last_call() {{

}}

void {}::study() {{
    for (int index = sc.UpdateStartIndex; index < sc.ArraySize; index++) {{
        // todo
    }}
}}

SCSFExport scsf_{}Main(SCStudyInterfaceRef sc) {{
    {} study(sc);
    study.run();
}}
    "#,
        config.name, config.name, config.name, config.name, config.name,
    )
}

fn gen_class(config: &Study) -> String {
    let mut s = format!(
        r#"
#ifndef ACS_{}_H
#define ACS_{}_H
        
#include "SCSFBase.h"

class {} : public SCSFBase {{"#,
        config.name.to_ascii_uppercase(),
        config.name.to_ascii_uppercase(),
        config.name,
    );

    s.push_str(&gen_input_defs(&config.inputs, 1));
    s.push_str(&gen_graph_defs(&config.outputs, 1));
    s.push_str(&gen_defaults(&config, 1));
    s.push_str(&gen_constructor(&config, 1));
    s.push_str(&gen_methods_decl(&config, 1));

    s.push_str(&format!(
        "}};\n#endif // ACS_{}_H\n",
        config.name.to_ascii_uppercase()
    ));
    s
}

fn gen_graph_defs(graphs: &Vec<Output>, depth: i8) -> String {
    let mut s = String::new();
    let prefix = indent(depth);

    if graphs.len() > 0 {
        s = format!("\n{}enum Graphs {{\n", prefix);
        for graph in graphs.iter() {
            s.push_str(&format!("{}{},\n", indent(depth + 1), graph.enum_name()));
        }
        s.push_str(&format!("{}}};\n", prefix));

        for graph in graphs.iter() {
            s.push_str(&format!("{}SCSubgraphRef {};\n", prefix, graph.var_name()));
        }
    }
    return s;
}

fn gen_input_defs(inputs: &Vec<Input>, depth: i8) -> String {
    let mut s = String::new();
    let prefix = indent(depth);

    if inputs.len() > 0 {
        s = format!("\n{}enum Inputs {{\n", prefix);

        for input in inputs.iter() {
            s.push_str(&format!("{}{},\n", indent(depth + 1), input.enum_name()));
        }
        s.push_str(&format!("\n{}}};\n", prefix));

        for input in inputs.iter() {
            s.push_str(&format!("{}SCInputRef {};\n", prefix, input.var_name()));
        }
    }
    s
}

fn gen_defaults(config: &Study, depth: i8) -> String {
    let mut s = String::new();
    let prefix = indent(depth);
    s.push_str(&format!("\n\n{}void defaults() override {{\n", prefix));
    {
        let prefix = indent(depth + 1);
        s.push_str(&format!("{}sc.GraphName = \"{}\";\n", prefix, config.name));
        s.push_str(&format!(
            "{}sc.StudyDescription = \"{}\";\n",
            prefix,
            escape_str(&config.description),
        ));
        s.push_str(&format!("{}sc.AutoLoop = 0;\n", prefix));
        s.push_str(&format!("{}sc.GraphRegion = 0;\n", prefix));
        s.push_str(&format!(
            "{}sc.MaintainAdditionalChartDataArrays = 0;\n",
            prefix
        ));

        for input in config.inputs.iter() {
            s.push_str(&input_default(&input, &prefix))
        }
        for graph in config.outputs.iter() {
            s.push_str(&format!(
                "{}subgraph_default({}, \"{}\", {}, {});\n",
                prefix,
                graph.var_name(),
                graph.name,
                graph.sc_style(),
                graph.color
            ));
        }
    }
    s.push_str(&format!("{}}}\n", prefix));
    s
}

pub fn input_default(input: &Input, prefix: &str) -> String {
    let mut s = String::new();

    match &input.intype {
        InputType::Int(n) => s.push_str(&format!(
            "{}input_default_int({}, \"{}\", {});\n",
            prefix,
            input.var_name(),
            input.name,
            n,
        )),
        InputType::Float(n) => s.push_str(&format!(
            "{}input_default_float({}, \"{}\", {});\n",
            prefix,
            input.var_name(),
            input.name,
            n
        )),
        InputType::Bool(b) => s.push_str(&format!(
            "{}input_default_bool({}, \"{}\", {});\n",
            prefix,
            input.var_name(),
            input.name,
            b
        )),
        InputType::Color(c) => s.push_str(&format!(
            "{}input_default_color({}, \"{}\", {});\n",
            prefix,
            input.var_name(),
            input.name,
            c
        )),
        InputType::MovingAvg(ma) => {
            s.push_str(&format!(
                "{}{}.Name = \"{}\";\n",
                prefix,
                input.var_name(),
                input.name
            ));
            s.push_str(&format!(
                "{}{}.SetMovAvgType({});\n",
                prefix,
                input.var_name(),
                ma
            ));
        }
        InputType::Selection(values) => s.push_str(&format!(
            "{}input_default_select({}, \"{}\", {})",
            prefix,
            input.var_name(),
            input.name,
            values
        )),
    }
    s.push_str(&format!(
        "{}{}.SetDescription(\"{}\");\n",
        prefix,
        input.var_name(),
        escape_str(&input.description),
    ));
    s
}

fn gen_constructor(config: &Study, depth: i8) -> String {
    let mut s = String::new();
    let prefix = indent(depth);

    s.push_str(&format!("{}public:\n", indent(depth - 1)));
    s.push_str(&format!(
        "{}explicit {}(SCStudyInterfaceRef sc) :\n",
        prefix, config.name
    ));
    {
        let prefix = indent(depth + 1);
        for input in config.inputs.iter() {
            s.push_str(&format!(
                "{}{}(sc.Input[{}]),\n",
                prefix,
                input.var_name(),
                input.enum_name()
            ));
        }

        for graph in config.outputs.iter() {
            s.push_str(&format!(
                "{}{}(sc.Subgraph[{}]),\n",
                prefix,
                graph.var_name(),
                graph.enum_name()
            ));
        }
        s.push_str(&format!("{}SCSFBase(sc) {{}};\n", prefix));
    }
    s
}

fn gen_methods_decl(config: &Study, depth: i8) -> String {
    let mut s = String::new();

    s.push_str(&format!(
        "{}bool debugEnabled() override {{ return true; }}\n",
        indent(depth)
    ));
    s.push_str(&format!("{}void last_call() override;\n", indent(depth)));
    s.push_str(&format!("{}void study() override;\n", indent(depth)));
    s
}

#[derive(Parser, Debug, Default)]
struct Arguments {
    file: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Arguments::parse();
    println!("{:#?}", args);

    let config = read_to_string(&args.file).unwrap();
    let config: Study = serde_json::from_str(&config).unwrap();

    // let config = Study {
    //     name: "TestStudy".to_string(),
    //     description: "A new study from \"generated code".to_string(),
    //     inputs: vec![
    //         Input {
    //             label: "ma_length".to_string(),
    //             name: "Length".to_string(),
    //             intype: InputType::Int(1),
    //             description: "This is a length of the moving average lookback.".to_string(),
    //         },
    //         Input {
    //             label: "ma_type".to_string(),
    //             name: "Moving Average Type".to_string(),
    //             intype: InputType::MovingAvg("MOVAVGTYPE_EXPONENTIAL".to_string()),
    //             description: "The type of the moving average.".to_string(),
    //         },
    //     ],
    //     outputs: vec![
    //         Output::new("ma1".to_string(), "Moving Average".to_string()),
    //         Output::new("ma2".to_string(), String::new()),
    //     ],
    // };

    // match serde_json::to_string_pretty(&config) {
    //     Err(err) => eprintln!("Error: {:?}", err),
    //     Ok(s) => println!("{}", s),
    // }

    let class = gen_class(&config);

    let mut header = args.file.clone();
    header.set_extension("h");
    println!("class file: {:?}", header);
    let mut hfile = File::create(header)?;
    hfile.write_all(&class.into_bytes())?;

    let mut main_file = args.file.clone();
    main_file.set_extension("cpp");
    if !main_file.exists() {
        let main = gen_main(&config);
        let mut out = File::create(main_file)?;
        out.write_all(&main.into_bytes());
    }
    Ok(())
}
