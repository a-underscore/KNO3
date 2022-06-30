use std::{env, fs::File, io::Write};

const DEBUG_OPT_LEVEL: &str = "0";
const REACTOR_TECH_GDNLIB_FILE_NAME: &str = "kno3.gdnlib";
const DEBUG_FILE_CONTENTS: &str = include_str!("kno3.gdnlib.debug");
const RELEASE_FILE_CONTENTS: &str = include_str!("kno3.gdnlib.release");

fn main() {
    let mut file = File::create(REACTOR_TECH_GDNLIB_FILE_NAME).unwrap();
    let opt_level = env::var("OPT_LEVEL").unwrap();

    if opt_level == DEBUG_OPT_LEVEL {
        file.write(DEBUG_FILE_CONTENTS.as_bytes()).unwrap();
    } else {
        file.write(RELEASE_FILE_CONTENTS.as_bytes()).unwrap();
    }
}
