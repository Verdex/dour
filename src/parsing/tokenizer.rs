
use array_pattern::{Success, MatchError, seq, alt, pred, group};

/*#[derive(Debug)]
pub struct TMeta {
    pub start : usize,
    pub end : usize,
}

#[derive(Debug)]
pub enum Token {
    LowerSymbol(String),
    UpperSymbol(String),
    Bool(bool),
    Number(f64),
    String(String),
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
}*/

#[derive(Debug)]
enum InternalToken {
    LowerSymbol(String),
    UpperSymbol(String),
    Bool(bool),
    Number(f64),
    String(String),
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

group!(string<'a>: char => InternalToken = |input| {
    seq!(slash_n<'a>: char => char = _slash <= '\\', _n <= 'n', { '\n' });
    seq!(slash_r<'a>: char => char = _slash <= '\\', _r <= 'r', { '\r' });
    seq!(slash_t<'a>: char => char = _slash <= '\\', _t <= 't', { '\t' });
    seq!(slash_slash<'a>: char => char = _slash <= '\\', _s <= '\\', { '\\' });
    seq!(slash_zero<'a>: char => char = _slash <= '\\', _z <= '0', { '\0' });
    seq!(slash_quote<'a>: char => char = _slash <= '\\', _q <= '"', { '"' });
    pred!(any<'a>: char => char = |c| c != '"');
    alt!(str_char<'a>: char => char = slash_n 
                                    | slash_r
                                    | slash_t
                                    | slash_quote
                                    | slash_slash
                                    | slash_zero
                                    | any  
                                    );

    seq!(zero_or_more ~ str_chars<'a>: char => char = sc <= str_char, { sc });
    seq!(quote<'a>: char => () = _q <= '"', { () });

    seq!(main<'a>: char => InternalToken = _q1 <= quote, sc <= str_chars, _q2 <= quote, {
        InternalToken::String(sc.into_iter().collect::<String>())
    });

    main(input)
});

group!(number<'a>: char => InternalToken = |input| { 
    pred!(digit<'a>: char => char = |c : char| c.is_digit(10));
    seq!(zero_or_more ~ digits<'a>: char => char = d <= digit, { d });
    seq!(maybe ~ dot<'a>: char => char = d <= '.', { d });

    seq!(little_e<'a>: char => char = e <= 'e', { e });
    seq!(big_e<'a>: char => char = e <= 'E', { e });
    alt!(e<'a>: char => char = little_e | big_e);

    seq!(plus<'a>: char => char = p <= '+', { p });
    seq!(minus<'a>: char => char = m <= '-', { m });
    alt!(sign<'a>: char => char = plus | minus );
    seq!(maybe ~ maybe_sign<'a>: char => char = s <= sign, { s });

    seq!(maybe ~ science<'a>: char => String = _e <= e, ms <= maybe_sign, init <= digit, ds <= digits, {
        match ms {
            Some(x) => format!("e{}{}{}", x, init, ds.into_iter().collect::<String>()),
            None => format!("e{}{}", init, ds.into_iter().collect::<String>()),
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
            Ok(Success { item: InternalToken::Number(ret), start, end })
        },
        Err(e) => Err(e),
    }
});

group!(lower_symbol<'a>: char => InternalToken = |input| {
    pred!(init_lower_symbol_char<'a>: char => char = |c : char| c.is_lowercase() || c == '_');
    pred!(rest_lower_symbol_char<'a>: char => char = |c : char| c.is_alphanumeric() || c == '_');
    alt!( rest<'a> : char => char = init_lower_symbol_char | rest_lower_symbol_char );
    seq!( zero_or_more ~ rests<'a> : char => char = r <= rest, {
        r
    } );
    seq!( main<'a> : char => InternalToken = init <= init_lower_symbol_char, rs <= rests, {
        match format!( "{}{}", init, rs.into_iter().collect::<String>()) {
            x if x == "true" => InternalToken::Bool(true),
            x if x == "false" => InternalToken::Bool(false),
            x => InternalToken::LowerSymbol(x),
        }
    } );

    main(input)
});

group!(upper_symbol<'a>: char => InternalToken = |input| { 
    pred!(init_upper_symbol_char<'a>: char => char = |c : char| c.is_uppercase());
    pred!(rest_upper_symbol_char<'a>: char => char = |c : char| c.is_alphanumeric());
    alt!( rest<'a> : char => char = init_upper_symbol_char | rest_upper_symbol_char );
    seq!( zero_or_more ~ rests<'a> : char => char = r <= rest, { r } );
    seq!( main<'a> : char => InternalToken = init <= init_upper_symbol_char, rs <= rests, {
        InternalToken::UpperSymbol(format!( "{}{}", init, rs.into_iter().collect::<String>() ))
    } );

    main(input)
});

fn internal_tokenize( input : &str ) -> Result<Vec<Success<InternalToken>>, MatchError> {

    let mut x = input.char_indices();

    alt!( token<'a> : char => InternalToken = lower_symbol | upper_symbol | number | string );

    let mut ret = vec![];
    loop {
        match token(&mut x) {
            Ok(t) => ret.push(t),
            Err(e) => return Err(e),
        }
    }

    Ok(ret)
}

#[cfg(test)]
mod test { 
    use super::*;

    #[test]
    fn should_parse_string() -> Result<(), MatchError> {
        fn t(input : &str, expected : &str) -> Result<(), MatchError> {
            let output = tokenize(input)?;

            assert_eq!( output.start, 0 );
            assert_eq!( output.end, input.len() - 1 );

            assert_eq!( output.item.len(), 1 );
            let value = match &output.item[0] {
                InternalToken::String(n) => n.clone(),
                _ => panic!("not string"),
            };

            assert_eq!( value, expected );
            Ok(())
        }

        t(r#""string input""#, "string input")?;
        // TODO more cases

        Ok(())
    }

    #[test]
    fn should_parse_numbers() -> Result<(), MatchError> {
        fn t(input : &str, expected : f64) -> Result<(), MatchError> {
            let output = tokenize(input)?;

            assert_eq!( output.start, 0 );
            assert_eq!( output.end, input.len() - 1 );

            assert_eq!( output.item.len(), 1 );
            let value = match &output.item[0] {
                InternalToken::Number(n) => *n,
                _ => panic!("not number"),
            };

            assert_eq!( value, expected );
            Ok(())
        }

        t("0", 0.0)?;
        t("0.0", 0.0)?;
        t("1E1", 1E1)?;
        t("1e1", 1e1)?;
        t("+1.0", 1.0)?;
        t("-1.0", -1.0)?;
        t("1E+1", 1E+1)?;
        t("1e+1", 1e+1)?;
        t("1234.5678", 1234.5678)?;
        t("1234.5678E-90", 1234.5678E-90)?;
        t("1234.5678e-90", 1234.5678e-90)?;
        t("1234.5678e-901", 1234.5678e-901)?;
        t("1234", 1234.0)?;

        Ok(())
    }

    #[test]
    fn should_parse_boolean_starting_lower_symbol() -> Result<(), MatchError> {
        let input = "false_";
        let output = tokenize(input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, input.len() - 1 );

        assert_eq!( output.item.len(), 1 );
        let name = match &output.item[0] {
            InternalToken::LowerSymbol(n) => n.clone(),
            _ => panic!("not lower symbol"),
        };

        assert_eq!( name, "false_" );

        Ok(())
    }

    #[test]
    fn should_parse_false() -> Result<(), MatchError> {
        let input = "false";
        let output = tokenize(input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, input.len() - 1 );

        assert_eq!( output.item.len(), 1 );
        let name = match &output.item[0] {
            InternalToken::Bool(n) => *n,
            _ => panic!("not bool"),
        };

        assert_eq!( name, false );

        Ok(())
    }

    #[test]
    fn should_parse_true() -> Result<(), MatchError> {
        let input = "true";
        let output = tokenize(input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, input.len() - 1 );

        assert_eq!( output.item.len(), 1 );
        let name = match &output.item[0] {
            InternalToken::Bool(n) => *n,
            _ => panic!("not bool"),
        };

        assert_eq!( name, true );

        Ok(())
    }

    #[test]
    fn should_parse_lower_symbol() -> Result<(), MatchError> {
        let input = "lower_symbol";
        let output = tokenize(input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, input.len() - 1 );

        assert_eq!( output.item.len(), 1 );
        let name = match &output.item[0] {
            InternalToken::LowerSymbol(n) => n.clone(),
            _ => panic!("not lower symbol"),
        };

        assert_eq!( name, "lower_symbol" );

        Ok(())
    }

    #[test]
    fn should_parse_single_lower_symbol() -> Result<(), MatchError> {
        let input = "l";
        let output = tokenize(input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, input.len() - 1 );

        assert_eq!( output.item.len(), 1 );
        let name = match &output.item[0] {
            InternalToken::LowerSymbol(n) => n.clone(),
            _ => panic!("not lower symbol"),
        };

        assert_eq!( name, "l" );

        Ok(())
    }

    #[test]
    fn should_parse_upper_symbol() -> Result<(), MatchError> {
        let input = "UpperSymbol";
        let output = tokenize(input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, input.len() - 1 );

        assert_eq!( output.item.len(), 1 );
        let name = match &output.item[0] {
            InternalToken::UpperSymbol(n) => n.clone(),
            _ => panic!("not upper symbol"),
        };

        assert_eq!( name, "UpperSymbol" );

        Ok(())
    }

    #[test]
    fn should_parse_single_upper_symbol() -> Result<(), MatchError> {
        let input = "U";
        let output = tokenize(input)?;

        assert_eq!( output.start, 0 );
        assert_eq!( output.end, input.len() - 1 );

        assert_eq!( output.item.len(), 1 );
        let name = match &output.item[0] {
            InternalToken::UpperSymbol(n) => n.clone(),
            _ => panic!("not upper symbol"),
        };

        assert_eq!( name, "U" );

        Ok(())
    }
}