use std::collections::HashMap;

pub fn argh(args: &mut Vec<String>) -> HashMap<&str, Option<&str>> {
    args.remove(0); // Removing the first command_line_info
    
    let mut mapped_args : HashMap<&str, Option<&str>> = HashMap::new();
    
    for arg in &*args {
        let split_arg : Vec<&str> = arg.split(":").collect();

        if let Some(value) = split_arg.get(1) {
            mapped_args.insert( split_arg[0], Some(value));
        } else {
            mapped_args.insert( split_arg[0], None);
        }
    };
    mapped_args
}
