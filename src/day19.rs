
#[derive(Clone, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone, Debug, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>
}

#[derive(Clone, Debug, Hash)]
enum Category { X,M,A,S }

#[derive(Clone, Debug, Hash)]
enum Rule {
    Lt(Category, usize, String),
    Gt(Category, usize, String),
    Jump(String),
}

peg::parser! {
    grammar workflow() for str {
        rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule label() -> String
            = n:$(['a'..='z' | 'A'..='Z']+) { n.to_string() }

        rule rl() -> Rule
            = l:(lt() / gt() / jump()) { l }

        rule category() -> Category = c:(category_x() / category_m() / category_a() / category_s()) { c }
        rule category_x() -> Category = "x" { Category::X }
        rule category_m() -> Category = "m" { Category::M }
        rule category_a() -> Category = "a" { Category::A }
        rule category_s() -> Category = "s" { Category::S }

        rule gt() -> Rule = c:(category()) ">" n:(number()) ":" w:(label()) { Rule::Gt(c, n, w) }
        rule lt() -> Rule = c:(category()) "<" n:(number()) ":" w:(label()) { Rule::Lt(c, n, w) }
        rule jump() -> Rule = w:(label()) { Rule::Jump(w) }

        pub rule workflow() -> Workflow
            = name :(label()) "{" rules:(rl() ** ",") "}" { Workflow { name, rules } }

        pub rule part() -> Part
            = "{x=" x:(number()) ",m=" m:(number()) ",a=" a:(number()) ",s=" s:(number()) "}" { Part {x,m,a,s} }
    }
}

impl Part {
    pub fn get(&self, c: &Category) -> usize {
        match c {
            Category::X => { self.x },
            Category::M => { self.m },
            Category::A => { self.a },
            Category::S => { self.s },
        }
    }
}

impl Rule {
    pub fn apply(&self, part: &Part) -> Option<String> {
        match self {
            Self::Lt(cat, value, target) => { if part.get(cat) < *value { return Some(target.clone()); } }
            Self::Gt(cat, value, target) => { if part.get(cat) > *value { return Some(target.clone()); } }
            Self::Jump(target) => { return Some(target.clone()); }
        }

        None
    }

    pub fn target(&self) -> String {
        match self {
            Self::Lt(_, _, target) => { target.clone() }
            Self::Gt(_, _, target) => { target.clone() }
            Self::Jump(target) => { target.clone() }
        }
    }

    pub fn apply_filter(&self, r: &Ranges) -> Option<(String, Ranges)> {
        let r2 = self.accept(r.clone());

        if r2.count() > 0 {
            Some((self.target(), r2))
        } else {
            None
        }
    }

    pub fn accept(&self, r: Ranges) -> Ranges {
        match self {
            Self::Lt(Category::X, value, _) => { Ranges { x: (r.x.0, r.x.1.min(*value)), m: r.m, a: r.a, s: r.s } }
            Self::Gt(Category::X, value, _) => { Ranges { x: (r.x.0.max(*value + 1), r.x.1), m: r.m, a: r.a, s: r.s } }

            Self::Lt(Category::M, value, _) => { Ranges { x: r.x, m: (r.m.0, r.m.1.min(*value)), a: r.a, s: r.s } }
            Self::Gt(Category::M, value, _) => { Ranges { x: r.x, m: (r.m.0.max(*value + 1), r.m.1), a: r.a, s: r.s } }

            Self::Lt(Category::A, value, _) => { Ranges { x: r.x, m: r.m, a: (r.a.0, r.a.1.min(*value)), s: r.s } }
            Self::Gt(Category::A, value, _) => { Ranges { x: r.x, m: r.m, a: (r.a.0.max(*value + 1), r.a.1), s: r.s } }

            Self::Lt(Category::S, value, _) => { Ranges { x: r.x, m: r.m, a: r.a, s: (r.s.0, r.s.1.min(*value)) } }
            Self::Gt(Category::S, value, _) => { Ranges { x: r.x, m: r.m, a: r.a, s: (r.s.0.max(*value + 1), r.s.1) } }

            Self::Jump(_) => { r }
        }
    }

    pub fn reject(&self, r: Ranges) -> Ranges {
        match self {
            Self::Lt(Category::X, value, _) => { Ranges { x: (r.x.0.max(*value), r.x.1), m: r.m, a: r.a, s: r.s } }
            Self::Gt(Category::X, value, _) => { Ranges { x: (r.x.0, r.x.1.min(*value + 1)), m: r.m, a: r.a, s: r.s } }

            Self::Lt(Category::M, value, _) => { Ranges { x: r.x, m: (r.m.0.max(*value), r.m.1), a: r.a, s: r.s } }
            Self::Gt(Category::M, value, _) => { Ranges { x: r.x, m: (r.m.0, r.m.1.min(*value + 1)), a: r.a, s: r.s } }

            Self::Lt(Category::A, value, _) => { Ranges { x: r.x, m: r.m, a: (r.a.0.max(*value), r.a.1), s: r.s } }
            Self::Gt(Category::A, value, _) => { Ranges { x: r.x, m: r.m, a: (r.a.0, r.a.1.min(*value + 1)), s: r.s } }

            Self::Lt(Category::S, value, _) => { Ranges { x: r.x, m: r.m, a: r.a, s: (r.s.0.max(*value), r.s.1) } }
            Self::Gt(Category::S, value, _) => { Ranges { x: r.x, m: r.m, a: r.a, s: (r.s.0, r.s.1.min(*value + 1)) } }

            Self::Jump(_) => { Ranges::default() }
        }
    }
}

impl Workflow {
    pub fn apply(&self, part: &Part) -> String {
        for rule in self.rules.iter() {
            if let Some(tgt) = rule.apply(part) {
                return tgt;
            }
        }

        panic!("");
    }

    pub fn filter(&self, mut r: Ranges, workflows: &std::collections::HashMap<String, Workflow>) -> usize {
        let mut total = 0;
        for rule in self.rules.iter() {
            if let Some((tgt, r2)) = rule.apply_filter(&r) {
                if tgt == "A" {
                    total += r2.count();
                } else if tgt == "R" {
                    total += 0;
                } else {
                    total += workflows[&tgt].filter(r2, workflows);
                }
            }

            r = rule.reject(r);
        }

        total
    }
}

#[derive(Debug, Clone, Default)]
struct Ranges {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}
impl Ranges {
    pub fn count(&self) -> usize {
        if self.x.0 < self.x.1 && self.m.0 < self.m.1 && self.a.0 < self.a.1 && self.s.0 < self.s.1 {
            (self.x.1 - self.x.0) *
            (self.m.1 - self.m.0) *
            (self.a.1 - self.a.0) *
            (self.s.1 - self.s.0)
        } else {
            0
        }
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut in_workflows = true;

    let mut workflows = std::collections::HashMap::new();
    let mut parts: Vec<Part> = vec![];

    for line in input.lines() {
        if line == "" {
            in_workflows = false;
        } else if in_workflows {
            let wf = workflow::workflow(line).unwrap();
            workflows.insert(wf.name.clone(), wf);
        } else {
            let p = workflow::part(line).unwrap();
            parts.push(p);
        }
    }

    let mut p1 = 0;
    for p in parts {
        let mut wf = "in".to_string();
        while wf != "A" && wf != "R" {
            wf = workflows[&wf].apply(&p);
        }

        if wf == "A" {
            p1 += p.x + p.m + p.a + p.s;
        }
    }

    let all = Ranges {x:(1,4001), m:(1,4001), a: (1,4001), s: (1,4001)};
    let p2 = workflows["in"].filter(all, &workflows);
    //let p2 = all.count();


    (p1, p2)
}
