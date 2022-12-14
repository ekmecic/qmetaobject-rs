/* Copyright (C) 2018 Olivier Goffart <ogoffart@woboq.com>
   Copyright (C) 2021 ivan tkachenko a.k.a. ratijas <me@ratijas.tk>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use semver::Version;

fn main() {
    let cargo_target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let cargo_target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();

    if (cargo_target_os == "windows") && (cargo_target_env != "msvc") {
        println!("cargo:warning=On Windows, WebEngine module is only available under MSVC 2017 or MSVC2019.");
        println!("cargo:rustc-cfg=no_qt");
    }

    let qt_version = std::env::var("DEP_QT_VERSION")
        .unwrap()
        .parse::<Version>()
        .expect("Parsing Qt version failed");

    if qt_version >= Version::new(6, 0, 0) && qt_version < Version::new(6, 2, 0) {
        println!(
            "cargo:warning=WebEngine is not supported on Qt {} yet. It is planned for Qt 6.2 LTS.",
            qt_version
        );
        println!("cargo:rustc-cfg=no_qt");
    }
}
