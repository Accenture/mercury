//
// Copyright 2018-2026 Accenture Technology
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! `graph-packager` - pack, unpack and inspect graph sets (see the library).

fn main() {
    graph_packager::init_logging();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = graph_packager::run(&args, &mut std::io::stdout(), &mut std::io::stderr());
    std::process::exit(code);
}
