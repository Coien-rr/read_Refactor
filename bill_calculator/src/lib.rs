use std::cmp;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum PlayGenre {
    Tragedy,
    Comedy,
}

#[derive(Debug, PartialEq)]
pub struct Play {
    name: String,
    genre: PlayGenre,
}

#[derive(Debug)]
struct Performance {
    play_id: String,
    audience: u32,
}

#[derive(Debug)]
pub struct Invoice {
    customer: String,
    performances: Vec<Performance>,
}

pub fn init() -> (HashMap<String, Play>, Invoice) {
    let mut plays: HashMap<String, Play> = HashMap::new();

    plays.insert(
        String::from("hamlet"),
        Play {
            name: String::from("Hamlet"),
            genre: PlayGenre::Tragedy,
        },
    );

    plays.insert(
        String::from("as-like"),
        Play {
            name: String::from("As You Like It"),
            genre: PlayGenre::Comedy,
        },
    );

    plays.insert(
        String::from("othello"),
        Play {
            name: String::from("Othello"),
            genre: PlayGenre::Tragedy,
        },
    );

    let mut invoices = Invoice {
        customer: String::from("BigCo"),
        performances: Vec::new(),
    };

    invoices.performances.push(Performance {
        play_id: String::from("hamlet"),
        audience: 55,
    });

    invoices.performances.push(Performance {
        play_id: String::from("as-like"),
        audience: 35,
    });

    invoices.performances.push(Performance {
        play_id: String::from("othello"),
        audience: 40,
    });

    (plays, invoices)
}

pub fn statement(invoice: Invoice, plays: HashMap<String, Play>) -> Vec<String> {
    let mut total_amount = 0;
    let mut volume_credits = 0;
    let mut result: Vec<String> = Vec::new();
    result.push(String::from("Statement for {}").replace("{}", &invoice.customer));

    for perf in &invoice.performances {
        let play = play_for(perf, &plays);

        let mut this_amount = amount_for(perf, play);

        volume_credits += cmp::max(perf.audience - 30, 0);

        if let PlayGenre::Comedy = play.genre {
            volume_credits += perf.audience / 5;
        }

        result.push(format!(
            "  {0}: {1} ({2} seats)",
            &play.name,
            this_amount / 100,
            perf.audience,
        ));

        total_amount += this_amount;
    }

    result.push(format!("Amount owed is {}", total_amount / 100));
    result.push(format!("You earned {} credits", volume_credits));

    result
}

fn amount_for(perf: &Performance, play: &Play) -> u32 {
    let mut this_amount = 0;

    match play.genre {
        PlayGenre::Tragedy => {
            this_amount = 40000;
            if perf.audience > 30 {
                this_amount += 1000 * (perf.audience - 30);
            }
        }
        PlayGenre::Comedy => {
            this_amount = 30000;
            if perf.audience > 20 {
                this_amount += 10000 + 500 * (perf.audience - 20);
            }

            this_amount += 300 * perf.audience;
        }
    }

    this_amount
}

fn play_for<'a>(perf: &Performance, plays: &'a HashMap<String, Play>) -> &'a Play {
    if let Some(p) = plays.get(&perf.play_id) {
        p
    } else {
        panic!("ERROR");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_for() {
        let perf: Performance = Performance {
            play_id: String::from("hamlet"),
            audience: 55,
        };

        let play: Play = Play {
            name: String::from("Hamlet"),
            genre: PlayGenre::Tragedy,
        };

        assert_eq!(amount_for(&perf, &play), 65000);

        let perf: Performance = Performance {
            play_id: String::from("as-like"),
            audience: 35,
        };

        let play: Play = Play {
            name: String::from("As You Like It"),
            genre: PlayGenre::Comedy,
        };

        assert_eq!(amount_for(&perf, &play), 58000);
    }

    #[test]
    fn test_play_for() {
        let (plays, invoices) = init();

        for perf in &invoices.performances {
            let play = if let Some(p) = plays.get(&perf.play_id) {
                p
            } else {
                panic!("ERROR");
            };

            assert_eq!(play, play_for(perf, &plays));
        }
    }
}

