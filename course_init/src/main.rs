use std::io;

/// Parrot input module.
/// Validate user input and parrot it or else, display an error message. 

fn main() {

    //! ```
    //! fn main()
    //! ```
    //! Print validated user input to the console.
    //! 
    
    // let mut input = String::new();

    // println!("Say something");

    // match io::stdin().read_line(&mut input) {
    //     Ok(_) => {
    //         println!("You said {}", input);
    //     }
    //     Err(e) => {
    //         println!("Something went wrong {}", e);
    //     }
    // }

    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 50, 50, 50);
    println!("Array {:?}", ["ide", "gas"]);
}
