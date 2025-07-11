use std::{ops::Range, sync::Arc};

use hemtt_common::config::{LintConfig, LintEnabled};
use hemtt_workspace::{
    lint::{AnyLintRunner, Lint, LintRunner},
    reporting::{Code, Codes, Diagnostic, Processed, Severity},
};

use crate::{analyze::LintData, BinaryCommand, Expression, UnaryCommand};

crate::analyze::lint!(LintS11IfNotElse);

impl Lint<LintData> for LintS11IfNotElse {
    fn ident(&self) -> &'static str {
        "if_not_else"
    }

    fn sort(&self) -> u32 {
        110
    }

    fn description(&self) -> &'static str {
        "Checks for unneeded not"
    }

    fn documentation(&self) -> &'static str {
        r"### Example

**Incorrect**
```sqf
if (!alive player) then { player } else { objNull };
```
**Correct**
```sqf
if (alive player) then { objNull } else { player };
```
`!` can be removed and `else` order swapped
"
    }

    fn default_config(&self) -> LintConfig {
        // Disabled by default because it requires a lot of code moving to remove a single `!`
        LintConfig::help().with_enabled(LintEnabled::Pedantic)
    }

    fn runners(&self) -> Vec<Box<dyn AnyLintRunner<LintData>>> {
        vec![Box::new(Runner)]
    }
}

struct Runner;
impl LintRunner<LintData> for Runner {
    type Target = crate::Expression;

    fn run(
        &self,
        _project: Option<&hemtt_common::config::ProjectConfig>,
        config: &LintConfig,
        processed: Option<&hemtt_workspace::reporting::Processed>,
        _runtime: &hemtt_common::config::RuntimeArguments,
        target: &Self::Target,
        _data: &LintData,
    ) -> Codes {
        let Some(processed) = processed else {
            return Vec::new();
        };
        let Expression::BinaryCommand(BinaryCommand::Named(name), if_cmd, code, _) = target else {
            return Vec::new();
        };
        if name.to_lowercase() != "then" {
            return Vec::new();
        }
        let Expression::UnaryCommand(UnaryCommand::Named(_), condition, _) = &**if_cmd else {
            return Vec::new();
        };
        let Expression::BinaryCommand(BinaryCommand::Else, _, _, _) = &**code else {
            return Vec::new();
        };
        let Expression::UnaryCommand(UnaryCommand::Not, _, _) = &**condition else {
            return Vec::new();
        };
        vec![Arc::new(CodeS11IfNot::new(
            condition.span(),
            processed,
            config.severity(),
        ))]
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct CodeS11IfNot {
    span: Range<usize>,
    severity: Severity,
    diagnostic: Option<Diagnostic>,
}

impl Code for CodeS11IfNot {
    fn ident(&self) -> &'static str {
        "L-S11"
    }

    fn link(&self) -> Option<&str> {
        Some("/analysis/sqf.html#if_not_else")
    }

    fn severity(&self) -> Severity {
        self.severity
    }

    fn message(&self) -> String {
        "Unneeded not in if".to_string()
    }

    fn label_message(&self) -> String {
        "unnecessary `!` operation".to_string()
    }

    fn diagnostic(&self) -> Option<Diagnostic> {
        self.diagnostic.clone()
    }
}

impl CodeS11IfNot {
    #[must_use]
    pub fn new(span: Range<usize>, processed: &Processed, severity: Severity) -> Self {
        Self {
            span,
            severity,
            diagnostic: None,
        }
        .generate_processed(processed)
    }

    fn generate_processed(mut self, processed: &Processed) -> Self {
        self.diagnostic = Diagnostic::from_code_processed(&self, self.span.clone(), processed);
        self
    }
}
