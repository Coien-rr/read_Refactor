use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
enum PlayGenre {
    Tragedy,
    Comedy,
}

#[derive(Debug)]
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
        let play = if let Some(p) = plays.get(&perf.play_id) {
            p
        } else {
            panic!("ERROR");
        };

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
