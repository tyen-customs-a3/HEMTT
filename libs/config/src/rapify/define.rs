use hemtt_common::io::WriteExt;

use crate::Define;

use super::Rapify;

impl Rapify for Define {
    fn rapify<O: std::io::Write>(
        &self,
        output: &mut O,
        _offset: usize,
    ) -> Result<usize, std::io::Error> {
        // Defines are not rapified directly as they are preprocessor directives
        // We just write them as a comment in their string form
        let define_str = match self {
            Self::Simple { name, value, .. } => format!("#define {name} {value}"),
            Self::Macro { name, params, body, .. } => {
                let params_str = params.join(",");
                format!("#define {name}({params_str}) {body}")
            }
        };
        
        output.write_cstring(&define_str)?;
        Ok(define_str.len() + 1)
    }

    fn rapified_length(&self) -> usize {
        // Calculate the length of the string representation
        match self {
            Self::Simple { name, value, .. } => {
                // #define NAME VALUE + null terminator
                9 + name.len() + 1 + value.len() + 1
            }
            Self::Macro { name, params, body, .. } => {
                // #define NAME(PARAM1,PARAM2,...) BODY + null terminator
                9 + name.len() + 1 + 
                params.iter().map(|p| p.len()).sum::<usize>() + 
                params.len().saturating_sub(1) + 2 + // commas and parentheses
                body.len() + 1
            }
        }
    }
    
    fn rapified_code(&self) -> u8 {
        // Use string code for defines
        0
    }
} 