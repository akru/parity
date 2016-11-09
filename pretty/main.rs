// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Simple pretty address generator.

#![warn(missing_docs)]
#![cfg_attr(feature="dev", feature(plugin))]
#![cfg_attr(feature="dev", plugin(clippy))]
#![cfg_attr(feature="dev", allow(useless_format))]
#![cfg_attr(feature="dev", allow(match_bool))]

#[macro_use]
extern crate log as rlog;

extern crate uuid;
extern crate regex;
extern crate ethcore;

use std::env;
use uuid::Uuid;
use regex::Regex;
use ethcore::ethstore::{EthStore, SafeAccount, Error};
use ethcore::ethstore::ethkey::{KeyPair, Random, Generator};
use ethcore::ethstore::dir::{ParityDirectory, DirectoryType};

fn new(re: &Regex, password: &str) -> Result<SafeAccount, Error> {
    let iterations: u32 = 10244;
	let acc = Random.generate().expect("secp context has generation capabilities; qed");
	let secret = acc.secret().clone();
    let keypair = try!(KeyPair::from_secret(secret).map_err(|_|Error::CreationFailed));
    let uuid = Uuid::new_v4(); 
    let account = SafeAccount::create(&keypair, uuid.as_bytes().clone(),
                                      password, iterations, String::from(""), "{}".to_owned()); 
    let address = account.address.clone().to_string();
    println!("Check {}...", address);

    match re.is_match(address.as_str()) {
        true  => Ok(account),
        false => new(re, password)
    }
}

fn run() -> Result<SafeAccount, Error> {
    let args: Vec<String> = env::args().collect();
    let magic = args[1].clone();
    let password = args[2].clone();
    let dir = try!(ParityDirectory::create(DirectoryType::Testnet));
    let store = try!(EthStore::open(Box::new(dir)));
    let re: Regex = Regex::new(magic.as_str()).unwrap();
    let account = try!(new(&re, password.as_str()));
    try!(store.save(account.clone()));
    Ok(account)
}

fn main() {
	// Always print backtrace on panic.
	::std::env::set_var("RUST_BACKTRACE", "1");

    match run() { 
        Ok(account) => println!("Found: {}", account.address.clone()),
        Err(err) => println!("Failure {}", err)
    }
}
