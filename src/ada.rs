use regex::Regex;

pub struct PathX {
    path_regex: Regex,
}

impl PathX {
    pub fn new(path: String) -> PathX {
        let mut path_re = regex::escape(&path);
        if path.contains("(") {
            // path
        } else {
            path_re = Regex::new(r"\\\[\d\d\\\]")
                .unwrap()
                .replace_all(&path_re, r"\[(\d\d)\]")
                .to_string();
        }
        PathX {
            path_regex: Regex::new(&path_re).unwrap(),
        }
    }

    pub fn get_groups(&self, path: &str) -> Option<Vec<String>> {
        match self.path_regex.captures(path) {
            Some(caps) => Some(
                caps.iter()
                    .skip(1)
                    .map(|c| c.unwrap().as_str().to_string())
                    .collect(),
            ),
            None => None,
        }
    }

    pub fn get_path(&self, path_src: &str, groups: Vec<&str>) -> Option<String> {
        if Some(groups.len() + 1) != self.path_regex.static_captures_len() {
            return None;
        }
        let Some(caps) = self.path_regex.captures(&path_src) else {
            return None;
        };
        let mut path = String::new();
        let mut last_match_end = 0;
        for (_, cap) in caps.iter().skip(1).enumerate() {
            let cur_match = cap.unwrap();
            path.push_str(&path_src[last_match_end..cur_match.start()]);
            path.push_str(groups[cur_match.as_str().parse::<usize>().unwrap()]);
            last_match_end = cur_match.end();
        }
        path.push_str(&path_src[last_match_end..]);
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ada_path() {
        let path = PathX::new("C:\\[00]\\[01]\\[02]".to_string());
        assert!(path.path_regex.is_match("C:\\[00]\\[01]\\[02]"));
        assert_eq!(
            path.path_regex
                .captures("C:\\[00]\\[01]\\[02]")
                .unwrap()
                .get(1)
                .unwrap()
                .as_str(),
            "00"
        );
        assert_eq!(
            path.get_groups(
                path.path_regex
                    .captures("C:\\[00]\\[01]\\[02]")
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
            )
            .unwrap(),
            vec!["00", "01", "02"]
        );
        assert_eq!(
            path.get_path("C:\\[00]\\[01]\\[02]", vec!["11", "22", "33"])
                .unwrap(),
            "C:\\[11]\\[22]\\[33]"
        );
    }
}
