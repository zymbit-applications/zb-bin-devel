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
pub struct Args {
    pub use_hw: bool,
    pub zb_version: Option<String>,
}

pub fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut use_hw = false;
    let mut zb_version = None;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                println!("usage: zb-install [--with-hardware-signing] [--zb-version <latest|VERSION>]");
                println!("       zb-install [-h | --help]");
                std::process::exit(0);
            }
            Long("with-hardware-signing") => use_hw = true,
            Long("zb-version") => zb_version = Some(parser.value()?.parse()?),
            _ => return Err(arg.unexpected())
        }
    }

    Ok(Args {
        use_hw,
        zb_version,
    })
}