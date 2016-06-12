extern crate yaml_rust;

use std::collections::HashMap;
use yaml_rust::YamlLoader;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Error as IOError;
use yaml_rust::scanner::ScanError;

/// Encapsulation of potential errors
/// that may happen when resolving
/// a kubernetets config
#[derive(Debug)]
pub enum Error {
    /// A failure to resolve a home directory
    Homeless,
    /// IO errors
    IO(IOError),
    /// Failure to parse yaml data
    Yaml(ScanError),
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

/// A type alias for the result operations which may return an `kubecfg::Error`
pub type Result<T> = std::result::Result<T, Error>;

/// Represents a kubernetes cluster and namespace authentication
#[derive(Debug, PartialEq)]
pub struct Context {
    /// The name of a cluster
    pub cluster: Option<String>,
    /// The name of a namespace
    pub namespace: Option<String>,
    /// The name of a user
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

/// Represents a way to resolve content
#[derive(Debug, PartialEq)]
pub enum Content {
    /// Location of content on disk
    Path(String),
    /// Raw content string
    Data(String),
}

/// Describes information needed to resolve
/// a connection to a cluster
#[derive(Debug, PartialEq)]
pub struct Cluster {
    /// The clusters supported api version
    pub api_version: String,
    /// The server URI
    pub server: String,
    /// Predicate used to determine if a client should skip tls verification
    pub insecure_skip_tls_verify: bool,
    /// Content used by client to certify the server is authentic
    pub certificate_authority: Option<Content>,
}

/// User authentication credentials
/// to authenticate requests to a kubernetes cluster
#[derive(Debug, PartialEq)]
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

/// Represents local kubernetes configuration settings
#[derive(Debug, PartialEq)]
pub struct Config {
    /// A map of cluster name to cluster
    pub clusters: HashMap<String, Cluster>,
    /// A map of context name to context
    pub contexts: HashMap<String, Context>,
    /// A map of user name to user
    pub users: HashMap<String, User>,
    /// The current context's name
    pub current_context: String,
}

impl Config {
    /// Reads a Config object from the default location on disk
    pub fn from_std_path() -> Result<Config> {
        let home = try!(std::env::home_dir().ok_or(Error::Homeless))
            .join(".kube")
            .join("config");
        Self::from_path(home)
    }

    /// Reads a Config object from a custom location on disk
    pub fn from_path<P>(path: P) -> Result<Config>
        where P: AsRef<Path>
    {
        let mut f = try!(File::open(path));
        let mut s = String::new();
        try!(f.read_to_string(&mut s));
        Self::from_str(s.as_ref())
    }

    /// Reads a Config object from a raw string payload
    pub fn from_str(raw: &str) -> Result<Config> {
        let yml = try!(YamlLoader::load_from_str(raw));

        let doc = &yml[0];
        let mut clusters = HashMap::new();
        let mut contexts = HashMap::new();
        let mut users = HashMap::new();
        let current_context = doc["current-context"].as_str().map(|c| c.to_owned()).unwrap();

        for binding in doc["users"].as_vec().unwrap().iter() {
            let name = &binding["name"];
            let user = &binding["user"];
            let token = user["token"].as_str().map(|s| s.to_owned());
            let username = user["username"].as_str().map(|s| s.to_owned());
            let password = user["password"].as_str().map(|s| s.to_owned());
            let client_cert =
                user["client-certificate"].as_str().map(|s| Content::Path(s.to_owned()));
            let client_key = user["client-key"].as_str().map(|s| Content::Path(s.to_owned()));
            let client_cert_data =
                user["client-certificate-data"].as_str().map(|s| Content::Data(s.to_owned()));
            let client_key_data =
                user["client-key-data"].as_str().map(|s| Content::Data(s.to_owned()));
            let u = User::new(client_cert_data.or(client_cert),
                              client_key_data.or(client_key),
                              token,
                              username,
                              password);
            users.insert(name.as_str().map(|s| s.to_owned()).unwrap(), u);
        }

        for binding in doc["contexts"].as_vec().unwrap().iter() {
            let name = &binding["name"];
            let context = &binding["context"];

            let cluster = context["cluster"].as_str().map(|s| s.to_owned());
            let namespace = context["namespace"].as_str().map(|s| s.to_owned());
            let user = context["user"].as_str().map(|s| s.to_owned());
            let ctx = Context::new(cluster, namespace, user);
            contexts.insert(name.as_str().map(|s| s.to_owned()).unwrap(), ctx);
        }

        for binding in doc["clusters"].as_vec().unwrap().iter() {
            let name = &binding["name"];
            let cluster = &binding["cluster"];

            let server = cluster["server"].as_str().map(|s| s.to_owned());
            let cluster_version = cluster["api-version"].as_str().map(|s| s.to_owned());
            let cert_authority_path =
                cluster["certificate-authority"].as_str().map(|s| Content::Path(s.to_owned()));
            let cert_authority_data =
                cluster["certificate-authority-data"].as_str().map(|s| Content::Data(s.to_owned()));
            let skip_tls_verify = cluster["insecure-skip-tls-verify"].as_bool();
            let cl = Cluster::new(cluster_version,
                                  server,
                                  skip_tls_verify,
                                  cert_authority_data.or(cert_authority_path));
            clusters.insert(name.as_str().map(|s| s.to_owned()).unwrap(), cl);
        }
        Ok(Config {
            clusters: clusters,
            contexts: contexts,
            users: users,
            current_context: current_context,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn parse() {
        let yaml = r#"current-context: federal-context
apiVersion: v1
clusters:
- cluster:
    api-version: v1
    server: http://cow.org:8080
  name: cow-cluster
- cluster:
    certificate-authority: path/to/my/cafile
    server: https://horse.org:4443
  name: horse-cluster
- cluster:
    insecure-skip-tls-verify: true
    server: https://pig.org:443
  name: pig-cluster
contexts:
- context:
    cluster: horse-cluster
    namespace: chisel-ns
    user: green-user
  name: federal-context
- context:
    cluster: pig-cluster
    namespace: saw-ns
    user: black-user
  name: queen-anne-context
kind: Config
preferences:
  colors: true
users:
- name: blue-user
  user:
    token: blue-token
- name: green-user
  user:
    client-certificate: path/to/my/client/cert
    client-key: path/to/my/client/key
"#;

        assert!(Config::from_str(yaml).is_ok());
    }
}
