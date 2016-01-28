// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Configuration for bldr.
//!
//! This module is populated from the CLI options in `main.rs`, and then passed through to the
//! [command](../command) modules. Check out the `config_from_args(..)` function there for more
//! details.
//!
//! See the [Config](struct.Config.html) struct for the specific options available.

use std::net;

use topology::Topology;
use repo;

#[derive(Debug, Clone, PartialEq, Eq)]
/// An enum with the various CLI commands. Used to keep track of what command was called.
pub enum Command {
    Install,
    Config,
    Start,
    ImportKey,
    ExportKey,
    UploadRepoKey,
    DownloadRepoKey,
    GenerateUserKey,
    GenerateServiceKey,
    ListKeys,
    Encrypt,
    Decrypt,
    Shell,
    Repo,
    Upload,
    Configuration,
}

// We provide a default command primarily so the Config struct can have sane defaults.
impl Default for Command {
    fn default() -> Command {
        Command::Install
    }
}

/// Holds our configuration options.
#[derive(Default, Debug)]
pub struct Config {
    command: Command,
    package: String,
    url: Option<String>,
    topology: Topology,
    group: String,
    path: String,
    deriv: String,
    version: Option<String>,
    release: Option<String>,
    watch: Vec<String>,
    key: String,
    password: Option<String>,
    email: Option<String>,
    expire_days: Option<u16>,
    listen_addr: repo::ListenAddr,
    port: repo::ListenPort,
    gossip_listen: String,
    userkey: Option<String>,
    servicekey: Option<String>,
    infile: Option<String>,
    outfile: Option<String>,
}

impl Config {
    /// Create a default `Config`
    pub fn new() -> Config {
        Config::default()
    }

    /// Set the `Command` we used
    pub fn set_command(&mut self, command: Command) -> &mut Config {
        self.command = command;
        self
    }

    /// Return the command we used
    pub fn command(&self) -> Command {
        self.command.clone()
    }

    /// Set the key
    pub fn set_key(&mut self, key: String) -> &mut Config {
        self.key = key;
        self
    }

    /// Return the key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Set the password
    pub fn set_password(&mut self, password: String) -> &mut Config {
        self.password = Some(password);
        self
    }

    /// Return the password
    pub fn password(&self) -> &Option<String> {
        &self.password
    }

    /// Set the email address
    pub fn set_email(&mut self, email: String) -> &mut Config {
        self.email = Some(email);
        self
    }

    /// Return the email address
    pub fn email(&self) -> &Option<String> {
        &self.email
    }

    /// Set the user key
    pub fn set_user_key(&mut self, userkey: String) -> &mut Config {
        self.userkey = Some(userkey);
        self
    }

    /// Return the user key
    pub fn user_key(&self) -> &Option<String> {
        &self.userkey
    }

    /// Set the service key
    pub fn set_service_key(&mut self, set_servicekey: String) -> &mut Config {
        self.servicekey = Some(set_servicekey);
        self
    }

    /// Return the service key
    pub fn service_key(&self) -> &Option<String> {
        &self.servicekey
    }

    /// Set the input file to encrypt/decrypt
    pub fn set_infile(&mut self, infile: String) -> &mut Config {
        self.infile = Some(infile);
        self
    }

    /// Return the input file to encrypt/decrypt
    pub fn infile(&self) -> &Option<String> {
        &self.infile
    }

    /// Set the input file to encrypt/decrypt
    pub fn set_outfile(&mut self, outfile: String) -> &mut Config {
        self.outfile = Some(outfile);
        self
    }

    /// Return the input file to encrypt/decrypt
    pub fn outfile(&self) -> &Option<String> {
        &self.outfile
    }

    /// Set the package name
    pub fn set_package(&mut self, package: String) -> &mut Config {
        self.package = package;
        self
    }

    /// Return the package name
    pub fn package(&self) -> &str {
        &self.package
    }

    /// Set the key expire days
    pub fn set_expire_days(&mut self, expire_days: u16) -> &mut Config {
        self.expire_days = Some(expire_days);
        self
    }

    pub fn expire_days(&self) -> &Option<u16> {
        &self.expire_days
    }

    /// Set the derivation
    pub fn set_deriv(&mut self, deriv: String) -> &mut Config {
        self.deriv = deriv;
        self
    }

    /// Return the derivation
    pub fn deriv(&self) -> &str {
        &self.deriv
    }

    /// Set the version
    pub fn set_version(&mut self, version: String) -> &mut Config {
        self.version = Some(version);
        self
    }

    /// Return the version
    pub fn version(&self) -> &Option<String> {
        &self.version
    }

    /// Set the release
    pub fn set_release(&mut self, release: String) -> &mut Config {
        self.release = Some(release);
        self
    }

    /// Return the release
    pub fn release(&self) -> &Option<String> {
        &self.release
    }

    /// Set the path
    pub fn set_path(&mut self, path: String) -> &mut Config {
        self.path = path;
        self
    }

    /// Return the path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Set the group
    pub fn set_group(&mut self, group: String) -> &mut Config {
        self.group = group;
        self
    }

    /// Return the group
    pub fn group(&self) -> &str {
        &self.group
    }

    /// Set the watch
    pub fn set_watch(&mut self, watch: Vec<String>) -> &mut Config {
        self.watch = watch;
        self
    }

    /// Return the watch
    pub fn watch(&self) -> &[String] {
        &self.watch
    }

    /// Set the url
    pub fn set_url(&mut self, url: String) -> &mut Config {
        self.url = Some(url);
        self
    }

    /// Return the url
    pub fn url(&self) -> &Option<String> {
        &self.url
    }

    /// Set the topology
    pub fn set_topology(&mut self, topology: Topology) -> &mut Config {
        self.topology = topology;
        self
    }

    /// Return the topology
    pub fn topology(&self) -> &Topology {
        &self.topology
    }

    pub fn set_port(&mut self, port: u16) -> &mut Config {
        self.port = repo::ListenPort(port);
        self
    }

    pub fn repo_addr(&self) -> net::SocketAddrV4 {
        net::SocketAddrV4::new(self.listen_addr.0.clone(), self.port.0.clone())
    }

    pub fn gossip_listen(&self) -> &str {
        &self.gossip_listen
    }

    pub fn set_gossip_listen(&mut self, gl: String) -> &mut Config {
        self.gossip_listen = gl;
        self
    }

    pub fn package_id(&self) -> String {
        if self.version.is_some() && self.release.is_some() {
            format!("{}/{}/{}/{}",
                    &self.deriv,
                    &self.package,
                    self.version.as_ref().unwrap(),
                    self.release.as_ref().unwrap())
        } else if self.version.is_some() {
            format!("{}/{}/{}",
                    self.deriv,
                    self.package,
                    self.version.as_ref().unwrap())
        } else {
            format!("{}/{}", self.deriv, self.package)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Command};
    use topology::Topology;

    #[test]
    fn new() {
        let c = Config::new();
        assert_eq!(*c.topology(), Topology::Standalone);
    }

    #[test]
    fn command() {
        let mut c = Config::new();
        c.set_command(Command::Install);
        assert_eq!(c.command(), Command::Install);
    }

    #[test]
    fn key() {
        let mut c = Config::new();
        c.set_key(String::from("foolio"));
        assert_eq!(c.key(), "foolio");
    }

    #[test]
    fn package() {
        let mut c = Config::new();
        c.set_package(String::from("foolio"));
        assert_eq!(c.package(), "foolio");
    }

    #[test]
    fn path() {
        let mut c = Config::new();
        c.set_path(String::from("foolio"));
        assert_eq!(c.path(), "foolio");
    }

    #[test]
    fn url() {
        let mut c = Config::new();
        c.set_url(String::from("http://foolio.com"));
        assert_eq!(c.url().as_ref().unwrap(), "http://foolio.com");
    }

    #[test]
    fn topology() {
        let mut c = Config::new();
        c.set_topology(Topology::Leader);
        assert_eq!(*c.topology(), Topology::Leader);
    }
}
