// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//  ucd-generate general-category tmp/ucd-11.0.0/ --exclude unassigned --enum --fst-dir benches/tables/fst
//
// ucd-generate is available on crates.io.

pub const GENERAL_CATEGORY_ENUM: &'static [&'static str] = &[
  "Close_Punctuation", "Connector_Punctuation", "Control", "Currency_Symbol",
  "Dash_Punctuation", "Decimal_Number", "Enclosing_Mark", "Final_Punctuation",
  "Format", "Initial_Punctuation", "Letter_Number", "Line_Separator",
  "Lowercase_Letter", "Math_Symbol", "Modifier_Letter", "Modifier_Symbol",
  "Nonspacing_Mark", "Open_Punctuation", "Other_Letter", "Other_Number",
  "Other_Punctuation", "Other_Symbol", "Paragraph_Separator", "Private_Use",
  "Space_Separator", "Spacing_Mark", "Surrogate", "Titlecase_Letter",
  "Uppercase_Letter",
];

lazy_static! {
  pub static ref GENERAL_CATEGORY: ::fst::Map = 
    ::fst::Map::from(::fst::raw::Fst::from_static_slice(
      include_bytes!("general_category.fst")).unwrap());
}
