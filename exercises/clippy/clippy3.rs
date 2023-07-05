// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(x) = my_option {
        my_option.unwrap()
    };

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let  value_a = 45;
    let  value_b = 66;
    // Let's swap these two!
    let  temp = value_b;
    let value_b = value_a;
    let value_a = temp;
    println!("value a: {}; value b: {}", value_a, value_b);
}
