//! Calculator QObject implemented in Rust.
//!
//! All expression handling (tokenising, shunting-yard to RPN, evaluation) and
//! the input state machine are pure Rust. The QML UI only calls the invokable
//! methods and binds to the `displayText` property.
#![allow(non_snake_case)]

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, displayText)]
        #[qproperty(QString, expression)]
        #[qproperty(bool, justEvaluated)]
        type Calculator = super::CalculatorRust;

        #[qinvokable]
        #[cxx_name = "appendValue"]
        fn append_value(self: Pin<&mut Self>, value: &QString);

        #[qinvokable]
        fn backspace(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "resetAll"]
        fn reset_all(self: Pin<&mut Self>);

        #[qinvokable]
        fn evaluate(self: Pin<&mut Self>);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

/// Backing data for the `Calculator` QObject.
pub struct CalculatorRust {
    displayText: QString,
    expression: QString,
    justEvaluated: bool,
}

impl Default for CalculatorRust {
    fn default() -> Self {
        Self {
            displayText: QString::from("0"),
            expression: QString::from(""),
            justEvaluated: false,
        }
    }
}

impl qobject::Calculator {
    /// Append a digit, operator, dot or parenthesis to the expression.
    pub fn append_value(self: Pin<&mut Self>, value: &QString) {
        let mut st = self.load();
        st.append_value(&value.to_string());
        self.store(st);
    }

    /// Delete the last character (or reset if a result is showing).
    pub fn backspace(self: Pin<&mut Self>) {
        let mut st = self.load();
        st.backspace();
        self.store(st);
    }

    /// Clear everything.
    pub fn reset_all(self: Pin<&mut Self>) {
        let mut st = self.load();
        st.reset();
        self.store(st);
    }

    /// Evaluate the current expression.
    pub fn evaluate(self: Pin<&mut Self>) {
        let mut st = self.load();
        st.evaluate();
        self.store(st);
    }

    fn load(&self) -> CalcState {
        CalcState {
            expression: self.expression().to_string(),
            display: self.displayText().to_string(),
            just_evaluated: *self.justEvaluated(),
        }
    }

    fn store(mut self: Pin<&mut Self>, st: CalcState) {
        self.as_mut()
            .set_expression(QString::from(st.expression.as_str()));
        self.as_mut()
            .set_displayText(QString::from(st.display.as_str()));
        self.as_mut().set_justEvaluated(st.just_evaluated);
    }
}

// ---------------------------------------------------------------------------
// Pure calculator logic (no Qt types) — easy to reason about and test.
// ---------------------------------------------------------------------------

struct CalcState {
    expression: String,
    display: String,
    just_evaluated: bool,
}

impl CalcState {
    fn append_value(&mut self, value: &str) {
        if self.just_evaluated && !is_operator(value) {
            self.expression.clear();
        }
        self.just_evaluated = false;

        if value == "." {
            // Walk back through the current number; bail if it already has a dot.
            let chars: Vec<char> = self.expression.chars().collect();
            let mut i = chars.len() as isize - 1;
            while i >= 0 {
                let c = chars[i as usize];
                if is_operator_char(c) || c == '(' || c == ')' {
                    break;
                }
                if c == '.' {
                    return;
                }
                i -= 1;
            }
            let last = self.expression.chars().last();
            if self.expression.is_empty()
                || last.map(|c| is_operator_char(c) || c == '(').unwrap_or(false)
            {
                self.expression.push('0');
            }
        }

        if is_operator(value) {
            if self.expression.is_empty() {
                if value == "-" {
                    self.expression = "-".to_string();
                    self.display = self.expression.clone();
                }
                return;
            }
            let last = self.expression.chars().last().unwrap();
            if is_operator_char(last) {
                // Replace a trailing operator rather than stacking two.
                self.expression.pop();
                self.expression.push_str(value);
                self.display = self.expression.clone();
                return;
            }
        }

        self.expression.push_str(value);
        self.display = self.expression.clone();
    }

    fn backspace(&mut self) {
        if self.just_evaluated {
            self.reset();
            return;
        }
        if !self.expression.is_empty() {
            self.expression.pop();
            self.display = if self.expression.is_empty() {
                "0".to_string()
            } else {
                self.expression.clone()
            };
        }
    }

    fn reset(&mut self) {
        self.expression.clear();
        self.display = "0".to_string();
        self.just_evaluated = false;
    }

    fn evaluate(&mut self) {
        if self.expression.is_empty() {
            return;
        }
        let result = tokenize(&self.expression)
            .as_deref()
            .and_then(to_rpn)
            .as_deref()
            .and_then(eval_rpn);

        match result.and_then(format_result) {
            Some(s) => {
                self.expression = s.clone();
                self.display = s;
                self.just_evaluated = true;
            }
            None => {
                self.display = "Error".to_string();
                self.just_evaluated = true;
            }
        }
    }
}

fn is_operator_char(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/')
}

fn is_operator(s: &str) -> bool {
    s.len() == 1 && is_operator_char(s.chars().next().unwrap())
}

fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}

fn tokenize(input: &str) -> Option<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() || c == '.' {
            current.push(c);
        } else if is_operator_char(c) || c == '(' || c == ')' {
            if !current.is_empty() {
                tokens.push(std::mem::take(&mut current));
            }
            tokens.push(c.to_string());
        } else {
            return None;
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Some(tokens)
}

fn to_rpn(tokens: &[String]) -> Option<Vec<String>> {
    let mut output: Vec<String> = Vec::new();
    let mut ops: Vec<String> = Vec::new();

    for token in tokens {
        if token.parse::<f64>().is_ok() {
            output.push(token.clone());
        } else if is_operator(token) {
            while let Some(top) = ops.last() {
                if is_operator(top) && precedence(top) >= precedence(token) {
                    output.push(ops.pop().unwrap());
                } else {
                    break;
                }
            }
            ops.push(token.clone());
        } else if token == "(" {
            ops.push(token.clone());
        } else if token == ")" {
            let mut found_open = false;
            while let Some(top) = ops.pop() {
                if top == "(" {
                    found_open = true;
                    break;
                }
                output.push(top);
            }
            if !found_open {
                return None;
            }
        }
    }

    while let Some(top) = ops.pop() {
        if top == "(" {
            return None;
        }
        output.push(top);
    }

    Some(output)
}

fn eval_rpn(rpn: &[String]) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::new();

    for token in rpn {
        if let Ok(n) = token.parse::<f64>() {
            stack.push(n);
            continue;
        }

        if !is_operator(token) || stack.len() < 2 {
            return None;
        }

        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        let result = match token.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    return None;
                }
                a / b
            }
            _ => return None,
        };
        stack.push(result);
    }

    if stack.len() != 1 {
        return None;
    }
    Some(stack[0])
}

/// Mimic JS `Number(result.toPrecision(12)).toString()`: round to 12
/// significant digits, then render with the shortest round-trip form.
fn format_result(x: f64) -> Option<String> {
    if !x.is_finite() {
        return None;
    }
    let rounded = round_sig(x, 12);
    Some(format!("{}", rounded))
}

fn round_sig(x: f64, sig: i32) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    let digits = (sig - 1) - x.abs().log10().floor() as i32;
    let factor = 10f64.powi(digits);
    (x * factor).round() / factor
}
