use crate::prelude::*;
mod admin;

pub fn migrate() {
    admin::main();
}