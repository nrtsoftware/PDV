use crate::prelude::*;
pub mod user_controller;

pub fn user_controller() {
    user_controller::search_user();
    user_controller::database_test();
}


pub fn boot() {
    user_controller();
}