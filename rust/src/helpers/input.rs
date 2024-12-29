use itertools::Itertools;

use super::AocDay;

pub struct Input {
    pub user: &'static str,
    pub test_id: Option<usize>,
}

impl Input {
    /// Computes the project relative path for a day input.
    ///
    /// Input files for are expected to be stored
    /// at `data/<user>/year_<YYYY>/day<DD>/input.txt`.\
    /// Test input files are expected to be stored the same way,
    /// just named `test_input<ID>.txt`.
    pub fn path(&self, day: &AocDay) -> String {
        format!(
            "data/{}/year_{}/day{:0>2}/{}.txt",
            self.user,
            day.year,
            day.day,
            match self.test_id {
                Some(id) => format!("test_input{}", id),
                None => "input".to_string(),
            }
        )
    }

    pub fn get_all(user: &'static str, day: &AocDay) -> Vec<Input> {
        let dir = format!("data/{}/year_{}/day{:0>2}/", user, day.year, day.day);

        std::fs::read_dir(dir)
            .unwrap()
            .map(|r| r.unwrap())
            .sorted_by_key(|dir| dir.path())
            .filter_map(|file| {
                let name: String = String::from(file.file_name().to_str().unwrap());

                if !name.contains("input") {
                    return None;
                }

                Some(if name.starts_with("test") {
                    let test_id: usize = name["test_input".len()..].parse().unwrap();

                    Input {
                        user,
                        test_id: Some(test_id),
                    }
                } else {
                    Input {
                        user,
                        test_id: None,
                    }
                })
            })
            .collect_vec()
    }
}
