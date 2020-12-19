use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Debug, Clone)]
enum Rule<'a> {
    Char(&'a str, char),
    Union(&'a str, Vec<Rule<'a>>),
    Pair(&'a str, usize, usize),
    Triple(&'a str, usize, usize, usize),
    Ref(&'a str, usize),
}

impl Rule<'_> {
    fn to_string(&self) -> &str {
        match self {
            Self::Char(s, _) => s,
            Self::Union(s, _) => s,
            Self::Pair(s, _, _) => s,
            Self::Triple(s, _, _, _) => s,
            Self::Ref(s, _) => s,
        }
    }
}

fn check<'a>(
    r: &'a Rule<'a>,
    message: &'a str,
    ruleset: &'a Vec<Rule<'a>>,
    cache: &mut HashMap<(&'a str, &'a str), bool>,
) -> bool {
    if let Some(ok) = cache.get(&(r.to_string(), message)) {
        return *ok;
    }

    let ok = match r {
        Rule::Char(_, c) => message.len() == 1 && message.chars().nth(0).unwrap() == *c,
        Rule::Union(_, rules) => rules.iter().any(|r| check(r, message, ruleset, cache)),
        Rule::Pair(_, a, b) => (1..message.len()).any(|split| {
            let (sa, sb) = message.split_at(split);
            check(&ruleset[*a], sa, ruleset, cache) && check(&ruleset[*b], sb, ruleset, cache)
        }),
        Rule::Triple(_, a, b, c) => {
            let mut ok = false;
            'outer: for i in 1..message.len() {
                for j in (i + 1)..message.len() {
                    let sa = &message[0..i];
                    let sb = &message[i..j];
                    let sc = &message[j..];
                    if check(&ruleset[*a], sa, ruleset, cache)
                        && check(&ruleset[*b], sb, ruleset, cache)
                        && check(&ruleset[*c], sc, ruleset, cache)
                    {
                        ok = true;
                        break 'outer;
                    }
                }
            }
            ok
        }
        Rule::Ref(_, n) => check(&ruleset[*n], message, ruleset, cache),
    };

    cache.insert((r.to_string(), message), ok);

    ok
}

fn parse_rule(def: &str) -> Rule {
    if let Some(pos) = def.chars().position(|c| c == '"') {
        Rule::Char(def, def.chars().nth(pos + 1).unwrap())
    } else if def.contains("|") {
        Rule::Union(def, def.split(" | ").map(|seq| parse_rule(seq)).collect())
    } else if def.contains(" ") {
        let toks: Vec<&str> = def.split(" ").collect();
        if toks.len() == 2 {
            Rule::Pair(def, toks[0].parse().unwrap(), toks[1].parse().unwrap())
        } else {
            Rule::Triple(
                def,
                toks[0].parse().unwrap(),
                toks[1].parse().unwrap(),
                toks[2].parse().unwrap(),
            )
        }
    } else {
        Rule::Ref(def, def.parse().unwrap())
    }
}

fn main() {
    let input = get_input();
    let sections: Vec<&str> = input.split("\n\n").collect();
    let (rule_input, message_input) = (sections[0], sections[1]);

    let mut rules: Vec<Rule> = vec![Rule::Char("", '\0'); rule_input.lines().count()];
    for line in rule_input.lines() {
        let chunks: Vec<&str> = line.split(": ").collect();
        let (num, def) = (chunks[0], chunks[1]);
        rules[num.parse::<usize>().unwrap()] = parse_rule(def);
    }

    let messages: Vec<&str> = message_input.lines().collect();
    let mut cache = HashMap::new();

    let res = messages
        .iter()
        .filter(|message| check(&rules[0], message, &rules, &mut cache))
        .count();
    println!("{}", res);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
