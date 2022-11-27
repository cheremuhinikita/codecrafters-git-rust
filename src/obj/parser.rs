#[derive(Debug, PartialEq)]
pub struct RawObject {
    pub kind: String,
    pub size: usize,
    pub content: String,
}

type ParseResult<'a, O> = Result<(&'a str, O), &'a str>;

trait Parser<'a, O> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, O>;
}

impl<'a, O, F> Parser<'a, O> for F
where
    F: Fn(&'a str) -> ParseResult<'a, O>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, O> {
        self(input)
    }
}

// combinators

fn pair<'a, P1, P2, A, B>(parser1: P1, parser2: P2) -> impl Fn(&'a str) -> ParseResult<'a, (A, B)>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, B>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&'a str) -> ParseResult<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&'a str) -> ParseResult<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

fn one_or_more<'a, P, A>(parser: P) -> impl Fn(&'a str) -> ParseResult<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |input| {
        let mut result = Vec::<A>::new();
        let mut current_input = input;

        if let Ok((next_input, first_result)) = parser.parse(current_input) {
            current_input = next_input;
            result.push(first_result);
        } else {
            return Err(input);
        }

        while let Ok((next_input, subsequent_result)) = parser.parse(current_input) {
            current_input = next_input;
            result.push(subsequent_result);
        }

        Ok((current_input, result))
    }
}

fn pred<'a, P, F, A>(parser: P, predicate: F) -> impl Fn(&'a str) -> ParseResult<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        parser.parse(input).and_then(|(next_input, result)| {
            if predicate(&result) {
                Ok((next_input, result))
            } else {
                Err(input)
            }
        })
    }
}

fn and_then<'a, P1, P2, F, A, B>(
    parser1: P1,
    and_then_fn: F,
) -> impl Fn(&'a str) -> ParseResult<'a, B>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, B>,
    F: Fn(A) -> P2,
{
    move |input| {
        parser1
            .parse(input)
            .and_then(|(next_input, result)| and_then_fn(result).parse(next_input))
    }
}

fn match_literal(expected: &'static str) -> impl Fn(&str) -> ParseResult<()> {
    move |input| match input.get(..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

fn take(len: usize) -> impl Fn(&str) -> ParseResult<String> {
    move |input| {
        input
            .get(0..len)
            .map(|result| (&input[len..], result.to_string()))
            .ok_or(input)
    }
}

fn any_char(input: &str) -> ParseResult<char> {
    input
        .chars()
        .next()
        .map(|next| (&input[next.len_utf8()..], next))
        .ok_or(input)
}

fn kind(input: &str) -> ParseResult<String> {
    left(
        map(one_or_more(pred(any_char, |c| *c != ' ')), |chars| {
            chars.into_iter().collect()
        }),
        match_literal(" "),
    )(input)
}

fn size(input: &str) -> ParseResult<usize> {
    map(
        left(
            map(one_or_more(pred(any_char, |c| *c != '\0')), |chars| {
                chars.into_iter().collect::<String>()
            }),
            match_literal("\0"),
        ),
        |chars| chars.parse::<usize>().unwrap(),
    )(input)
}

fn content(input: &str) -> ParseResult<(usize, String)> {
    and_then(size, move |len| {
        map(take(len), move |content| (len, content))
    })(input)
}

pub fn raw_object(input: &str) -> ParseResult<RawObject> {
    map(pair(kind, content), |(kind, (size, content))| RawObject {
        kind,
        size,
        content,
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_parser() {
        assert_eq!(Ok(("", String::from("blob"))), kind("blob "));
    }

    #[test]
    fn test_size_parser() {
        assert_eq!(Ok(("", 16)), size("16\0"));
    }

    #[test]
    fn test_content_parser() {
        assert_eq!(
            Ok(("", (16, String::from("what is up, doc?")))),
            content("16\0what is up, doc?")
        );
    }

    #[test]
    fn test_raw_object_parser() {
        assert_eq!(
            Ok((
                "",
                RawObject {
                    kind: String::from("blob"),
                    size: 16,
                    content: String::from("what is up, doc?")
                }
            )),
            raw_object("blob 16\0what is up, doc?")
        );
    }
}
