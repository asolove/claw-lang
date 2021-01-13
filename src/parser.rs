
use codespan_reporting::files::SimpleFile;

use crate::lexer::{ Token, TokenData };
use crate::ast::{ M, Module };

struct ParserData {
    file: SimpleFile<String, String>,
    module: Module
}

fn parse(token_data: TokenData) -> Option<ParserData> {
    None
}