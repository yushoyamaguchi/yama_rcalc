use crate::parser;
use tracing::debug;

use crossterm::style::{Print};
use crossterm::{execute};
use std::io::Write;

pub struct Calc;

impl Calc {
    pub fn new() -> Self {
        Self
    }

    pub fn run_expr(&mut self, expr: &str)  {
        execute!(std::io::stdout(), Print("result\r\n")).ok();
    }
}
