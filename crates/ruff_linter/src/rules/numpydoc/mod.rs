//! Rules from [numpydoc](https://pypi.org/project/numpydoc/).
pub(crate) mod rules;

// #[cfg(test)]
// mod tests {
//     #[test_case(Rule::WrongSectionOrder, Path::new("wrong_section_order.py"))]
//     fn rules(rule_code: Rule, path: &Path) -> Result<()> {
//         let snapshot = format!("{}_{}", rule_code.noqa_code(), path.to_string_lossy());
//         let diagnostics = test_path(
//             Path::new("pylint").join(path).as_path(),
//             &LinterSettings {
//                 pylint: pylint::settings::Settings {
//                     allow_dunder_method_names: FxHashSet::from_iter([
//                         "__special_custom_magic__".to_string()
//                     ]),
//                     ..pylint::settings::Settings::default()
//                 },
//                 ..LinterSettings::for_rule(rule_code)
//             },
//         )?;
//         assert_messages!(snapshot, diagnostics);
//         Ok(())
//     }
// }
