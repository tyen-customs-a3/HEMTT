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
        output.write_cstring(&self.name().value)?;
        let mut written = self.name().value.len() + 1;

        // Write number of arguments
        output.write_u8(self.args().len() as u8)?;
        written += 1;

        // Write each argument
        for arg in self.args() {
            output.write_cstring(&arg.value)?;
            written += arg.value.len() + 1;
        }

        Ok(written)
    }

    fn rapified_length(&self) -> usize {
        // Name length + null terminator + arg count byte + 
        // sum of (arg length + null terminator) for each arg
        self.name().value.len() + 1 + 1 + 
            self.args().iter().map(|arg| arg.value.len() + 1).sum::<usize>()
    }

    fn rapified_code(&self) -> u8 {
        5 // Special code for macros
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Str;

    #[test]
    fn test_macro_rapify() {
        let name_str = Str {
            value: "TEST".to_string(),
            span: 0..4,
        };
        let args = vec![
            Str {
                value: "arg1".to_string(),
                span: 5..9,
            },
            Str {
                value: "arg2".to_string(),
                span: 10..14,
            },
        ];
        let macro_expr = MacroExpression::new(name_str, args, 0..15).unwrap();

        let mut buffer = Vec::new();
        let written = macro_expr.rapify(&mut buffer, 0).unwrap();

        assert_eq!(written, macro_expr.rapified_length());
        assert_eq!(buffer, b"TEST\0\x02arg1\0arg2\0");
    }

    #[test]
    fn test_no_args_macro_rapify() {
        // Test macro with no args (WEAPON_FIRE_BEGIN with no parentheses)
        let name_str = Str {
            value: "WEAPON_FIRE_BEGIN".to_string(),
            span: 0..16,
        };
        let macro_expr = MacroExpression::new(name_str, vec![], 0..18).unwrap();

        let mut buffer = Vec::new();
        let written = macro_expr.rapify(&mut buffer, 0).unwrap();

        assert_eq!(written, macro_expr.rapified_length());
        // Should contain just the name, null terminator, and arg count of 0
        assert_eq!(buffer, b"WEAPON_FIRE_BEGIN\0\x00");
        
        // Test macro with empty parentheses (WEAPON_FIRE_END())
        let name_str = Str {
            value: "WEAPON_FIRE_END".to_string(),
            span: 0..14,
        };
        let args = vec![
            Str {
                value: "".to_string(), 
                span: 16..16
            }
        ];
        let macro_expr = MacroExpression::new(name_str, args, 0..18).unwrap();

        let mut buffer = Vec::new();
        let written = macro_expr.rapify(&mut buffer, 0).unwrap();

        assert_eq!(written, macro_expr.rapified_length());
        // Should contain the name, null terminator, arg count of 1, and an empty string arg
        assert_eq!(buffer, b"WEAPON_FIRE_END\0\x01\0");
    }
} 