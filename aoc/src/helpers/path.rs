use itertools::Itertools;

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

pub fn get_input_paths(name: &str) -> Vec<String> {
    assert_eq!(name.len(), 7);
    assert_eq!(name.get(4..5).unwrap(), "_");

    let dir = format!(
        "data/year_{}/day{}/",
        name.get(0..4).unwrap(),
        name.get(5..7).unwrap()
    );

    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    paths.sort_by_key(|dir| dir.path());

    paths
        .into_iter()
        .filter_map(|file| {
            let name: String = String::from(file.file_name().to_str().unwrap());
            if name.get(0..4).unwrap() == "test" {
                Some(String::from(file.path().to_str().unwrap()))
            } else {
                None
            }
        })
        .collect_vec()
}
