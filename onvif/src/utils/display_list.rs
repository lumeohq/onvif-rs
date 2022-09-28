use std::fmt::{Debug, Display, Formatter, Write};

pub struct DisplayList<'a, T>(pub &'a [T]);

impl<T: Display> Debug for DisplayList<'_, T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_char('[')?;

        let mut peekable = self.0.iter().peekable();

        while let Some(element) = peekable.next() {
            element.fmt(formatter)?;
            if peekable.peek().is_some() {
                formatter.write_str(", ")?;
            }
        }

        formatter.write_char(']')
    }
}
