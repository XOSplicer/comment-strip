use super::CommentMatch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    Start,
    Normal,
    ShebangOrComment,
    Shebang,
    Comment,
    StringDoubleQuotes,
    StringDoubleQuotesEscaped,
    StringSingleQuotes,
    StringSingleQuotesEscaped,
    End
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseAction {
    Nothing,
    CommentStarts,
    CommentEnds,
    CommentCouldStart,
    CommentAbort
}

fn state_transition(from: ParseState, current_char: Option<char>) -> (ParseState, ParseAction) {
    match current_char {
        Some(c) => match from {
            ParseState::Start => match c {
                '#'     => (ParseState::ShebangOrComment, ParseAction::CommentCouldStart),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::Normal => match c {
                '#'     => (ParseState::Comment, ParseAction::CommentStarts),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::ShebangOrComment => match c {
                '!'     => (ParseState::Shebang, ParseAction::CommentAbort),
                _       => (ParseState::Comment, ParseAction::Nothing)
            },
            ParseState::Shebang => match c {
                '\n'    => (ParseState::Normal, ParseAction::Nothing),
                '#'     => (ParseState::Comment, ParseAction::CommentStarts),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Shebang, ParseAction::Nothing)
            },
            ParseState::Comment => match c {
                '\n'    => (ParseState::Normal, ParseAction::CommentEnds),
                _       => (ParseState::Comment, ParseAction::Nothing)
            },
            ParseState::StringDoubleQuotes => match c {
                '"'     => (ParseState::Normal, ParseAction::Nothing),
                '\\'    => (ParseState::StringDoubleQuotesEscaped, ParseAction::Nothing),
                _       => (ParseState::StringDoubleQuotes, ParseAction::Nothing)
            },
            ParseState::StringDoubleQuotesEscaped =>
                (ParseState::StringDoubleQuotes, ParseAction::Nothing),
            ParseState::StringSingleQuotes => match c {
                '\''     => (ParseState::Normal, ParseAction::Nothing),
                '\\'    => (ParseState::StringSingleQuotesEscaped, ParseAction::Nothing),
                _       => (ParseState::StringSingleQuotes, ParseAction::Nothing)
            },
            ParseState::StringSingleQuotesEscaped =>
                (ParseState::StringSingleQuotes, ParseAction::Nothing),
            ParseState::End =>
                (ParseState::End, ParseAction::Nothing)
        },
        None => match from {
            // ..... return if over and comment was finished or not
            ParseState::Comment => (ParseState::End, ParseAction::CommentEnds),
            ParseState::ShebangOrComment => (ParseState::End, ParseAction::CommentAbort),
            _ => (ParseState::End, ParseAction::Nothing)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommentState {
    NotInComment,
    MaybeInComment(usize),
    InComment(usize)
}

pub fn find_comments(input: &str) -> Result<Vec<CommentMatch>, &'static str> {
    let mut matches = Vec::new();
    let mut current_state = ParseState::Start;
    let mut comment_state = CommentState::NotInComment;
    let mut chars = input.chars();
    let mut position = 0;
    while current_state != ParseState::End {
        let current_char = chars.next();
        println!("parsing state {:?} with input '{:?}'", &current_state, &current_char );
        let (next_state, action) = state_transition(current_state, current_char);
        match action {
            ParseAction::Nothing => {},
            ParseAction::CommentStarts => {
                comment_state = CommentState::InComment(position);
            },
            ParseAction::CommentCouldStart =>  {
                comment_state = CommentState::MaybeInComment(position);
            },
            ParseAction::CommentAbort => {
                comment_state = CommentState::NotInComment;
            },
            ParseAction::CommentEnds => {
                match comment_state {
                    CommentState::NotInComment => {
                        return Err("parse error");
                    },
                    CommentState::MaybeInComment(from) => {
                        matches.push(CommentMatch{from: from, to: position});
                        comment_state = CommentState::NotInComment;
                    },
                    CommentState::InComment(from) => {
                        matches.push(CommentMatch{from: from, to: position});
                        comment_state = CommentState::NotInComment;
                    }
                }
            }
        }
        //println!("action {:?}", &action);
        current_state = next_state;
        position += 1;
    }
    Ok(matches)
}
