use ruff_diagnostics::Diagnostic;
use ruff_diagnostics::Violation;
use ruff_macros::derive_message_formats;
use ruff_macros::ViolationMetadata;

use crate::checkers::ast::Checker;
use crate::codes::Rule;
use crate::docstrings::Docstring;
use ruff_text_size::Ranged;

/// ## What it does
/// Checks for order of sections in docstrings.
///
/// ## Why is this bad?
/// The numpydoc format lists a specific order for docstring sections
/// (though not all sections need to be present):
/// 1. Short summary
/// 2. Deprecation warning
/// 3. Extended Summary
/// 4. Parameters
/// 5. Returns
/// 6. Yields
/// 7. Receives
/// 8. Other Parameters
/// 9. Raises
/// 10. Warns
/// 11. Warnings
/// 12. See Also
/// 13. Notes
/// 14. References
/// 15. Examples
///
/// ## Example
/// ```python
/// def foo():
///     """Do something.
///
///     See also
///     --------
///     Another function
///
///     Returns
///     -------
///     Nothing
///     """
///     pass
/// ```
///
/// Write instead:
/// ```python
/// def foo():
///     """Do something.
///
///     Returns
///     -------
///     Nothing
///
///     See also
///     --------
///     Another function
///     """
///     pass
/// ```
///
/// ## References
/// - [numpydoc docstring standard](https://numpydoc.readthedocs.io/en/latest/format.html#docstring-standard)

#[derive(ViolationMetadata)]
pub(crate) struct WrongSectionOrder;

impl Violation for WrongSectionOrder {
    #[derive_message_formats]
    fn message(&self) -> String {
        let correct_sections = vec![
            "Short summary",
            "Deprecation warning",
            "Extended Summary",
            "Parameters",
            "Returns",
            "Yields",
            "Receives",
            "Other Parameters",
            "Raises",
            "Warns",
            "Warnings",
            "See Also",
            "Notes",
            "References",
            "Examples",
        ];

        format!(
            "Sections are in the wrong order. Correct order is: {}",
            correct_sections.join(", ")
        )
    }
}

/// GL07
pub(crate) fn wrong_section_order(checker: &Checker, docstring: &Docstring) -> bool {
    if checker.enabled(Rule::WrongSectionOrder) {
        checker.report_diagnostic(Diagnostic::new(WrongSectionOrder, docstring.range()));
    }
    false
}
