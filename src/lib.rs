
type QueryParam<'a> = (&'a str, &'a str);
type QueryParams<'a> = Vec<QueryParam<'a>>;

pub mod querystring {
    use crate::QueryParams;
    use std::collections::HashMap;

    pub trait QueryParamGet {
        fn get_value(&self, key: &str) -> Option<&str>;
        fn replace_key(&self, old_key: &str, new_key: &str) -> String;
        fn replace_keys(&self, old_key: Vec<&str>, new_key: Vec<&str>) -> String;
        fn replace_value(&self, old_value: &str, new_value: &str) -> String;
        fn replace_values(&self, old_value: Vec<&str>, new_value: Vec<&str>) -> String;
    }

    impl QueryParamGet for &str {
        fn get_value(&self, key: &str) -> Option<&str> {
            let parameters: Vec<&str> = self.split('&').collect();
            for value in parameters {
                if value.contains('=') &&  value.contains(key){
                    return Some(value.splitn(2, "=").collect::<Vec<&str>>()[1]);
                } else if value.contains(key) {
                    return Some("");
                } else {
                    continue;
                }
            }
            None
        }

        fn replace_key(&self, old_key: &str, new_key: &str) -> String {
            let k1 = "&".to_owned() + old_key;
            if self.contains(&k1) {
                let k2 = "&".to_owned() + new_key;
                self.replacen(&k1, &k2, 1)
            } else if self.starts_with(old_key){
                self.replacen(old_key, new_key, 1)
            } else {
                String::from("")
            }
        }

        fn replace_keys(&self, old_keys: Vec<&str>, new_keys: Vec<&str>) -> String {
            assert_eq!(old_keys.len(), new_keys.len());
            let mut res : String = String::from(*self);
            for ( idx, old_key ) in old_keys.iter().enumerate() {
                let k1 = "&".to_owned() + old_key;
                if res.contains(&k1) {
                    let k2 = "&".to_owned() + new_keys[idx];
                    res = res.replacen(&k1, &k2, 1);
                } else if res.starts_with(old_key) {
                    res = res.replacen(old_key, new_keys[idx], 1);
                }
            }
            res
        }

        fn replace_value(&self, old_value: &str, new_value: &str) -> String {
            let k1 = "=".to_owned() + old_value + "&";
            if self.contains(&k1) {
                let k2 = "=".to_owned() + new_value + "&";
                self.replacen(&k1, &k2, 1)
            } else if self.ends_with(old_value) {
                self.replacen(old_value, new_value, 1)
            } else {
                String::from("")
            }
        }

        fn replace_values(&self, old_values: Vec<&str>, new_values: Vec<&str>) -> String {
            assert_eq!(old_values.len(), new_values.len());
            let mut res : String = String::from(*self);
            for (idx, old_value) in old_values.iter().enumerate() {
                let k1 = "=".to_owned() + old_value + "&";
                if res.contains(&k1) {
                    let k2 = "=".to_owned() + new_values[idx] + "&";
                    res = res.replacen(&k1, &k2, 1);
                } else if res.ends_with(old_value) {
                    res = res.replacen(old_value, new_values[idx], 1);
                }
            }
            res
        }

    }

    pub fn stringify(query: QueryParams) -> String {
        query.iter().fold(String::new(), |acc, x| {
            acc + &escape(&x.0) + "=" + escape(&x.1).as_ref() + "&"
        })
    }

    pub fn json(query: &str) -> String {
        let parameters: Vec<&str> = query.split('&').collect();
        let mut res: String = String::new();
        for (idx, kvs) in parameters.iter().enumerate() {
            let kv: Vec<&str>;
            if kvs.contains('=') {
                kv = kvs.splitn(2, "=").collect();
            } else {
                kv = vec![kvs, ""];
            }
            if idx == parameters.len() - 1 {
                res.push_str(&format!(r#""{}":"{}""#, kv[0], kv[1]));
            }
            else {
                res.push_str(&format!(r#""{}":"{}","#, kv[0], kv[1]));
            }
        }
        format!("{{{}}}", res)
    }

    pub fn parse(query: &str) -> HashMap<&str,&str> {
        let parameters: Vec<&str> = query.split('&').collect();
        let mut res: HashMap<&str,&str> = HashMap::new();
        for kvs in parameters {
            let kv: Vec<&str>;
            if kvs.contains('=') {
                kv = kvs.splitn(2, "=").collect();
            } else {
                kv = vec![kvs, ""];
            }
            res.insert(kv[0], kv[1]);
        }
        res
    }

    fn escape(str: &str) -> String {
        let mut enc = Vec::<u8>::new();
        for ch in str.as_bytes() {
            if keep_as(*ch) {
                enc.push(*ch);
            } else {
                enc.push(0x25);
                let n1 = (*ch >> 4) & 0xf;
                let n2 = *ch & 0xf;
                enc.push(to_dec_ascii(n1));
                enc.push(to_dec_ascii(n2));
            }
        }
        String::from_utf8(enc).unwrap()
    }

    fn keep_as(n: u8) -> bool {
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

    fn to_dec_ascii(n: u8) -> u8 {
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

#[cfg(test)]
mod tests {
    use super::querystring;
    use std::collections::HashMap;
    use crate::querystring::QueryParamGet;

    #[test]
    fn it_works() {
        test_stringify();
        test_parse();
        test_json();
        test_query_get_traits();
    }

    fn test_stringify() {
        let enc1 = querystring::stringify(vec![
            ("params", "www. baidu. com/百度搜索"),
            ("encSecKey", "查询字=-)(*&^%$#@!~符串+~·！@￥%……%^%$:\"'*','-','.' and '_'"),
        ]);
        let res1: String= String::from("params=www.%20baidu.%20com%2F%E7%99%BE%E5%BA%A6%E6%90%9C%E7%B4%A2&encSecKey=%E6%9F%A5%E8%AF%A2%E5%AD%97%3D-)(*%26%5E%25%24%23%40!~%E7%AC%A6%E4%B8%B2%2B~%C2%B7%EF%BC%81%40%EF%BF%A5%25%E2%80%A6%E2%80%A6%25%5E%25%24%3A%22'*'%2C'-'%2C'.'%20and%20'_'&");
        assert_eq!(enc1, res1);
    }

    fn test_parse() {
        let query_str = "params=www. baidu. com/百度搜索&encSecKey=查询字-=)(*^%$#@!~符串+~·！@￥%……%^%$:\"'*','-','.' and '_'&love=lu";
        let res = querystring::parse(query_str);

        assert_eq!(res.get("params").unwrap(), &"www. baidu. com/百度搜索");
        assert_eq!(res.get("encSecKey").unwrap(), &"查询字-=)(*^%$#@!~符串+~·！@￥%……%^%$:\"'*','-','.' and '_'");
        assert_eq!(res.get("love").unwrap(), &"lu");
    }


    fn test_json() {
        let query_str1 = "idx=1024&name=lumi&family=ti=an&love";
        let res1 = querystring::json(query_str1);

        let query_str2 = "encSecKey=查询字=-)(*&^%$#@!~符串+~·！@￥%……%^%$:\"'*','-','.' and '_'";
        let res2 = querystring::json(query_str2);

        assert_eq!(res1, r#"{"idx":"1024","name":"lumi","family":"ti=an","love":""}"#);

        assert_eq!(res2, r#"{"encSecKey":"查询字=-)(*","^%$#@!~符串+~·！@￥%……%^%$:"'*','-','.' and '_'":""}"#);
    }

    fn test_query_get_traits() {
        let query_str1 = "idx=1024&name=lumin&family=ti=an&love&ti=liang";
        let name = query_str1.get_value("name").unwrap();
        let idx = query_str1.get_value("idx").unwrap();
        let family = query_str1.get_value("family").unwrap();
        let love = query_str1.get_value("love").unwrap();
        let no = query_str1.get_value("error").unwrap_or("error");

        assert_eq!(idx, "1024");
        assert_eq!(name, "lumin");
        assert_eq!(family, "ti=an");
        assert_eq!(love, "");
        assert_eq!(no, "error");

        let query2 = query_str1.replace_key("ti", "tian");
        let query3 = query_str1.replace_key("love", "ai");
        let query4 = query_str1.replace_value("ti=an", "[ti=an]");
        let query5 = query_str1.replace_value("liang", "[liang]");

        assert_eq!(query2, "idx=1024&name=lumin&family=ti=an&love&tian=liang");
        assert_eq!(query3, "idx=1024&name=lumin&family=ti=an&ai&ti=liang");
        assert_eq!(query4, "idx=1024&name=lumin&family=[ti=an]&love&ti=liang");
        assert_eq!(query5, "idx=1024&name=lumin&family=ti=an&love&ti=[liang]");

        let q1 = "uid=1024&type=1086&/update/desc=desc_update_value";
        let r1 = q1.replace_keys(vec!["uid", "/update/desc"], vec!["userId", "update_desc"]);
        let r2 = q1.replace_values(vec!["1086", "desc_update_value"], vec!["10086", "update_desc_value"]);

        assert_eq!(r1, "userId=1024&type=1086&update_desc=desc_update_value");
        assert_eq!(r2, "uid=1024&type=10086&/update/desc=update_desc_value")


    }
}
