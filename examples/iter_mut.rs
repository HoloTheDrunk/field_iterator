use field_iterator::Iterable;

#[derive(Iterable, Default)]
struct MyStruct {
    first: usize,
    second: usize,
    third: usize,
}

fn main() {
    for (i, (name, value)) in MyStruct::default().iter_mut().enumerate() {
        let cast = value.downcast_mut::<usize>().unwrap();
        *cast = i + 1;
        println!("{name}: {cast}");
    }
}
