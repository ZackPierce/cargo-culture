mod builds_cleanly_without_warnings_or_errors;
mod cargo_metadata_readable;
mod has_continuous_integration_file;
mod has_contributing_file;
mod has_license_file;
mod has_readme_file;
mod has_rustfmt_file;
mod passes_multiple_tests;
mod uses_property_based_test_library;

pub use self::builds_cleanly_without_warnings_or_errors::BuildsCleanlyWithoutWarningsOrErrors;
pub use self::cargo_metadata_readable::CargoMetadataReadable;
pub use self::has_continuous_integration_file::HasContinuousIntegrationFile;
pub use self::has_contributing_file::HasContributingFile;
pub use self::has_license_file::HasLicenseFile;
pub use self::has_readme_file::HasReadmeFile;
pub use self::has_rustfmt_file::HasRustfmtFile;
pub use self::passes_multiple_tests::PassesMultipleTests;
pub use self::uses_property_based_test_library::UsesPropertyBasedTestLibrary;

use cargo_metadata::Metadata;
use std::fmt::Debug;
use std::io::Write;
use std::path::Path;

/// The result of a `Rule.evaluate` call.
///
/// Currently represented as a tri-valued flat enum rather than a `Result<bool,
/// Error>` to reduce the temptation to use a fancy error management scheme.
/// This is also to bring attention to 3rd party implementers that a
/// `RuleOutcome::Failure` is not an anomalous situation from the operational
/// standpoint of a `Rule` evaluation, and is distinct from a `RuleOutcome::
/// Undetermined` value.
#[derive(Clone, Debug, PartialEq)]
pub enum RuleOutcome {
    /// The Rule's `description` is definitely true for this project
    Success,
    /// The Rule's `description` definitely is not upheld for this project
    Failure,
    /// Something went wrong in the process of determining whether the Rule was
    /// upheld or not for this project. Let's admit that we don't know for
    /// sure one way or the other.
    Undetermined,
}

/// The core trait of this crate. A `Rule` describes an idiom or best-practice
/// for projects and provides a means of evaluating whether that rule of thumb
/// is being upheld.
pub trait Rule: Debug {
    /// The central tenet of this `Rule`. Serves as a **unique identifier** for
    /// Rule instances, as well as a human-readable summary of what this
    /// `Rule` means for a given project.
    fn description(&self) -> &str;

    /// Does the Rust project found at `cargo_manifest_path` uphold this
    /// `Rule`, as summarized in the `description`?
    ///
    ///
    /// Pre-parsed cargo `metadata` may be available, and supplemental
    /// human-readable `verbose` content may be written to the
    /// `print_output`.
    fn evaluate(
        &self,
        cargo_manifest_file_path: &Path,
        verbose: bool,
        metadata: &Option<Metadata>,
        print_output: &mut Write,
    ) -> RuleOutcome;
}

pub fn default_rules() -> Vec<Box<Rule>> {
    vec![
        Box::new(CargoMetadataReadable::default()),
        Box::new(HasContributingFile::default()),
        Box::new(HasLicenseFile::default()),
        Box::new(HasReadmeFile::default()),
        Box::new(HasRustfmtFile::default()),
        Box::new(BuildsCleanlyWithoutWarningsOrErrors::default()),
        Box::new(HasContinuousIntegrationFile::default()),
        Box::new(UsesPropertyBasedTestLibrary::default()),
        Box::new(PassesMultipleTests::default()),
    ]
}

#[cfg(test)]
mod tests {}

#[cfg(test)]
pub(crate) mod test_support {
    use super::{Rule, RuleOutcome};
    use cargo_metadata;
    use std::path::Path;

    pub struct VerbosityOutcomes {
        pub verbose: OutcomeCapture,
        pub not_verbose: OutcomeCapture,
    }

    pub struct OutcomeCapture {
        pub outcome: RuleOutcome,
        pub print_output: Vec<u8>,
    }

    pub fn execute_rule_against_project_dir_all_verbosities(
        project_dir: &Path,
        rule: &Rule,
    ) -> VerbosityOutcomes {
        VerbosityOutcomes {
            verbose: execute_rule_against_project_dir(project_dir, rule, true),
            not_verbose: execute_rule_against_project_dir(project_dir, rule, false),
        }
    }

    pub fn execute_rule_against_project_dir(
        project_dir: &Path,
        rule: &Rule,
        verbose: bool,
    ) -> OutcomeCapture {
        let cargo_manifest_file_path = project_dir.join("Cargo.toml");
        let metadata = cargo_metadata::metadata(Some(cargo_manifest_file_path.as_ref()))
            .map_err(|e| {
                println!("cargo_metadata error: {:?}", e);
                e
            })
            .ok();
        let mut print_output: Vec<u8> = Vec::new();
        let outcome = rule.evaluate(
            &cargo_manifest_file_path,
            verbose,
            &metadata,
            &mut print_output,
        );
        OutcomeCapture {
            outcome,
            print_output,
        }
    }
}