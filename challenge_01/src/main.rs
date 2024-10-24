use crate::wc::get_statistics;

mod cli;
mod wc;

fn print_result(result: wc::ResultWc) {
    print!("{}", result.format());
}

fn main() {
    print_result(get_statistics());
}
