extern crate yaml_rust;

use std::collections::HashMap;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::{Error as IOError};
use yaml_rust::scanner::ScanError;

pub enum Error {
    UserHome,
    IO(IOError),
    Yaml(ScanError)
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error::IO(error)
    }
}

impl From<ScanError> for Error {
    fn from(error: ScanError) -> Error {
        Error::Yaml(error)
    }
}

#[derive(Debug)]
pub struct Context {
    pub cluster: Option<String>,
    pub namespace: Option<String>,
    pub user: Option<String>,
}

impl Context {
    pub fn new(cluster: Option<String>,
               namespace: Option<String>,
               user: Option<String>)
               -> Context {
        Context {
            cluster: cluster,
            namespace: namespace,
            user: user,
        }
    }
}

#[derive(Debug)]
pub enum Content {
    Path(String),
    Data(String),
}

#[derive(Debug)]
pub struct Cluster {
    pub api_version: String,
    pub server: String,
    pub insecure_skip_tls_verify: bool,
    pub certificate_authority: Option<Content>,
}

#[derive(Debug)]
pub struct User {
    pub client_certificate: Option<Content>,
    pub client_key: Option<Content>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub fn new(client_cert: Option<Content>,
               client_key: Option<Content>,
               token: Option<String>,
               username: Option<String>,
               password: Option<String>)
               -> User {
        User {
            client_certificate: client_cert,
            client_key: client_key,
            token: token,
            username: username,
            password: password,
        }
    }
}

impl Cluster {
    pub fn new(api_version: Option<String>,
               server: Option<String>,
               insecure_skip_tls_verify: Option<bool>,
               certificate_authority: Option<Content>)
               -> Cluster {
        Cluster {
            api_version: api_version.unwrap_or("v1".to_owned()),
            server: server.unwrap_or("http://localhost:8001".to_owned()),
            insecure_skip_tls_verify: insecure_skip_tls_verify.unwrap_or(false),
            certificate_authority: certificate_authority,
        }
    }
}

impl Default for Cluster {
    fn default() -> Cluster {
        Cluster {
            api_version: "v1".to_owned(),
            server: "http://localhost:8001".to_owned(),
            insecure_skip_tls_verify: false,
            certificate_authority: None,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub clusters: HashMap<String, Cluster>,
    pub contexts: HashMap<String, Context>,
    pub users: HashMap<String, User>,
    pub current_context: String,
}

impl Config {
    pub fn from_std_path() -> Result<Config, Error> {
        let mut home = try!(std::env::home_dir().ok_or(Error::UserHome));
        home.push(".kube");
        home.push("config");
        Self::from_path(home)
    }

    pub fn from_path<P>(path: P) -> Result<Config, Error> where P: AsRef<Path> {
        let mut f = try!(File::open(path));
        let mut s = String::new();
        try!(f.read_to_string(&mut s));
        Self::from_str(s.as_ref())
    }

    pub fn from_str(raw: &str) -> Result<Config, Error> {
        let yml = try!(YamlLoader::load_from_str(raw));

        let doc = &yml[0];
        //    println!("{:#?}", doc);
        let mut cluster_map = HashMap::new();
        let mut context_map = HashMap::new();
        let mut user_map = HashMap::new();
        let current_context = doc["current-context"].as_str().map(|c| c.to_owned()).unwrap();

        for c in doc["users"].as_vec().unwrap().iter() {
            let name = &c["name"];
            let user = &c["user"];
            let token = match &user["token"] {
                &Yaml::String(ref value) => Some(value.to_owned()),
                _ => None,
            };
            let username = match &user["username"] {
                &Yaml::String(ref value) => Some(value.to_owned()),
                _ => None,
            };
            let password = match &user["password"] {
                &Yaml::String(ref value) => Some(value.to_owned()),
                _ => None,
            };
            let client_cert = match &user["client-certificate"] {
                &Yaml::String(ref value) => Some(Content::Path(value.to_owned())),
                _ => None,
            };
            let client_key = match &user["client-key"] {
                &Yaml::String(ref value) => Some(Content::Path(value.to_owned())),
                _ => None,
            };
            let client_cert_data = match &user["client-certificate-data"] {
                &Yaml::String(ref value) => Some(Content::Data(value.to_owned())),
                _ => None,
            };
            let client_key_data = match &user["client-key-data"] {
                &Yaml::String(ref value) => Some(Content::Data(value.to_owned())),
                _ => None,
            };
            let u = User::new(client_cert_data.or(client_cert),
                              client_key_data.or(client_key),
                              token,
                              username,
                              password);
            user_map.insert(name.as_str().map(|s| s.to_owned()).unwrap(), u);
        }

        for c in doc["contexts"].as_vec().unwrap().iter() {
            let name = &c["name"];
            let context = &c["context"];

            let cluster = match &context["cluster"] {
                &Yaml::String(ref value) => Some(value.to_owned()),
                _ => None,
            };
            let namespace = match &context["namespace"] {
                &Yaml::String(ref value) => Some(value.to_owned()),
                _ => None,
            };
            let user = match &context["user"] {
                &Yaml::String(ref value) => Some(value.to_owned()),
                _ => None,
            };
            let ctx = Context::new(cluster, namespace, user);
            context_map.insert(name.as_str().map(|s| s.to_owned()).unwrap(), ctx);
        }

        for c in doc["clusters"].as_vec().unwrap().iter() {
            let name = &c["name"];
            let cluster = &c["cluster"];

            let server = match &cluster["server"] {
                &Yaml::String(ref value) => Some(value.to_owned()),
                _ => None,
            };
            let cluster_version = match &cluster["api-version"] {
                &Yaml::String(ref value) => Some(value.to_owned()),
                _ => None,
            };
            let cert_authority_path = match &cluster["certificate-authority"] {
                &Yaml::String(ref value) => Some(Content::Path(value.to_owned())),
                _ => None,
            };
            let cert_authority_data = match &cluster["certificate-authority-data"] {
                &Yaml::String(ref value) => Some(Content::Data(value.to_owned())),
                _ => None,
            };
            let skip_tls_verify = match &cluster["insecure-skip-tls-verify"] {
                &Yaml::Boolean(ref value) => Some(value.to_owned()),
                _ => None,
            };
            let cl = Cluster::new(cluster_version,
                                  server,
                                  skip_tls_verify,
                                  cert_authority_data.or(cert_authority_path));
            cluster_map.insert(name.as_str().map(|s| s.to_owned()).unwrap(), cl);
        }
        Ok(Config {
            clusters: cluster_map,
            contexts: context_map,
            users: user_map,
            current_context: current_context,
        })
    }
}
