


use stack::Stack;

mod stack;
mod queue;




fn main() {
    let mut my_stack = Stack::new();
    println!("{:?}", my_stack.pop());

    my_stack.push(32);

    println!("{:?}", my_stack.peek() );

}
