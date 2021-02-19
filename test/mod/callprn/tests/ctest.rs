use callprn;
use callprn::{server,client};


#[test]
fn pub_callprn_test() {
	assert_eq!(true, callprn::connect());
}

#[test]
fn pub_server_callprn_test() {
	assert_eq!(true, server::connect());
}

#[test]
fn pub_client_callprn_test() {
	assert_eq!(true, client::connect());
}