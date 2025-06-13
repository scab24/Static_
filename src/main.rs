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

fn main() {
    let program = Program {
        stmts: vec![
            Stmt::Assign("x".to_string(), Expr::Int(5)), //x = 5
            Stmt::Assign("y".to_string(), Expr::Int(1)), //y = 1
            // while (x > 0)
            Stmt::While(
                Expr::BinOp(
                    ">".to_string(),
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Int(0)),
                ),
                vec![
                    // y = y * x;
                    Stmt::Assign(
                        "y".to_string(),
                        Expr::BinOp(
                            "*".to_string(),
                            Box::new(Expr::Var("y".to_string())),
                            Box::new(Expr::Var("x".to_string())),
                        ),
                    ),
                    // x = x - 1;
                    Stmt::Assign(
                        "x".to_string(),
                        Expr::BinOp(
                            "-".to_string(),
                            Box::new(Expr::Var("x".to_string())),
                            Box::new(Expr::Int(1)),
                        ),
                    ),
                ],
            ),
        ],
    };
    println!("Program: {:?}", program);
}