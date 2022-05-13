
#[allow(unused_imports)]
use scraper::{Html, Selector};
use crate::scrapper::scrap::Scrapper;

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct Urlhandler {
    pub Baseurl: String,
    pub targeturl: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RequestData {
    csrfmiddlewaretoken: String,
    targetip: String,
    user: String,
}


#[allow(non_snake_case, dead_code)]
impl Urlhandler {
    pub fn new(Baseurl: String, targeturl: String) -> Urlhandler {
        Urlhandler {
            Baseurl: Baseurl,
            targeturl: targeturl,
        }
    }

    pub fn request(&self) {
        let client = reqwest::blocking::Client::new();
        let response = client.get(&self.Baseurl).send().unwrap_or_else(|_| panic!("Cannot read Response"));

        let document = response
            .text()
            .unwrap_or_else(|_| panic!("Could not parse get request"));
        let cookie = self.Parse_Cookie(document);
        let document = self.Get_Data(cookie);
        Scrapper::new(document).ProduceData();
    }

    fn Parse_Cookie(&self, document: String) -> String {
        // this is for parsing the entire document
        let doc = Html::parse_document(&document);
        let selector = Selector::parse(r#"input[name="csrfmiddlewaretoken"]"#).unwrap();
        let mut cookie = String::from("");
        for elements in doc.select(&selector) {
            let title = elements.value().attr("value").unwrap();
            cookie = title.into();
        }
        return cookie;
    }

    fn Get_Data(&self, CsrfTok: String)->String {

        let p = RequestData  { 
            csrfmiddlewaretoken: CsrfTok.to_owned(),
            targetip: self.targeturl.to_owned(),
            user: "free".to_owned()
        };


        let client = reqwest::blocking::Client::new();
        let res = client
            .post(&self.Baseurl)
            .header("csrftoken", CsrfTok.clone())
            .header("csrfmiddlewaretoken", CsrfTok.clone())
            .header("Referer", self.Baseurl.clone())
            .header("Cookie", "csrftoken=".to_owned()+&CsrfTok)
            .header("Content-Type", "application/json")
            .form(&p).send();
            let htmldoc = res.unwrap().text();
            htmldoc.unwrap_or_else(|_| panic!("Could not fetch document"))

    }
}
