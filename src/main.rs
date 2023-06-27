// THEORY 1: references
// We use references in Rust to get around ownership transference.
// A reference is essentially a pointer to an address that contains data we need. 
// The data that a reference points to is owned by another variable. 
// But unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the duration of its life.
// References allow functions to use values without taking ownership of the variable.
// References stop being valid when they go out of scope. 
// This act is called "borrowing"!  

fn main() {
     let s1 = String::from("Hello world!"); 

     let len = calculate_length(&s1); //Reference of s1, symbolised by an ampersand "&". It does not own s1. 

     println!("The length of '{s1}' is {len}."); 

     //To theory 2
     mutable_reference();
}

// &String makes sure that we know that we're using a reference
fn calculate_length(s: &String) -> usize {
    s.len()
    //The reference (s) goes out of scope and is dropped. 
}

// Theory 2: mutable references
// We run into issues if we want to mutate reference's data. By default, we cannot. The act results in an error.
// The biggest limitation is that using a mutable reference means that you can't have more than one reference to that value. 
// making two mutable references would make this app crash!
// The advantage of this design is that it prevents "data races" from the very start. 
// A data race is defined by these three conditions: 1. Two or more pointers access the same data - 2. At least one of the pointers is being used to write to the data - 3. There is no mechanism being used to sunchronize access to the data. 
// This is enforced across all references. Having two normal references is fine, but having two normal references and one mutable reference simultaneously causes a crash.
 
fn mutable_reference() {
    let mut s2 = String::from("Hello world again!"); // Create a mutable reference! 

    change(&mut s2); // Specify a mutable reference

    println!("{s2}");

    preventative_scopes(); // Theory 3
}

fn change(s: &mut String) {
    s.push_str(" but I'm changed!");
}

// Theory 3
// As always, we can use scopes to get around this. We are allowed to have multiple mutable references across multiple scopes, just not simultaneous ones.

fn preventative_scopes() {

    let mut s3 = String::from("Hello world again again!"); 
    {
        let _r1 = &mut s3;
    }   //Reference r1 goes out of scope -- it's dropped. We can make another mutable reference without issues.

    let r2 = &mut s3; 

    println!(" We used two mutable references r1 and r2. r2 works: {r2}"); 

    preventative_scopes_2();
}

// Note that reference's scope starts from where it is introduced and continues through the last time the reference is used
fn preventative_scopes_2() {
    let mut s4 = String::from ("Hello world again again again!");

    let r4 = &s4; //No problem
    let r5 = &s4; //No problem
    println!("{r4} and {r5} will never be used again!");
    //r4 and r5 won't be used after this. THe scopes of r4 and r5 end here!

    let r6 = &mut s4; //No problem
    change(r6);
    println!("{r6} and that's fine!");

    dangling_references();
}

// Theory 4: dangling references 
// In many languages that allow references, it can be possible to make dangling references, which are references that point at a space given to something else. 
// In Rust, these aren't allowed. The compiler makes it impossible by giving us the "Missing lifetime specifier" error

fn dangling_references() {
    let no_content = dangler();

    println!("{no_content}");
}

fn dangler() -> String { //&String would fail
    let dangle = String::from("Dangler!");

    //&dangle would crash the program.
    dangle //works because ownership is moved out
}