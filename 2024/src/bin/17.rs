//  rustc 17.rs && ./17

use std::cmp::min;
use std::fs::File;
use std::i64;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "17.in";
const FAILED: &str = "FAILED";

struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: Vec<usize>,
}

impl Computer {
    pub fn reset(&mut self, a: i64, b: i64, c: i64) {
        self.reg_a = a;
        self.reg_b = b;
        self.reg_c = c;
    }

    pub fn run(&mut self, part2: bool) -> String {
        let n = self.program.len();
        let mut output: Vec<usize> = vec![];
        let mut i = 0;
        while i < n {
            let opcode = self.program[i];
            let operand = self.program[i + 1];
            let mut jumped = false;
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => {
                    if self.reg_a != 0 {
                        jumped = true;
                        i = operand;
                    }
                }
                4 => self.bxc(),
                5 => {
                    output.push(self.out(operand));
                    if part2 && (*output.last().unwrap() != self.program[output.len() - 1]) {
                        return FAILED.to_string();
                    }
                }
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => unreachable!(),
            }
            // self.print(i, opcode, operand, &output);
            if !jumped {
                i += 2; // if it did not jump, then +2.
            }
        }
        if part2 && output.len() != self.program.len() {
            return FAILED.to_string();
        }
        let v: Vec<String> = output.iter().map(|x| (*x).to_string()).collect();
        return v.join(",");
    }

    // fn print(&self, i: usize, opcode: usize, operand: usize, output: &Vec<usize>) {
    //     println!(
    //         "{} opcode={} operand={}  : (a, b, c) = ({}, {}, {})  :  out = {:?}",
    //         i, opcode, operand, self.reg_a, self.reg_b, self.reg_c, output
    //     );
    // }

    fn combo_operand(&self, operand: usize) -> i64 {
        if operand < 4 {
            return operand as i64;
        }
        return match operand {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        };
    }

    fn div_and_truncate(&self, val: i64, operand: usize) -> i64 {
        return val / 2_i64.pow(self.combo_operand(operand) as u32);
    }

    fn adv(&mut self, operand: usize) {
        self.reg_a = self.div_and_truncate(self.reg_a, operand);
    }

    fn bxl(&mut self, operand: usize) {
        self.reg_b ^= operand as i64;
    }

    fn bst(&mut self, operand: usize) {
        self.reg_b = self.combo_operand(operand) % 8;
    }

    fn bxc(&mut self) {
        self.reg_b ^= self.reg_c;
    }

    fn out(&mut self, operand: usize) -> usize {
        return (self.combo_operand(operand) % 8) as usize;
    }

    fn bdv(&mut self, operand: usize) {
        self.reg_b = self.div_and_truncate(self.reg_a, operand);
    }

    fn cdv(&mut self, operand: usize) {
        self.reg_c = self.div_and_truncate(self.reg_a, operand);
    }
}

/*
    The program is expected to execute a loop:
        + Calculates B and C based on A.
        + Outputs B at the end.
        + Jumps back to opcode 0.
    At the end of the 1st execution:
        + B = ((A%8) XOR 5 XOR floor(A / 2^((A%8) XOR 5)) XOR 6) % 8
        + A = floor(A / 8)
        + C = does not matter, since it will be overriden in the next execution.
    Notice that:
        + A%8 means the last 3 bits of A.
        + A/2^x means shifting A 2^x bits to the right.
    Let x = (A%8)^5, then B = (x XOR floor(A / 2^x) XOR 6) % 8
        and B at the end of the 1st execution must be equal to 2,
        which reflects the relationship between the first few bits of A.
    Thus we can do a backtracking for each trio bits of A.
    Assuming A has 3 * (lens of the program) bits = 3*16 = 48, allow leading 0s.
*/
const NULL: i32 = i32::MAX;

fn bits_to_num(bits: &Vec<i32>) -> i64 {
    let mut ans: i64 = 0;
    for i in 0..bits.len() {
        ans += (bits[i] as i64) * (1 << i);
    }
    return ans;
}

fn set_bits(input_bits: &Vec<i32>, start: usize, candidate: i32) -> Vec<i32> {
    assert!(candidate < 8);
    let len = input_bits.len();
    let bits = [candidate & 1, (candidate >> 1) & 1, (candidate >> 2) & 1];
    for i in 0..3 {
        if start + i >= len {
            if bits[i] != 0 {
                return vec![];
            }
            continue;
        }
        if input_bits[start + i] != NULL && input_bits[start + i] != bits[i] {
            return vec![];
        }
    }
    let mut ans_bits = input_bits.clone();
    for i in 0..3 {
        if start + i < len {
            ans_bits[start + i] = bits[i];
        }
    }
    return ans_bits;
}

fn tryf(ans_bits: &Vec<i32>, start: usize, program: &Vec<usize>) -> i64 {
    if start >= ans_bits.len() {
        return bits_to_num(ans_bits);
    }
    let mut ans = i64::MAX; // without this, 136933420830465 is to high
    let expected_instruction = program[start / 3] as i32;
    for candidate in 0..8 {
        // try to set 3 bits [start, start+1, start+2] = candidate:
        let new_ans_bits_0 = set_bits(&ans_bits, start, candidate);
        if new_ans_bits_0.is_empty() {
            continue;
        }

        // verify
        // B = ((A%8) XOR 5 XOR floor(A / 2^((A%8) XOR 5)) XOR 6) % 8
        // Let x = (A%8)^5, then B = (x XOR floor(A / 2^x) XOR 6) % 8
        let x = candidate ^ 5;
        for next_candidate in 0..8 {
            // next_candidate = last 3 bits of floor(A / 2^x)
            if (x ^ next_candidate ^ 6) % 8 != expected_instruction {
                continue;
            }
            // try to set 3 bits [start+x, start+x+1, start+x+2] = next_candidate:
            let new_ans_bits_1 = set_bits(&new_ans_bits_0, start + x as usize, next_candidate);
            if new_ans_bits_1.is_empty() {
                continue;
            }
            ans = min(ans, tryf(&new_ans_bits_1, start + 3, &program));
        }
    }
    return ans;
}

fn part2(program: &Vec<usize>) -> i64 {
    let n = program.len();
    assert!(n == 16);
    let ans = tryf(&vec![NULL; 3 * n], 0, program);
    if ans < i64::MAX {
        return ans;
    }
    unreachable!();
}

fn main() {
    let (mut reg_a, mut reg_b, mut reg_c): (i64, i64, i64) = (0, 0, 0);
    let mut program: Vec<usize> = vec![];

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let v: Vec<&str> = l.split(": ").collect();
        if l.starts_with("Register A") {
            reg_a = v[1].parse().unwrap();
        } else if l.starts_with("Register B") {
            reg_b = v[1].parse().unwrap();
        } else if l.starts_with("Register C") {
            reg_c = v[1].parse().unwrap();
        } else if l.starts_with("Program") {
            program = v[1].split(",").map(|c| (*c).parse().unwrap()).collect();
        }
    }
    println!("reg_a = {}\nreg_b = {}\nreg_c = {}", reg_a, reg_b, reg_c);
    println!("program = {:?}", program);

    let mut computer = Computer {
        reg_a: reg_a,
        reg_b: reg_b,
        reg_c: reg_c,
        program: program.clone(),
    };
    let output = computer.run(false);
    println!("ans_part1 = {}\n", output);

    // part 2
    let a = part2(&program);
    println!("ans_part2 = {}", a);
    computer.reset(a, reg_b, reg_c);
    println!("verify_part2 = {}", computer.run(true));
}
