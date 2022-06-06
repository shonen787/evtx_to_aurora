use evtx::EvtxParser;
use std::path::PathBuf;
use serde_json::{Result, Value};

fn main() {
    // Change this to a path of your .evtx sample. 
    let fp = PathBuf::from(format!("security.evtx")); 

    
    let mut parser = EvtxParser::from_path(fp).unwrap();
    for record in parser.records_json_value() {
        match record {
            Ok(r) =>
                {
                    if r.data["Event"]["System"]["EventID"] == 4624{
                        println!("{}\n", r.data["Event"])
                    }
                },
            Err(e) => eprintln!("{}", e),
        }
    }
   
    //["Event"]["EventData"]["IpAddress"]
}