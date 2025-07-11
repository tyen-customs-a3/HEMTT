pub mod book;
pub mod build;
pub mod check;
pub mod dev;
pub mod launch;
pub mod localization;
pub mod new;
pub mod release;
pub mod script;
pub mod utils;
pub mod value;
pub mod wiki;

#[cfg(windows)]
pub mod photoshoot;

/// Adds modules that should apply to:
/// - hemtt check
/// - hemtt dev
/// - hemtt build
/// - hemtt release
pub fn global_modules(executor: &mut crate::executor::Executor) {
    executor.add_module(Box::<crate::modules::bom::BOMCheck>::default());
    executor.add_module(Box::<crate::modules::fnl::FineNewLineCheck>::default());
    executor.add_module(Box::<crate::modules::Hooks>::default());
    executor.add_module(Box::<crate::modules::Stringtables>::default());
    executor.add_module(Box::<crate::modules::SQFCompiler>::default());
}

#[derive(clap::Args)]
pub struct JustArgs {
    #[arg(long, action = clap::ArgAction::Append)]
    /// Only build the given addon
    pub(crate) just: Vec<String>,
}
