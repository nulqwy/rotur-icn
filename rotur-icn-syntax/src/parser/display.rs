use std::fmt;

use super::ast::{Argument, Command, Icon};

impl fmt::Display for Icon<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Icon commands ({} total):", self.commands.len())?;

        if self.commands.is_empty() {
            writeln!(f, "<no commands>")?;
        } else {
            for command in &self.commands {
                writeln!(f, "- {command}")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Command<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" [ ", self.name)?;

        let mut first = true;
        for arg in &self.args {
            if !first {
                write!(f, ", ")?;
            } else {
                first = false;
            }

            write!(f, "{arg}")?;
        }

        write!(f, " ]")
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lit)
    }
}
