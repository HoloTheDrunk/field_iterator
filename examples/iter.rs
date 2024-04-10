use field_iterator::Iterable;

#[derive(Iterable, Default)]
struct MyStruct {
    field: i32,
    another: u32,
    yaf: f32,
}

fn main() {
    for (name, value) in MyStruct::default().iter() {
        println!("{}: {:?}", name, value);
    }
}
