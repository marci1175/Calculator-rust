use anyhow::{self, Error};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
enum Math {
    Multiplication,
    Divide,
    Addition,
    Subtraction,
    // ^
    Power,

    //Invalid
    KurvaAnyad,
}

#[derive(Clone)]
struct Engine {
    pub buf: Vec<String>,
    num_list: Vec<f64>,
    expr_list: Vec<Math>,
}
impl Default for Engine {
    fn default() -> Self {
        Self {
            buf: Vec::new(),
            num_list: Vec::new(),
            expr_list: Vec::new(),
        }
    }
}

impl Engine {
    fn invalid_equation(&self, err: Error, item_at_fault: String) {
        println!(
            "[{err}]\n{}\n{}",
            format!(
                "Item(s) at fault => {} [at index : {}]",
                item_at_fault,
                *self
                    .buf
                    .iter()
                    .position(|element| *element == item_at_fault)
                    .get_or_insert(0)
                    + 1 as usize
            ),
            self.buf
                .iter()
                .map(|f| format!("{} ", f))
                .collect::<String>()
        );
        let _  = std::io::stdin();

        Engine::mathmain(self.clone());
    }

    fn mathmain(mut self) -> f64 {
        self.expr_list.clear();
        self.num_list.clear();

        let mut buf = String::default();

        std::io::stdin().read_line(&mut buf).unwrap();

        self.buf = buf
            .split_whitespace()
            .map(|f| f.to_string())
            .collect();
        if !self.buf.is_empty() {
            self.mathdivider()
        } else {
            self.invalid_equation(Error::msg("Invalid equation (Empty equation)"), "-".into());
            0.0
        }
    }

    fn mathdivider(mut self) -> f64 {
        for item in self.buf.clone() {
            match Engine::mathdecide(&item) {
                Ok(ok) => self.num_list.push(ok),
                Err(_expr) => self.expr_list.push(match item.as_str() {
                    "+" => Math::Addition,
                    "-" => Math::Subtraction,
                    "*" => Math::Multiplication,
                    "/" | "%" | ":" => Math::Divide,
                    "^" | "pow" => Math::Power,
                    _ => {
                        /*Go apeshit*/
                        self.invalid_equation(
                            Error::msg("Syntax Error (Invalid expression)"),
                            item.clone(),
                        );
                        Math::KurvaAnyad
                    }
                }),
            }
        }
        //finsihed sorting the 2 vectors
        self.mathengine()
    }

    fn mathdecide(token: &str) -> anyhow::Result<f64> {
        let token = token.parse::<f64>()?;

        Ok(token)
    }
    // 30 + 30 * 30
    fn mathengine(&mut self) -> f64 {
        let mut len: usize = self.expr_list.len();
        let mut index = 0;

        while index < len {
            if self.expr_list[index] == Math::Multiplication {
                let result = self.multiplication(self.num_list[index], self.num_list[index + 1]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1; // Decrement the length since we removed an element
            } else if self.expr_list[index] == Math::Divide {
                let result = self.divide(self.num_list[index], self.num_list[index + 1]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1; // Decrement the length since we removed an element
            } else if self.expr_list[index] == Math::Power {
                let result = self.power(self.num_list[index], self.num_list[index + 1]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1; // Decrement the length since we removed an element
            } else {
                index += 1;
            }
        }
        index = 0;
        while index < len {
            if self.expr_list[index] == Math::Addition {
                let result = self.addition(self.num_list[index], self.num_list[index + 1]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1; // Decrement the length since we removed an element
            } else if self.expr_list[index] == Math::Subtraction {
                let result = self.subtraction(self.num_list[index], self.num_list[index + 1]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1; // Decrement the length since we removed an element
            } else {
                index += 1;
            }
        }

        if self.num_list.len() > 1 {
            self.invalid_equation(
                Error::msg("Syntax Error (Invalid equation)"),
                self.num_list
                    .iter()
                    .map(|f| format!("{} ", f).to_string())
                    .collect(),
            );
        }

        self.num_list[0]
    }

    fn multiplication(&self, num1: f64, num2: f64) -> f64 {
        num1 * num2
    }
    fn divide(&self, num1: f64, num2: f64) -> f64 {
        num1 / num2
    }
    fn addition(&self, num1: f64, num2: f64) -> f64 {
        num1 + num2
    }
    fn subtraction(&self, num1: f64, num2: f64) -> f64 {
        num1 - num2
    }
    fn power(&self, num1: f64, num2: f64) -> f64 {
       num1.powf(num2)
    }
}

fn main() {
    println!("Use the calculator, with spaces in between each number / expression. (ie.: 30 + 30)\nStart typing :");
    loop {
        println!("Result : {}", Engine::mathmain(Engine::default()));
    }
}
