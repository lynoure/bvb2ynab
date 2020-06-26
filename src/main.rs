use structopt::StructOpt;

#[derive(Debug)]
struct ConversionError(String);

#[derive(StructOpt)]
struct ConversionCLI {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    outfile: std::path::PathBuf,
}

fn main() -> Result<(), ConversionError> {
    //TODO better place for the help txt
    println!("CSV converter, from Bremische Volksbank format to YNAB import format");
    let args = ConversionCLI::from_args();
    let infilename = args.infile.as_path().display().to_string();
    println!("outfile: {:?}", args.outfile);
    
    //TODO Consider BufReader, though month or two of personal finance fits into memory fine
    let result = std::fs::read_to_string(&args.infile);
    
    let content = match result {
        Ok(content) => { content },
        // TODO make even prettier
        Err(err) => { return Err(ConversionError(format!("Error reading {}: {}",
                                                         infilename, err))); }
    };

    //TODO handle or convert ISO-8859-15, instead of leaving it to the user
    for line in content.lines() {
        println!("{}", line);
    }
    Ok(())
}
