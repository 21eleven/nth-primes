use clap::{App, Arg, SubCommand, values_t_or_exit, value_t_or_exit};

fn main() {
    //let default = &(100_000_000).to_string()[..];
    let default = &(25_000_000).to_string()[..];
    let m = App::new("nth primes")
                .about("identity hunting")
                .subcommand(SubCommand::with_name("from")
                    .alias("f")
                    .arg(Arg::with_name("index")
                        .short("i")
                        .help("return index of primes along with values")
                        .required(false))
                    .arg(Arg::with_name("range")
                        .required(true)
                        .help("return primes in this range")
                        .multiple(true)
                        .number_of_values(2)))
                .subcommand(SubCommand::with_name("nth")
                    .alias("n")
                    .arg(Arg::with_name("index")
                        .help("nth prime to print")
                        .required(true))
                    .arg(Arg::with_name("max")
                        .required(false)
                        .help("limit for prime search")
                        .number_of_values(1)
                        .default_value(default)))
                .get_matches();
    if let Some(matches) = m.subcommand_matches("from") {
        let range = values_t_or_exit!(matches.values_of("range"), usize);
        print_primes(range, matches.is_present("index"));
    }
    if let Some(matches) = m.subcommand_matches("nth") {
        let nth = value_t_or_exit!(matches.value_of("index"), usize);
        let max = value_t_or_exit!(matches.value_of("max"), usize);
        print_nth_prime(nth, max);
        //let range = values_t_or_exit!(matches.values_of("range"), usize);
        //print_primes(range, matches.is_present("index"));
    }
}
fn print_nth_prime(nth: usize, upper: usize) -> () {
    for (_, prime) in fetch_primes(upper).into_iter().enumerate()
        .filter(|(idx, _)| idx == &nth) {
            println!("{}", prime);
    }
}

fn print_primes(range: Vec<usize>, idx: bool) -> () {
    let lower = range[0];
    let upper = range[1];
    for (nth, prime) in fetch_primes(upper).into_iter().enumerate()
        .filter(|(_, prime)| prime >= &lower) {
            if idx {
                println!("{} {}", nth, prime);
            } else {
                println!("{}", prime);
            }
    }
}

fn fetch_primes(upper: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = (0..upper + 1).map(|n| n == 2 || n & 1 != 0).collect();
    let mut falsifier = 3usize;
    while falsifier * falsifier <= upper {
        let mut mark: usize = falsifier * falsifier;
        while mark <= upper {
            primes[mark] = false;
            mark += falsifier;
        }
        falsifier += 2;
    }
    primes.into_iter().enumerate().skip(2)
          .filter_map(
              |(prime, is_prime)| 
                if is_prime 
                    {Some(prime)} 
                else 
                    {None}
          ).collect::<Vec<usize>>()
}