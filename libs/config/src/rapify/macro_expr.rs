use byteorder::WriteBytesExt;
use hemtt_common::io::WriteExt;

use crate::MacroExpression;

use super::Rapify;

impl Rapify for MacroExpression {
    fn rapify<O: std::io::Write>(
        &self,
        output: &mut O,
        _offset: usize,
    ) -> Result<usize, std::io::Error> {
        // Write macro name
        output.write_cstring(&self.name.value)?;
        let mut written = self.name.value.len() + 1;

        // Write number of arguments
        output.write_u8(self.args.len() as u8)?;
        written += 1;

        // Write each argument
        for arg in &self.args {
            output.write_cstring(&arg.value)?;
            written += arg.value.len() + 1;
        }

        Ok(written)
    }

    fn rapified_length(&self) -> usize {
        // Name length + null terminator + arg count byte + 
        // sum of (arg length + null terminator) for each arg
        self.name.value.len() + 1 + 1 + 
            self.args.iter().map(|arg| arg.value.len() + 1).sum::<usize>()
    }

    fn rapified_code(&self) -> u8 {
        5 // Special code for macros
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Str;
    use std::ops::Range;

    #[test]
    fn test_macro_rapify() {
        let macro_expr = MacroExpression {
            name: Str {
                value: "TEST".to_string(),
                span: 0..4,
            },
            args: vec![
                Str {
                    value: "arg1".to_string(),
                    span: 5..9,
                },
                Str {
                    value: "arg2".to_string(),
                    span: 10..14,
                },
            ],
            span: 0..15,
        };

        let mut buffer = Vec::new();
        let written = macro_expr.rapify(&mut buffer, 0).unwrap();

        assert_eq!(written, macro_expr.rapified_length());
        assert_eq!(buffer, b"TEST\0\x02arg1\0arg2\0");
    }
} 