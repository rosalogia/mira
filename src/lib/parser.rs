use nom::error::ParseError;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace0},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

#[derive(Debug)]
pub enum Query {
    Tag(String),
    And(Vec<Query>),
    Or(Vec<Query>),
    Not(Box<Query>),
}

impl Query {
    pub fn to_sql_unsafe(self) -> String {
        match self {
            Query::Tag(s) => format!("bool_or(t.name = '{}')", s),
            Query::And(sq) => {
                let queries: String = sq
                    .into_iter()
                    .map(|s| s.to_sql_unsafe())
                    .collect::<Vec<String>>()
                    .join(" AND ");
                format!("({})", queries)
            }
            Query::Or(sq) => {
                let queries: String = sq
                    .into_iter()
                    .map(|s| s.to_sql_unsafe())
                    .collect::<Vec<String>>()
                    .join(" OR ");
                format!("({})", queries)
            },
            Query::Not(q) => {
                let query = q.to_sql_unsafe();
                format!("(NOT {})", query)
            }
            _ => panic!("Unimplemented"),
        }
    }

    pub fn to_sql_inner(self, counter: &mut i32) -> (String, Vec<String>) {
        match self {
            Query::Tag(s) => {
                let r = (format!("bool_or(t.name = ${})", *counter), vec![s]);
                *counter += 1;
                r
            }
            Query::And(sq) => {
                let (queries, values): (Vec<String>, Vec<Vec<String>>) =
                    sq.into_iter().map(|s| s.to_sql_inner(counter)).unzip();
                let queries: String = queries.join(" AND ");
                let values: Vec<String> = values.into_iter().flatten().collect();

                (format!("({})", queries), values)
            }
            Query::Or(sq) => {
                let (queries, values): (Vec<String>, Vec<Vec<String>>) =
                    sq.into_iter().map(|s| s.to_sql_inner(counter)).unzip();
                let queries: String = queries.join(" OR ");
                let values: Vec<String> = values.into_iter().flatten().collect();

                (format!("({})", queries), values)
            }
            Query::Not(q) => {
                let (query, value) = q.to_sql_inner(counter);
                (format!("(NOT {})", query), value)
            }
            _ => panic!("Unimplemented"),
        }
    }

    pub fn to_sql(self) -> (String, Vec<String>) {
        self.to_sql_inner(&mut 1)
    }
}

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn tag_parser(input: &str) -> IResult<&str, Query> {
    // println!("Tag called on {}", input);
    let result = ws(map(is_not("&|!()"), |s: &str| {
        Query::Tag(s.trim().into())
    }))(input);
    match result {
        Err(e) => {
            // println!("Tag error: {:?}", e);
            Err(e)
        }
        Ok((input, res)) => {
            // println!("Worked: {:?}. Left: {}", res, input);
            Ok((input, res))
        }
    }
}

fn not_parser(input: &str) -> IResult<&str, Query> {
    let result = preceded(
        char('!'),
        map(alt((tag_parser, parenthesized_parser)), |query| {
            Query::Not(Box::new(query))
        }),
    )(input);
    // println!("Not called on {}: {}", input, result.is_ok());
    result
}

fn parenthesized_parser(input: &str) -> IResult<&str, Query> {
    // println!("Paren called on {}", input);
    let (i, r) = preceded(tag("("), query_parser)(input)?;
    // println!("Got here. Remaining input: {}", i);
    let result = char(')')(i);
    match result {
        Ok((i, _)) => {
            // println!("Succeeded. Remaining input: {}", i);
            Ok((i, r))
        }
        Err(e) => {
            // println!("Failed to parse ): {:?}", e);
            Err(e)
        }
    }
}

fn and_parser(input: &str) -> IResult<&str, Query> {
    // println!("And called on {}", input);
    let (input, first_tag) =
        terminated(and_complement_parser, preceded(multispace0, tag("&")))(input)?;
    let (input, mut rest) = separated_list1(tag("&"), and_complement_parser)(input)?;
    rest.insert(0, first_tag);
    let result = Query::And(rest);
    Ok((input, result))
}

fn or_parser(input: &str) -> IResult<&str, Query> {
    // println!("And called on {}", input);
    let (input, first_tag) =
        terminated(or_complement_parser, preceded(multispace0, tag("|")))(input)?;
    let (input, mut rest) = separated_list1(tag("|"), or_complement_parser)(input)?;
    rest.insert(0, first_tag);
    let result = Query::Or(rest);
    Ok((input, result))
}

fn and_complement_parser(input: &str) -> IResult<&str, Query> {
    // println!("AC called on {}", input);
    alt((not_parser, tag_parser, parenthesized_parser))(input)
}

fn or_complement_parser(input: &str) -> IResult<&str, Query> {
    // println!("OC called on {}", input);
    alt((not_parser, tag_parser, parenthesized_parser))(input)
}

fn query_parser(input: &str) -> IResult<&str, Query> {
    // println!("Base called on {}", input);
    alt((
        and_parser,
        or_parser,
        not_parser,
        tag_parser,
        parenthesized_parser,
    ))(input)
}

pub fn parse_boolean_expression(input: &str) -> Result<Query, nom::Err<nom::error::Error<&str>>> {
    query_parser(input).map(|(_, query)| query)
}
