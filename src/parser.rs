use nom::{IResult,digit,multispace,alphanumeric};
use nom::IResult::*;
use nom::Needed;

// Parser definition

use std::str;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq)]
struct Value{
    b: Option<bool>,
    num: Option<i64>,
    s: Option<String>,
    o: Option<(String, Box<Value>)>,
    a: Option<Vec<Value>>
}

named!(factor<Value>,
  map_res!(
    map_res!(
      map_res!(delimited!(opt!(multispace), digit, opt!(multispace)), str::from_utf8),
      FromStr::from_str),
    |v| Ok::<Value, ()>(Value{num : Some(v), ..Default::default()})
  )
);
named!(quoted_str <Value>, chain!(
   opt!(multispace)
  ~ q_string: map_res!(map_res!(delimited!(tag!("\""), alt!(alphanumeric | multispace), tag!("\"")), str::from_utf8), String::from_str)
  ~tag!("\""),
  || Value{ s : Some(q_string), ..Default::default()}));

named!(true_value <Value>, chain!(opt!(multispace) ~ tag!("true") ~ opt!(multispace), || Value {b : Some(true), ..Default::default()}));
named!(false_value <Value>, chain!(opt!(multispace) ~ tag!("false") ~ opt!(multispace), || Value {b : Some(false), ..Default::default()}));

named!(boolean_value <Value>, alt!(true_value | false_value ));

named!(null_value <Value>, chain!(opt!(multispace) ~ tag!("null") ~ opt!(multispace), || Default::default() ));

named!(json_value <Value>,
  alt!(
    json_obj |
    null_value |
    boolean_value |
    factor |
    quoted_str
  ));

named!(json_key_value_pair <Value>, 
       map_res!(
         map_opt!(
          chain!(
          key: quoted_str ~ 
          opt!(multispace) ~ tag!(":") ~ 
          val: json_value, 
          || Some((key.s.clone(), Box::new(val)))),
          |TPL:Option<(Option<String>, Box<Value>)>| {
            match TPL {
              None => None,
              Some((K, V)) => {
                match K {
                  Some(K) => Some((K, V)),
                  None => None,
                }
              },
            }
            }), 
         |V| Ok::<Value, ()>(Value{o: Some(V), ..Default::default()})));

named!(json_empty_obj <Value>, chain!(opt!(multispace) ~ tag!("{") ~opt!(multispace) ~ tag!("}"), || Default::default()));

named!(json_obj <Value>,
alt!(
  json_empty_obj |
  chain!(
      opt!(multispace)
    ~ tag!("{")
    ~ opt!(multispace)
    ~ key_pairs: separated_list!(tag!(",") , json_key_value_pair)
    ~ opt!(multispace)
    ~ tag!("}")
    ~ opt!(multispace), || Value{a: Some(key_pairs), ..Default::default()})));

#[test]
fn json_obj_test() {
    assert_eq!(json_obj(&b"{"[..]),     IResult::Incomplete(Needed::Size(2)) );
    assert_eq!(json_obj(&b"  {"[..]),   IResult::Incomplete(Needed::Size(4)) );
    assert_eq!(json_obj(&b"{   "[..]),  IResult::Incomplete(Needed::Size(5)) );
    assert_eq!(json_obj(&b"{   }"[..]), IResult::Done(&b""[..], Default::default()) );
   // assert_eq!(json_obj(&b"{\"somekey\" : \"somevalue\"}"[..]), IResult::Done(&b""[..], 0) );
   // assert_eq!(json_obj(&b"{  \"somekey\" : \"somevalue\"  }"[..]), IResult::Done(&b""[..], 0) );
   // assert_eq!(json_obj(&b"{\"somekey\" : \"somevalue\", \"someotherkey\" : true}"[..]), IResult::Done(&b""[..], 0) );

}
/*
#[test]
fn quoted_str_test() {
  assert_eq!(quoted_str(&b"\"\""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(quoted_str(&b"\" \""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(quoted_str(&b"\"somekey\""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(quoted_str(&b"\"some key\""[..]), IResult::Done(&b""[..], 0) );
}

#[test]
fn json_value_test() {
  assert_eq!(json_value(&b"12345"[..]), IResult::Done(&b""[..], 12345) );
  assert_eq!(json_value(&b"true"[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_value(&b"false"[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_value(&b"\"some text here\""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_value(&b"null"[..]), IResult::Done(&b""[..], 0) );
}

#[test]
fn json_key_value_pair_test() {
  assert_eq!(json_key_value_pair(&b"\"some key\" : \"some value\""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_key_value_pair(&b"\"some key\" : null"[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_key_value_pair(&b"\"some key\" : 12345"[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_key_value_pair(&b"\"some key\" : true"[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_key_value_pair(&b"\"some key\" : { }"[..]), IResult::Done(&b""[..], 0) );
}

*/