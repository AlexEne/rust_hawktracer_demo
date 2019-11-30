extern crate rayon;
extern crate rust_hawktracer;

use rayon::prelude::*;
use rust_hawktracer::*;
use std::{thread, time};

#[hawktracer(fancy_name)]
fn profile_me() {
    for _ in 0..100 {
        scoped_tracepoint!(another_inner_scope);
    }    
}

fn main() {
    let hawktracer_instance = HawktracerInstance::new();
    let _listener = hawktracer_instance.create_listener(HawktracerListenerType::ToFile {
        file_path: "fancy_tracepoint.out".into(),
        buffer_size: 4096,
    });
    {
        scoped_tracepoint!(_fe);
    }
    println!("Hello, world!");

    {
        scoped_tracepoint!(_outer_scope);
        let arr = vec![0; 3000];

        arr.iter().for_each(|_| {
            scoped_tracepoint!(_inner_scope);
            profile_me();
            thread::sleep(time::Duration::from_millis(10));
        });
    }
}
