use std::env;

const AUTHOR:&str = "Jp";
const VERSION:&str = "0.1a";
const MEMORY_SIZE:usize = 1000;

fn main() {
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE]; // Ruban memoire

    println!("Welcome on bfi - the Brain Fuck Interpreter by {} (ver. {})",
        AUTHOR, VERSION);

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} file.bf", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let contents = std::fs::read_to_string(filename).expect("Can not open source file"); 
    let cleaned_source = clean_source(contents);
    run_interpreter(cleaned_source, &mut memory);
}

fn run_interpreter(source : String, memory: &mut [u8; MEMORY_SIZE]) {
    let mut ptr_m:usize = 0; // memory pointer starts on the first case of the 'memory'
    let mut ptr_i:usize = 0; // instruction pointer
    let mut stack: Stack<usize> = Stack::new();  
  
    let src = source.as_bytes();//.to_owned();
    while ptr_i < src.len() {
        let c = src[ptr_i] as char;
        //println!("{:?} --- {} ptr_i={}, ptr_m={}", memory, c, ptr_i, ptr_m);
        match c {
            '>' => ptr_m += 1,
            '<' => ptr_m -= 1,
            '+' => memory[ptr_m] += 1,
            '-' => if memory[ptr_m] > 0 { memory[ptr_m] -= 1 },
            '.' => print!("{}", memory[ptr_m] as char),
            ',' => todo!(), 
            '[' => stack.push(ptr_i),
            ']' => {
                    if memory[ptr_m] == 0 {
                        stack.pop().unwrap();
                    } else {
                        ptr_i = *stack.peek().unwrap();
                    }
                },
             _  => (),
        }
        ptr_i += 1;
    }
}

// Remove spaces, tabs, carriage return and commented lines starting with a '#'
fn clean_source(contents: String) -> String {
    let mut cleaned_source : String = "".to_owned();
    for line in contents.lines() {
        let _line = remove_whitespace(line);
        if _line.starts_with("#") { continue; }
        cleaned_source = format!("{}{}", cleaned_source, _line);
    }
    cleaned_source
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

// --- STACK -------------------------------------------------------------------
struct Stack<T> {
  stack: Vec<T>,
}

impl<T> Stack<T> {
  fn new() -> Self {
    Stack { stack: Vec::new() }
  }
#[allow(dead_code)]
  fn length(&self) -> usize {
    self.stack.len()
  }

  fn pop(&mut self) -> Option<T> {
    self.stack.pop()
  }

  fn push(&mut self, item: T) {
    self.stack.push(item)
  }
#[allow(dead_code)]
  fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }
  fn peek(&self) -> Option<&T> {
    self.stack.last()
  }
}
// USAGE OF STACK --------------------------------------------------------------
// let mut stack: Stack<isize> = Stack::new();
// stack.push(1);
// let item = stack.pop();
// assert_eq!(item.unwrap(), 1);
// -----------------------------------------------------------------------------
