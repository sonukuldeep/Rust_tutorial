fn main() {
    let array: [u32; 3] = [1,2,3]; 

    let first_element = array[0];

    // let error = array[100];

    println!("first element: {}", first_element);

    let tuple: (u32, u8, bool) = (5, 2, true);

    let first_element = tuple.0;
    println!("first element: {}", first_element);

}
