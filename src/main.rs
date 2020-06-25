use structopt::StructOpt;

#[derive(StructOpt)]
struct ConversionCLI {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    outfile: std::path::PathBuf,
}

fn main() {
    //TODO better place for the help txt
    println!("CSV converter, from Bremische Volksbank format to YNAB import format");
    let args = ConversionCLI::from_args();
    println!("infile: {:?}", args.infile);
    println!("outfile: {:?}", args.outfile);
    
    //TODO better error handling, consider BufReader
    let content = std::fs::read_to_string(&args.infile).expect("could not read file");
    
    //TODO extend to do actual conversion
    //TODO handle or convert ISO-8859-15
    for line in content.lines() {
        println!("{}", line);
    }
}
