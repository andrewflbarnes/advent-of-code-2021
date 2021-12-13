use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone)]
pub enum FoldParseError {
    BadAxis(String),
    BadLine(String),
    Unparseable(String),
}

impl fmt::Display for FoldParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FoldParseError::BadAxis(axis) => write!(f, "Invalid axis: {}", axis),
            FoldParseError::BadLine(line) => write!(f, "Invalid line: {}", line),
            FoldParseError::Unparseable(str) => write!(f, "Invalid string: {}", str),
        }
    }
}

#[derive(Debug)]
pub struct Fold {
    axis: String,
    line: u16,
}

impl Fold {
    pub fn new(axis: &str, line: u16) -> Self {
        Fold {
            axis: axis.to_string(),
            line,
        }
    }

    pub fn reflect_point(&self, point: &(u16, u16)) -> (u16, u16) {
        if self.axis == "x" && self.line < point.0 {
            return (2 * self.line - point.0, point.1);
        }

        if self.axis == "y" && self.line < point.1 {
            return (point.0, 2 * self.line - point.1);
        }

        *point
    }
}

impl FromStr for Fold {
    type Err = FoldParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("fold along ") {
            return Err(FoldParseError::Unparseable(s.to_string()))
        }

        let instruction = &s[11..].to_string();
        let (axis, line) =  instruction.split_once("=").unwrap();

        if axis != "x" && axis != "y" {
            return Err(FoldParseError::BadAxis(axis.to_string()));
        }

        let line = line.parse::<u16>()
            .map_err(|_| FoldParseError::BadLine(line.to_string()))?;

        Ok(Fold::new(axis, line))
    }
}