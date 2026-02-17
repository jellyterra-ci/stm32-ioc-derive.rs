// Copyright 2026 Jelly Terra <jellyterra@proton.me>
// Use of this source code form is governed under the MIT license.

mod stm32f1;

use core::panic;
use proc_macro::TokenStream;
use std::{env::var, path::PathBuf};
use stm32_ioc::ioc_project_from_file;
use syn::{Ident, LitStr, parse_macro_input};

fn new_ident(ident: &str) -> Ident {
    Ident::new(&ident, proc_macro2::Span::call_site())
}

#[proc_macro]
pub fn ioc(input: TokenStream) -> TokenStream {
    let location = parse_macro_input!(input as LitStr).value();

    let project = ioc_project_from_file(PathBuf::from(location).as_path(), PathBuf::from(var("STM32CUBEMX_DB_MCU_DIR").expect("mcu db variable missing")).as_path()).unwrap();

    match project.family.as_str() {
        "STM32F1" => stm32f1::generate(project),
        _ => panic!("unsupported platform: please explore the source code and contribute your implementation"),
    }
}
