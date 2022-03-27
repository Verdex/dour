
use array_pattern::{Success, MatchError, seq, alt};

#[derive(Debug)]
pub enum Token {
    LowerSymbol(String),
    UpperSymbol(String),
    Bool(bool),
    Number(f64),
    LParen,
    RParen,
    LCurl,
    RCurl,
    LSquare,
    RSquare,
    LAngle,
    RAngle,
    Comma,
    SemiColon,
    Colon,
}

fn number( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Success<Token>, MatchError> {
    fn digit( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Success<char>, MatchError> {
        let mut rp = input.clone();
        match input.next() {
            Some((i, c)) if c.is_digit(10) => Ok(Success { start: i, end: i, item: c }),
            Some((i, _)) => { 
                std::mem::swap(&mut rp, input);
                Err(MatchError::Error(i))
            },
            None => {
                std::mem::swap(&mut rp, input);
                Err(MatchError::ErrorEndOfFile)
            },
        }
    }

    seq!(zero_or_more ~ digits<'a>: char => char = d <= digit, { d });
    seq!(maybe ~ dot<'a>: char => char = d <= '.', { d });

    seq!(little_e<'a>: char => char = e <= 'e', { e });
    seq!(big_e<'a>: char => char = e <= 'E', { e });
    alt!(e<'a>: char => char = little_e | big_e);

    seq!(plus<'a>: char => char = p <= '+', { p });
    seq!(minus<'a>: char => char = m <= '-', { m });
    alt!(sign<'a>: char => char = plus | minus );
    seq!(maybe ~ maybe_sign<'a>: char => char = s <= sign, { s });

    seq!(maybe ~ science<'a>: char => String = _e <= e, ms <= maybe_sign, ds <= digits, {
        match ms {
            Some(x) => format!("e{}{}", x, ds.into_iter().collect::<String>()),
            None => format!("e{}", ds.into_iter().collect::<String>()),
        }
    } );

    alt!(initial<'a>: char => char = sign | digit );

    seq!(main<'a>: char => String = init <= initial, whole <= digits, d <= dot, fractional <= digits, s <= science, {
        let ret = format!("{}{}", init, whole.into_iter().collect::<String>());
        let ret = match d { 
            Some(_) => format!("{}.{}", ret, fractional.into_iter().collect::<String>()),
            None => ret,
        };
        match s {
            Some(s) => format!("{}{}", ret, s),
            None => ret,
        }
    });

    match main(input) {
        Ok(Success { item, start, end }) => {
            let ret = item.parse::<f64>().expect("allowed number string that rust fails to parse with parse::<f64>()");
            Ok(Success { item: Token::Number(ret), start, end })
        },
        Err(e) => Err(e),
    }
}

fn lower_symbol( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Success<Token>, MatchError> {

    fn init_lower_symbol_char( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Success<char>, MatchError> {
        let mut rp = input.clone();
        match input.next() {
            Some((i, c)) if c.is_lowercase() || c == '_' => Ok(Success { start: i, end: i, item: c }),
            Some((i, _)) => { 
                std::mem::swap(&mut rp, input);
                Err(MatchError::Error(i))
            },
            None => {
                std::mem::swap(&mut rp, input);
                Err(MatchError::ErrorEndOfFile)
            },
        }
    }

    fn rest_lower_symbol_char( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Success<char>, MatchError> {
        let mut rp = input.clone();
        match input.next() {
            Some((i, c)) if c.is_alphanumeric() || c == '_' => Ok(Success { start: i, end: i, item: c }),
            Some((i, _)) => { 
                std::mem::swap(&mut rp, input);
                Err(MatchError::Error(i))
            },
            None => {
                std::mem::swap(&mut rp, input);
                Err(MatchError::ErrorEndOfFile)
            },
        }
    }
    
    alt!( rest<'a> : char => char = init_lower_symbol_char | rest_lower_symbol_char );
    seq!( zero_or_more ~ rests<'a> : char => char = r <= rest, {
        r
    } );
    seq!( main<'a> : char => Token = init <= init_lower_symbol_char, rs <= rests, {
        match format!( "{}{}", init, rs.into_iter().collect::<String>()) {
            x if x == "true" => Token::Bool(true),
            x if x == "false" => Token::Bool(false),
            x => Token::LowerSymbol(x),
        }
    } );

    main(input)
}

fn upper_symbol( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Success<Token>, MatchError> {

    fn init_upper_symbol_char( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Success<char>, MatchError> {
        let mut rp = input.clone();
        match input.next() {
            Some((i, c)) if c.is_uppercase() => Ok(Success { start: i, end: i, item: c }),
            Some((i, _)) => { 
                std::mem::swap(&mut rp, input);
                Err(MatchError::Error(i))
            },
            None => {
                std::mem::swap(&mut rp, input);
                Err(MatchError::ErrorEndOfFile)
            },
        }
    }

    fn rest_upper_symbol_char( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Success<char>, MatchError> {
        let mut rp = input.clone();
        match input.next() {
            Some((i, c)) if c.is_alphanumeric() => Ok(Success { start: i, end: i, item: c }),
            Some((i, _)) => { 
                std::mem::swap(&mut rp, input);
                Err(MatchError::Error(i))
            },
            None => {
                std::mem::swap(&mut rp, input);
                Err(MatchError::ErrorEndOfFile)
            },
        }
    }

    alt!( rest<'a> : char => char = init_upper_symbol_char | rest_upper_symbol_char );
    seq!( zero_or_more ~ rests<'a> : char => char = r <= rest, {
        r
    } );
    seq!( main<'a> : char => Token = init <= init_upper_symbol_char, rs <= rests, {
        Token::UpperSymbol(format!( "{}{}", init, rs.into_iter().collect::<String>() ))
    } );

    main(input)
}


pub fn tokenize( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Success<Token>, MatchError> {
    alt!( main<'a> : char => Token = lower_symbol | upper_symbol | number );

    main(input)
}

#[cfg(test)]
mod test { 
    use super::*;

    #[test]
    fn should_parse_numbers() -> Result<(), MatchError> {
        fn t(s : &str, expected : f64) -> Result<(), MatchError> {
            let mut input = s.char_indices();
            let output = tokenize(&mut input)?;

            assert_eq!( output.start, 0 );
            assert_eq!( output.end, s.len() - 1 );

            let value = match output.item {
                Token::Number(n) => n,
                _ => panic!("not number"),
            };

            assert_eq!( value, expected );
            Ok(())
        }

        t("0", 0.0)?;
        t("0.0", 0.0)?;

        Ok(())
    }

    #[test]
    fn should_parse_boolean_starting_lower_symbol() -> Result<(), MatchError> {
        let s = "false_";
        let mut input = s.char_indices();
        let output = tokenize(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, s.len() - 1 );

        let name = match output.item {
            Token::LowerSymbol(n) => n,
            _ => panic!("not lower symbol"),
        };

        assert_eq!( name, "false_" );

        Ok(())
    }

    #[test]
    fn should_parse_false() -> Result<(), MatchError> {
        let s = "false";
        let mut input = s.char_indices();
        let output = tokenize(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, s.len() - 1 );

        let name = match output.item {
            Token::Bool(n) => n,
            _ => panic!("not bool"),
        };

        assert_eq!( name, false );

        Ok(())
    }

    #[test]
    fn should_parse_true() -> Result<(), MatchError> {
        let s = "true";
        let mut input = s.char_indices();
        let output = tokenize(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, s.len() - 1 );

        let name = match output.item {
            Token::Bool(n) => n,
            _ => panic!("not bool"),
        };

        assert_eq!( name, true );

        Ok(())
    }

    #[test]
    fn should_parse_lower_symbol() -> Result<(), MatchError> {
        let s = "lower_symbol";
        let mut input = s.char_indices();
        let output = tokenize(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, s.len() - 1 );

        let name = match output.item {
            Token::LowerSymbol(n) => n,
            _ => panic!("not lower symbol"),
        };

        assert_eq!( name, "lower_symbol" );

        Ok(())
    }

    #[test]
    fn should_parse_single_lower_symbol() -> Result<(), MatchError> {
        let s = "l";
        let mut input = s.char_indices();
        let output = tokenize(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, s.len() - 1 );

        let name = match output.item {
            Token::LowerSymbol(n) => n,
            _ => panic!("not lower symbol"),
        };

        assert_eq!( name, "l" );

        Ok(())
    }

    #[test]
    fn should_parse_upper_symbol() -> Result<(), MatchError> {
        let s = "UpperSymbol";
        let mut input = s.char_indices();
        let output = tokenize(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, s.len() - 1 );

        let name = match output.item {
            Token::UpperSymbol(n) => n,
            _ => panic!("not upper symbol"),
        };

        assert_eq!( name, "UpperSymbol" );

        Ok(())
    }

    #[test]
    fn should_parse_single_upper_symbol() -> Result<(), MatchError> {
        let s = "U";
        let mut input = s.char_indices();
        let output = tokenize(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, s.len() - 1 );

        let name = match output.item {
            Token::UpperSymbol(n) => n,
            _ => panic!("not upper symbol"),
        };

        assert_eq!( name, "U" );

        Ok(())
    }
}