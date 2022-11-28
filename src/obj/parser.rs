use super::{raw::RawObject, tree::TreeEntry};

type ParseResult<'a, O> = Result<(&'a [u8], O), &'a [u8]>;

trait Parser<'a, O> {
    fn parse(&self, input: &'a [u8]) -> ParseResult<'a, O>;
}

impl<'a, O, F> Parser<'a, O> for F
where
    F: Fn(&'a [u8]) -> ParseResult<'a, O>,
{
    fn parse(&self, input: &'a [u8]) -> ParseResult<'a, O> {
        self(input)
    }
}

// combinators

fn pair<'a, P1, P2, A, B>(parser1: P1, parser2: P2) -> impl Fn(&'a [u8]) -> ParseResult<'a, (A, B)>
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

fn pair3<'a, P1, P2, P3, A, B, C>(
    parser1: P1,
    parser2: P2,
    parser3: P3,
) -> impl Fn(&'a [u8]) -> ParseResult<'a, (A, B, C)>
where
    P1: Parser<'a, A>,
    P2: Parser<'a, B>,
    P3: Parser<'a, C>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
                .and_then(|(next_input, (result1, result2))| {
                    parser3
                        .parse(next_input)
                        .map(|(last_input, result3)| (last_input, (result1, result2, result3)))
                })
        })
    }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Fn(&'a [u8]) -> ParseResult<'a, B>
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

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Fn(&'a [u8]) -> ParseResult<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

fn one_or_more<'a, P, A>(parser: P) -> impl Fn(&'a [u8]) -> ParseResult<'a, Vec<A>>
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

fn pred<'a, P, F, A>(parser: P, predicate: F) -> impl Fn(&'a [u8]) -> ParseResult<'a, A>
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
) -> impl Fn(&'a [u8]) -> ParseResult<'a, B>
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

fn match_literal(expected: u8) -> impl Fn(&[u8]) -> ParseResult<()> {
    move |input| match input.get(0) {
        Some(next) if *next == expected => Ok((&input[1..], ())),
        _ => Err(input),
    }
}

fn take(len: usize) -> impl Fn(&[u8]) -> ParseResult<&[u8]> {
    move |input| {
        input
            .get(0..len)
            .map(|result| (&input[len..], result))
            .ok_or(input)
    }
}

fn any_char(input: &[u8]) -> ParseResult<u8> {
    input
        .get(0)
        .map(|next| (&input[1..], next.to_owned()))
        .ok_or(input)
}

fn identifier(input: &[u8]) -> ParseResult<String> {
    left(
        map(one_or_more(pred(any_char, |c| *c != b' ')), |bytes| {
            String::from_utf8_lossy(&bytes).to_string()
        }),
        match_literal(b' '),
    )(input)
}

fn null_byte(input: &[u8]) -> ParseResult<String> {
    left(
        map(one_or_more(pred(any_char, |c| *c != b'\0')), |bytes| {
            String::from_utf8_lossy(&bytes).into_owned()
        }),
        match_literal(b'\0'),
    )(input)
}

fn size(input: &[u8]) -> ParseResult<usize> {
    map(null_byte, |result| result.parse::<usize>().unwrap())(input)
}

fn content(input: &[u8]) -> ParseResult<(usize, &[u8])> {
    and_then(size, move |len| {
        map(take(len), move |content| (len, content))
    })(input)
}

pub fn parse_raw_object(input: &[u8]) -> ParseResult<RawObject> {
    map(pair(identifier, content), |(kind, (_, content))| {
        RawObject::new(kind, content)
    })(input)
}

fn entry_sha(input: &[u8]) -> ParseResult<String> {
    map(take(20), |bytes| {
        String::from_utf8_lossy(&bytes).into_owned()
    })(input)
}

fn tree_entry(input: &[u8]) -> ParseResult<TreeEntry> {
    map(
        pair3(identifier, null_byte, entry_sha),
        |(mode, name, sha)| TreeEntry {
            mode: mode.try_into().unwrap(),
            name,
            sha,
        },
    )(input)
}

pub fn parse_tree_entries(input: &[u8]) -> ParseResult<Vec<TreeEntry>> {
    one_or_more(tree_entry)(input)
}

#[cfg(test)]
mod tests {
    use crate::obj::tree::TreeEntryMode;

    use super::*;

    #[test]
    fn test_identifier_parser() {
        assert_eq!(
            Ok(("".as_bytes(), String::from("blob"))),
            identifier(b"blob ")
        );
    }

    #[test]
    fn test_null_byte_parser() {
        assert_eq!(
            Ok(("".as_bytes(), String::from("text"))),
            null_byte(b"text\0")
        );
    }

    #[test]
    fn test_size_parser() {
        assert_eq!(Ok(("".as_bytes(), 16)), size(b"16\0"));
    }

    #[test]
    fn test_content_parser() {
        assert_eq!(
            Ok(("".as_bytes(), (16, "what is up, doc?".as_bytes()))),
            content(b"16\0what is up, doc?")
        );
    }

    #[test]
    fn test_raw_object_parser() {
        assert_eq!(
            Ok((
                "".as_bytes(),
                RawObject {
                    kind: String::from("blob"),
                    size: 16,
                    content: "what is up, doc?".as_bytes().to_vec()
                }
            )),
            parse_raw_object(b"blob 16\0what is up, doc?")
        );
    }

    #[test]
    fn test_tree_entry_parser() {
        assert_eq!(
            Ok((
                "".as_bytes(),
                TreeEntry {
                    mode: TreeEntryMode::Tree,
                    name: String::from("octopus-admin"),
                    sha: String::from("a84943494657751ce187be401d6bf59ef7a2583c")
                }
            )),
            tree_entry(b"40000 octopus-admin\0a84943494657751ce187be401d6bf59ef7a2583c")
        );
    }

    #[test]
    fn test_tree_entries_parser() {
        assert_eq!(
            Ok((
                "".as_bytes(),
                vec![
					TreeEntry {
						mode: TreeEntryMode::Tree,
						name: String::from("octopus-admin"),
						sha: String::from("a84943494657751ce187be401d6bf59ef7a2583c")
					},
					TreeEntry {
						mode: TreeEntryMode::Blob,
						name: String::from("pom.xml"),
						sha: String::from("97e5b6b292d248869780d7b0c65834bfb645e32a")
					}
				]
            )),
            parse_tree_entries(b"40000 octopus-admin\0a84943494657751ce187be401d6bf59ef7a2583c100644 pom.xml\097e5b6b292d248869780d7b0c65834bfb645e32a")
        );
    }
}
