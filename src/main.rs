use regex::Regex;
use std::{env, fs, process::exit};
fn convert_reg(reg: &str) -> u32 {
    let mut end: usize = reg.len();
    if reg.contains(",") || reg.contains(")") {
        end -= 1;
    }
    return reg[2..end].parse().unwrap();
}
fn convert_imm(imm: &str) -> u32 {
    return imm[1..imm.len()].parse().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file name>", args[0]);
        exit(1);
    }

    let source_file = fs::read_to_string(args[1].trim()).unwrap();
    let lines: Vec<&str> = source_file.split('\n').collect();
    for line in lines {
        let tokens: Vec<&str> = line.split(' ').collect();
        let mut outputs_line: Vec<String> = Vec::new();
        match tokens[0] {
            "ORI" => {
                outputs_line.push("010000".to_string());
                outputs_line.push(format!("{:05b}", convert_reg(tokens[2])));
                outputs_line.push(format!("{:05b}", convert_reg(tokens[1])));
                outputs_line.push(format!("{:016b}", convert_imm(tokens[3])));
            }
            "ORUI" => {
                outputs_line.push("010001".to_string());
                outputs_line.push(format!("{:05b}", convert_reg(tokens[2])));
                outputs_line.push(format!("{:05b}", convert_reg(tokens[1])));
                outputs_line.push(format!("{:016b}", convert_imm(tokens[3])));
            }
            "ADD" | "SUB" | "OR" | "AND" | "XOR" | "NOT" | "COMA" | "COMB" => {
                outputs_line.push("000001".to_string());
                outputs_line.push(format!("{:05b}", convert_reg(tokens[2])));
                outputs_line.push(format!("{:05b}", convert_reg(tokens[3])));
                outputs_line.push(format!("{:05b}", convert_reg(tokens[1])));
                let alu_ops = match tokens[0] {
                    "ADD" => "000",
                    "SUB" => "001",
                    "OR" => "010",
                    "AND" => "011",
                    "XOR" => "100",
                    "NOT" => "101",
                    "COMA" => "110",
                    "COMB" => "111",
                    _ => panic!(),
                };
                outputs_line.push(format!("00000000{}", alu_ops).to_string());
            }
            "LW" => {
                outputs_line.push("011000".to_string());
                let re = Regex::new(r"\(.*\)").unwrap();
                let displacement_source = re.captures(&tokens[2]).unwrap().get(0).unwrap().as_str();
                outputs_line.push(format!(
                    "{:05b}",
                    convert_reg(&displacement_source[1..displacement_source.len()])
                ));
                outputs_line.push(format!("{:05b}", convert_reg(tokens[1])));
                let displacement_raw = tokens[2].split('(').collect::<Vec<&str>>()[0];
                let without_prefix = displacement_raw.trim_start_matches("0x");
                let displacement = i64::from_str_radix(without_prefix, 16).unwrap();
                outputs_line.push(format!("{:016b}", displacement));
            }
            "SW" => {
                outputs_line.push("011100".to_string());
                let re = Regex::new(r"\(.*\)").unwrap();
                let displacement_source = re.captures(&tokens[2]).unwrap().get(0).unwrap().as_str();
                outputs_line.push(format!(
                    "{:05b}",
                    convert_reg(&displacement_source[1..displacement_source.len()])
                ));
                outputs_line.push(format!("{:05b}", convert_reg(tokens[1])));
                let displacement_raw = tokens[2].split('(').collect::<Vec<&str>>()[0];
                let without_prefix = displacement_raw.trim_start_matches("0x");
                let displacement = i64::from_str_radix(without_prefix, 16).unwrap();
                outputs_line.push(format!("{:016b}", displacement));
            }
            "BEQ" => {
                outputs_line.push("100100".to_string());
                outputs_line.push(format!("{:05b}", convert_reg(tokens[1])));
                outputs_line.push(format!("{:05b}", convert_reg(tokens[2])));
                outputs_line.push(format!("{:016b}", convert_imm(tokens[3])));
            }
            "JMP" => {
                outputs_line.push("110000".to_string());
                outputs_line.push(format!("{:026b}", tokens[1].parse::<i64>().unwrap()));
            }
            _ => panic!("Compilation Error: {}", line),
        }
        print!("{}", outputs_line.join("_"));
        println!(" // {}", line);
    }
}
