extern crate csv;
extern crate opener;
extern crate warp;


use warp::{
    Filter,
    path,
};

use std::{
    thread::spawn,
    sync::mpsc::channel,
    collections::HashMap,
    path::PathBuf,
    io::prelude::*,
    io::BufReader,
};

fn main() {
    let (tx, rx) = channel();
    let fork = spawn(move || {
        let _ = rx.recv();
        println!("");
        open_browser();
    });

    let form = path("submit")
        .and(warp::body::form())
        .map(|params: HashMap<String, String>| {
            match write_form(params) {
                Ok(_) => warp::http::Response::builder()
                            .header("location", "/")
                            .status(301)
                            .body(format!("")),
                Err(e) => warp::http::Response::builder()
                            .status(500)
                            .body(format!("failed to write form to csv\n{}", e)),
            }
        });
    let server = warp::serve(warp::fs::dir("assets").or(form));
    tx.send(()).unwrap();
    ::std::panic::set_hook(Box::new(|_| {
        println!("Failed to start server")
    }));
    let _ = ::std::panic::catch_unwind(move || {
        server.run(([127,0,0,1], 8787));
    });
    let _ = fork.join();
}

fn open_browser() {
    let _ = opener::open("http://localhost:8787");
}

fn write_form(params: HashMap<String, String>) -> Result<(), String> {
    let path = PathBuf::from("form-entries.1.csv");
    let mut file = ::std::fs::OpenOptions::new().create(true).append(true).read(true).open(&path).map_err(map_err)?;
    let mut s = String::new();
    let (headers, needs_header) = if file.metadata().map_err(map_err)?.len() < 1 {
        (params.keys().map(|s| s.as_str()).collect::<Vec<&str>>(), true)
    } else {
        let mut reader = BufReader::new(&mut file);
        reader.read_line(&mut s).map_err(map_err)?;
        (s.split(',').into_iter().map(|f| f.trim()).collect::<Vec<&str>>(), false)
    };
    let mut writer = csv::Writer::from_writer(&file);
    if needs_header {
        writer.write_record(&headers)
            .map_err(map_err)?;
    }
    for header in headers {
        if let Some(header) = params.get(header) {
            writer.write_field(header).map_err(map_err)?;
        } else {
            writer.write_field("").map_err(map_err)?;
        }
    }
    writer
        .write_record(None::<&[u8]>)
        .map_err(map_err)?;
    Ok(())
}

fn map_err(e: impl ::std::error::Error) -> String {
    format!("{}", e)
}