#![feature(core,libc)]

extern crate libc;
extern crate jack;
use jack::{JackClient,JackPort};

#[test]
fn new_client_and_close() {
    let client = JackClient::open("new_client",jack::JackNullOption);
    assert!(client.close());
}

#[test]
fn client_name_size() {
    let size = JackClient::name_size();
    assert!(size == 33);
}

#[test]
fn get_client_name() {
    let name = "check_me";
    let client = JackClient::open(name,jack::JackNullOption);
    let get_name = client.get_name();
    client.close();
    assert!(get_name.as_slice() == name);
}

#[test]
fn activate() {
    let client = JackClient::open("activate_client",jack::JackNullOption);
    assert!(client.activate());
    assert!(client.deactivate());
    assert!(client.close());
}

#[test]
fn port_test() {
    let client = JackClient::open("port_test",jack::JackNullOption);
    let port = client.register_port("test_port",
                                    jack::JACK_DEFAULT_AUDIO_TYPE,
                                    jack::JackPortIsOutput | jack::JackPortIsTerminal,
                                    0);
    assert!(port.name().as_slice() == "port_test:test_port");
    assert!(port.short_name().as_slice() == "test_port");
    assert!(port.flags() == jack::JackPortIsTerminal | jack::JackPortIsOutput);
    assert!(port.get_type().as_slice() == jack::JACK_DEFAULT_AUDIO_TYPE);
    assert!(client.port_is_mine(port));
    assert!(port.connected() == 0);
    client.unregister_port(&port);
    assert!(client.close());
}

#[test]
fn port_connect_test() {
    let client = JackClient::open("port_connect_test",jack::JackNoStartServer);
    let in_port = client.register_port("input_test",
                                       jack::JACK_DEFAULT_AUDIO_TYPE,
                                       jack::JackPortIsInput,
                                       0);
    let out_port = client.register_port("output_test",
                                        jack::JACK_DEFAULT_AUDIO_TYPE,
                                        jack::JackPortIsOutput,
                                        0);

    client.activate(); // need to be activated to connect ports

    let res = client.connect("port_connect_test:output_test",
                             "port_connect_test:input_test");
    match res {
        Ok(_) => {}
        Err(s) => panic!(s)
    }

    assert!(in_port.connected() == 1);
    assert!(out_port.connected() == 1);

    let conns = in_port.get_connections();
    assert!(conns[0].as_slice() == "port_connect_test:output_test");

    assert!(client.disconnect("port_connect_test:output_test",
                              "port_connect_test:input_test"));

    assert!(in_port.connected() == 0);
    assert!(out_port.connected() == 0);

    let noconns = in_port.get_connections();
    assert!(noconns.len() == 0);

    assert!(client.close());
}



#[test]
fn port_type_size() {
    assert!(JackPort::type_size() == 32); // Might fail if this changes
}

#[test]
fn port_alias() {
    let client = JackClient::open("port_alias_test",jack::JackNoStartServer);
    let in_port = client.register_port("alias_test",
                                       jack::JACK_DEFAULT_AUDIO_TYPE,
                                       jack::JackPortIsInput,
                                       0);
    assert!(in_port.set_alias("alias1"));
    let aliases = in_port.get_aliases();
    assert!(aliases.len() == 1);
    assert!(aliases[0].as_slice() == "alias1");
    assert!(in_port.set_alias("alias2"));
    let aliases2 = in_port.get_aliases();
    assert!(aliases2.len() == 2);
    assert!(aliases2[0].as_slice() == "alias1");
    assert!(aliases2[1].as_slice() == "alias2");
    assert!(client.close());
}
