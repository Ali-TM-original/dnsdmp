use cli_table::{format::Justify, print_stdout,Style,Table};use easy_scraper::Pattern;
use regex::Regex;

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct Scrapper {
    pub document: String,
}

#[allow(dead_code)]
#[derive(Table)]
#[table(color = "Color::Red")]
struct DataTable {
    #[table(title = "ID", justify = "Justify::Right")]
    id: u64,
    #[table(title = "Record")]
    record: String,
    #[table(title = "Ip")]
    ip: String,
    #[table(title = "Supplier")]
    supplier: String,
}

#[allow(non_snake_case)]
impl Scrapper {
    pub fn new(HtmlDocument: String) -> Scrapper {
        Scrapper {
            document: HtmlDocument,
        }
    }
    // literally worlds worst solution but this is what i come up at 3 Am :0
    pub fn ProduceData(&self) {

        // length of all vector remains constant throughout 
        let mut Ipvector:Vec<String>  = vec![];
        let mut Urlvector: Vec<String> = vec![];
        let mut Identifiervec : Vec<String> = vec![];

        let mut Prettyvec:Vec<DataTable> = vec![];

        // These are made as to capture individual data since id maye i scraped like a bastard        
        let ipregex = Regex::new(r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
        let urlregex = Regex::new(
            r"[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
        )
        .unwrap();

        let testselector = Pattern::new(
            r#"
        <tbody><tr><td class="col-md-4">{{name}}<br>
        "#,
        )
        .unwrap();

        let mut ms = testselector.matches(&self.document);
        ms.remove(0);
        for data in ms.iter() {
            for (_, valu) in data {
                let ipcaps = ipregex.captures(valu).is_none();
                let urlcaps = urlregex.captures(valu).is_none();
                if ipcaps == false {
                    Ipvector.push(valu.into());
                } else if urlcaps == false {
                    // add to a url vector
                    Urlvector.push(valu.into());
                } else {
                    // add to a name vector
                    Identifiervec.push(valu.into())
                }
            }
        }

        for n in 0..Ipvector.len(){

            let record = Urlvector[n].clone();
            let ip = Ipvector[n].clone();
            let supplier = Identifiervec[n].clone();

            Prettyvec.push(
                DataTable{
                    id: n as u64,
                    record: record,
                    ip:ip,
                    supplier:supplier
                }
            )
        }
        print_stdout(Prettyvec.table().bold(true).intense(true)).unwrap();
    }
}
