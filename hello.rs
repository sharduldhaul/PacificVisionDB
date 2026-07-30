#[derive(Debug)]
struct Printable(i32);

fn main(){
    println!("This is the debuggin portion, right here => {:?}", Printable(33));
}