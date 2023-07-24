use crate::prelude::*;
pub(crate) mod admin;

pub fn migrate() {
    admin::main();
}