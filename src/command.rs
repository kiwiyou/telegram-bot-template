pub struct Args<'a> {
    inner: &'a str,
}

pub struct Command<'a> {
    pub label: &'a str,
    pub username: Option<&'a str>,
    rest: &'a str,
}

impl<'a> Iterator for Args<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.inner.trim_start();
        if line.is_empty() {
            None
        } else if let Some((prefix, suffix)) = line.split_once(char::is_whitespace) {
            self.inner = suffix;
            Some(prefix)
        } else {
            self.inner = "";
            Some(line)
        }
    }
}

impl Args<'_> {
    pub fn rest(&self) -> &str {
        self.inner.trim_start()
    }
}

impl<'a> Command<'a> {
    pub fn new(line: &'a str) -> Self {
        let (label_with_username, rest) =
            line.split_once(char::is_whitespace).unwrap_or((line, ""));
        let (label, username) = label_with_username
            .split_once('@')
            .map(|(label, username)| (label, Some(username)))
            .unwrap_or((label_with_username, None));
        Self {
            label,
            username,
            rest,
        }
    }

    pub fn args(&self) -> Args<'a> {
        Args { inner: self.rest }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command() {
        let tests = [
            ("/test", "/test", None, ""),
            ("/test@username", "/test", Some("username"), ""),
            (
                "/test@username a b c d",
                "/test",
                Some("username"),
                "a b c d",
            ),
            ("/test a b c d", "/test", None, "a b c d"),
        ];
        for (line, label, username, rest) in tests {
            let command = Command::new(line);
            assert_eq!(label, command.label);
            assert_eq!(username, command.username);
            assert_eq!(rest, command.rest);
        }
    }

    #[test]
    fn test_args() {
        let line = "a b c\nd e f";
        let args = Args { inner: line };
        let arg_vec: Vec<_> = args.collect();
        assert_eq!(vec!["a", "b", "c", "d", "e", "f"], arg_vec);
    }
}
