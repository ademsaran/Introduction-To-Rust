fn main() {
    
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // Read the third element through indexing
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); // Read the third element through the `get` method -> Returns an Option<&T>
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }

        for i in &v {
            println!("{i}");
        }
    }

    // Using an enum to store different types in a vector
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        for i in &row {
            match i {
                SpreadsheetCell::Int(value) => println!("{value}"),
                SpreadsheetCell::Float(value) => println!("{value}"),
                SpreadsheetCell::Text(value) => println!("{value}"),
            }
        }
    }

    // Strings
    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");

         
        let mut s = String::from("lo");
        s.push('l');
        println!("{s}");

        { // Slicing strings
            let hello = "Здравствуйте";

            let s = &hello[0..4];
            println!("{s}");

            for c in hello.chars() { // Iterating over string characters
                println!("{c}");
            }
        }
    }

    // Hash maps
    {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("Score for {team_name}: {score}");

        //Overwriting a value in a hash map
        scores.insert(String::from("Red"), 10);
        scores.insert(String::from("Red"), 25);

        // Inserting a value if the key does not exist
        scores.entry(String::from("Green")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{scores:?}");

        for (key, value) in &scores {
            println!("{key}: {value}");
        }

            let text = "hello world wonderful world";

        // Counting the occurrences of each word in a string using a hash map
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{map:?}");
    }
}
