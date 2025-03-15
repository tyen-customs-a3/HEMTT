use hemtt_common::io::WriteExt;

use crate::Define;

use super::Rapify;

impl Rapify for Define {
    fn rapify<O: std::io::Write>(
        &self,
        output: &mut O,
        _offset: usize,
    ) -> Result<usize, std::io::Error> {
        let define_str = match self {
            Define::Simple { name, value, .. } => {
                format!("#define {name} {value}")
            }
            Define::Macro {
                name, params, body, ..
            } => {
                format!(
                    "#define {name}({}) {body}",
                    params.join(", "),
                )
            },
            Define::Include { path, .. } => {
                format!("#include \"{}\"", path)
            }
        };
        output.write_all(define_str.as_bytes())?;
        output.write_all(b"\n")?;
        Ok(define_str.len() + 1)
    }

    fn rapified_length(&self) -> usize {
        match self {
            Define::Simple { name, value, .. } => {
                format!("#define {name} {value}").len() + 1 // +1 for newline
            }
            Define::Macro {
                name, params, body, ..
            } => {
                format!(
                    "#define {name}({}) {body}",
                    params.join(", "),
                )
                .len()
                    + 1 // +1 for newline
            },
            Define::Include { path, .. } => {
                format!("#include \"{}\"", path).len() + 1 // +1 for newline
            }
        }
    }
    
    fn rapified_code(&self) -> u8 {
        // Use string code for defines
        0
    }
} 