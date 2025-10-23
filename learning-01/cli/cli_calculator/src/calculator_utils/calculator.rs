use std::io;

pub struct CalculatorState {
    pub is_running: bool,
    pub input: Vec<f64>,
    pub output: String,
}

impl CalculatorState {
    pub fn new() -> Self {
        Self {
            is_running: true,
            input: Vec::new(),    // ← Matches Vec<f64> type
            output: String::new(),
        }
    }
    
    pub fn run(&mut self) {
        println!("=== CLI Calculator ===");
        println!("Press 'q' to quit\n");
        
        while self.is_running {
            // Clear previous inputs
            self.input.clear();
            
            // Get first number
            println!("Enter first number:");
            let mut first_input = String::new();
            io::stdin().read_line(&mut first_input).unwrap();
            
            // Check for quit
            if first_input.trim().eq_ignore_ascii_case("q") {
                println!("Goodbye!");
                self.is_running = false;
                break;
            }
            
            // Parse first number
            let num1: f64 = match first_input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number! Please try again.\n");
                    continue;
                }
            };
            
            // Get operator
            println!("Enter operator (+, -, *, /):");
            let mut operator_input = String::new();
            io::stdin().read_line(&mut operator_input).unwrap();
            
            let operator = operator_input.trim().chars().next().unwrap_or(' ');
            
            // Validate operator
            if !['+', '-', '*', '/'].contains(&operator) {
                println!("Invalid operator! Please use +, -, *, or /\n");
                continue;
            }
            
            // Get second number
            println!("Enter second number:");
            let mut second_input = String::new();
            io::stdin().read_line(&mut second_input).unwrap();
            
            // Parse second number
            let num2: f64 = match second_input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number! Please try again.\n");
                    continue;
                }
            };
            
            // Perform calculation
            let result = self.do_math(operator, num1, num2);
            self.output = format!("{} {} {} = {}", num1, operator, num2, result);
            println!("\nResult: {}\n", self.output);
        }
    }
    pub fn do_math(&self, operator: char, num1: f64, num2: f64) -> f64 {
        match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => {
                if num2 == 0.0 {
                    println!("Warning: Division by zero!");
                    0.0
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("Invalid operator!");
                0.0
            }
        }
    }
    }

      

