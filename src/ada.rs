use regex::Regex;

pub struct PathX {
    path_regex: Regex,
}

static RE_MARKER: &str = r"r\*(.+?)\*";
static AUTO_GROUPS: [(&str, &str); 2] = [(r"\\\[\d\d\\\]", r"\[(\d\d)\]"), (r"\\\?", r"(\?)")];

impl PathX {
    pub fn new(path: &str) -> PathX {
        let marker_re = Regex::new(RE_MARKER).unwrap();
        let path_re = if marker_re.is_match(path) {
            panic!("Not implemented")
        } else {
            let mut path_re = regex::escape(path);
            for (auto_re, re) in AUTO_GROUPS.iter() {
                path_re = Regex::new(auto_re)
                    .unwrap()
                    .replace_all(&path_re, re.to_string())
                    .to_string();
            }
            path_re
        };
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

    pub fn get_path(&self, path_src: &str, groups: &Vec<String>) -> Result<String, &str> {
        if Some(groups.len() + 1) != self.path_regex.static_captures_len() {
            println!(
                "{} {}",
                groups.len(),
                self.path_regex.static_captures_len().unwrap()
            );
            return Err("Number of groups does not match");
        }
        let Some(caps) = self.path_regex.captures(&path_src) else {
            return Err("Path does not match");
        };
        let mut path = String::new();
        let mut last_match_end = 0;
        for (i, cap) in caps.iter().skip(1).enumerate() {
            let cur_match = cap.unwrap();
            path.push_str(&path_src[last_match_end..cur_match.start()]);
            path.push_str(groups[i].as_str());
            last_match_end = cur_match.end();
        }
        path.push_str(&path_src[last_match_end..]);
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ada_path() {
        let path = PathX::new("C:\\[00]\\[01]\\[02]");
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
            path.get_path(
                "C:\\[00]\\[01]\\[02]",
                &vec!["11".to_string(), "22".to_string(), "33".to_string()]
            )
            .unwrap(),
            "C:\\[11]\\[22]\\[33]"
        );
    }

    #[test]
    fn test_ada_path_with_parentheses() {
        let path = PathX::new(r"C:\1\1\(..)3");
        assert_eq!(
            path.get_path("C:\\1\\1\\023", &vec!["33".to_string()])
                .unwrap(),
            "C:\\1\\1\\333"
        );
    }
}
