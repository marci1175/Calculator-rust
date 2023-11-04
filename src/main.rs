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
    Fact,
    Cos,
    Tan,
    Sin,
    Log,
    Rad,
    Ln,
    CRoot,
    SRoot,

    //Invalid
    KurvaAnyad,
}

#[derive(Clone)]
pub struct Engine {
    buf: Vec<String>,
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
            "\n\n[{err}]\n{}\n{}",
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
        let _ = std::io::stdin();

        Engine::mathmain(self.clone());
    }

    fn mathmain(mut self) -> f64 {
        self.expr_list.clear();
        self.num_list.clear();

        let mut buf = String::default();

        std::io::stdin().read_line(&mut buf).unwrap();

        self.buf = buf.split_whitespace().map(|f| f.to_string()).collect();
        if !self.buf.is_empty() {
            self.mathdivider()
        } else {
            self.invalid_equation(Error::msg("Invalid equation (Empty equation)"), " ".into());
            0.0
        }
    }

    fn mathdivider(mut self) -> f64 {
        for item in self.buf.clone() {
            match Engine::mathdecide(&item) {
                Ok(ok) => self.num_list.push(ok),
                Err(_expr) => self.expr_list.push(match item.to_lowercase().as_str() {
                    "+" => Math::Addition,
                    "-" => Math::Subtraction,
                    "*" => Math::Multiplication,
                    "/" | "%" | ":" => Math::Divide,
                    "^" | "pow" | "**" => Math::Power,
                    "cos" => Math::Cos,
                    "tan" => Math::Tan,
                    "sin" => Math::Sin,
                    "rad" => Math::Rad,
                    "log" => Math::Log,
                    "!" => Math::Fact,
                    "ln" => Math::Ln,
                    "croot" => Math::CRoot,
                    "sroot" => Math::SRoot,
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
                len -= 1;
            } else if self.expr_list[index] == Math::Divide {
                let result = self.divide(self.num_list[index], self.num_list[index + 1]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::Power {
                let result = self.power(self.num_list[index], self.num_list[index + 1]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::Fact {
                let result = self.fact(self.num_list[index]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::Cos {
                let result = self.cos(self.num_list[index]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::Log {
                let result = self.log(self.num_list[index], self.num_list[index + 1]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::Tan {
                let result = self.tan(self.num_list[index]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::Sin {
                let result = self.sin(self.num_list[index]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::Rad {
                let result = self.rad(self.num_list[index]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::CRoot {
                let result = self.croot(self.num_list[index]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::Ln {
                let result = self.ln(self.num_list[index]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } else if self.expr_list[index] == Math::SRoot {
                let result = self.sroot(self.num_list[index]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
            } 
            else {
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
                len -= 1;
            } else if self.expr_list[index] == Math::Subtraction {
                let result = self.subtraction(self.num_list[index], self.num_list[index + 1]);
                self.expr_list.remove(index);
                self.num_list.remove(index);
                self.num_list.remove(index);
                self.num_list.insert(index, result);
                len -= 1;
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
    fn fact(&self, num1: f64) -> f64 {
        if num1 == 0.0 || num1 == 1.0 {
            1.0
        } else {
            let mut result = 1.0;
            for i in 2..=num1 as u64 {
                result *= i as f64;
            }
            result
        }
    }
    fn cos(&self, num1: f64) -> f64 {
        num1.cos()
    }
    fn tan(&self, num1: f64) -> f64 {
        num1.cos()
    }
    fn sin(&self, num1: f64) -> f64 {
        num1.sin()
    }
    fn log(&self, num1: f64, num2: f64) -> f64 {
        num1.log(num2)
    }
    fn rad(&self, num1: f64) -> f64 {
        num1.to_radians()
    }
    fn ln(&self, num1: f64) -> f64 {
        num1.ln()
    }
    fn croot(&self, num1: f64) -> f64 {
        num1.cbrt().abs()
    }
    fn sroot(&self, num1: f64) -> f64 {
        num1.sqrt().abs()
    }
}

fn main() {
    println!("Use the calculator, with spaces in between each number / expression. (ie.: 30 + 30)\nStart typing :");
    loop {
        println!("Result : {}", Engine::mathmain(Engine::default()));
    }
}
