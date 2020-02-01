//! A general library for url query string
//!
//! This crate is a general purpose library for common function found when working
//! with the request's query parameters.
//!
//! This library aims to implement the [querystring](https://nodejs.org/docs/latest-v11.x/api/querystring.html) function in Nodejs
//!
//! ## Supported Types
//! For the library user, urlqstring will supports struct, map, and the string of query parameters style
//!
//! ### Usage
//! See the examples as below.
//! ```rust
//! use urlqstring::QueryParams;
//! fn main() {
//!     let enc1 = QueryParams::from(vec![
//!         ("params", "度"),
//!         ("enc", "=-)(&#+~·@…%^$:*-_."),
//!     ]).stringify();
//!     let res1: String= String::from("params=%E5%BA%A6&enc=%3D-)(%26%23%2B~%C2%B7%40%E2%80%A6%25%5E%24%3A*-_.&");
//!     assert_eq!(enc1, res1);
//! }
//! ```

mod querystring;

#[macro_use]
mod macros;

pub use querystring::QueryParams;

#[cfg(test)]
mod tests {
    use super::querystring;
    use super::querystring::{
        QueryParams
    };
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        test_stringify();
        test_parse();
        test_json();
        test_query_get_traits();
        test_object_macros();
    }

    fn test_stringify() {
        let enc1 = QueryParams::from(vec![
            ("params", "www. baidu. com/百度搜索"),
            ("encSecKey", "查询字=-)(*&^%$#@!~符串+~·！@￥%……%^%$:\"'*','-','.' and '_'"),
        ]).stringify();
        let res1: String= String::from("params=www.%20baidu.%20com%2F%E7%99%BE%E5%BA%A6%E6%90%9C%E7%B4%A2&encSecKey=%E6%9F%A5%E8%AF%A2%E5%AD%97%3D-)(*%26%5E%25%24%23%40!~%E7%AC%A6%E4%B8%B2%2B~%C2%B7%EF%BC%81%40%EF%BF%A5%25%E2%80%A6%E2%80%A6%25%5E%25%24%3A%22'*'%2C'-'%2C'.'%20and%20'_'&");
        assert_eq!(enc1, res1);

        let e = QueryParams::from(
            vec![
                ("params", "度"),
                ("enc", "=-)(&#+~·@…%^$:*-_."),
            ]
        ).stringify();
        println!("s={}", e);
    }

    fn test_parse() {
        let query_str = "params=www. baidu. com/百度搜索&encSecKey=查询字-=)(*^%$#@!~符串+~·！@￥%……%^%$:\"'*','-','.' and '_'&love=lu";
        let q = QueryParams::from(query_str);
        let res1 = q.value("params").unwrap();
        let res2 = q.value("encSecKey").unwrap();
        let res3 = q.value("love").unwrap();

        assert_eq!(res1, String::from("www. baidu. com/百度搜索"));
        assert_eq!(res2, String::from("查询字-=)(*^%$#@!~符串+~·！@￥%……%^%$:\"'*','-','.' and '_'"));
        assert_eq!(res3, String::from("lu"));
    }


    fn test_json() {
        let query_str1 = "idx=1024&name=lumi&family=ti=an&love";
        let res1 = QueryParams::from(query_str1).json();

        let query_str2 = "encSecKey=查询字=-)(*&^%$#@!~符串+~·！@￥%……%^%$:\"'*','-','.' and '_'";
        let res2 = QueryParams::from(query_str2).json();

        assert_eq!(res1, r#"{"idx":"1024","name":"lumi","family":"ti=an","love":""}"#);

        assert_eq!(res2, r#"{"encSecKey":"查询字=-)(*","^%$#@!~符串+~·！@￥%……%^%$:"'*','-','.' and '_'":""}"#);
    }

    fn test_query_get_traits() {
        let query_params = QueryParams::from("idx=1024&name=lumin&family=ti=an&love&ti=liang");
        let name = query_params.value("name").unwrap();
        let idx = query_params.value("idx").unwrap();
        let family = query_params.value("family").unwrap();
        let love = query_params.value("love").unwrap();
        let no = query_params.value("error").unwrap_or("error");

        assert_eq!(idx, "1024");
        assert_eq!(name, "lumin");
        assert_eq!(family, "ti=an");
        assert_eq!(love, "");
        assert_eq!(no, "error");

        let mut query_params = query_params;

        let query2 = query_params.replace_key("ti", "tian").stringify();
        let query3 = query_params.replace_key("love", "ai").stringify();

        assert_eq!(query2, String::from("idx=1024&name=lumin&family=ti%3Dan&love&tian=liang&"));
        assert_eq!(query3, String::from("idx=1024&name=lumin&family=ti%3Dan&ai&ti=liang&"));
    }

    fn test_object_macros()
    {
        let value = proto_object!({
            "params": "www. baidu. com/百度搜索",
            "encSecKey": "查询字=-)(*&^%$#@!~符串+~·！@￥%……%^%$:'*','-','.' and '_'"
        }).stringify();

        let res1: String= String::from("params=www.%20baidu.%20com%2F%E7%99%BE%E5%BA%A6%E6%90%9C%E7%B4%A2&encSecKey=%E6%9F%A5%E8%AF%A2%E5%AD%97%3D-)(*%26%5E%25%24%23%40!~%E7%AC%A6%E4%B8%B2%2B~%C2%B7%EF%BC%81%40%EF%BF%A5%25%E2%80%A6%E2%80%A6%25%5E%25%24%3A'*'%2C'-'%2C'.'%20and%20'_'&");
        assert_eq!(value, res1);
    }
}
