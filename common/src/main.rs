fn main() {
    println!("Hello, again!");

    // println!("{}", Register::An(4).to_string());
}

// enum Register {
//     PC,
//     SP,
//     An(u8),
//     Dn(u8),
// }

// impl std::string::ToString for Register {
//     fn to_string(&self) -> String {
//         match self {
//             Register::PC => String::from("PC"),
//             Register::SP => String::from("SP"),
//             Register::An(n) => format!("A{}", n),
//             Register::Dn(n) => format!("D{}", n),
//         }
//     }
// }
