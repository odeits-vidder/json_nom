use nom::{IResult,digit,multispace,alphanumeric};
use nom::IResult::*;
use nom::Needed;

// Parser definition

use std::str;
use std::str::FromStr;

named!(parens<i64>, delimited!(
    delimited!(opt!(multispace), tag!("("), opt!(multispace)),
    expr,
    delimited!(opt!(multispace), tag!(")"), opt!(multispace))
  )
);

// We transform an integer string into a i64
// we look for a digit suite, and try to convert it.
// if either str::from_utf8 or FromStr::from_str fail,
// the parser will fail
named!(factor<i64>,
  alt!(
    map_res!(
      map_res!(
        delimited!(opt!(multispace), digit, opt!(multispace)),
        str::from_utf8
      ),
      FromStr::from_str
    )
  | parens
  )
);

// we define acc as mutable to update its value whenever a new term is found
named!(term <i64>,
  chain!(
    mut acc: factor  ~
             many0!(
               alt!(
                 tap!(mul: preceded!(tag!("*"), factor) => acc = acc * mul) |
                 tap!(div: preceded!(tag!("/"), factor) => acc = acc / div)
               )
             ),
    || { return acc }
  )
);

named!(expr <i64>,
  chain!(
    mut acc: term  ~
             many0!(
               alt!(
                 tap!(add: preceded!(tag!("+"), term) => acc = acc + add) |
                 tap!(sub: preceded!(tag!("-"), term) => acc = acc - sub)
               )
             ),
    || { return acc }
  )
);

#[test]
fn main() {
  assert_eq!(expr(&b" 1 +  2 "[..]),             IResult::Done(&b""[..], 3));
  assert_eq!(expr(&b" 12 + 6 - 4+  3"[..]),      IResult::Done(&b""[..], 17));
  assert_eq!(expr(&b" 1 + 2*3 + 4"[..]),         IResult::Done(&b""[..], 11));

  assert_eq!(expr(&b" (  2 )"[..]),              IResult::Done(&b""[..], 2));
  assert_eq!(expr(&b" 2* (  3 + 4 ) "[..]),      IResult::Done(&b""[..], 14));
  assert_eq!(expr(&b"  2*2 / ( 5 - 1) + 3"[..]), IResult::Done(&b""[..], 4));
}

named!(true_value <i64>, chain!(opt!(multispace) ~ tag!("true") ~ opt!(multispace), || { return 0;}));
named!(false_value <i64>, chain!(opt!(multispace) ~ tag!("false") ~ opt!(multispace), || { return 0;}));

named!(boolean_value <i64>, alt!(true_value | false_value ));

named!(null_value <i64>, chain!(opt!(multispace) ~ tag!("null") ~ opt!(multispace), || { return 0;}));

named!(json_obj <i64>, chain!(opt!(multispace) ~ tag!("{") ~ opt!(multispace) ~ tag!("}"), || {return 0;}));

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

#[test]
fn json_obj_test() {
    assert_eq!(json_obj(&b"{"[..]),     IResult::Incomplete(Needed::Size(2)) );
    assert_eq!(json_obj(&b"  {"[..]),   IResult::Incomplete(Needed::Size(4)) );
    assert_eq!(json_obj(&b"{   "[..]),  IResult::Incomplete(Needed::Size(5)) );
    assert_eq!(json_obj(&b"{   }"[..]), IResult::Done(&b""[..], 0) );
}

#[test]
fn json_key_test() {
  assert_eq!(json_key(&b"\"\""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_key(&b"\" \""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_key(&b"\"somekey\""[..]), IResult::Done(&b""[..], 0) );
}

#[test]
fn json_value_test() {
  assert_eq!(json_value(&b"12345"[..]), IResult::Done(&b""[..], 12345) );
  assert_eq!(json_value(&b"true"[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_value(&b"false"[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_value(&b"\"some text here\""[..]), IResult::Done(&b""[..], 0) );
  assert_eq!(json_value(&b"null"[..]), IResult::Done(&b""[..], 0) );

}

