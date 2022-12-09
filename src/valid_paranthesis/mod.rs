use std::collections::HashMap;

struct Stack<T> {
    stack: Vec<T>,
  }
  
  impl<T> Stack<T> {
    fn new() -> Self {
      Stack { stack: Vec::new() }
    }
  
    fn length(&self) -> usize {
      self.stack.len()
    }
  
    fn pop(&mut self) -> Option<T> {
      self.stack.pop()
    }
  
    fn push(&mut self, item: T) {
      self.stack.push(item)
    }
  
    fn is_empty(&self) -> bool {
      self.stack.is_empty()
    }
  
    fn peek(&self) -> Option<&T> {
      self.stack.last()
    }
  }

pub fn is_valid(s: String) -> bool {
    let mut paranthesis:HashMap<char,char> = HashMap::new();
    paranthesis.insert('{', '}');
    paranthesis.insert('[', ']');
    paranthesis.insert('(', ')');
    let mut stack: Stack<char> = Stack::new();
    let c = s.chars();
    for i in c{
        if paranthesis.contains_key(&i){
            stack.push(i);
        }
        else{
            if stack.is_empty(){
                return false;
            }
            let top = stack.pop().unwrap();
            if paranthesis.get(&top).unwrap() != &i{
                return false;
            }
        }
    }
    if stack.is_empty(){
        return true;
    }
    false
}