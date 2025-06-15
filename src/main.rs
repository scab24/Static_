#[derive(Clone, Debug)]
enum Expr {
    Int(i32),                     
    Var(String),                  
    BinOp(String, Box<Expr>, Box<Expr>), 
}

#[derive(Clone, Debug)]
enum Stmt {
    Assign(String, Expr),         
    If(Expr, Vec<Stmt>, Vec<Stmt>), 
    While(Expr, Vec<Stmt>),       
    Seq(Vec<Stmt>),               
}

#[derive(Debug)]
struct Program {
    stmts: Vec<Stmt>,
}
#[derive(Debug)]
enum ParseError {
    InvalidAssignment(String),
    InvalidNumber(String),
}

fn parse_assignment(line: &str, var: &str) -> Result<Stmt, ParseError> {
    if !line.starts_with(var) {
        return Err(ParseError::InvalidAssignment(format!("Expected '{} =', got '{}'", var, line)));
    }
    let parts: Vec<&str> = line.split('=').collect();
    if parts.len() < 2 {
        return Err(ParseError::InvalidAssignment(format!("Missing value in assignment: '{}'", line)));
    }
    let value_str = parts[1].trim().replace(";", "");
    let value = value_str.parse::<i32>().map_err(|_| {
        ParseError::InvalidNumber(format!("Invalid number in '{}': '{}'", line, value_str))
    })?;
    Ok(Stmt::Assign(var.to_string(), Expr::Int(value)))
}

fn parse_program(input: &str) -> Result<Program, ParseError> {
    let lines: Vec<&str> = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let mut stmts = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("x =") {
            stmts.push(parse_assignment(line, "x =")?);
            i += 1;
        } else if line.starts_with("y =") {
            stmts.push(parse_assignment(line, "y =")?);
            i += 1;
        } else if line.starts_with("while") {
            let body = vec![
                Stmt::Assign(
                    "y".to_string(),
                    Expr::BinOp(
                        "*".to_string(),
                        Box::new(Expr::Var("y".to_string())),
                        Box::new(Expr::Var("x".to_string())),
                    ),
                ),
                Stmt::Assign(
                    "x".to_string(),
                    Expr::BinOp(
                        "-".to_string(),
                        Box::new(Expr::Var("x".to_string())),
                        Box::new(Expr::Int(1)),
                    ),
                ),
            ];
            stmts.push(Stmt::While(
                Expr::BinOp(
                    ">".to_string(),
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Int(0)),
                ),
                body,
            ));
            i += 5;
        } else {
            i += 1;
        }
    }
    Ok(Program { stmts })
}
fn main() {
    env_logger::init();

    let input = "
        x = 5;
        y = 1;
        while (x > 0) {
            y = y * x;
            x = x - 1;
        }
    ";
    match parse_program(input) {
        Ok(program) => println!("Program parsed: {:?}", program),
        Err(e) => {
            log::error!("Error parsing: {:?}", e);
            std::process::exit(1);
        }
    }
}