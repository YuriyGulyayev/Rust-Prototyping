// #![allow(dead_code)]

#[macro_use]
pub mod macros;

/*crate::macros::*/configure_lints!();


/// Test comment.
/// [Comment-202309115]abc[/Comment-202309115]
/// <Comment202309115>abc</Comment202309115>
/// [ToDo202309116-1]xyz[/ToDo202309116-1]
pub fn add(left: usize, right: usize) -> usize
{
    // let c = 'a';
    let i: u32;
    i = 5;

    println!("From lib. {}", i);
    // my_module::function_1();
    // function_1();

    left + right
}

fn get_five() -> u32 {
    // 5                            // no semi-colon

    return 5;
}

fn fun1()
{
    // let name = format!("{} {}", get_first_name(), get_last_name());

    // let image_data = include_bytes!("./image.png");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
