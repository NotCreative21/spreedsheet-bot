use std::io::Result;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use std::fs::read_to_string;
use serde::{Serialize, Deserialize};


use google_sheets4::api::ValueRange;
use std::default::Default;
use yup_oauth2::{self, read_application_secret};
use google_sheets4::Sheets;

pub struct Storage {
    db_dir: PathBuf,
    pub backup_dir: PathBuf,
    pub main: PathBuf
}

#[derive(Serialize, Deserialize)]
pub struct Creds {
    client_id: String,
    project_id: String,
    auth_uri: String,
    token_uri: String,
    autho_provider_x509_cert_url: String,
    client_secret: String,
    redirect_uris: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct CredWrapper {
    installed: Creds 
}

impl Storage {
    fn new(path: String) -> Storage {
        let path = match &path[path.len() - 1..] {
            "/" => &path[..1],
            _ => path.as_str()
        };
       create_dir_all(path.to_owned() + "/")
           .expect("failed to access directory");

       let storage = Storage {
           db_dir: PathBuf::from(path),
           backup_dir: PathBuf::from(path.to_owned() + "/backups"),
           main: PathBuf::from(path.to_owned() + "/main")
       };

       storage
    }
    pub fn clear(&self, location: String) -> Result<()> {
        match location.as_str() {
            "main" => {
                remove_dir_all(self.main.clone())?;
            },
            _ => {}
        };
        Ok(())
    }
    //pub fn load(&self, )
    //pub fn backup
    pub async fn push() -> Result<()> {
        //let details: CredWrapper = serde_json::from_str(&read_to_string("./credentials.json").unwrap()).unwrap();
        let secret: yup_oauth2::ApplicationSecret = read_application_secret("./credentials.json")
            .await
            .expect("invalid");

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await
        .unwrap();

        let hub = Sheets::new(
            hyper::Client::builder()
                .build(hyper_rustls::HttpsConnector::with_native_roots()), 
            auth
        );

        let req = ValueRange {
            major_dimension: Some("COLUMNS".to_string()),
            range: Some("A1".to_string()),
            values: Some(vec![vec!["hi".to_string()]])
        };

        let result = hub
            .spreadsheets()
            .values_append(req, "1d632CaklmwAhmZ7msfOZeyc1i3Xd3fRXPXNDAzJjNWk", "A1")
            .doit()
            .await;

        println!("{:?}", result);


        Ok(())
    }

    pub fn pull(&self) {

    }
}
