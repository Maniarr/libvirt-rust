/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <http://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

//! Integration tests using a real qemu:///system connection they are
//! all ignored by default.

extern crate virt;

mod common;

use virt::connect::{Connect, ConnectAuth, ConnectCredential};

#[test]
#[ignore]
fn test_create_domain_with_flags() {
    let c = common::qemu_conn();
    let d = common::build_qemu_domain(&c, "create", false);
    assert_eq!(Ok(0), d.create_with_flags(0));
    assert_eq!(Ok((::virt::domain::VIR_DOMAIN_START_PAUSED, 1)),
               d.get_state());
    assert_eq!(Ok(String::from("libvirt-rs-test-create")), d.get_name());
    common::clean(d);
    common::close(c);
}

#[test]
#[ignore]
fn test_create_storage_pool() {
    let c = common::qemu_conn();
    let p = common::build_storage_pool(&c, "create", false);
    assert_eq!(Ok(0), p.create(0));
    assert_eq!(Ok(String::from("libvirt-rs-test-create")), p.get_name());
    common::clean_pool(p);
    common::close(c);
}

#[test]
#[ignore]
fn test_connection_with_auth() {
    fn callback(creds: &mut Vec<ConnectCredential>) {
        for cred in creds {
            match cred.typed {
                ::virt::connect::VIR_CRED_AUTHNAME => {
                    cred.result = String::from("user");
                    cred.result_set = true;
                }
                ::virt::connect::VIR_CRED_PASSPHRASE => {
                    cred.result = String::from("pass");
                    cred.result_set = true;
                }
                _ => {
                    panic!("Should not be here...");
                }
            }
        }
    };

    let mut auth = ConnectAuth::new(vec![::virt::connect::VIR_CRED_AUTHNAME,
                                         ::virt::connect::VIR_CRED_PASSPHRASE],
                                    callback);
    match Connect::open_auth("test+tcp://127.0.0.1/default", &mut auth, 0) {
        Ok(c) => common::close(c),
        Err(e) => {
            panic!("open_auth did not work: code {}, message: {}",
                   e.code,
                   e.message)
        }
    }
}


#[test]
#[ignore]
fn test_connection_with_auth_wrong() {
    fn callback(creds: &mut Vec<ConnectCredential>) {
        for cred in creds {
            match cred.typed {
                ::virt::connect::VIR_CRED_AUTHNAME => {
                    cred.result = String::from("user");
                    cred.result_set = true;
                }
                ::virt::connect::VIR_CRED_PASSPHRASE => {
                    cred.result = String::from("passwrong");
                    cred.result_set = true;
                }
                _ => {
                    panic!("Should not be here...");
                }
            }
        }
    };

    let mut auth = ConnectAuth::new(vec![::virt::connect::VIR_CRED_AUTHNAME,
                                         ::virt::connect::VIR_CRED_PASSPHRASE],
                                    callback);
    if Connect::open_auth("test+tcp://127.0.0.1/default", &mut auth, 0).is_ok() {
        panic!("open_auth did not work: code {}, message:");
    }
}
