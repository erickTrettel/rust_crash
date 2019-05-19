use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // re-assign value
    numbers[2] = 20;

    // add on to vector
    numbers.push(5);
    numbers.push(6);

    // pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // get single val
    println!("Single val: {}", numbers[0]);

    // get vector len
    println!("vector len: {}", numbers.len());

    // vectors are stack allocated
    println!("vec occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers vec: {:?}", numbers);
}