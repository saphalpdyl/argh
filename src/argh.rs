use std::collections::HashMap;

const SPLITTER_DEFAULT : &str = ":";

/// Parses command line arguments formatted as key-value pairs and returns a HashMap.
///
/// # Arguments
///
/// * `args` - A mutable reference to a vector of strings representing command line arguments.
/// * `splitter_overload` - Character to split with other than SPLITTER_DEFAULT.
///
/// # Returns
///
/// A HashMap with keys as string slices and values as Option<&str>.
/// If a key has a corresponding value, it is represented as Some(value).
/// If a key has no value, it is represented as None.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use argh::argh;
///
/// let mut args = vec![
///     "path/to/target".to_string(),
///     "key1:value1".to_string(),
///     "key2:value2".to_string(),
///     "key3".to_string(),
///     ];
/// let result = argh(&mut args, None);
///
/// assert_eq!(result.get("key1"), Some(&Some("value1")));
/// assert_eq!(result.get("key2"), Some(&Some("value2")));
/// assert_eq!(result.get("key3"), Some(&None));
/// ```
pub fn argh<'sp>(args: &'sp mut Vec<String>, splitter_overload: Option<&str>) -> HashMap<&'sp str, Option<&'sp str>> {
    args.remove(0); // Removing the first command_line_info
    
    let mut mapped_args : HashMap<&str, Option<&str>> = HashMap::new();
    
    for arg in &*args {
        let split_arg : Vec<&str> = arg
            .split(splitter_overload.unwrap_or(SPLITTER_DEFAULT))
            .collect();

        if let Some(value) = split_arg.get(1) {
            mapped_args.insert( split_arg[0], Some(value));
        } else {
            mapped_args.insert( split_arg[0], None);
        }
    };
    mapped_args
}
