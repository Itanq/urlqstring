use std::collections::HashMap;
use std::thread::sleep;

//type QueryParam<'a> = (&'a str, &'a str);
//type QueryParams<'a> = Vec<QueryParam<'a>>;

struct QueryParam<'a> {
    key: &'a str,
    value: &'a str,
}

#[feature(associated_type_defaults)]
impl<'a> Iterator for QueryParam<'a> {
    type Item = (&'a str, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.key, self.value))
    }
}

trait IntoQueryParam<'a> {
    type Output = QueryParam<'a>;
    fn iter(&self) -> Self::Output;
}

impl<'a, K: Into<&'a str>, V: Into<&'a str>> IntoQueryParam<'a> for HashMap<K,V> {
    type Output = Vec<QueryParam<'a>>;

    fn iter(&self) -> Self::Output {
        self.iter().map(|item| {
            QueryParam {
                key: item.0,
                value: item.1,
            }
        }).collect()
    }
}

pub mod querystring {
    use crate::{IntoQueryParam};
    use std::collections::HashMap;

    pub fn stringify<'a, T: IntoQueryParam<'a>>(query: T) -> String {
        query.iter().fold(String::new(), |acc, x| {
            acc + &escape(&x.0) + "=" + escape(&x.1).as_ref() + "&"
        })
    }

    fn escape(str: &str) -> String {
        let mut enc = String::from("");
        for ch in str.chars() {
            if ch.is_ascii_alphanumeric() {
                enc.push(ch);
            } else if ch == '+' {
                enc.push_str("%2B");
            } else if ch ==  '!' {
                enc.push_str("%21");
            } else if ch == '*' {
                enc.push_str("%2A");
            } else if ch == '"' {
                enc.push_str("%22");
            } else if ch == '\'' {
                enc.push_str("%27");
            } else if ch == '(' {
                enc.push_str("%28");
            } else if ch == ')' {
                enc.push_str("%29");
            } else if ch == ';' {
                enc.push_str("%3B");
            } else if ch == ':' {
                enc.push_str("%3A");
            } else if ch == '@' {
                enc.push_str("%40");
            } else if ch == '&' {
                enc.push_str("%26");
            } else if ch == '=' {
                enc.push_str("%3D");
            } else if ch == '$' {
                enc.push_str("%24");
            } else if ch == ',' {
                enc.push_str("%2C");
            } else if ch == '/' {
                enc.push_str("%2F");
            } else if ch == '?' {
                enc.push_str("%3F");
            } else if ch == '%' {
                enc.push_str("%25");
            } else if ch == '#' {
                enc.push_str("%23");
            } else if ch == '[' {
                enc.push_str("%5B");
            } else if ch == ']' {
                enc.push_str("%5D");
            }
        }
        enc
    }
}

#[cfg(test)]
mod tests {
    use super::querystring;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
//        let enc = querystring::stringify(vec![
//            ("params", "3EC4ojigTl0OgjyYtcd+97P7YKarculWrOxSgNO5clkQftvO1jOvS8aAhK6diyOb"),
//            ("encSecKey", "5ff8bdb3ed3dd15a26e9025e9abcff0d7a3764dafbc70e33859a892584c681f1aab314b8ad1f3418650ff851bdb0685fc5136a88e059c592da104bbeaba666fbe89eb405c7b66eab4db8ee3ab13a3f98cb41b2ac9981ed4e441ed8e1870524d001ee6ebc1c09f7a945677e5b56a3e964a224c3ee75ac43fbf513f6a8bf7472ee"),
//        ]);
        let mut params = HashMap::new();
        params.insert("encSecKey", "5ff8bdb3ed3dd15a26e9025e9abcff0d7a3764dafbc70e33859a892584c681f1aab314b8ad1f3418650ff851bdb0685fc5136a88e059c592da104bbeaba666fbe89eb405c7b66eab4db8ee3ab13a3f98cb41b2ac9981ed4e441ed8e1870524d001ee6ebc1c09f7a945677e5b56a3e964a224c3ee75ac43fbf513f6a8bf7472ee");
        params.insert("params", "3EC4ojigTl0OgjyYtcd+97P7YKarculWrOxSgNO5clkQftvO1jOvS8aAhK6diyOb" );
        let enc = querystring::stringify(params);
        let res1: String= String::from("params=3EC4ojigTl0OgjyYtcd%2B97P7YKarculWrOxSgNO5clkQftvO1jOvS8aAhK6diyOb&encSecKey=5ff8bdb3ed3dd15a26e9025e9abcff0d7a3764dafbc70e33859a892584c681f1aab314b8ad1f3418650ff851bdb0685fc5136a88e059c592da104bbeaba666fbe89eb405c7b66eab4db8ee3ab13a3f98cb41b2ac9981ed4e441ed8e1870524d001ee6ebc1c09f7a945677e5b56a3e964a224c3ee75ac43fbf513f6a8bf7472ee&");
        assert_eq!(enc, res1);
    }
}
