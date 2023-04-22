mod machines;
use machines::*;

enum MatchTactic {
    AllowOvershoot,
    Exact,
}

#[derive(Debug, PartialEq, Eq)]
struct Pipeline {
    assembly: Vec<Machine<Assembly>>,
    soldering: Vec<Machine<Soldering>>,
    qcheck: Vec<Machine<QualityChecking>>,
}

fn match_capacity(goal: usize, tactic: MatchTactic) -> (Pipeline, usize) {
    let assembly = Machine::<Assembly>::match_capacity(goal);
    let soldering: Vec<Machine<Soldering>>;
    let qcheck: Vec<Machine<QualityChecking>>;
    match tactic {
        MatchTactic::AllowOvershoot => {
            let new_goal = assembly.iter().map(|m| m.capacity).sum();
            soldering = Machine::<Soldering>::match_capacity(new_goal);
            qcheck = Machine::<QualityChecking>::match_capacity(new_goal);
        }
        MatchTactic::Exact => {
            soldering = Machine::<Soldering>::match_capacity(goal);
            qcheck = Machine::<QualityChecking>::match_capacity(goal);
        }
    }
    let price = assembly.iter().map(|m| m.price()).sum::<usize>()
        + soldering.iter().map(|m| m.price()).sum::<usize>()
        + qcheck.iter().map(|m| m.price()).sum::<usize>();
    return (
        Pipeline {
            assembly,
            soldering,
            qcheck,
        },
        price,
    );
}

// TODO: make this actually not stupid
// right now it's using some binary-search-esque
// algorithm to find the best fit for a certain
// price, which takes a really long time in cases
// like price = 51400.
fn match_price(goal: usize) -> (Pipeline, usize) {
    let mut higher_bound = usize::MAX >> 2;
    let mut lower_bound = 1;
    let mut guess = goal / 1725;
    let mut last_fit = (
        Pipeline {
            assembly: Vec::new(),
            soldering: Vec::new(),
            qcheck: Vec::new(),
        },
        0,
    );
    loop {
        let (pipe, price) = match_capacity(guess, MatchTactic::Exact);
        if (&last_fit.0, last_fit.1) == (&pipe, price) {
            break;
        }
        if price > goal {
            higher_bound = guess;
        } else if price < goal {
            lower_bound = guess;
            last_fit = (pipe, price);
        } else if price == goal {
            return (pipe, price);
        }
        guess = (higher_bound + lower_bound) / 2;
    }
    return last_fit;
}

fn main() {
    let (pipe, price) = match_price(51400);
    println!("{}", price);
    for m in pipe.assembly {
        println!("{m:?}");
    }
    for m in pipe.soldering {
        println!("{m:?}");
    }
    for m in pipe.qcheck {
        println!("{m:?}");
    }
}
