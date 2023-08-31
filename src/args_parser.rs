// This code defines a parse_args function that takes a reference to a vector
// of strings (&[String]) and returns an Option containing a tuple of
// two usize numbers if argument processing is successful.

pub fn parse_args(args: &[String]) -> Option<(usize, usize)> {
    let mut n = 0;
    let mut f = 0;

    // Enumerating command line arguments.
    for i in 1..args.len() {
        // Parsing of -N and -F arguments.
        if args[i] == "-N" && i + 1 < args.len() {
            n = args[i + 1].parse::<usize>().ok()?;
        } else if args[i] == "-F" && i + 1 < args.len() {
            f = args[i + 1].parse::<usize>().ok()?;
        }
    }

    // Check the validity of the arguments and return the result.
    if n > 0 && f > 0 {
        Some((n, f))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_valid() {
        let args = vec![
            "program_name".to_string(),
            "-N".to_string(),
            "3".to_string(),
            "-F".to_string(),
            "5".to_string(),
        ];
        let parsed = parse_args(&args);
        assert_eq!(parsed, Some((3, 5)));
    }

    #[test]
    fn test_parse_args_invalid() {
        let args = vec![
            "program_name".to_string(),
            "-N".to_string(),
            "-F".to_string(),
        ];
        let parsed = parse_args(&args);
        assert_eq!(parsed, None);
    }
}
