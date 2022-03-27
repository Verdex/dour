
use array_pattern::{Success, MatchError, seq, alt};

#[derive(Debug)]
pub enum Token {
    LowerSymbol(String),
    UpperSymbol(String),
    Int(i64),
    Bool(bool),
    Float(f64),
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
        Token::LowerSymbol(format!( "{}{}", init, rs.into_iter().collect::<String>() ))
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



pub fn tokenize( input : &mut (impl Iterator<Item = (usize, char)> + Clone) ) -> Result<Token, MatchError> {
    
    Err(MatchError::FatalEndOfFile)
}

#[cfg(test)]
mod test { 
    use super::*;

    #[test]
    fn lower_symbol_should_parse_lower_symbol() -> Result<(), MatchError> {
        let s = "lower_symbol";
        let mut input = s.char_indices();
        let output = lower_symbol(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, 11 );

        let name = match output.item {
            Token::LowerSymbol(n) => n,
            _ => panic!("not lower symbol"),
        };

        assert_eq!( name, "lower_symbol" );

        Ok(())
    }

    #[test]
    fn lower_symbol_should_parse_single_upper_symbol() -> Result<(), MatchError> {
        let s = "l";
        let mut input = s.char_indices();
        let output = lower_symbol(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, 0 );

        let name = match output.item {
            Token::LowerSymbol(n) => n,
            _ => panic!("not lower symbol"),
        };

        assert_eq!( name, "l" );

        Ok(())
    }

    #[test]
    fn upper_symbol_should_parse_upper_symbol() -> Result<(), MatchError> {
        let s = "UpperSymbol";
        let mut input = s.char_indices();
        let output = upper_symbol(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, 10 );

        let name = match output.item {
            Token::UpperSymbol(n) => n,
            _ => panic!("not upper symbol"),
        };

        assert_eq!( name, "UpperSymbol" );

        Ok(())
    }

    #[test]
    fn upper_symbol_should_parse_single_upper_symbol() -> Result<(), MatchError> {
        let s = "U";
        let mut input = s.char_indices();
        let output = upper_symbol(&mut input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, 0 );

        let name = match output.item {
            Token::UpperSymbol(n) => n,
            _ => panic!("not upper symbol"),
        };

        assert_eq!( name, "U" );

        Ok(())
    }
}