#![allow(dead_code)]  // Locally that would be without the exclamation

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;


#[derive(StructOpt)]
struct ConversionCLI {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
}

/// Converts the transaction date into a format that YNAB understands
/// by pruning away the quotes
/// YNAB support says it autodetects the format and asks if unclear
fn convert_date(input: &str) -> &str {
    input.trim_matches('\"')
}

/// Converts the payee in a very naive way by pruning the quotation marks
/// and commas
fn convert_payee(input: Vec<&str>) -> String {
    //TODO Maybe take from upper case to lower with initial
    //TODO Maybe prune the usual suspects of their chattiness
    //TODO Maybe in case of PayPal, get the final payee from the message
    input[3].trim_matches('\"').replace(',', "").to_string()
}
    
/// Formats the multiline String into YNAB format
fn format(content: String) {
    //TODO Error if these are is not found, as it means not BVB bank statement CSV
    // Could also just skip the first 13 lines, but that would be fragile
    let beginning = content.find("\"Buchungstag\";\"Valuta\"").unwrap();
    let transactions = content.get(beginning..).unwrap();
    let mut complete = "".to_string();
    for line in transactions.lines().skip(1) {
        // Memo is split to multiple lines, " " needed to avoid joining words
        if complete == "" {
            complete = line.to_string();
        } else {
        complete = complete + " " + line;
        }
        // "S" or "H" is the last field on a complete transaction
        if complete.rfind("\"S\"") == None && complete.rfind("\"H\"") == None {
            continue;
        } else {
            // Balance lines have very little details and are easy to spot
            if complete.contains(";;;;;;;;;") {
                break;
            };
            println!("{}", complete); // XXX Remove when no longer debugging
            // TODO parse the line into YNAB format
            // TODO the throwing away of quotes could fit here?
            let parts: Vec<&str> = complete.split(";").collect();
            println!("{},{},", convert_date(parts[0]), convert_payee(parts));

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
    assert_eq!("28.6.2020", convert_date("\"28.6.2020\""));
    assert_eq!("8.6.2020", convert_date("\"8.6.2020\""));
}

#[test]
fn test_convert_payee_simple_input() {
    let input = vec!["\"28.6.2020\"",
        "\"28.6.2020\"", 
        "\"ISSUER\"", 
        "\"Tolle Laden GmbH\""];
    assert_eq!("Tolle Laden GmbH", convert_payee(input));
}

#[test]
fn test_convert_payee_removes_commas() {
    let input = vec!["\"28.6.2020\"",
        "\"28.6.2020\"",
        "\"ISSUER\"",
        "\"DANKE, IHR SUPERMARKT\""];
    // Will joyfully break if the chatty formats ever get cleaned real good
    assert_eq!("DANKE IHR SUPERMARKT", convert_payee(input));

    // TODO prettified use of case
    // assert_eq!("Tolle Laden GmbH", convert_payee("\"TOLLE LADEN GMBH\""));
    // TODO for pruning the chattiness from the most usual suspects
    // assert_eq("Lidl", convert_payee("\"DANKE, IHR LIDL\""));
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
