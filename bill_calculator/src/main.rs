use bill_calculator::{init, statement};
fn main() {
    let (plays, invoices) = init();

    for line in statement(invoices, plays) {
        println!("{line}");
    }
}
