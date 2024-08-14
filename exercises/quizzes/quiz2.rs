// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

impl Command {
    fn Uppercase(&self, s: String) -> String {
        s.to_uppercase()
    }

    fn Trim(&self, s: String) -> String {
        s.trim().to_string()
    }

    fn Append(&self, s: String, num: usize) -> String {
        // close the string to be appended
        let mut result = s.clone();

        // for the number passed in, append bar to it and return result
        for _ in 0..num {
            result.push_str("bar");
        }
        result
    }
}

mod my_module {
    use super::Command;

    // TODO: Complete the function.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> { 
        // make our vector to store the results
        let mut result = Vec::new();

        // for each tuple in the input, match the command and push on the string with the command applied
        for (string, command) in input {
            match command {
                Command::Uppercase => result.push(command.Uppercase(string)),
                Command::Trim => result.push(command.Trim(string)),
                Command::Append(num) => result.push(command.Append(string, num)),
            }
        }
        
        // return the result
        result
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
