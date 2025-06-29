use dioxus::prelude::*;
use log;

#[component]
pub fn Home() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);

    rsx! {
        div {
            class: "container mx-auto",
            h1 { class: "text-2xl font-bold", "Users" }
            div {
                class: "my-4",
                input {
                    class: "border p-2 mr-2",
                    placeholder: "Name",
                    value: name.read().as_str(),
                    oninput: move |evt| name.set(evt.value().clone()),
                }
                input {
                    class: "border p-2 mr-2",
                    placeholder: "Email",
                    value: email.read().as_str(),
                    oninput: move |evt| email.set(evt.value().clone()),
                }
                button {
                    class: "bg-blue-500 text-white p-2",
                    onclick: move |_| {
                        let name = name.read().clone();
                        let email = email.read().clone();
                        spawn(async move {
                        });
                    },
                    "Add User"
                }
            }
            ul {

            }
        }
    }
}
