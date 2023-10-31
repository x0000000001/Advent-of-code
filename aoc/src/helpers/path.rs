/// Computes the project relative path for a day input.
///
/// Input files for `YYYY_DD` are expected to be stored
/// at `data/year_YYYY/dayDD/input.txt`.\
/// Test input files are expected to be stored the same way,
/// just named `test_input_ID.txt`.
///
/// # Arguments
///
/// * `name` - Expected format of a day name : YYYY_DD.,
///  e.g. `2015_01`, `2018_20`
/// * `option_id` - If this is an example input, specify its id.
pub fn get_path(name: &str, option_id: Option<usize>) -> String {
    assert_eq!(name.len(), 7);
    assert_eq!(name.get(4..5).unwrap(), "_");

    format!(
        "data/year_{}/day{}/{}.txt",
        name.get(0..4).unwrap(),
        name.get(5..7).unwrap(),
        match option_id {
            Some(id) => format!("test_input{}", id),
            None => "input".to_string(),
        }
    )
}
