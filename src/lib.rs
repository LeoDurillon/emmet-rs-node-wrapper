#![deny(clippy::all)]

use emmet_rs::{checker::{input_correctly_close,input_correctly_structured}, statement::Statement};

#[macro_use]
extern crate napi_derive;

#[napi]
fn expand(input:String) -> Option<String>{
  let trimmed_input = input.trim().split(' ').collect::<Vec<&str>>();
  let mut input_start= 0; 
  for (i,str) in trimmed_input.iter().enumerate() {
    if i == trimmed_input.len()-1 ||  str.chars().any(|x| x == '+' || x == '>' || x == '<' ) {
      break;
    }
    input_start += 1;
  }
  let mut before_input= trimmed_input[..input_start].join(" ");
  let input = trimmed_input[input_start..].join(" ");
  
  if !input_correctly_close(&input) || !input_correctly_structured(&input){
    return None
  }
  before_input = format!("{}{}",before_input,if before_input.len() > 0 { " " } else{ "" }); 

  Some(format!("{}{}",before_input,Statement::new(&input, 0).parse()))
}

#[cfg(test)]
mod test_expand {
    use crate::expand;

  #[test]
  fn should_expand(){
    assert_eq!(Some("<div></div>".to_string()),expand("div".to_string()));
  }

  #[test]
  fn should_ignore_before(){
    assert_eq!(Some("return <div></div>".to_string()),expand("return div".to_string()));
    assert_eq!(Some("return <div> test</div>".to_string()),expand("return div< test".to_string()));
  }


}