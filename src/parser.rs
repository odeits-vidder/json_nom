use nom::{IResult,digit,multispace,alphanumeric};
use nom::IResult::*;
use nom::Needed;

// Parser definition

use std::str;
use std::str::FromStr;


named!(factor<i64>,
  map_res!(
    map_res!(
      delimited!(opt!(multispace), digit, opt!(multispace)),
      str::from_utf8
    ),
    FromStr::from_str
  )

);

named!(true_value <i64>, chain!(opt!(multispace) ~ tag!("true") ~ opt!(multispace), || { return 0;}));
named!(false_value <i64>, chain!(opt!(multispace) ~ tag!("false") ~ opt!(multispace), || { return 0;}));

named!(boolean_value <i64>, alt!(true_value | false_value ));

named!(null_value <i64>, chain!(opt!(multispace) ~ tag!("null") ~ opt!(multispace), || { return 0;}));


named!(json_key <i64>, chain!(
   opt!(multispace)
  ~tag!("\"")
  ~many0!(
     alt!(alphanumeric | multispace)
     )
  ~tag!("\""),
  || {return 0;}));

named!(json_value <i64>,
  alt!(
    null_value |
    boolean_value |
    factor |
    json_key
  ));

named!(json_key_value_pair <i64>, chain!(json_key ~ opt!(multispace) ~tag!(":") ~ json_value, || {return 0;}));

named!(json_empty_obj <i64>, chain!(opt!(multispace) ~ tag!("{") ~opt!(multispace) ~ tag!("}"), || {return 0;}));

named!(json_obj <i64>,
alt!(
  json_empty_obj |
  chain!(
      opt!(multispace)
    ~ tag!("{")
    ~ opt!(multispace)
    ~ separated_list!(chain!(opt!(multispace) ~ tag!(",") ~ opt!(multispace), || {return 0}), json_key_value_pair)
    ~ opt!(multispace)
    ~ tag!("}")
    ~ opt!(multispace), || {return 0;})));

#[test]
fn json_obj_test() {
    assert_eq!(json_obj(&b"{"[..]),     IResult::Incomplete(Needed::Size(2)) );
    assert_eq!(json_obj(&b"  {"[..]),   IResult::Incomplete(Needed::Size(4)) );
    assert_eq!(json_obj(&b"{   "[..]),  IResult::Incomplete(Needed::Size(5)) );
    assert_eq!(json_obj(&b"{   }"[..]), IResult::Done(&b""[..], 0) );
    assert_eq!(json_obj(&b"{\"somekey\" : \"somevalue\"}"[..]), IResult::Done(&b""[..], 0) );
    assert_eq!(json_obj(&b"{  \"somekey\" : \"somevalue\"  }"[..]), IResult::Done(&b""[..], 0) );
    assert_eq!(json_obj(&b"{\"somekey\" : \"somevalue\", \"someotherkey\" : true}"[..]), IResult::Done(&b""[..], 0) );

}

#[test]
fn json_key_test() {
  assert_eq!(json_key(&b"\"\""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_key(&b"\" \""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_key(&b"\"somekey\""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_key(&b"\"some key\""[..]), IResult::Done(&b""[..], 0) );
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
}

