//! Excel Base Format
use std::borrow::Cow;
use super::nom::{ branch::alt, 
    bytes::complete::tag_no_case,
    combinator::{ complete, value },
    IResult};
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
            IResult::Done(_, output) => true,
            _ => false
        }
    }

    fn is_numeric_ary<'a, S>(value: S) -> bool 
    where S: Into<Cow<'a, str>>
    {
        match numeric_ary(value.into().into_owned().as_str()) {
            IResult::Done(_, output) => true,
            _ => false
        }
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_date_formats(&self) -> Option<Vec<&str>> {
        match ymdhms(self.content.as_str()) {
            IResult::Done(_, output) => {
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
            IResult::Done(_, output) => {
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
    value("{{era1}}", tag_no_case("gg")).parse(input)
}

fn gengou3(input: &str) -> IResult<&str, &str> {
    value("{{era1}}", tag_no_case("ggg")).parse(input)
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

/* named!(year4<&str, &str>, 
    map!(alt!(tag!("yyyy") | tag!("YYYY")), |_| "%Y"));

named!(year2<&str, &str>, 
    map!(alt!(tag!("yy") | tag!("YY")), |_| "%y"));

named!(era1<&str, &str>, 
    map!(alt!(tag!("e") | tag!("E")), |_| "{{era1}}"));

named!(era2<&str, &str>, 
    map!(alt!(tag!("ee") | tag!("EE")), |_| "{{era2}}"));

named!(gengou1<&str, &str>, 
    map!(alt!(tag!("g") | tag!("G")), |_| "{{gengou1}}"));

named!(gengou2<&str, &str>, 
    map!(alt!(tag!("gg") | tag!("GG")), |_| "{{gengou2}}"));

named!(gengou3<&str, &str>, 
    map!(alt!(tag!("ggg") | tag!("GGG")), |_| "{{gengou3}}"));

named!(year<&str, &str>,
    alt!(
        complete!(year4) | 
        complete!(year2) | 
        complete!(era2) | 
        complete!(era1) | 
        complete!(gengou3) | 
        complete!(gengou2) | 
        complete!(gengou1)
    )
);

named!(month1<&str, &str>, 
    map!(alt!(tag!("m") | tag!("M")), |_| "%-m"));

named!(month2<&str, &str>, 
    map!(alt!(tag!("mm") | tag!("MM")), |_| "%m"));

named!(month3<&str, &str>, 
    map!(alt!(tag!("mmm") | tag!("MMM")), |_| "%b"));

named!(month4<&str, &str>, 
    map!(alt!(tag!("mmmm") | tag!("MMMM")), |_| "%B"));

named!(month5<&str, &str>, 
    map!(alt!(tag!("mmmmm") | tag!("MMMMM")), |_| "{{month5}}"));

named!(month<&str, &str>,
    alt!(
        complete!(month5) | 
        complete!(month4) | 
        complete!(month3) | 
        complete!(month2) | 
        complete!(month1)
    )
);

named!(day1<&str, &str>, 
    map!(alt!(tag!("d") | tag!("D")), |_| "%-d"));

named!(day2<&str, &str>, 
    map!(alt!(tag!("dd") | tag!("DD")), |_| "%d"));

named!(dow3<&str, &str>, 
    map!(alt!(tag!("ddd") | tag!("DDD")), |_| "%a"));

named!(dow4<&str, &str>, 
    map!(alt!(tag!("dddd") | tag!("DDDD")), |_| "%A"));

named!(youbi3<&str, &str>, 
    map!(alt!(tag!("aaa") | tag!("AAA")), |_| "{{youbi3}}"));

named!(youbi4<&str, &str>, 
    map!(alt!(tag!("aaaa") | tag!("AAAA")), |_| "{{youbi4}}"));

named!(day<&str, &str>,
    alt!(
        complete!(youbi4) | 
        complete!(youbi3) | 
        complete!(dow4) | 
        complete!(dow3) | 
        complete!(day2) | 
        complete!(day1)
    )
);

named!(hour1<&str, &str>, 
    map!(alt!(tag!("h") | tag!("H")), |_| "%-H"));

named!(hour2<&str, &str>, 
    map!(alt!(tag!("hh") | tag!("HH")), |_| "%H"));

named!(hour<&str, &str>, 
    alt!(
        complete!(hour2) | 
        complete!(hour1)
    )
);

named!(minute1<&str, &str>, 
    map!(alt!(tag!("m") | tag!("M")), |_| "%-M"));

named!(minute2<&str, &str>, 
    map!(alt!(tag!("mm") | tag!("MM")), |_| "%M"));

named!(minute<&str, &str>, 
    alt!(
        complete!(minute2) | 
        complete!(minute1)
    )
);

named!(second1<&str, &str>, 
    map!(alt!(tag!("s") | tag!("S")), |_| "%-S"));

named!(second2<&str, &str>, 
    map!(alt!(tag!("ss") | tag!("SS")), |_| "%S"));

named!(second<&str, &str>, 
    alt!(
        complete!(second2) | 
        complete!(second1)
    )
);

named!(special_word<&str, &str>, 
    map!(alt!(tag!("/") | tag!(":")), |x| x));

named!(escaped_word<&str, &str>, 
    do_parse!(
        tag!("\\") >>
        res: take_s!(1) >>
        (res)
    ));

named!(quoted_word<&str, &str>,
    delimited!(tag!("\""), take_until_s!("\""), tag!("\""))
);

named!(word<&str, &str>, 
    alt!(
        complete!(quoted_word) | 
        complete!(escaped_word) | 
        complete!(special_word)
    )
);

named!(hm<&str, Vec<&str> >,
    do_parse!(
        h: hour >>
        w: opt!(word) >>
        m: minute >>
        (
            if let Some(w) = w {
                vec![h, w ,m] 
            } else {
                 vec![h, m] 
            }
        )
    )
);

named!(ms<&str, Vec<&str> >,
    do_parse!(
        m: minute >>
        w: opt!(word) >>
        s: second >>
        (
            if let Some(w) = w {
                vec![m, w ,s] 
            } else {
                 vec![m, s] 
            }
        )
    )
);

named!(ymdhms<&str, Vec<Vec<&str>> >,
    many0!(alt!(hm | ms | 
        map!(second, |x| vec![x]) |
        map!(hour, |x| vec![x]) |
        map!(year, |x| vec![x]) |
        map!(month, |x| vec![x]) |
        map!(day, |x| vec![x]) |
        map!(word, |x| vec![x])
    ))
);

named!(currency_jp<&str, &str>, 
    map!(tag_s!("[$￥-411]"), |_| "{{currency_jp}}")
);

named!(red<&str, &str>, 
    map!(alt!(tag_s!("[赤]") | tag_s!("[RED]")), |_| "{{red}}")
);

named!(black<&str, &str>, 
    map!(alt!(tag_s!("[黒]") | tag_s!("[BLACK]")), |_| "{{black}}")
);

named!(color<&str, &str>,
    alt!(red | black)
);

named!(number<&str, &str>,
    take_while1_s!(call!(|c| c == '0' || c == '#' || c == '.' || c == ',' || c == '?'))
);

named!(numeric<&str, Vec<&str> >,
    do_parse!(
        c: opt!(color) >>
        w1: many0!(alt!(word | currency_jp)) >>
        nums: number >>
        w2: many0!(alt!(word | currency_jp)) >>
        ({
            let mut res = vec![];
            if let Some(n) = c {
                res.push(n);
            }
            for item in w1 {
                res.push(item);
            }
            res.push(nums);
            for item in w2 {
                res.push(item);
            }
            res
        })
    )
);

named!(numeric_ary<&str, Vec<Vec<&str> > >,
    many_m_n!(1,4,
        do_parse!(
            opt!(tag_s!(";")) >>
            res: numeric >>
            (res)
        )
    )
); */