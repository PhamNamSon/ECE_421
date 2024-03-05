/*
* The code will not run because of 2 reasons:
* 1. The data field is a string slice, which is a reference to a string. Rust's 
*    borrowing rules require that there must be a defined lifetime for the reference.
* 2. The left_child and right_child fields are trying to directly store an instance of
*    the TreeNode struct, which leads to a recursive type definition. Rust needs to know
*    the size of the type at compile time, and this is not possible with recursive types.
*/

#[derive(Debug)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}

fn main() {
}
