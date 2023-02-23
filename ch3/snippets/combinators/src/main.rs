

fn main() {
   
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};

    #[test]
    fn vector() {
        let v = vec![1, 2, 3];

        for x in v {
            println!("{}", x);
        }
    }

    #[test]
    fn hashmap() {
        let mut h = HashMap::new();
        h.insert(String::from("Hello"), String::from("World"));

        for (key, value) in h.iter() {
            println!("{}: {}", key, value);
        }
    }

    
}
