use crate::prelude::*;
mod user_controller;

fn user_controller() {
    user_controller::search_user();
    user_controller::database_test();
}


pub fn boot() {
    user_controller();
}