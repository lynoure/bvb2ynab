use structopt::StructOpt;

#[derive(Debug)]
struct ConversionError(String);

#[derive(StructOpt)]
struct ConversionCLI {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
}

fn format(content: String) {
    for line in content.lines() {
        println!("{}", line);
    }
}

fn main() -> Result<(), ConversionError> {
    //TODO Help txt with 'cli'
    let args = ConversionCLI::from_args();
    let infilename = args.infile.to_string_lossy();
    
    //TODO Consider BufReader, though month or two of personal finance fits into memory fine
    let result = std::fs::read_to_string(&args.infile);
    
    let content = match result {
        Ok(content) => { content },
        // TODO make even prettier & return unique error codes
        Err(err) => { return Err(ConversionError(format!("Error reading {}: {}",
                                                         infilename, err))); }
    };

    //TODO handle or convert ISO-8859-15, instead of leaving it to the user
    
    format(content);

    Ok(())
}
