/// Allows you create a cycle with mutable conditions.
/// # Arguments list:
/// - cond: mutable condition (for example, `i!=array.len()`)
/// - code: the following code
/// # Examples
/// ```
///  fn main() {
///    let mut v = vec![1i32,2,3,4,5];
///    
///    let mut i = 0;
///    while_mut!{
///        i<v.len(),
///        {
///            v.remove(0);
///            println!("{} {} {}", v[0], v.len(), i);
///            i += 1;
///        }
///    }
/// }
/// ```
#[macro_export]
macro_rules! while_mut{
    {$cond: expr, $code: block} => {
        loop {
            if !($cond) { break; }
            $code
        }
    }
}
