#[macro_use]
extern crate rustfbp;
extern crate capnp;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

agent! {
    example_wrangle_processchunk_file_open, edges(path, value_string, file_error)
    inputs(input: path),
    inputs_array(),
    outputs(output: value_string, error: file_error),
    outputs_array(),
    option(),
    acc(),
    fn run(&mut self) -> Result<()> {
        let mut ip = try!(self.ports.recv("input"));
        let path: path::Reader = try!(ip.read_schema());

        let path = try!(path.get_path());

        if path == "end" {
            let mut new_ip = IP::new();
            {
                let mut ip = new_ip.build_schema::<value_string::Builder>();
                ip.set_value("end");
            }
            try!(self.ports.send("output", new_ip));
        }
        else {
            let file = match File::open(path) {
                Ok(file) => { file },
                Err(_) => {
                    let mut new_ip = IP::new();
                    {
                        let mut ip = new_ip.build_schema::<file_error::Builder>();
                        ip.set_not_found(&path);
                    }
                    let _ = self.ports.send("error", new_ip);
                    return Ok(());
                }
            };

            let file = BufReader::new(&file);
            for line in file.lines() {
                let l = try!(line);
                let mut new_ip = IP::new();
                {
                    let mut ip = new_ip.build_schema::<value_string::Builder>();
                    ip.set_value(&l);
                }
                try!(self.ports.send("output", new_ip));
            }
        }
        Ok(())
    }
}