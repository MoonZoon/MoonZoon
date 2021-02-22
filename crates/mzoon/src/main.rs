use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Opt  {
    New { project_name: String },
    Start,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
