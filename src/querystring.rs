//! querystring module include the struct QueryParams

use std::slice::Iter;
use std::ops::Deref;

type QueryParam<'a> = (&'a str, &'a str);

#[derive(Debug, Clone)]
pub struct QueryParams<'a> {
    inner: Vec<QueryParam<'a>>,
}

impl<'a> QueryParams<'a> {

    /// Produces a URL query string from the QueryParams struct.
    /// ```rust
    /// use urlqstring::QueryParams;
    /// fn main() {
    ///     let vec = vec![("id","1024"),("name","rust")];
    ///     let q = QueryParams::from(vec).stringify();
    ///     assert_eq!(q, "id=1024&name=rust&");
    /// }
    /// ```
    pub fn stringify(&self) -> String {
        self.iter().fold(String::new(), |acc, x| {
            if x.1.is_empty() {
                acc + &self.escape(&x.0) + "&"
            } else {
                acc + &self.escape(&x.0) + "=" + self.escape(&x.1).deref() + "&"
            }
        })
    }

    /// Produces a json style string from the QueryParams struct.
    ///```rust
    /// use urlqstring::QueryParams;
    /// fn main() {
    ///     let res = QueryParams::from(vec![("id","1024"), ("name","rust")]).json();
    ///     assert_eq!(res, r#"{"id":"1024","name":"rust"}"#)
    /// }
    ///```
    pub fn json(&self) -> String {
        let mut res: String = String::new();
        let len = self.inner.len();
        let mut idx = 0;
        for (k,v) in &self.inner {
            res.push_str(&format!(r#""{}":"{}""#, k, v));
            idx += 1;
            if idx < len {
                res.push_str(",")
            }
        }
        format!("{{{}}}", res)
    }

    /// Get a value of specific key of URL query params
    ///```rust
    /// use urlqstring::QueryParams;
    /// fn main() {
    ///     let res = QueryParams::from(vec![("id","1024"), ("name","rust")]).value("name");
    ///     assert_eq!(res, "rust");
    /// }
    /// ```
    pub fn value(&self, key: &str) -> Option<&str> {
        for query in &self.inner {
            if query.0 == key {
                return Some(query.1);
            }
        }
        None
    }

    /// Replace the specific key
    ///```rust
    /// use urlqstring::QueryParams;
    /// fn main() {
    ///     let res = QueryParams::from(vec![("id","1024"), ("name","rust")])
    ///         .replace_key("name", "language")
    ///         .stringify();
    ///     assert_eq!(res, "id=1024&language=rust&");
    /// }
    /// ```
    pub fn replace_key<'b: 'a>(&self, old_key: &str, new_key: &'b str) -> Self {
        let mut res: Vec<QueryParam<'a>> = Vec::new();
        self.iter().map(|(k, v)| {
            if *k == old_key {
                Some((new_key, *v))
            } else {
                Some((*k, *v))
            }
        }).for_each(|vec|{
            res.push(vec.unwrap())
        });

        QueryParams {
            inner: res
        }
    }

    /// Replace the specific value
    ///```rust
    /// use urlqstring::QueryParams;
    /// fn main() {
    ///     let res = QueryParams::from(vec![("id","1024"), ("name","rust")])
    ///         .replace_value("rust", "rust-lang")
    ///         .stringify();
    ///     assert_eq!(res, "id=1024&name=rust-lang&");
    /// }
    /// ```
    pub fn replace_value<'b: 'a>(&self, old_val: &str, new_val: &'b str) -> Self {
        let mut res: Vec<QueryParam<'a>> = Vec::new();
        self.iter().map(|(k, v)| {
            if *v == old_val {
                Some((*k, new_val))
            } else {
                Some((*k,*v))
            }
        }).for_each(|vec| {
            res.push(vec.unwrap())
        });

        QueryParams {
            inner: res
        }
    }

    /// Returns an iterator over the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = QueryParams::from(vec![("id","1024"), ("name","rust")])
    ///     .replace_value("rust", "rust-lang");
    /// let iterator = res.iter();
    /// assert_eq!(iterator.next(), Some(&("id","1024")));
    /// assert_eq!(iterator.next(), Some(&("name", "rust-lang")));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<QueryParam<'a>> {
        self.inner.iter()
    }

    fn from_str(s: &'a str) -> Self {
        let query_str = s.split('&').collect::<Vec<&str>>();
        let mut result: Vec<QueryParam<'a>> = Vec::new();
        for query in query_str {
            let key_value = query.splitn(2, '=').collect::<Vec<&str>>();
            if key_value.len() > 1 {
                result.push(( key_value[0], key_value[1] ) );
            } else {
                result.push(( key_value[0], "" ) );
            }
        }
        QueryParams {
            inner: result
        }
    }

    fn from_vec(vec: Vec<QueryParam<'a>>) -> Self {
        QueryParams {
            inner: vec
        }
    }

    fn escape(&self, str: &str) -> String {
        let mut enc = Vec::<u8>::new();
        for ch in str.as_bytes() {
            if self.keep_as(*ch) {
                enc.push(*ch);
            } else {
                enc.push(0x25);
                let n1 = (*ch >> 4) & 0xf;
                let n2 = *ch & 0xf;
                enc.push(self.to_dec_ascii(n1));
                enc.push(self.to_dec_ascii(n2));
            }
        }
        String::from_utf8(enc).unwrap()
    }

    fn keep_as(&self, n: u8) -> bool {
        return n.is_ascii_alphanumeric()
            || n == b'*'
            || n == b'-'
            || n == b'.'
            || n == b'_'
            || n == b'\''
            || n == b'~'
            || n == b'!'
            || n == b'('
            || n == b')';
    }

    fn to_dec_ascii(&self, n: u8) -> u8 {
        match n {
            0 => 48,
            1 => 49,
            2 => 50,
            3 => 51,
            4 => 52,
            5 => 53,
            6 => 54,
            7 => 55,
            8 => 56,
            9 => 57,
            10 => b'A',
            11 => b'B',
            12 => b'C',
            13 => b'D',
            14 => b'E',
            15 => b'F',
            _ => 127
        }
    }
}


impl<'a> From<&'a str> for QueryParams<'a> {
    fn from(s: &'a str) -> Self {
        QueryParams::from_str(s)
    }
}

impl<'a> From<&'a String> for QueryParams<'a> {
    fn from(s: &'a String) -> Self {
        QueryParams::from_str(s)
    }
}

impl<'a> From<Vec<QueryParam<'a>>> for QueryParams<'a> {
    fn from(vec: Vec<QueryParam<'a>>) -> Self {
        QueryParams::from_vec(vec)
    }
}

impl<'a> Iterator for QueryParams<'a> {
    type Item = QueryParam<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[cfg(test)]
mod test {
    use crate::querystring::{ QueryParams };

    #[test]
    fn it_works() {
        let res1 = QueryParams::from(vec![("id","1024"), ("idx","1,3,5")])
            .replace_key("idx", "ids")
            .json();
        assert_eq!(res1, r#"{"id":"1024","ids":"1,3,5"}"#);

        let res2 = QueryParams::from(vec![("id","1024"), ("name","rust")]).stringify();
        assert_eq!(res2, "id=1024&name=rust&");

        let res = QueryParams::from(vec![("id","1024"), ("name","rust")])
            .replace_value("rust", "rust-lang");
        let mut iterator = res.iter();
        assert_eq!(iterator.next(), Some(&("id","1024")));
        assert_eq!(iterator.next(), Some(&("name", "rust-lang")));
        assert_eq!(iterator.next(), None);
    }
}

