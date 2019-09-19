use clap::{App, SubCommand};
use clap::ArgMatches;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::io::Write;

const INCLUDE: &'static str = "INCLUDE";
const SKIP: &'static str = "SKIP";
const HIDE: &'static str = "HIDE";
const EXTERNAL: &'static str = "EXTERNAL";

fn main() -> std::io::Result<()> {
    let matches = App::new("single source")
        .version("0.1.0")
        .author("Tom Gowan <tom.gowan@holo.host>")
        .about("Turns annotated md into working code")
        .subcommand(SubCommand::with_name("code")
                    .about("Generates code file.")
                    .args_from_usage(
                        "<INPUT>              'Sets the input md file to use'
                        <OUTPUT>              'Sets the file to render output to'
                        <LANG>              'Sets the language so only these tags will be generated ie. rust will get all ```rust tags'"))
        .subcommand(SubCommand::with_name("md")
                    .about("Generates code file.")
                    .args_from_usage(
                        "<INPUT>              'Sets the input md file to use'
                        <OUTPUT>              'Sets the file to render output to'"))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("code") {
        generate_code(matches)?;
    }
    if let Some(matches) = matches.subcommand_matches("md") {
        generate_md(matches)?;
    }
    Ok(())
}

fn generate_code(matches: &ArgMatches)-> std::io::Result<()> {
    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();
    let lang = matches.value_of("LANG").unwrap();

    let mut input_file = File::open(input_file)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    let input_buffer = String::from_utf8(buffer).expect("Failed to parse buffer as U8");
    let just_code = remove_non_code(&input_buffer, lang);
    let mut out = File::create(output_file)?;
    write!(&mut out, "{}", just_code)?;
    Ok(())
}

fn generate_md(matches: &ArgMatches) -> std::io::Result<()> {
    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();

    let mut input_file = File::open(input_file)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    let input_buffer = String::from_utf8(buffer).expect("Failed to parse buffer as U8");
    let just_md = remove_code(&input_buffer);
    let mut out = File::create(output_file)?;
    write!(&mut out, "{}", just_md)?;
    Ok(())
}

fn remove_non_code(buffer: &String, lang: &str) -> String {
    let re_start = Regex::new(&format!("```{}.*", lang)).expect("Failed to create regex");
    let re_end = Regex::new(r"```$").expect("Failed to create regex");
    let re_tag = Regex::new(r"\\#S:([\w,=/\.]+)").expect("Failed to create regex");
    let mut keep = false;
    let mut include = false;
    let mut output = String::with_capacity(buffer.len());
    for line in buffer.lines() {
        if re_start.is_match(line) {
            keep = true;
            continue;
        }
        if re_end.is_match(line) {
            keep = false;
            continue;
        }
        if re_tag.is_match(line) {
            for cap in re_tag.captures_iter(line) {
                let tags: Vec<&str> = cap[1].split(',').collect();
                for tag in tags.iter() {
                    match *tag {
                        INCLUDE => include = true,
                        SKIP => include = false,
                        _ => (),
                    }
                    if tag.contains(EXTERNAL) {
                        let mut t = tag.split('=');
                        t.next();
                        if let Some(external_file) = t.next() {
                            output.push_str(&add_external(external_file));
                        }
                    }
                }
            }
        }
        if keep && include {
            output.push_str(&format!("{}\n", line));
        }
    }
    output
}

fn remove_code(buffer: &String) -> String {
    let re_code = Regex::new(r"```").expect("Failed to create regex");
    let re_tag = Regex::new(r"\\#S:([\w,]+)").expect("Failed to create regex");
    let mut code = false;
    let mut show = true;
    let mut output = String::with_capacity(buffer.len());
    for line in buffer.lines() {
        if re_code.is_match(line) {
            code = !code;
            if !code {
                let s = show;
                show = true;
                if !s {
                    continue;
                }
            }
        }
        if re_tag.is_match(line) {
            for cap in re_tag.captures_iter(line) {
                let tags: Vec<&str> = cap[1].split(',').collect();
                for tag in tags.iter() {
                    match *tag {
                        HIDE => show = false,
                        _ => (),
                    }
                }
            }
            continue;
        }
        if !code || code && show {
            output.push_str(&format!("{}\n", line));
        }
    }
    output
}

fn add_external(file: &str) -> String {
    let mut file = File::open(file).expect("Missing external file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read external file");

    String::from_utf8(buffer).expect("Failed to parse buffer as U8")
}
