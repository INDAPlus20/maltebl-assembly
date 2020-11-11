#![allow(unused)]
use std::str::SplitWhitespace;

const INSTRUCT_LEN: u8 = 8;
const ADDR_LEN: u8 = 5;

#[repr(u8)]
enum OPCODE {
    ADD = 0b000,
    SUB = 0b001,
    IFEQ = 0b010,
    ADDI = 0b011,
    SUBI = 0b100,
    LI = 0b101,
    J = 0b110,
    SYSCALL = 0b111,
}
#[repr(u8)]
enum REGISTRY {
    R1 = 0b00,
    R2 = 0b01,
    R3 = 0b10,
    R4 = 0b11,
}

#[repr(u8)]
enum SYSCALL {
    PRINT = 0b00,
    READ = 0b01,
    TERMINATE = 0b10,
}

const OP_DICTIONARY: [(char, u8); 8] = [
    ('💘', OPCODE::ADD as u8),
    ('💔', OPCODE::SUB as u8),
    ('🤔', OPCODE::IFEQ as u8),
    ('👆', OPCODE::ADDI as u8),
    ('👇', OPCODE::SUBI as u8),
    ('👈', OPCODE::LI as u8),
    ('♿', OPCODE::J as u8),
    ('🤖', OPCODE::SYSCALL as u8),
];

const REG_DICTIONARY: [(char, u8); 4] = [
    ('🍩', REGISTRY::R1 as u8),
    ('👀', REGISTRY::R2 as u8),
    ('🍎', REGISTRY::R3 as u8),
    ('🍊', REGISTRY::R4 as u8),
];

const SYSCALL_DICTIONARY: [(char, u8); 3] = [
    ('📢', SYSCALL::PRINT as u8),
    ('📜', SYSCALL::READ as u8),
    ('🔪', SYSCALL::TERMINATE as u8),
];

const CONSTANTS: char = '💭';
const MACRO: char = '🔓';
const END_MACRO: char = '🔒';
const CODE: char = '💬';

const SECTION: [char; 4] = [CONSTANTS, MACRO, END_MACRO, CODE];

pub fn compile(code: String) -> Result<(), String> {
    let mut code_lines = code.lines().map(|l| {
        l.split_whitespace()
            .take_while(|w| !w.starts_with('#'))
            .map(|w| w.to_string())
    });

    let mut executable: Vec<u8> = Vec::new();
    let mut pc = 0;
    let mut sign_table: Vec<(String, Vec<u8>)> = Vec::new();
    for mut line in code_lines {
        pc += 1;
        if let Some(bits) =
            parse_line(line).map_err(|err| err + format!(" at line {}", pc).as_str())?
        {
            println!("{:#010b}", bits);
            executable.push(bits);
        }
    }
    Ok(())
}

fn first_pass<I: Iterator<Item = String>>(lines: &mut I) {
    let mut pc = 0;
    for mut line in lines {
        pc += 1;
        if let Some(word)
    }
}

fn parse_line<I: Iterator<Item = String>>(mut line: I) -> Result<Option<u8>, String> {
    let mut instruction: u8 = 0;
    if let Some(word) = line.next() {
        if word.ends_with(':') {
            parse_label(word)?;
        } else {
            let op_code = parse_op(word)?;
            instruction += op_code;
            if op_code <= 64 {
                instruction += parse_r(line)?;
            } else if op_code <= 160 {
                instruction += parse_i(line)?;
            } else if op_code == 192 {
                instruction += parse_jump(line)?;
            } else {
                instruction += parse_call(line)?;
            }
            return Ok(Some(instruction));
        }
        return Err("Error parsing line".to_string());
    }
    Ok(None)
}

fn parse_op(word: String) -> Result<u8, String> {
    if let Some(symbol) = word.chars().next() {
        for (c, b) in OP_DICTIONARY.iter() {
            if *c == symbol {
                let mut bits = *b;
                bits <<= 5;
                return Ok(bits);
            }
        }
    }
    Err("Error parsing op_code".to_string())
}

fn parse_reg(word: String) -> Result<u8, String> {
    if let Some(symbol) = word.chars().next() {
        for (c, b) in REG_DICTIONARY.iter() {
            if *c == symbol {
                return Ok(*b);
            }
        }
    }
    Err("Error parsing registry".to_string())
}

fn parse_label(label: String) -> Result<(), String> {
    todo!()
}

fn parse_r<I: Iterator<Item = String>>(mut line: I) -> Result<u8, String> {
    if let (Some(reg1), Some(reg2), Some(imm)) = (line.next(), line.next(), line.next()) {
        if let (Ok(mut reg1), Ok(mut reg2), Ok(imm)) =
            (parse_reg(reg1), parse_reg(reg2), imm.parse::<u8>())
        {
            reg1 <<= 3;
            reg2 <<= 1;
            let bits = reg1 + reg2 + imm;
            return Ok(bits);
        }
    }
    Err("Error parsing r-type instruction".to_string())
}

fn parse_i<I: Iterator<Item = String>>(mut line: I) -> Result<u8, String> {
    if let (Some(reg1), Some(imm)) = (line.next(), line.next()) {
        if let (Ok(mut reg1), Ok(imm)) = (parse_reg(reg1), imm.parse::<u8>()) {
            reg1 <<= 3;
            let bits = reg1 + imm;
            return Ok(bits);
        }
    }
    Err("Error parsing i-type instruction".to_string())
}

fn parse_jump<I: Iterator<Item = String>>(mut line: I) -> Result<u8, String> {
    if let Some(s_code) = line.next() {
        todo!()
    }
    Err("Error parsing jump instruction".to_string())
}

fn parse_call<I: Iterator<Item = String>>(mut line: I) -> Result<u8, String> {
    if let Some(s_code) = line.next() {
        if let Some(symbol) = s_code.chars().next() {
            for (c, b) in SYSCALL_DICTIONARY.iter() {
                if *c == symbol {
                    return Ok(*b);
                }
            }
        }
        return Err("Error parsing syscall_symbol".to_string());
    }
    Err("Error parsing syscall instruction".to_string())
}
