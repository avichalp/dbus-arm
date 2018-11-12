extern crate dbus;

use std::thread;
use std::time;
use dbus::{Connection, BusType, NameFlag, Message};
use dbus::arg::Array;
use dbus::tree::Factory;


fn main() {
    // Let's start by starting up a connection to the session bus and register a name.
    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name("com.example.dbustest", NameFlag::ReplaceExisting as u32).unwrap();

    // The choice of factory tells us what type of tree we want,
    // and if we want any extra data inside. We pick the simplest variant.
    let f = Factory::new_fn::<()>();


    // We create a tree with one object path inside and make that path introspectable.
    let tree = f.tree(()).add(f.object_path("/hello", ()).introspectable().add(

        // We add an interface to the object path...
        f.interface("com.example.dbustest", ()).add_m(

            // ...and a method inside the interface.
            f.method("Hello", (), move |m| {

                // Simulate Interagtion APIs
                thread::sleep(time::Duration::from_millis(200));
                
                // This is the callback that will be called when another peer on the bus calls our method.
                // the callback receives "MethodInfo" struct and can return either an error, or a list of
                // messages to send back.

                let name: &str = m.msg.read1()?;
                let s = format!("Hello {}!", name);
                let mret = m.msg.method_return().append1(s);

                // one messages will be returned - one is the method return (and should always be there),
                Ok(vec!(mret))

            // Our method has one output argument and one input argument.
            }).outarg::<&str,_>("reply")
            .inarg::<&str,_>("name")

        )
    ));

    // We register all object paths in the tree.
    tree.set_registered(&c, true).unwrap();

    // We add the tree to the connection so that incoming method calls will be handled
    // automatically during calls to "incoming".
    c.add_handler(tree);

    // run a client in a different thread
    thread::spawn(move || {
        let c1 = Connection::get_private(BusType::Session).unwrap();

        for i in 0..10000 {
            let message = format!("Message from Controller :: {}", i);

            let t1 = time::Instant::now();
            
            let m1 = Message::new_method_call(
                "com.example.dbustest", "/hello", "com.example.dbustest", "Hello"
            ).unwrap().append1(message);

            let r1 = c1.send_with_reply_and_block(m1, 2000).unwrap();

            // ListNames returns one argument, which is an array of strings.
            let response: &str = r1.get1().expect("Cannot read Reply from Core");

            println!("response :: {:?} :: time taken :: {:?}", response, t1.elapsed());
            
            thread::sleep(time::Duration::from_millis(10));
        }

    });

    // Serve other peers forever.
    loop { c.incoming(1000).next(); }
}
