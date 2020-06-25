use structopt::StructOpt;

#[derive(StructOpt)]
struct ConversionCLI {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    outfile: std::path::PathBuf,
}

fn main() {
    println!("CSV converter, from Bremische Volksbank format to YNAB import format");
    let args = ConversionCLI::from_args();
    println!("infile: {:?}", args.infile);
    println!("outfile: {:?}", args.outfile);
}
