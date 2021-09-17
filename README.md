# Macro while_mut!
This crate defines macro that allows you use while with mutable conditions, such as collection's length. To use that just type:
```
while_mut!{
    $condition,
    {
        $code
    }
```

For example,
```
let mut i=0usize;
let mut v = (1..10).collect::<Vec<i32>>();

while_mut!{
    i<v.len(),
    {
        v.remove(0);
        i+=1;
        println!("{} {}",i,v.len());
    }
}

println!("Done");
```

## License
Licensed under MIT.
