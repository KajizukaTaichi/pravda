use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let memory: &mut HashMap<String, Type> = &mut HashMap::from([
        (
            "+".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result += i;
                }
                Type::Number(result)
            })),
        ),
        (
            "-".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                if params.len() == 1 {
                    Type::Number(-params[0])
                } else {
                    let mut result: f64 = if let Some(result) = params.get(0) {
                        result.to_owned()
                    } else {
                        return Type::Null;
                    };
                    for i in params[1..params.len()].to_vec().iter() {
                        result -= i;
                    }
                    Type::Number(result)
                }
            })),
        ),
        (
            "*".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result *= i;
                }
                Type::Number(result)
            })),
        ),
        (
            "/".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result /= i;
                }
                Type::Number(result)
            })),
        ),
        (
            "%".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result %= i;
                }
                Type::Number(result)
            })),
        ),
        (
            "^".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();
                let mut result: f64 = if let Some(result) = params.get(0) {
                    result.to_owned()
                } else {
                    return Type::Null;
                };
                for i in params[1..params.len()].to_vec().iter() {
                    result = result.powf(*i);
                }
                Type::Number(result)
            })),
        ),
        (
            "equal".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::Bool({
                    let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                    params.windows(2).all(|window| window[0] == window[1])
                })
            })),
        ),
        (
            "or".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::Bool({
                    let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                    params.iter().any(|&x| x)
                })
            })),
        ),
        (
            "and".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::Bool({
                    let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                    params.iter().all(|&x| x)
                })
            })),
        ),
        (
            "concat".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                let params: Vec<String> = params.iter().map(|i| i.get_string()).collect();
                Type::String(params.join(""))
            })),
        ),
        (
            "repeat".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::String(
                    if let Some(count) = params.get(0) {
                        count.get_string()
                    } else {
                        return Type::Null;
                    }
                    .repeat(if let Some(count) = params.get(1) {
                        count.get_number() as usize
                    } else {
                        return Type::Null;
                    }),
                )
            })),
        ),
        (
            "split".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::List({
                    let text = if let Some(count) = params.get(0) {
                        count.get_string().to_owned()
                    } else {
                        return Type::Null;
                    };
                    let key = if let Some(count) = params.get(1) {
                        count.get_string()
                    } else {
                        return Type::Null;
                    };

                    text.split(&key)
                        .into_iter()
                        .map(|i| Type::String(i.to_string()))
                        .collect()
                })
            })),
        ),
        (
            "join".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::String({
                    let list: Vec<String> = if let Some(count) = params.get(0) {
                        count.get_list().iter().map(|i| i.get_string()).collect()
                    } else {
                        return Type::Null;
                    };
                    let key = if let Some(count) = params.get(1) {
                        count.get_string()
                    } else {
                        return Type::Null;
                    };

                    list.join(&key)
                })
            })),
        ),
        (
            "input".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                Type::String(input(&if let Some(prompt) = params.get(0) {
                    prompt.get_string()
                } else {
                    "".to_string()
                }))
            })),
        ),
        (
            "print".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                println!(
                    "{}",
                    if let Some(count) = params.get(0) {
                        count.get_string()
                    } else {
                        "".to_string()
                    }
                );
                Type::Null
            })),
        ),
        (
            "read-file".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if let Some(path) = params.get(0) {
                    let path = path.get_string(); // Create a binding to extend the lifetime
                    if let Ok(data) = read_to_string(Path::new(&path)) {
                        Type::String(data)
                    } else {
                        Type::Null
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "list".to_string(),
            Type::Function(Function::BuiltIn(|params, _| Type::List(params))),
        ),
        (
            "car".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if let Some(list) = params.get(0) {
                    if let Some(car) = list.get_list().get(0) {
                        car.clone()
                    } else {
                        Type::Null
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "cdr".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if let Some(list) = params.get(0) {
                    if list.get_list().len() >= 2 {
                        Type::List(list.get_list()[1..list.get_list().len()].to_vec())
                    } else {
                        Type::Null
                    }
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "len".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if let Some(Type::List(list)) = params.get(0) {
                    Type::Number(list.len() as f64)
                } else if let Some(Type::String(string)) = params.get(0) {
                    Type::Number(string.chars().count() as f64)
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "range".to_string(),
            Type::Function(Function::BuiltIn(|params, _| {
                if params.len() == 1 {
                    let mut range: Vec<Type> = vec![];
                    let mut current: f64 = 0.0;
                    while current < params[0].get_number() {
                        range.push(Type::Number(current));
                        current += 1.0;
                    }
                    Type::List(range)
                } else if params.len() == 2 {
                    let mut range: Vec<Type> = vec![];
                    let mut current: f64 = params[0].get_number();
                    while current < params[1].get_number() {
                        range.push(Type::Number(current));
                        current += 1.0;
                    }
                    Type::List(range)
                } else if params.len() >= 3 {
                    let mut range: Vec<Type> = vec![];
                    let mut current: f64 = params[0].get_number();
                    while current < params[1].get_number() {
                        range.push(Type::Number(current));
                        current += params[2].get_number();
                    }
                    Type::List(range)
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "map".to_string(),
            Type::Function(Function::BuiltIn(|params, memory| {
                if params.len() >= 2 {
                    let func = if let Type::Function(func) = params[1].clone() {
                        func
                    } else {
                        return Type::Null;
                    };
                    let mut memory = memory.clone();
                    Type::List(
                        params[0]
                            .get_list()
                            .iter()
                            .map(|i| call_function(func.clone(), vec![i.clone()], &mut memory))
                            .collect(),
                    )
                } else {
                    Type::Null
                }
            })),
        ),
        (
            "filter".to_string(),
            Type::Function(Function::BuiltIn(|params, memory| {
                if params.len() >= 2 {
                    let func = if let Type::Function(func) = params[1].clone() {
                        func
                    } else {
                        return Type::Null;
                    };
                    let mut memory = memory.clone();
                    let mut result = Vec::new();

                    for item in params[0].get_list() {
                        if call_function(func.clone(), vec![item.clone()], &mut memory).get_bool() {
                            result.push(item);
                        }
                    }
                    Type::List(result)
                } else {
                    Type::Null
                }
            })),
        ),
    ]);

    let args: Vec<String> = args().collect();
    if args.len() >= 2 {
        if let Ok(code) = read_to_string(Path::new(&args[1])) {
            run(code, memory);
        } else {
            eprintln!("Error! it fault to open the script file")
        }
    } else {
        println!("Pravda 0.6.1");
        loop {
            let mut code = String::new();
            loop {
                let enter = input("> ").trim().to_string();
                if enter.is_empty() {
                    break;
                }
                code += &format!("{enter} ");
            }

            if !code.is_empty() {
                println!("{}", run(code, memory).get_symbol());
            }
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

fn search_vec(
    vec: Vec<(Vec<Type>, (String, HashMap<String, Type>))>,
    target: Vec<Type>,
) -> Option<(String, HashMap<String, Type>)> {
    let mut temp = None;
    for item in vec {
        if item
            .0
            .iter()
            .map(|i| i.get_symbol())
            .collect::<Vec<String>>()
            .join("\n")
            == target
                .iter()
                .map(|i| i.get_symbol())
                .collect::<Vec<String>>()
                .join("\n")
        {
            temp = Some(item.1);
        }
    }
    temp
}

#[derive(Clone, Debug)]
enum Type {
    Code(String),
    Block(String),
    Symbol(String),
    Function(Function),
    Number(f64),
    String(String),
    Bool(bool),
    List(Vec<Type>),
    Null,
}

impl Type {
    fn parse(source: String) -> Type {
        let mut source = source.trim().to_string();
        if let Ok(value) = source.parse::<f64>() {
            Type::Number(value)
        } else if let Ok(value) = source.parse::<bool>() {
            Type::Bool(value)
        } else if source == "null" {
            Type::Null
        } else if source.starts_with('"') && source.starts_with('"') {
            Type::String({
                source.remove(source.find('"').unwrap_or_default());
                source.remove(source.rfind('"').unwrap_or_default());
                source.to_string()
            })
        } else if source.starts_with("lambda(") && source.ends_with(")") && source.contains("->") {
            source = source.replacen("lambda(", "", 1);
            source.remove(source.rfind(")").unwrap_or_default());
            let define: Vec<&str> = source.split("->").collect();
            Type::Function(Function::UserDefined(vec![(
                tokenize(define[0].to_string())
                    .iter()
                    .map(|i| Type::parse(i.to_string()))
                    .collect(),
                (
                    define[1..define.len()].join("->").to_string(),
                    HashMap::new(),
                ),
            )]))
        } else if source.starts_with("(") && source.ends_with(")") {
            Type::Code({
                source.remove(source.find("(").unwrap_or_default());
                source.remove(source.rfind(")").unwrap_or_default());
                source
            })
        } else if source.starts_with("{") && source.ends_with("}") {
            Type::Block({
                source.remove(source.find("{").unwrap_or_default());
                source.remove(source.rfind("}").unwrap_or_default());
                source
            })
        } else if source.starts_with("[") && source.ends_with("]") {
            Type::List({
                source.remove(source.find("[").unwrap_or_default());
                source.remove(source.rfind("]").unwrap_or_default());
                tokenize(source)
                    .into_iter()
                    .map(|item| Type::parse(item.to_string()))
                    .collect()
            })
        } else {
            Type::Symbol(source.to_string())
        }
    }

    fn get_number(&self) -> f64 {
        match self {
            Type::Number(value) => *value,
            Type::String(value) | Type::Symbol(value) => value.parse().unwrap_or_default(),
            Type::Bool(value) => {
                if *value {
                    1.0
                } else {
                    0.0
                }
            }
            Type::List(value) => value.get(0).unwrap_or(&Type::Null).get_number(),
            Type::Null => 0.0,
            Type::Function(Function::UserDefined(value)) => value.len() as f64,
            Type::Function(Function::BuiltIn(_)) => 0.0,
            Type::Code(value) | Type::Block(value) => value.len() as f64,
        }
    }

    fn get_string(&self) -> String {
        match self {
            Type::Number(value) => value.to_string(),
            Type::String(value) | Type::Symbol(value) => value.to_string(),
            Type::Bool(value) => value.to_string(),
            Type::Code(value) => format!("({})", value),
            Type::List(value) => format!(
                "[{}]",
                value
                    .iter()
                    .map(|i| i.get_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Null => "null".to_string(),
            Type::Function(Function::BuiltIn(function)) => {
                format!("<Built-in function: {:?}>", function)
            }
            Type::Function(Function::UserDefined(_)) => "<User-defined function>".to_string(),
            Type::Block(value) => format!("{{ {} }}", value),
        }
    }

    fn get_symbol(&self) -> String {
        match self {
            Type::Number(value) => value.to_string(),
            Type::String(value) => format!("\"{}\"", value),
            Type::Symbol(value) => value.to_string(),
            Type::Bool(value) => value.to_string(),
            Type::Code(value) => format!("({})", value),
            Type::List(value) => format!(
                "[{}]",
                value
                    .iter()
                    .map(|i| i.get_symbol())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Type::Null => "null".to_string(),
            Type::Function(Function::BuiltIn(function)) => {
                format!("<Built-in function: {:?}>", function)
            }
            Type::Function(Function::UserDefined(_)) => "<User-defined function>".to_string(),
            Type::Block(value) => format!("{{ {} }}", value),
        }
    }

    fn get_bool(&self) -> bool {
        match self {
            Type::Number(value) => *value != 0.0,
            Type::String(value) | Type::Symbol(value) => value.parse().unwrap_or_default(),
            Type::Bool(value) => *value,
            Type::List(value) => value.get(0).unwrap_or(&Type::Null).get_bool(),
            Type::Null => false,
            Type::Function(_) => true,
            Type::Code(value) | Type::Block(value) => !value.is_empty(),
        }
    }

    fn get_list(&self) -> Vec<Type> {
        match self {
            Type::List(value) => value.to_owned(),
            Type::String(value) => value
                .chars()
                .into_iter()
                .map(|c| Type::String(c.to_string()))
                .collect(),
            other => vec![other.to_owned()],
        }
    }
}

#[derive(Clone, Debug)]
enum Function {
    BuiltIn(fn(Vec<Type>, HashMap<String, Type>) -> Type),
    UserDefined(Vec<(Vec<Type>, (String, HashMap<String, Type>))>),
}

fn run(source: String, memory: &mut HashMap<String, Type>) -> Type {
    let source = tokenize_program(source);
    let mut result = Type::Null;
    for lines in source {
        if lines.len() == 2 {
            let define = tokenize(lines[0].to_string());
            if define.len() > 1 {
                if let Some(Type::Function(Function::UserDefined(exist))) = memory.get(&define[0]) {
                    let mut exist = exist.clone();
                    let args: Vec<Type> = define[1..define.len()]
                        .to_vec()
                        .iter()
                        .map(|i| Type::parse(i.to_string()))
                        .collect();
                    if exist[0].0.len() == args.len() {
                        exist.push((args, (lines[1].clone(), memory.to_owned())));
                        let object = Type::Function(Function::UserDefined(exist));
                        result = object.clone();
                        memory.insert(define[0].to_string(), object);
                    } else {
                        eprintln!("Error! the function arguments length should be immutable");
                    }
                } else {
                    let object = Type::Function(Function::UserDefined(vec![(
                        define[1..define.len()]
                            .to_vec()
                            .iter()
                            .map(|i| Type::parse(i.to_string()))
                            .collect(),
                        (
                            lines[1..lines.len()].to_vec().join(" = "),
                            memory.to_owned(),
                        ),
                    )]));
                    result = object.clone();
                    memory.insert(define[0].to_string(), object);
                }
            } else {
                result = eval(lines[1..lines.len()].to_vec().join(" = "), memory);
                memory.insert(define[0].to_string(), result.clone());
            }
        } else {
            result = eval(lines[0].to_string(), memory);
        }
    }
    result
}

fn tokenize_program(input: String) -> Vec<Vec<String>> {
    let mut tokens: Vec<Vec<String>> = Vec::new();
    let mut current_token = String::new();
    let mut after_equal = String::new();
    let mut is_equal = false;
    let mut in_parentheses: usize = 0;

    for c in input.chars() {
        match c {
            '{' => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses += 1;
            }
            '}' => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
                in_parentheses -= 1;
            }
            ';' => {
                if in_parentheses != 0 {
                    if is_equal {
                        after_equal.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else {
                    if !current_token.is_empty() {
                        if is_equal {
                            is_equal = false;
                            tokens.push(vec![current_token.clone(), after_equal.clone()]);
                            current_token.clear();
                            after_equal.clear();
                        } else {
                            tokens.push(vec![current_token.clone()]);
                            current_token.clear();
                        }
                    }
                }
            }
            '=' => {
                if in_parentheses != 0 {
                    if is_equal {
                        after_equal.push(c);
                    } else {
                        current_token.push(c);
                    }
                } else {
                    is_equal = true;
                }
            }
            _ => {
                if is_equal {
                    after_equal.push(c);
                } else {
                    current_token.push(c);
                }
            }
        }
    }

    if in_parentheses == 0 && !current_token.is_empty() {
        if is_equal {
            tokens.push(vec![current_token.clone(), after_equal]);
            current_token.clear();
        } else {
            tokens.push(vec![current_token.clone()]);
            current_token.clear();
        }
    }
    tokens
}

fn eval(programs: String, memory: &mut HashMap<String, Type>) -> Type {
    let programs: Vec<Type> = tokenize(programs)
        .iter()
        .map(|i| Type::parse(i.to_owned()))
        .collect();
    if programs.is_empty() {
        return Type::Null;
    }

    if let Type::Symbol(identify) = programs[0].clone() {
        if let Some(value) = memory.get(&identify) {
            if let Type::Function(name) = value {
                call_function(
                    name.to_owned(),
                    programs[1..programs.len()].to_vec(),
                    memory,
                )
            } else {
                value.to_owned()
            }
        } else {
            programs[0].clone()
        }
    } else if let Type::Function(liberal) = &programs[0] {
        call_function(
            liberal.clone(),
            programs[1..programs.len()].to_vec(),
            memory,
        )
    } else if let Type::Block(block) = &programs[0] {
        let result = run(block.to_owned(), &mut memory.clone());
        if let Type::Function(func) = result {
            call_function(
                func,
                programs[1..programs.len()].to_vec(),
                &mut memory.clone(),
            )
        } else {
            result
        }
    } else if let Type::Code(code) = &programs[0] {
        let result = eval(code.to_owned(), &mut memory.clone());
        if let Type::Function(func) = result {
            call_function(
                func,
                programs[1..programs.len()].to_vec(),
                &mut memory.clone(),
            )
        } else {
            result
        }
    } else {
        if programs.len() == 1 {
            programs[0].to_owned()
        } else {
            Type::List(programs)
        }
    }
}

fn call_function(function: Function, args: Vec<Type>, memory: &mut HashMap<String, Type>) -> Type {
    let mut params: Vec<Type> = vec![];
    for i in args {
        if let Type::Code(code) = i.clone() {
            params.push(eval(code, &mut memory.clone()))
        } else if let Type::Block(block) = i.clone() {
            params.push(run(block, &mut memory.clone()))
        } else if let Type::Symbol(name) = i.clone() {
            if name.starts_with("~") {
                let name = name[1..name.len()].to_string();
                if let Some(value) = memory.get(&name) {
                    for j in value.get_list() {
                        params.push(j.to_owned())
                    }
                } else if let Type::List(list) = Type::parse(name.clone()) {
                    for j in list {
                        params.push(j.to_owned())
                    }
                } else if let Type::Code(code) = Type::parse(name.clone()) {
                    let result = eval(code, memory);
                    for j in result.get_list() {
                        params.push(j.to_owned())
                    }
                } else if let Type::Block(code) = Type::parse(name.clone()) {
                    let result = run(code, memory);
                    for j in result.get_list() {
                        params.push(j.to_owned())
                    }
                } else {
                    params.push(Type::parse(name))
                }
            } else {
                if let Some(value) = memory.get(&name) {
                    params.push(value.to_owned())
                } else {
                    params.push(i.to_owned())
                }
            }
        } else {
            params.push(i.to_owned());
        }
    }

    if let Function::BuiltIn(function) = function {
        function(params, memory.to_owned())
    } else if let Function::UserDefined(object) = function {
        if let Some((program, scope)) = search_vec(object.clone(), params.clone()) {
            let mut scope = scope.clone();
            eval(program.to_string(), &mut scope)
        } else {
            if let Some((args, (program, scope))) = {
                let mut flag = None;
                for item in {
                    let mut object = object.clone();
                    object.reverse();
                    object
                } {
                    if item
                        .0
                        .iter()
                        .all(|i| if let Type::Symbol(_) = i { true } else { false })
                    {
                        flag = Some(item);
                        break;
                    }
                }
                flag
            } {
                let mut scope: &mut HashMap<String, Type> = &mut scope.clone();
                scope.extend(memory.to_owned());
                if args[args.len() - 1].get_symbol().starts_with("~") {
                    for (arg, value) in args.iter().zip(params.to_vec()) {
                        if arg.get_symbol().starts_with("~") {
                            scope.insert(
                                arg.get_symbol()[1..arg.get_symbol().len()].to_string(),
                                Type::List(
                                    params[params
                                        .iter()
                                        .position(|i| i.get_symbol() == value.get_symbol())
                                        .unwrap()
                                        ..params.len()]
                                        .to_vec(),
                                ),
                            );
                        } else {
                            scope.insert(arg.get_symbol(), value);
                        }
                    }
                } else {
                    for (arg, value) in args.iter().zip(params.to_vec()) {
                        scope.insert(arg.get_symbol(), value);
                    }
                }

                if args.len() <= params.len() {
                    if let Type::Block(block) = Type::parse(program.clone()) {
                        run(block, &mut scope)
                    } else {
                        eval(program.to_string(), &mut scope)
                    }
                } else {
                    let mut object = object.clone();
                    object.push((
                        args[params.len()..args.len()].to_vec(),
                        (program.clone(), scope.to_owned()),
                    ));
                    Type::Function(Function::UserDefined(object))
                }
            } else {
                Type::Null
            }
        }
    } else {
        return Type::Null;
    }
}

fn tokenize(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            ')' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            '[' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            ']' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            '{' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            '}' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            '"' => {
                if in_parentheses == 0 {
                    if in_quote {
                        current_token.push(c);
                        in_quote = false;
                        tokens.push(current_token.clone());
                        current_token.clear();
                    } else {
                        in_quote = true;
                        current_token.push(c);
                    }
                } else {
                    current_token.push(c);
                }
            }
            ' ' | '\n' | '\t' | '\r' | '　' => {
                if in_parentheses != 0 || in_parentheses != 0 || in_parentheses != 0 || in_quote {
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if !(in_parentheses != 0 || in_parentheses != 0 || in_parentheses != 0 || in_quote)
        && !current_token.is_empty()
    {
        tokens.push(current_token);
    }
    tokens
}
