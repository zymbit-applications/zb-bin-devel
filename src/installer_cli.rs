// -------------------------------------------------------------------------------------------------------
// Copyright (C) 2023, 2024 Zymbit. All rights reserved.
// Use of this software and associated documentation files (the "Software") is subject to Zymbit
// terms and conditions, and license, found here:
//
// https://www.zymbit.com/terms-and-conditions-of-sale-general/
// https://www.zymbit.com/software-license-general/
//
// Permission to install, use, copy, and modify this software and its documentation for educational,
// research, and not-for-profit purposes, without fee and without a signed licensing agreement,
// is hereby granted, provided that the above copyright notice, this paragraph and the following
// three paragraphs appear in all copies, modifications, and distributions.  Commercial use
// of any kind requires a written license from Zymbit. Redistribution of this software in original or
// modified form requires a written license from Zymbit. Refer to full license for details.
// IN NO EVENT SHALL ZYMBIT INC. OR ITS AGENTS BE LIABLE TO ANY PARTY FOR DIRECT, INDIRECT,
// SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES, INCLUDING LOST PROFITS, ARISING OUT OF
// THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF ZYMBIT HAS BEEN ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
// ZYMBIT SPECIFICALLY DISCLAIMS ANY WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE. THE SOFTWARE AND
// ACCOMPANYING DOCUMENTATION, IF ANY, PROVIDED HEREUNDER IS PROVIDED "AS IS". ZYMBIT HAS
// NO OBLIGATION TO PROVIDE MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
//
// You may not use any Zymbit products in life-critical equipment unless authorized officers
// of the parties have executed a special contract specifically governing such use.
// -------------------------------------------------------------------------------------------------------
use anyhow::{bail, Result};

#[derive(Debug)]
pub struct InstallerArgs {
    pub use_hw: Option<bool>,
    pub zb_version: Option<String>,
    pub rpi_model: Option<String>,
}

pub fn parse_args() -> Result<InstallerArgs> {
    let mut use_hw = None;
    let mut zb_version = None;
    let mut argv = std::env::args().into_iter();
    let mut rpi_model = None;
    argv.next(); // skip argv[0]

    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!(
                    "usage: zb-install [--with-hardware-signing | --with-software-signing] \
                                            [--zb-version <latest|VERSION_TAG>] | [ --rpi-model <rpi4 or rpi5> ]"
                );
                println!("       zb-install [-h | --help]");
                std::process::exit(0);
            }

            "--with-hardware-signing" => use_hw = Some(true),
            "--with-software-signing" => use_hw = Some(false),

            "--zb-version" => {
                if let Some(val) = argv.next() {
                    if !val.starts_with('-') {
                        zb_version = Some(val);
                        continue;
                    }
                }
                bail!("option '--zb-version' requires an argument");
            }

            "--rpi-model" => {
                if let Some(val) = argv.next() {
                    if val == "rpi4" || val == "rpi5" {
                        rpi_model = Some(val);
                        continue;
                    }
                }
                bail!("option '--zb-version' requires an argument");
            }

            _ => bail!("unexpected argument {}", arg),
        }
    }

    Ok(InstallerArgs { use_hw, zb_version, rpi_model })
}
