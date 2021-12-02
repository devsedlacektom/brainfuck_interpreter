use self::BrainfuckInstructions::*;
use core::panic;
use std::{
    fs::File,
    io::{stdin, Read},
};

/// This is an enum containing all brainfuck instructions
#[derive(Clone, Copy, PartialEq)]
pub enum BrainfuckInstructions {
    PointerIncrement,
    PointerDecrement,
    CellIncrement,
    CellDecrement,
    PutChar,
    GetChar,
    WhileStart,
    WhileEnd,
}

/// Interpreter containing all necessary data together
pub struct Interpreter {
    memory: [u8; 30000],
    memory_index: usize,
    instructions: Vec<BrainfuckInstructions>,
    instruction_index: usize,
}

impl Interpreter {
    /// Initialize a new intepreter
    /// Returns
    /// -------
    /// new interpreter instance
    pub fn new() -> Interpreter {
        Interpreter {
            memory: [0; 30000],
            memory_index: 0,
            instructions: Vec::new(),
            instruction_index: 0,
        }
    }

    /// Retain only vital parts of the code (i.e. -> only the instructions)
    /// Parameters
    /// ----------
    /// code: content of the source file loaded into a string
    ///
    /// Returns
    /// -------
    /// String which contains only relevant source code characters
    fn lexical_analysis(&self, code: String) -> String {
        let result: String = code
            .chars()
            .map(|character| match character {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => character,
                _ => '_',
            })
            .filter(|character| character != &'_')
            .collect();
        result
    }

    /// Check whether the syntax is correct
    /// in our case -> we can only check if the loops are closed correctly
    ///
    /// Parameters
    /// ----------
    /// code: filtered source code
    ///
    /// Returns
    /// -------
    /// true if all loops were written correctly
    /// false otherwise
    fn syntactic_analysis(&self, code: &str) -> bool {
        let mut stack: Vec<char> = Vec::new();

        let parentheses = code
            .chars()
            .filter(|character| character == &'[' || character == &']');

        for parenthesis in parentheses {
            match parenthesis {
                '[' => stack.push(parenthesis),
                _ => match stack.pop() {
                    Some(_) => {}
                    None => return false,
                },
            }
        }

        stack.is_empty()
    }

    /// Convert the file content into an internal representation of the code
    /// stored in the structure
    ///
    /// Parameters
    /// ----------
    /// code: filtered source code
    fn generate_internal_representation(&mut self, code: String) {
        self.instructions = code
            .chars()
            .map(|character| match character {
                '>' => PointerIncrement,
                '<' => PointerDecrement,
                '+' => CellIncrement,
                '-' => CellDecrement,
                '.' => PutChar,
                ',' => GetChar,
                '[' => WhileStart,
                _ => WhileEnd,
            })
            .collect();
    }

    /// Obtain instructions from file and store them inside the interpreter
    /// Runs lexical and syntactic analysis
    ///
    /// Parameters
    /// ----------
    /// instructions_file: source code file
    pub fn initialize_instructions(&mut self, instructions_file: &mut File) {
        let mut file_content: String = String::new();

        if instructions_file.read_to_string(&mut file_content).is_err() {
            panic!("Cannot read the file, cannot proceed any longer");
        }

        // run lexical analysis
        let code = self.lexical_analysis(file_content);

        // run syntactic analysis
        if !self.syntactic_analysis(&code) {
            panic!(
                "The file does not contain a valid brainfuck file, fix loop enclosure and rerun!"
            );
        }

        // generate internal representation
        self.generate_internal_representation(code);
    }

    // ======================
    // ======================
    // INTERPRET FUNCTIONS
    // ======================
    // ======================

    // NECESSARY AUXILIARY FUNCTIONS
    // ======================

    /// Get value of current cell pointer
    fn get_memory_value(&self) -> u8 {
        match self.memory.get(self.memory_index) {
            Some(x) => *x,
            _ => panic!("Cannot access memory!"),
        }
    }

    /// Get current instruction
    fn get_instruction(&self) -> BrainfuckInstructions {
        match self.instructions.get(self.instruction_index) {
            Some(instruction) => *instruction,
            _ => panic!("Cannot obtain next instruction"),
        }
    }

    /// Update memory cell - increment / decrement
    ///
    /// Parameters
    /// ----------
    /// increment: true to increment, false to decrement
    fn update_memory_cell(&mut self, increment: bool) {
        let current_value = self.get_memory_value();

        // incorrect memory update
        if current_value == 255 && increment || current_value == 0 && !increment {
            let message = if increment {
                "exceed value 255"
            } else {
                "go below 0 value"
            };

            panic!("Memory overflow! Cannot {}.", message);
        }

        // if the code before proceeds, this operation is safe
        match increment {
            true => self.memory[self.memory_index] += 1,
            false => self.memory[self.memory_index] -= 1,
        }
    }

    /// Set memory cell to desired value
    ///
    /// Parameters
    /// ----------
    /// value: new memory cell value
    fn set_memory_cell(&mut self, value: u8) {
        // checks memory access
        let _ = self.get_memory_value();

        self.memory[self.memory_index] = value;
    }

    /// Update memory cell pointer - increment / decrement
    ///
    /// Parameters
    /// ----------
    /// increment: true to increment, false to decrement
    fn update_memory_pointer(&mut self, increment: bool) {
        if self.memory_index == self.memory.len() - 1 && increment
            || self.memory_index == 0 && !increment
        {
            let message = if increment { "over 30KB" } else { "below 0B" };
            panic!("Memory pointer overflow {} address", message);
        }

        match increment {
            true => self.memory_index += 1,
            false => self.memory_index -= 1,
        }
    }

    /// Set instruction pointer to desired value
    /// -> used to be able to do "jumps" -> for loop purposes
    ///
    /// Parameters
    /// ----------
    /// value: new instruction pointer value
    fn set_instruction_pointer(&mut self, value: usize) {
        if value > self.instructions.len() {
            panic!("Unable to move the instruction pointer! Instructions out of bounds");
        }

        self.instruction_index = value;
    }

    /// Increment instruction pointer
    fn increment_instruction_pointer(&mut self) {
        self.instruction_index += 1;
    }

    // INTERPRETABLE CALLS
    // ======================

    /// Increment pointer
    fn pointer_increment(&mut self) {
        self.update_memory_pointer(true);
    }

    /// Decrement pointer
    fn pointer_decrement(&mut self) {
        self.update_memory_pointer(false);
    }

    /// Increment cell value
    fn cell_increment(&mut self) {
        self.update_memory_cell(true);
    }

    /// Decrement cell value
    fn cell_decrement(&mut self) {
        self.update_memory_cell(false);
    }

    /// Print character (value of currently pointed memory cell) to stdin
    fn print_character(&mut self) {
        print!("{}", self.get_memory_value() as char);
    }

    /// Obtain one byte from stdin and write it to current memory cell
    fn get_character(&mut self) {
        let mut input: [u8; 1] = [0; 1];

        match stdin().read_exact(&mut input) {
            Ok(_) => self.set_memory_cell(input[0]),
            _ => panic!("Cannot obtain this value!"),
        }
    }

    /// Start executing a loop
    /// if another loop is encountered this function is called recursively
    fn exec_loop(&mut self) {
        // set the initial instruction as the instruction for the loop start
        let initial_instruction = self.instruction_index;

        // the smallest possible correct offset is one
        let mut offset_last_position = self.instruction_index + 1;

        // loop to always get back to the initial condition after
        // executing more instructions
        loop {
            // check whether to perform loop
            if self.get_memory_value() == 0 {
                // move after the loop (at least one position over)
                self.set_instruction_pointer(offset_last_position);
                return;
            }

            // get to first non-loop instruction and start interpreting them in the loop
            self.increment_instruction_pointer();

            // execute instructions until you find another loop (that is handled in exec_instruction)
            // or find the end of the loop
            loop {
                match self.get_instruction() {
                    WhileEnd => {
                        // set point of the loop end, if the loop breaks
                        // the exec_instruction
                        // will move the instruction pointer past the loop end
                        // and carry out rest of the code
                        offset_last_position = self.instruction_index;
                        break;
                    }
                    _ => self.exec_instruction(),
                }
            }

            // jump back to loop start
            self.set_instruction_pointer(initial_instruction);
        }
    }

    /// Executes instruction that's currently due to be executed
    fn exec_instruction(&mut self) {
        // execute instruction
        match self.get_instruction() {
            PointerIncrement => self.pointer_increment(),
            PointerDecrement => self.pointer_decrement(),
            CellIncrement => self.cell_increment(),
            CellDecrement => self.cell_decrement(),
            PutChar => self.print_character(),
            GetChar => self.get_character(),
            WhileStart => self.exec_loop(),
            _ => {} // end of loop => noop, just move the instruction pointer
        }

        // move to next instruction
        self.increment_instruction_pointer();
    }

    /// Start interpreting instructions, ends after interpreting last instruction
    pub fn interpret(&mut self) {
        while self.instruction_index < self.instructions.len() {
            self.exec_instruction();
        }
    }
}
