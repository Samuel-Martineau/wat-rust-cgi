extern crate rust_cgi as cgi;
use maud::{html, DOCTYPE};
use size::Size;
use sysinfo::System;

cgi::cgi_main! { |request: cgi::Request| -> cgi::Response {
    let mut sys = System::new_all();
    sys.refresh_all();

    let req = format!("{request:#?}");

    let markup = html! {
        (DOCTYPE)
        html {
            head {
                title { "System Information" }
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css";
            }
            body {
                main.container {
                    h1 { "System Information" }
                    ul {
                        li { strong { "Total memory: " } (Size::from_bytes(sys.total_memory())) }
                        li { strong { "Used memory: " } (Size::from_bytes(sys.used_memory())) }
                        li { strong { "Total swap: " } (Size::from_bytes(sys.total_swap())) }
                        li { strong { "Used swap: " } (Size::from_bytes(sys.used_swap())) }
                    }
                    hr;
                    ul {
                        li { strong { "OS: " } (System::name().unwrap()) " " (System::os_version().unwrap()) }
                        li { strong { "Kernel: " } (System::kernel_version().unwrap()) }
                        li { strong { "Host: " } (System::host_name().unwrap()) }
                    }
                    hr;
                    pre { code { (req) } }
                    hr;
                    p {
                        "My homepage :)"
                    }
                }
            }
        }
    };

    cgi::html_response(200, markup.into_string())
} }
