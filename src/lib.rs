#![deny(clippy::all)]

use emmet_rs::{checker::input_correctly_close, statement::Statement};

#[macro_use]
extern crate napi_derive;

#[napi]
fn expand(input:String) -> Option<String>{
  if !input_correctly_close(&input){
    return None
  }
  Some(Statement::new(&input, 0).parse())
}