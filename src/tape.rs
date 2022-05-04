mod doublevec;
use crate::Symbol;
use doublevec::DoubleVec;

#[derive(Debug, Clone)]
pub struct Tape {
    buf: DoubleVec<Cell>,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            buf: DoubleVec::new(),
        }
    }
    pub fn with<T: Into<Vec<Cell>>>(v: T) -> Self {
        Self {
            buf: DoubleVec::with(Vec::new(), v.into()),
        }
    }

    pub fn iter(&self) -> doublevec::Iter<Cell> {
        self.buf.iter()
    }
    pub fn iter_mut(&mut self) -> doublevec::IterMut<Cell> {
        self.buf.iter_mut()
    }
}

impl std::ops::Index<isize> for Tape {
    type Output = Cell;
    fn index(&self, index: isize) -> &Self::Output {
        match self.buf.get(index) {
            Some(s) => s,
            None => &Cell::None,
        }
    }
}

impl std::ops::IndexMut<isize> for Tape {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        if !self.buf.fits(index) {
            self.buf.grow(index + 1, Cell::None);
        }
        &mut self.buf[index]
    }
}

impl std::fmt::Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|")?;
        for i in self.buf.back().iter().rev() {
            if let Cell::Some(Symbol(s)) = i {
                write!(f, " {s}")?;
            } else {
                write!(f, " ")?;
            }
            write!(f, " |")?;
        }
        for i in self.buf.front().iter() {
            if let Cell::Some(Symbol(s)) = i {
                write!(f, " {s}")?;
            } else {
                write!(f, " ")?;
            }
            write!(f, " |")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Some(Symbol),
    None,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Some(ref s) => write!(f, "{}", s.0),
            Cell::None => write!(f, "_"),
        }
    }
}

#[macro_export]
macro_rules! tape {
    () => {{
        $crate::tape::Tape::new()
    }};
    ($($($e:tt)?)|*) => {{
        let mut tmp = Vec::new();
        $(
            $(
                let s = stringify!($e);
                if s == "_" {
                    tmp.push(None);
                } else {
                    tmp.push(Some(s.to_string()));
                }
            )?
        )*
        $crate::tape::Tape::with(tmp)
    }};
}

#[cfg(test)]
mod test {

    #[test]
    fn tape_test() {}
}
