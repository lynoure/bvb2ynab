#![allow(dead_code)]
#[macro_use] extern crate lazy_static;


use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use regex::Regex;


#[derive(StructOpt)]
struct ConversionCLI {
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
}

//TODO Struct for transaction?

/// Converts the transaction date into a format that YNAB understands.
/// As for card payments the actual date is often in the Vorgang/Verwendungszweck,
/// that date is preferred over Buchungstag and Valuta.
/// YNAB support says it autodetects the format and asks if unclear.
fn convert_date(input: &Vec<&str>) -> String {
    lazy_static! {
        static ref RE: Regex =  Regex::new(r"\d{2}\.\d{2}\.\d{4}").unwrap();
    }
    let mat = RE.find(&input[8]);
    match mat {
        Some(mat) => input[8].get(mat.start()..mat.end()).unwrap().to_string(),
        None => input[0].to_string()
    }
}

/// Gets the actual payee paid via PayPal
/// In case of the name being empty, returns "Unknown via PayPal"
/// In case of receiving payment from PayPal, or the memo being totally different for some
/// other reason, return merely "PayPal"
fn get_paypal_payee(memo: String) -> String {
    // TODO Deal with multi-word paypal payees?
    let begin = memo.find("bei ");
    match begin {
        Some(mut begin) => {
            begin = begin + 4;
            let end = memo[begin..].find(' ').unwrap() + begin;
            let recipient = memo[begin..end].to_string();
            if recipient.is_empty() {
                "Unknown via PayPal".to_string()
            } else {
                recipient
            }
        },
        // In case of receiving a payment from PayPal, or weird memo field
        None => "PayPal".to_string()
    }
}

/// Converts the payee in a naive way with the exception of PayPal, where actual payee is looked
/// for
fn convert_payee(input: &Vec<&str>) -> String {
    //TODO Maybe take from upper case to lower with initial
    //TODO Maybe prune the usual suspects of their chattiness
    //TODO "" is Bremische Volksbank for their fees
    //TODO "Verrechnungskunde intern" should have e.g. VISA in the memo,
    //and be for paying the card payments
    let payee: String = input[3].replace(',', "");
    if payee.contains("PayPal") {
        get_paypal_payee(input[8].replace(',', ""))
    } else {
        payee
    }
}

fn convert_memo(input: &Vec<&str>) -> String {
    input[8].replace(',', "").to_string()
}

fn convert_amount(input: &Vec<&str>) -> String {
    let mut sign = "";
    if input[12] == "S" {
        sign = "-";
    }
    let amount = input[11].replace(',', ".");
    format!("{}{}", sign, amount)
}

/// Formats the multiline String into YNAB format and prints it out
fn format(content: String, mut writer: impl std::io::Write) {
    //TODO Show a proper Error if these are is not found, as it means not BVB bank statement CSV
    let beginning = content.find("\"Buchungstag\";\"Valuta\"").unwrap();
    let _ = writeln!(writer, "Date,Payee,Memo,Amount"); // Checking all the other writeln cases
    let transactions = content.get(beginning..).unwrap();
    let mut complete = "".to_string();
    for line in transactions.lines().skip(1) {
        // Memo is split to multiple lines, " " needed to avoid joining words
        // even though sometimes that means an extra space
        if complete.is_empty() {
            complete = line.to_string();
        } else {
        complete = complete + " " + line;
        }
        // "S" or "H" is the last field on a complete transaction
        if !(complete.ends_with("\"S\"")) && !(complete.ends_with("\"H\"")) {
            continue;
        } else {
            // Balance lines in the end have very little detail and are easy to spot
            if complete.contains(";;;;;;;;;") {
                break;
            };
            // At this point 'complete' in whole and ready for converting

            let parts: Vec<&str> = complete.split(";")
                .map(|x| x.trim_matches('\"'))
                .collect();
            let result = writeln!(writer, "{},{},{},{}", convert_date(&parts),
                                  convert_payee(&parts),
                                  convert_memo(&parts),
                                  convert_amount(&parts));
            match result {
                Ok(_) => { },
                Err(e) => {
                    // TODO could use ExitFailure?
                    panic!("Printing a line failed! {}", e);
                }
            }
            complete = "".to_string(); 
        }
    }
}

fn main() -> Result<(), ExitFailure> {
    //TODO Help txt with 'cli'
    let args = ConversionCLI::from_args();
    let filename = args.infile.as_path().display().to_string();

    //TODO Could use BufReader, though month or two of personal finance fits into memory fine
    let content = std::fs::read_to_string(&args.infile)
        .with_context(|_| format!("Could not read file '{}'", filename))?;
    
    format(content, &mut std::io::stdout());

    Ok(())
}


#[test]
fn test_format_minimal() {
    let input = r#"
"Bremische Volksbank eG"

"Umsatzanzeige"

"BLZ:";"29190024";;"Datum:";"24.06.2020"
"Konto:";"123456700";;"Uhrzeit:";"23:21:46"
"Abfrage von:";"Mila Mustermann";;"Kontoinhaber:";"Mila Mustermann"

"Zeitraum:";;"von:";"27.05.2020";"bis:";
"Betrag in EUR:";;"von:";" ";"bis:";" "
"Sortiert nach:";"Buchungstag";"absteigend"

"Buchungstag";"Valuta";"Auftraggeber/Zahlungsempfänger";"Empfänger/Zahlungspflichtiger";"Konto-Nr.";"IBAN";"BLZ";"BIC";"Vorgang/Verwendungszweck";"Kundenreferenz";"Währung";"Umsatz";" "
"24.06.2020";"24.06.2020";"ISSUER";"ROSSMANN VIELEN DANK";;"DE89370400440532013000";;"GENODEF1HB1";"Basislastschrift
DIRK ROSSMANN GMBH/BREMEN/D
E
23.06.2020 um 14:02:16 Uhr
65338653/441343/ECTL/
29190024/123456700/1/1222
REF 107842/260046";;"EUR";"10,90";"S"
"27.05.2020";;;;;;;;;"Anfangssaldo";"EUR";"1.176,91";"H"
"24.06.2020";;;;;;;;;"Endsaldo";"EUR";"2666,55";"H""#;
    let expected = r#"Date,Payee,Memo,Amount
23.06.2020,ROSSMANN VIELEN DANK,Basislastschrift DIRK ROSSMANN GMBH/BREMEN/D E 23.06.2020 um 14:02:16 Uhr 65338653/441343/ECTL/ 29190024/123456700/1/1222 REF 107842/260046,-10.90
"#;
    let mut result = Vec::new();
    format(input.to_string(), &mut result);
    let string_output = String::from_utf8(result).unwrap();
    assert_eq!(expected, string_output);
}

#[test]
fn test_convert_date_no_memo() {
    let input = vec!["28.06.2020",
        "28.06.2020",
        "",
        "",
        "",
        "",
        "",
        "",
        ""]; // memo field
    assert_eq!("28.06.2020", convert_date(&input));
}

#[test]
fn test_convert_date_from_memo() {
    let input = vec!["28.06.2020",
        "28.06.2020",
        "",
        "",
        "",
        "",
        "",
        "",
        "26.06.2020"];
    assert_eq!("26.06.2020", convert_date(&input));
}


#[test]
fn test_convert_payee_simple_input() {
    let input = vec!["28.6.2020",
        "28.6.2020",
        "ISSUER",
        "Tolle Laden GmbH"];
    assert_eq!("Tolle Laden GmbH", convert_payee(&input));
}

#[test]
fn test_convert_payee_removes_commas() {
    let input = vec!["28.6.2020",
        "28.6.2020",
        "ISSUER",
        "DANKE, IHR SUPERMARKT"];
    // Will joyfully break if the chatty formats ever get cleaned real good
    assert_eq!("DANKE IHR SUPERMARKT", convert_payee(&input));

    // TODO prettified use of case
    // assert_eq!("Tolle Laden GmbH", convert_payee("\"TOLLE LADEN GMBH\""));
    // TODO for pruning the chattiness from the most usual suspects
    // assert_eq("Lidl", convert_payee("\"DANKE, IHR LIDL\""));
}

#[test]
fn test_get_paypal_payee() {
    assert_eq!("EBAY", get_paypal_payee("Basislastschrift . EBAY EBAY.C Ihr Einkauf bei \
    EBAY EBAY.C EREF: 10074 93828595  PAYPAL MREF: 5VRJ 224NEADVG CRED: LU96ZZZ0000 \
    000000000000058 IBAN: DE885 00700100175526303 BIC: DEUT DEFF".to_string()))
}

#[test]
fn test_convert_payee_paypal_payee() {
        let input = vec!["28.6.2020",
        "28.6.2020",
        "ISSUER",
        "PayPal (Europe)",
        "",
        "IBAN",
        "",
        "BIC",
        "Basislastschrift . EBAY EBAY.C Ihr Einkauf bei EBAY EBAY.C EREF: 10074 93828595  \
        PAYPAL MREF: 5VRJ 224NEADVG CRED: LU96ZZZ0000 000000000000058 IBAN: DE885 \
        00700100175526303 BIC: DEUT DEFF"];
    assert_eq!("EBAY", convert_payee(&input));
}

#[test]
fn test_convert_payee_paypal_unknown_recipient() {
        let input = vec!["28.6.2020",
        "28.6.2020",
        "ISSUER",
        "PayPal (Europe)",
        "",
        "IBAN",
        "",
        "BIC",
        "Basislastschrift . , Ihr Einkauf bei , EREF: 10074 93828595  PAYPAL MREF: 5VRJ \
        224NEADVG CRED: LU96ZZZ0000 000000000000058 IBAN: DE885 00700100175526303 \
        BIC: DEUT DEFF"];
    assert_eq!("Unknown via PayPal", convert_payee(&input));
}


#[test]
fn test_convert_memo() {
    let input = vec!["28.6.2020",
        "28.6.2020",
        "ISSUER",
        "SUPERMARKT",
        "",
        "IBAN",
        "",
        "BIC",
        "MEMO"];
    assert_eq!("MEMO", convert_memo(&input));
}


#[test]
fn test_convert_amount_outgoing() {
    let input = vec!["28.6.2020",
        "28.6.2020",
        "ISSUER",
        "SUPERMARKT",
        "",
        "IBAN",
        "",
        "BIC",
        "MEMO",
        "",
        "EUR",
        "11,97",
        "S"];
    assert_eq!("-11.97", convert_amount(&input));
}

