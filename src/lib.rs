/// Allows you create a cycle with mutable conditions.
/// # Arguments list:
/// - var: step-variable
/// - cond: mutable condition (for example, array length)
/// - exp: expression breaking the cycle (for example, closure: |a,b| a==b or std::cmp::PartialEq::eq)
/// - code: following code
/// # Examples
/// ```
///  fn main() {
///    let mut v = vec![1i32,2,3,4,5];
///    
///    let mut i = 0;
///    while_mut!{
///        i, v.len(), |a:&usize,b:&usize| a==b,
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
    {$var: ident, $cond: expr, $exp: expr, $code: block} => {
        let mut c = $cond;
        loop {
            if $exp(&$var,&c) { break; }
            c = $cond;
            $code
        }
    }
}
