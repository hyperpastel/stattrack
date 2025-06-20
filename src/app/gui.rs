/*
 * This application type is not implemented yet
 */


#![allow(unused)]

use crate::app::Context;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct GuiApplication {}

impl GuiApplication {
    pub async fn run(&self, ctx: Arc<Context>) {
        unimplemented!()
    }
}
