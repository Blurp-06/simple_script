use crate::variables::{VariableContent, VariableTypes};

// Math functions.
pub fn simple_add(args: Vec<VariableContent>) -> VariableContent {
    if args.len() < 2 {
        panic!(
            "Length of args was {}; there should atleast be 2 arguments.",
            args.len()
        );
    }

    // Check if all types are supported.
    {
        let mut tmp_args: Vec<VariableContent> = vec![
            VariableContent {
                value: "".to_string(),
                data_type: VariableTypes::NULL
            };
            args.len()
        ];
        tmp_args.clone_from_slice(&args[..]);
        for arg in tmp_args {
            match arg.data_type {
                VariableTypes::FLOAT => {}
                VariableTypes::INT => {}
                _ => panic!("Argument can't be of type '{:?}'.", arg.data_type),
            }
        }
    }

    let mut arg_iter = args.into_iter();
    let first = arg_iter.next().unwrap();
    let mut sum = first.value.parse::<f64>().unwrap();

    for arg in arg_iter {
        sum += arg.value.parse::<f64>().unwrap();
    }

    VariableContent {
        value: sum.to_string(),
        data_type: VariableTypes::FLOAT,
    }
}

pub fn simple_sub(args: Vec<VariableContent>) -> VariableContent {
    if args.len() < 2 {
        panic!(
            "Length of args was {}; there should atleast be 2 arguments.",
            args.len()
        );
    }

    // Check if all types are supported.
    {
        let mut tmp_args: Vec<VariableContent> = vec![
            VariableContent {
                value: "".to_string(),
                data_type: VariableTypes::NULL
            };
            args.len()
        ];
        tmp_args.clone_from_slice(&args[..]);
        for arg in tmp_args {
            match arg.data_type {
                VariableTypes::FLOAT => {}
                VariableTypes::INT => {}
                _ => panic!("Argument can't be of type '{:?}'.", arg.data_type),
            }
        }
    }

    let mut arg_iter = args.into_iter();
    let first = arg_iter.next().unwrap();
    let mut sum = first.value.parse::<f64>().unwrap();

    for arg in arg_iter {
        sum -= arg.value.parse::<f64>().unwrap();
    }

    VariableContent {
        value: sum.to_string(),
        data_type: VariableTypes::FLOAT,
    }
}
