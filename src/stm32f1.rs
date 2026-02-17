// Copyright 2026 Jelly Terra <jellyterra@proton.me>
// Use of this source code form is governed under the MIT license.

use crate::*;

use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use stm32_ioc::McuProject;

pub fn generate(project: McuProject) -> TokenStream {
    let mut decl = vec![];

    let mut gpio_clusters = HashSet::new();

    for pin in &project.io_pins {
        let pin_num = &pin.gpio_name[2..].parse::<i32>().unwrap();
        let pin_name = pin.user_defined_name.clone().unwrap_or(pin.gpio_name.clone());

        let reg = if *pin_num < 8 { "crl" } else { "crh" };

        let mux_mode = match &pin.gpio.mux_mode {
            stm32_ioc::GpioMux::Analog => "analog",
            stm32_ioc::GpioMux::Input => match &pin.gpio.pull_mode {
                stm32_ioc::GpioPullMode::Floating => "floating_input",
                stm32_ioc::GpioPullMode::Up => "pull_up_input",
                stm32_ioc::GpioPullMode::Down => "pull_down_input",
            },
            stm32_ioc::GpioMux::Output(gpio_drive_mode) => match gpio_drive_mode {
                stm32_ioc::GpioDriveMode::PushPull => "push_pull_output",
                stm32_ioc::GpioDriveMode::OpenDrain => "open_drain_output",
            },
        };

        let reg_sym = new_ident(reg);
        let gpio_cluster_sym = new_ident(&format!("GPIO{}", pin.gpio_cluster));
        let into_func_sym = new_ident(&format!("into_{}", mux_mode));
        let user_defined_pin_name_sym = new_ident(&pin_name);
        let pin_name_sym = new_ident(&pin.gpio_name.to_lowercase());

        if !gpio_clusters.contains(&pin.gpio_cluster) {
            decl.push(quote! {
                let mut #gpio_cluster_sym = dp.#gpio_cluster_sym.split(&mut rcc);
            });
        }

        decl.push(quote! {
            let #user_defined_pin_name_sym = #gpio_cluster_sym.#pin_name_sym.#into_func_sym(&mut #gpio_cluster_sym.#reg_sym);
        });

        gpio_clusters.insert(pin.gpio_cluster);
    }

    let expanded = quote! {
        #(#decl)*
    };

    TokenStream::from(expanded)
}