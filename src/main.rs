// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use crate::network::switch::Switch;

mod network;

fn main() {
    let mut switch = Switch::new("192.168.1.197", "admin", "Testing123!");

    loop {
        let send = switch.send_command("show network");

        println!("{:?}", send);
    }
}
