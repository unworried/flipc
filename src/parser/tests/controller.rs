use crate::{lexer::{Lexer, Token}, parser::Parser};

#[test]
fn parser_step() {
    let input = "1 2 3 4";
    let mut lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(&mut lex);
    assert_eq!(parser.current_token, Token::Int(1));
    assert_eq!(parser.next_token, Token::Int(2));
    parser.step();
    assert_eq!(parser.current_token, Token::Int(2));
    assert_eq!(parser.next_token, Token::Int(3));
    parser.step();
    assert_eq!(parser.current_token, Token::Int(3));
    assert_eq!(parser.next_token, Token::Int(4));
    parser.step();
    assert_eq!(parser.current_token, Token::Int(4));
    assert_eq!(parser.next_token, Token::Eof);
}

#[test]
fn parser_eat() {
    let input = "1 2 3 4";
    let mut lex = Lexer::new(input.to_string());
    let mut parser = Parser::new(&mut lex);
    assert_eq!(parser.current_token, Token::Int(1));
    assert_eq!(parser.eat(), Token::Int(1));
    assert_eq!(parser.current_token, Token::Int(2));
    assert_eq!(parser.eat(), Token::Int(2));
    assert_eq!(parser.current_token, Token::Int(3));
    assert_eq!(parser.eat(), Token::Int(3));
    assert_eq!(parser.current_token, Token::Int(4));
    assert_eq!(parser.eat(), Token::Int(4));
    assert_eq!(parser.current_token, Token::Eof);
}

#[test]
fn parser_comp_current_token() {
    let input = "1";
    let mut lex = Lexer::new(input.to_string());
    let parser = Parser::new(&mut lex);
    assert!(parser.current_token(&Token::Int(1)));
}

#[test]
fn parser_comp_next_token() {
    let input = "1";
    let mut lex = Lexer::new(input.to_string());
    let parser = Parser::new(&mut lex);
    assert!(parser.next_token(Token::Eof));
}