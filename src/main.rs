#![allow(dead_code)]  // Locally that would be without the exclamation

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;


#[derive(StructOpt)]
struct ConversionCLI {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
}

//TODO Struct for transaction?
//TODO Cleaning of the fields not to repeat

/// Converts the transaction date into a format that YNAB understands
/// by pruning away the quotes
/// YNAB support says it autodetects the format and asks if unclear
fn convert_date(input: &str) -> &str {
    //FIXME THIS WILL NEED TO CHANGE TO LOOK INTO THE MEMO AS WELL
    // Take the whole of parts and look for something matching the date 
    // format in the memo first
    input.trim_matches('\"')
}

/// Converts the payee in a very naive way by pruning the quotation marks
/// and commas
fn convert_payee(input: &Vec<&str>) -> String {
    //TODO Maybe take from upper case to lower with initial
    //TODO Maybe prune the usual suspects of their chattiness
    //TODO Maybe in case of PayPal, get the final payee from the message
    input[3].trim_matches('\"').replace(',', "").to_string()
}

fn convert_memo(input: &Vec<&str>) -> String {
    input[8].trim_matches('\"').replace(',', "").to_string()
    //NOTE The bank will put the actual transaction data here in case of card payments!!!
}

fn convert_amount(input: Vec<&str>) -> String {
    let mut sign = "";
    if input[12] == "\"S\"" {
        sign = "-";
    }
    let mut amount = input[11].trim_matches('\"').split(",");
    format!("{}{}.{}", sign, amount.next().unwrap(), amount.next().unwrap())
}

/// Formats the multiline String into YNAB format
fn format(content: String) {
    //TODO Error if these are is not found, as it means not BVB bank statement CSV
    // Could also just skip the first 13 lines, but that would be fragile
    let beginning = content.find("\"Buchungstag\";\"Valuta\"").unwrap();
    println!("Date,Payee,Memo,Amount");
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
            // println!("{}", complete); // XXX Remove when no longer debugging
            // TODO the throwing away of quotes could fit here?
            let parts: Vec<&str> = complete.split(";").collect();
            println!("{},{},{},{}", convert_date(parts[0]), 
                     convert_payee(&parts),
                     convert_memo(&parts),
                     convert_amount(parts));

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
    assert_eq!("Tolle Laden GmbH", convert_payee(&input));
}

#[test]
fn test_convert_payee_removes_commas() {
    let input = vec!["\"28.6.2020\"",
        "\"28.6.2020\"",
        "\"ISSUER\"",
        "\"DANKE, IHR SUPERMARKT\""];
    // Will joyfully break if the chatty formats ever get cleaned real good
    assert_eq!("DANKE IHR SUPERMARKT", convert_payee(&input));

    // TODO prettified use of case
    // assert_eq!("Tolle Laden GmbH", convert_payee("\"TOLLE LADEN GMBH\""));
    // TODO for pruning the chattiness from the most usual suspects
    // assert_eq("Lidl", convert_payee("\"DANKE, IHR LIDL\""));
}

#[test]
fn test_convert_memo() {
    let input = vec!["\"28.6.2020\"",
        "\"28.6.2020\"",
        "\"ISSUER\"",
        "\"SUPERMARKT\"",
        "",
        "\"IBAN\"",
        "",
        "\"BIC\"",
        "\"MEMO\""];
    assert_eq!("MEMO", convert_memo(&input));
}


#[test]
fn test_convert_amount_outgoing() {
    let input = vec!["\"28.6.2020\"",
        "\"28.6.2020\"",
        "\"ISSUER\"",
        "\"SUPERMARKT\"",
        "",
        "\"IBAN\"",
        "",
        "\"BIC\"",
        "\"MEMO\"",
        "",
        "\"EUR\"",
        "\"11,97\"",
        "\"S\""];
    assert_eq!("-11.97", convert_amount(input));
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
