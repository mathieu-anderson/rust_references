fn main() {
    // Without reference
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    // We can still use s2 because the tuple gets ownership of it
    println!("The length of '{}' is {}.", s2, len);

    // With reference
    // & denotes a reference to values
    // They do not take ownership
    let s2 = String::from("Great");
    // &s12refers to the value of s2 but does not own it
    let len = calculate_length_with_ref(&s2);
    println!("The length of '{}' is {}.", s2, len);

    // Invalid code
    // let s3 = String::from("Splendid");
    // change_borrowed_value(&s3);

    // This is valid
    // Create a mutable value
    let mut s3 = String::from("Incredible");
    println!("s3 is : {}", s3);
    // Pass a mjutable reference to the fn
    change_borrowed_mut_value(&mut s3);
    println!("mutated s3 is : {}", s3);

    // Only one reference per mutable value is allowed
    // This is invalid
    // let mut s4 = String::from("What what");
    // let r5 = &mut s4;
    // let r6 = &mut s4;
    // println!("{}, {}", r5, r6);
    // It prevents data race where :
    // -> several pointers access the same data at the same time
    // -> at least one pointer is used to write the data
    // -> no synchronisation in place to access the data

    // Creating a new scope allows multiple mutable refs
    // Avoids making them simultaneous
    let mut s4 = String::from("What what");
    {
        let r1 = &mut s4;
        println!("{}", r1);
    }
    let r2 = &mut s4;
    println!("{}", r2);

    // We can't have a mut ref when immut refs exists
    let mut s5 = String::from("Hohoho");
    // You can have several immutable refs
    // Because they only read the data
    let r3 = &s5;
    let r4 = &s5;
    // This is invalid
    // let r5 = &mut s5;
    // However, the scope of a ref is only until it's last use
    // After this line, it is allowed to use a mut ref
    println!("immut ref r3 is {}, immut ref r4 is {}", r3, r4);
    let r5 = &mut s5;
    println!("mut ref r5 is {}", r5);

    // Dangling references are not compiled
    // This is invalid
    // let dref_to_nothing = dangle();

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    // We need to return the String s or it will be dropped
    // Because it leaves the scope of calculate_length
    (s, length)
}

// Arg s is a reference : it is borrowing the value of s
fn calculate_length_with_ref(s: &String) -> usize {
    // s is a ref to a String
    // It can operate on it with methods like .len()
    // And does not need to return the value to keep it in scope
    s.len()
}
// s goes out of scope but nothing happens because it has no ownership

// fn change_borrowed_value(s: &String) {
// This is invalid
// References are immutable by default
// s.push_str(" and some more stuff");
// }

fn change_borrowed_mut_value(s: &mut String) {
    // We can modify mutable references
    s.push_str(" and some more stuff");
}

// Create a dangling reference :
// A pointer that reference a location in memory freed while preserving a pointer to it
// dangle() returns a ref to a string (-> &String)
// fn dangle() -> &String {
    // s is a new String
    // let s = String::from("Hello");

    // return value is a ref to the new String
    // &s
// }
// ... but the returned value drops out of scope immediately
// The memory allocated to it is freed == danger
// dangle() returns a ref pointing to an invalid String
