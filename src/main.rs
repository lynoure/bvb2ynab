#![allow(dead_code)]  // Locally that would be without the exclamation

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;


#[derive(StructOpt)]
struct ConversionCLI {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
}


// YNAB docs say date format would be "6/8/20" but my account shows "2020-06-08",
// YNAB support says it autodetects and asks if unclear
// Leaving the date unconverted for now until hitting a hickup
fn convert_date(input: String) -> String {
    input.trim_matches('\"').to_string()
}
    

fn format(content: String) {
    //TODO Error if these are is not found, as it means not BVB bank statement CSV
    // Could also just skip the first 13 lines, but that would be fragile
    let beginning = content.find("\"Buchungstag\";\"Valuta\"").unwrap();
    let transactions = content.get(beginning..).unwrap();
    let mut complete = "".to_string();
    for line in transactions.lines().skip(1) {
        complete = complete + line;
        // "S" or "H" is the last field on a complete transaction
        if complete.rfind("\"S\"") == None && complete.rfind("\"H\"") == None {
            continue;
        } else {
            // Balance lines have very little details and are easy to spot
            if complete.contains(";;;;;;;;;") {
                break;
            };
            // TODO process the complete line to YNAB format
            // TODO replace date with the processed one
            println!("{}", complete);
            complete = "".to_string();
        }
    }
}

fn main() -> Result<(), ExitFailure> {
    //TODO Help txt with 'cli'
    let args = ConversionCLI::from_args();
    let infilename = args.infile.as_path().display().to_string();

    //TODO Could use BufReader, though month or two of personal finance fits into memory fine
    let content = std::fs::read_to_string(&args.infile)
        .with_context(|_| format!("Could not read file '{}'", infilename))?;
    
    //TODO MAYBE handle or convert ISO-8859-15, instead of leaving it to the user
    
    // TODO use this example data in a test
    
    
    format(content);

    Ok(())
}

#[test]
fn test_convert_date() {
    assert_eq!("28.6.2020".to_string(), convert_date("\"28.6.2020\"".to_string()));
    assert_eq!("8.6.2020".to_string(), convert_date("\"8.6.2020\"".to_string()));
}

/*              
    let _example = "\"08.06.2020\";\"08.06.2020\";\"Mila Mustermann\";\"Tolle Laden GmbH\";;\"DE89370400440532013000\";;\"GENODEF1HB1\";\"Basislastschrift
PP.1234.PP . EXAMPLIFIED, I
hr Einkauf bei EXAMPLIFIED 
EREF: 1009202807490 PP.1234
.PP PAYPAL MREF: ABCD123LMN
DVG CRED: EXAMPLE0000000000
000000012 IBAN: DE123456789
98765432103 BIC: DEUTDEFF\";;\"EUR\";\"113,82\";\"S\"";
*/
