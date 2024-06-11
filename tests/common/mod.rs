use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

pub struct ZkCluster {
    process: Child,
    pub connect_string: String,
    closed: bool,
}

impl ZkCluster {
    pub fn start(instances: usize) -> ZkCluster {
        let mut process = match Command::new("java")
            .arg("-jar")
            .arg("zk-test-cluster/target/main.jar")
            .arg(instances.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
            Ok(p) => p,
            Err(e) => panic!("failed to start ZkCluster: {}", e),
        };
        let connect_string = Self::read_connect_string(&mut process);
        ZkCluster {
            process,
            connect_string,
            closed: false,
        }
    }

    pub fn read_connect_string(process: &mut Child) -> String {
        let mut reader = BufReader::new(process.stdout.as_mut().unwrap());
        let mut connect_string = String::new();
        if reader.read_line(&mut connect_string).is_err() {
            panic!("Couldn't read ZK connect_string")
        }
        connect_string.pop(); // remove '\n'
        connect_string
    }

    pub fn kill_an_instance(&mut self) {
        self.process.stdin.as_mut().unwrap().write(b"k").unwrap();
    }

    pub fn shutdown(&mut self) {
        if !self.closed {
            self.process.stdin.as_mut().unwrap().write(b"q").unwrap();
            assert!(self.process.wait().unwrap().success());
            self.closed = true
        }
    }
}

impl Drop for ZkCluster {
    fn drop(&mut self) {
        self.shutdown()
    }
}