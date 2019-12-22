
type QueryParam<'a> = (&'a str, &'a str);
type QueryParams<'a> = Vec<QueryParam<'a>>;

pub mod querystring {
    use crate::QueryParams;
    use std::collections::HashMap;

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

    #[test]
    fn it_works() {
        test_stringify();
        test_parse();
        test_json();
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
}
