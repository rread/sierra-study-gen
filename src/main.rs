use clap::Parser;
use std::fs::{read_to_string, File};
use std::io::Write;

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
        config.class_name(),
        config.class_name(),
        config.class_name(),
        config.class_name(),
        config.class_name(),
    )
}

fn gen_class(config: &Study) -> String {
    let mut s = format!(
        r#"
#ifndef ACS_{}_H
#define ACS_{}_H
        
#include "SCSFBase.h"

class {} : public SCSFBase {{"#,
        config.def_name(),
        config.def_name(),
        config.class_name(),
    );

    s.push_str(&gen_input_defs(&config.inputs, 1));
    s.push_str(&gen_graph_defs(&config.outputs, 1));
    s.push_str(&gen_defaults(&config, 1));
    s.push_str(&gen_constructor(&config, 1));
    s.push_str(&gen_methods_decl(&config, 1));
    s.push_str(&gen_private_class(&config, 1));

    s.push_str(&format!("}};\n#endif // ACS_{}_H\n", config.def_name()));
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
        s.push_str(&format!(
            "{}sc.AutoLoop = {};\n",
            prefix,
            if config.autoloop { 1 } else { 0 },
        ));
        s.push_str(&format!("{}sc.GraphRegion = {};\n", prefix, config.region));
        s.push_str(&format!(
            "{}sc.MaintainAdditionalChartDataArrays = {};\n",
            prefix,
            if config.enable_extra_data { 1 } else { 0 },
        ));
        if config.pointer_events {
            s.push_str(&format!(
                "{}sc.ReceivePointerEvents = ACS_RECEIVE_POINTER_EVENTS_WHEN_ACS_BUTTON_ENABLED;\n",
                prefix,
            ));
        }

        for input in config.inputs.iter() {
            s.push_str(&input_default(&input, &prefix))
        }
        for graph in config.outputs.iter() {
            s.push_str("\n");
            s.push_str(&format!(
                "{}subgraph_default({}, \"{}\", {}, {});\n",
                prefix,
                graph.var_name(),
                graph.name,
                graph.style,
                graph.color
            ));
            if graph.width != 1 {
                s.push_str(&format!(
                    "{}{}.LineWidth = {};\n",
                    prefix,
                    graph.var_name(),
                    graph.width,
                ));
            }
            if let Some(second_color) = &graph.second_color {
                s.push_str(&format!(
                    "{}{}.SecondaryColorUsed = TRUE;\n",
                    prefix,
                    graph.var_name(),
                ));
                s.push_str(&format!(
                    "{}{}.SecondaryColor = {};\n",
                    prefix,
                    graph.var_name(),
                    second_color,
                ));
            }
            if let Some(auto_color) = &graph.auto_color {
                s.push_str(&format!(
                    "{}{}.AutoColoring = {};\n",
                    prefix,
                    graph.var_name(),
                    auto_color,
                ));
            }
        }
    }
    s.push_str(&format!("{}}}\n", prefix));
    s
}

pub fn input_default(input: &Input, prefix: &str) -> String {
    let mut s = String::new();
    s.push_str("\n");

    match &input.intype {
        InputType::Int(n) => s.push_str(&format!(
            "{}input_default_int({}, \"{}\", {});\n",
            prefix,
            input.var_name(),
            input.name,
            n,
        )),
        InputType::Float(n) => s.push_str(&format!(
            "{}input_default_float({}, \"{}\", {}f);\n",
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
            "{}input_default_select({}, \"{}\", \"{}\");\n",
            prefix,
            input.var_name(),
            input.name,
            values
        )),
        InputType::Data(default) => s.push_str(&format!(
            "{}input_default_data({}, \"{}\", SC_{});\n",
            prefix,
            input.var_name(),
            input.name,
            default.to_ascii_uppercase(),
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
        prefix,
        config.class_name()
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

fn gen_methods_decl(_config: &Study, depth: i8) -> String {
    let mut s = String::new();

    s.push_str(&format!(
        "{}bool debugEnabled() override {{ return true; }}\n",
        indent(depth)
    ));
    s.push_str(&format!("{}void last_call() override;\n", indent(depth)));
    s.push_str(&format!("{}void study() override;\n", indent(depth)));
    s
}

fn gen_private_class(config: &Study, depth: i8) -> String {
    let mut s = String::new();
    if let Some(name) = &config.private_class {
        s.push_str(&format!("private:\n{}class {};\n", indent(depth), name));
    }
    s
}

#[derive(Parser, Debug, Default)]
struct Arguments {
    file: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Arguments::parse();
    // println!("{:#?}", args);

    let config = read_to_string(&args.file).unwrap();
    let config: Study = serde_json::from_str(&config).unwrap();

    let class = gen_class(&config);

    let mut header = args.file.clone();
    header.set_extension("h");
    println!("updated header: {:?}", header);
    let mut hfile = File::create(header)?;
    hfile.write_all(&class.into_bytes())?;

    let mut main_file = args.file.clone();
    main_file.set_extension("cpp");
    if !main_file.exists() {
        println!("creating class file: {:?}", main_file);
        let main = gen_main(&config);
        let mut out = File::create(main_file)?;
        out.write_all(&main.into_bytes())?;
    }
    Ok(())
}
