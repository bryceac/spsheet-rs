//! Excel Base Format
use std::borrow::Cow;
use super::nom::{ branch::alt, 
    bytes::complete::{
        tag,
        tag_no_case,
        take,
    take_until,
    take_while1},
    combinator::{ complete, 
        map, 
        value },
        multi::{ many0, many_m_n },
    IResult,
sequence::{ delimited } };
use chrono::prelude::*;
use nom::Parser;
use super::era_jp;

#[derive(Debug, Clone, PartialEq)]
pub struct Format {
    content: String,
}

impl Format {
    pub fn new<'a, S>(content: S) -> Format 
         where S: Into<Cow<'a, str>>
    {
        Format {
            content: content.into().into_owned(),
        }
    }

    fn is_ymdhm<'a, S>(value: S) -> bool 
        where S: Into<Cow<'a, str>>
    {
        match ymdhms(value.into().into_owned().as_str()) {
            IResult::Ok((_, _)) => true,
            _ => false
        }
    }

    fn is_numeric_ary<'a, S>(value: S) -> bool 
    where S: Into<Cow<'a, str>>
    {
        match numeric_ary(value.into().into_owned().as_str()) {
            IResult::Ok((_, _)) => true,
            _ => false
        }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_date_formats(&self) -> Option<Vec<&str>> {
        match ymdhms(self.content.as_str()) {
            IResult::Ok((_, output)) => {
                let mut result = vec![];
                for i in output {
                    for j in i {
                        result.push(j);
                    }
                }
                Some(result)
            },
            _ => None
        }
    }

    pub fn get_formated_date(&self, dt: &DateTime<Utc>) -> Option<String> {
        match ymdhms(self.content.as_str()) {
            IResult::Ok((_, output)) => {
                let mut format = String::from("");
                let era_year = era_jp::get_year(dt);
                for item_ary in output {
                    for item in item_ary {
                        match item {
                            "{{era1}}" => {
                                format = format!("{}{}", format, era_year);
                            },
                            "{{era2}}" => {
                                format = format!("{}{:>02}", format, era_year);
                            },
                            "{{gengou1}}" => {
                                format.push_str(era_jp::get_abbreviation_name(&dt));
                            },
                            "{{gengou2}}" => {
                                format.push_str(era_jp::get_short_name(&dt));
                            },
                            "{{gengou3}}" => {
                                format.push_str(era_jp::get_name(&dt));
                            },
                            _ => {
                                format.push_str(item);
                            }
                        }
                    }
                }
                Some(dt.format(format.as_str()).to_string())
            },
            _ => None
        }
    }
}

fn year4(input: &str) -> IResult<&str, &str> {
    value("%Y", tag_no_case("yyyy")).parse(input)
}

fn year2(input: &str) -> IResult<&str, &str> {
    value("%Y", tag_no_case("yy")).parse(input)
}

fn era1(input: &str) -> IResult<&str, &str> {
    value("{{era1}}", tag_no_case("e")).parse(input)
}

fn era2(input: &str) -> IResult<&str, &str> {
    value("{{era2}}", tag_no_case("ee")).parse(input)
}

fn gengou1(input: &str) -> IResult<&str, &str> {
    value("{{gengou1}}", tag_no_case("g")).parse(input)
}

fn gengou2(input: &str) -> IResult<&str, &str> {
    value("{{gengou2}}", tag_no_case("gg")).parse(input)
}

fn gengou3(input: &str) -> IResult<&str, &str> {
    value("{{gengou3}}", tag_no_case("ggg")).parse(input)
}

fn year(input: &str) -> IResult<&str, &str> {
    alt((
        complete(year4),
        complete(year2),
        complete(era2),
        complete(era1),
        complete(gengou3),
        complete(gengou2),
        complete(gengou1)
    )).parse(input)
}

fn month1(input: &str) -> IResult<&str, &str> {
    value("%-m", tag_no_case("m")).parse(input)
}

fn month2(input: &str) -> IResult<&str, &str> {
    value("%m", tag_no_case("mm")).parse(input)
}

fn month3(input: &str) -> IResult<&str, &str> {
    value("%b", tag_no_case("mmm")).parse(input)
}

fn month4(input: &str) -> IResult<&str, &str> {
    value("%B", tag_no_case("mmmm")).parse(input)
}

fn month5(input: &str) -> IResult<&str, &str> {
    value("{{month5}}", tag_no_case("mmmmm")).parse(input)
}

fn month(input: &str) -> IResult<&str, &str> {
    alt((
        complete(month5),
        complete(month4),
        complete(month3),
        complete(month2),
        complete(month1)
    )).parse(input)
}

fn day1(input: &str) -> IResult<&str, &str> {
    value("%-d", tag_no_case("d")).parse(input)
}

fn day2(input: &str) -> IResult<&str, &str> {
    value("%d", tag_no_case("dd")).parse(input)
}

fn dow3(input: &str) -> IResult<&str, &str> {
    value("%a", tag_no_case("ddd")).parse(input)
}

fn dow4(input: &str) -> IResult<&str, &str> {
    value("%A", tag_no_case("dddd")).parse(input)
}

fn youbi3(input: &str) -> IResult<&str, &str> {
    value("{{youbi3}}", tag_no_case("aaa")).parse(input)
}

fn youbi4(input: &str) -> IResult<&str, &str> {
    value("{{youbi4}}", tag_no_case("aaaa")).parse(input)
}

fn day(input: &str) -> IResult<&str, &str> {
    alt((
        complete(youbi4),
        complete(youbi3),
        complete(dow4),
        complete(dow3),
        complete(day2),
        complete(day1)
    )).parse(input)
}

fn hour1(input: &str) -> IResult<&str, &str> {
    value("%-H", tag_no_case("h")).parse(input)
}

fn hour2(input: &str) -> IResult<&str, &str> {
    value("%H", tag_no_case("hh")).parse(input)
}

fn hour(input: &str) -> IResult<&str, &str> {
    alt((
        complete(hour2),
        complete(hour1)
    )).parse(input)
}

fn minute1(input: &str) -> IResult<&str, &str> {
    value("%-M", tag_no_case("m")).parse(input)
}

fn minute2(input: &str) -> IResult<&str, &str> {
    value("%M", tag_no_case("mm")).parse(input)
}

fn minute(input: &str) -> IResult<&str, &str> {
    alt((
        complete(minute2),
        complete(minute1)
    )).parse(input)
}

fn second1(input: &str) -> IResult<&str, &str> {
    value("%-S", tag_no_case("s")).parse(input)
}

fn second2(input: &str) -> IResult<&str, &str> {
    value("%S", tag_no_case("ss")).parse(input)
}

fn second(input: &str) -> IResult<&str, &str> {
    alt((
        complete(second2),
        complete(second1)
    )).parse(input)
}

fn special_word(input: &str) -> IResult<&str, &str> {
    alt((
        map(tag("/"), |x| x),
        map(tag(":"), |x| x)
    )).parse(input)
}

fn escaped_word(input: &str) -> IResult<&str, &str> {
    map((tag("\\"), take(1u8)), |(_, rest)| rest).parse(input)
}

fn quoted_word(input: &str) -> IResult<&str, &str> {
    delimited(tag("\""), take_until("\""), tag("\"")).parse(input)
}

fn word(input: &str) -> IResult<&str, &str> {
    alt((
        complete(quoted_word),
        complete(escaped_word),
        complete(special_word)
    )).parse(input)
}

fn hm(input: &str) -> IResult<&str, Vec<&str>> {
    map((hour, word, minute), |(h, w, m)| vec![h, w, m]).parse(input)
}

fn ms(input: &str) -> IResult<&str, Vec<&str>> {
    map((minute, word, second), |(m, w, s)| vec![m, w, s]).parse(input)
}

fn ymdhms(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    many0(alt((
        hm,
        ms,
        map(second, |x| vec![x]),
        map(hour, |x| vec![x]),
        map(year, |x| vec![x]),
        map(month, |x| vec![x]),
        map(day, |x| vec![x]),
        map(word, |x| vec![x]),
    ))).parse(input)
}

fn currency_jp(input: &str) -> IResult<&str, &str> {
    value("{currency_jp}", tag("[$￥-411]")).parse(input)
}

fn red(input: &str) -> IResult<&str, &str> {
    alt((
        value("{{red}}", tag("[赤]")),
        value("{{red}}", tag("[RED]"))
    )).parse(input)
}

fn black(input: &str) -> IResult<&str, &str> {
    alt((
        value("{{balack}}", tag("[黒]")),
        value("{{balack}}", tag("[BLACK]"))
    )).parse(input)
}

fn color(input: &str) -> IResult<&str, &str> {
    alt((
        red,
        black
    )).parse(input)
}

fn number(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii()).parse(input)
}

fn numeric(input: &str) -> IResult<&str, Vec<&str>> {
    map((color, many0(
        alt((
            word,
            currency_jp
        ))
    ), number, many0(
        alt((
            word,
            currency_jp
        ))
    )), |(color, word1, number, word2)| {
        let mut res = vec![];

        res.push(color);

        for item in word1 {
            res.push(item);
        }

        res.push(number);

        for item in word2 {
            res.push(item);
        }

        res
    }).parse(input)
}

fn numeric_ary(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    many_m_n(1, 4, map(
        (tag(";"), numeric), 
        |(_, res)| res)).parse(input)
}