#[derive(Debug)]
struct Plexer {
    var_names: Vec<char>,
    var_count: usize,
    iter_count: usize,
    var_iter_matrix: Vec<Vec<bool>>,
}

impl Plexer {
    fn new(vars: &[char]) -> Plexer {
        let var_count = vars.len();
        let iter_count = 2usize.pow(var_count as u32);
        let mut var_iter_matrix = vec![Vec::with_capacity(iter_count); var_count];

        for var in 1..=var_count {
            let mut now = true; // current value
            let flip_target = iter_count >> var; // after target iterations, flip current value
            for iter in 1..=iter_count {
                var_iter_matrix[var - 1].push(now);
                if iter % flip_target == 0 {
                    now = !now;
                }
            }
        }

        Plexer {
            var_count,
            var_iter_matrix,
            iter_count,
            var_names: vars.to_vec(),
        }
    }
    fn get(&self, idx: usize, iter: usize) -> bool {
        self.var_iter_matrix[idx][iter]
    }
}

pub struct TableMaker {
    p: Plexer,
    expr: Value,
}

enum Value {
    Expr(Box<Op>),
    Var { var_index: usize },
}

enum Op {
    Not(Value),
    And(Value, Value),
    Or(Value, Value),
    Xor(Value, Value),
    Then(Value, Value),
}

impl Value {
    fn get(&self, vars: &Plexer, iter: usize) -> bool {
        use Value::*;
        match self {
            Expr(op) => op.get(vars, iter),
            Var { var_index } => vars.get(*var_index, iter),
        }
    }
    fn fmt(&self, vars: &Plexer) -> String {
        use Value::*;
        match self {
            Expr(x)=>x.fmt(vars),
            Var{var_index}=>{
                vars.var_names[*var_index].to_string()
            }
        }
    }

}

impl Op {
    fn expr(self) -> Value {
        Value::Expr(Box::new(self))
    }
    fn get(&self, vars: &Plexer, iter: usize) -> bool {
        use Op::*;
        match self {
            Not(x) => !x.get(vars, iter),
            And(a, b) => a.get(vars, iter) && b.get(vars, iter),
            Or(a, b) => a.get(vars, iter) || b.get(vars, iter),
            Xor(a, b) => a.get(vars, iter) ^ b.get(vars, iter),
            Then(a, b) => {
                let a = a.get(vars, iter);
                let b = b.get(vars, iter);
                !(a && !b)
            }
        }
    }
    fn fmt(&self, vars: &Plexer) -> String {
        use Op::*;
         match self {
            Not(x) => format!("¬({})", x.fmt(vars)),
            And(a, b)  => format!("({} ^ { })", a.fmt(vars), b.fmt(vars)),
            Or(a, b)   => format!("({} v { })", a.fmt(vars), b.fmt(vars)),
            Xor(a, b)  => format!("({} ≠ {})", a.fmt(vars), b.fmt(vars)),
            Then(a, b) => format!("({} -> {})", a.fmt(vars), b.fmt(vars)),
        }
    }
}

fn to_pt(v: bool) -> char {
    if v {'V'} else {'F'}
}

pub fn tryout() {
    use Op::*;
    use Value::*;
    let p = Plexer::new(&['A', 'B']);
    let expr = Xor(Var { var_index: 0 }, Var { var_index: 1 }).expr();
    println!("{}", expr.fmt(&p));
    print!("# ");
    for var in &p.var_names {
        print!("{var} ");
    }
    println!();
    for i in 0..p.iter_count {
        print!("{} ", i+1);
        for vidx in 0..p.var_count{
            print!("{} ", to_pt(p.get(vidx, i)));
        }
        let x = expr.get(&p, i);
        println!(": {}", to_pt(x));
    }
}

