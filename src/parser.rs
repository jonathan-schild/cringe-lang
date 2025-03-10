/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use std::{collections::VecDeque, io::BufRead};

use ast::AstRoot;
use lexer::Lexer;
use tokens::Token;

use crate::Error;

pub mod ast;
mod lexer;
pub mod tokens;

pub struct Parser<R: BufRead> {
    lexer: Lexer<R>,
    token_buffer: VecDeque<Token>,
    root: AstRoot,
}

impl<R: BufRead> Parser<R> {
    pub fn parse(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn parse_scope(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn parse_struct(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn parse_function(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn parse_member(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn parse_type(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn parse_identifier(&mut self) -> Result<(), Error> {
        todo!()
    }

    fn parse_literal(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_body(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_statement(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_compound(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_if(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_loop(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_for(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_return(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_break(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_continue(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_var_decl(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_val_decl(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_assing(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_expression(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_logical_or(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_logical_and(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_binary_or(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_binary_xor(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_binary_and(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_equal_rel(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_ord_rel(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_bit_op(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_sub_add(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_mul_div(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_unary_op(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_access_op(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_atom(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn parse_initialisation(&mut self) -> Result<(), Error> {
        todo!()
    }
}
