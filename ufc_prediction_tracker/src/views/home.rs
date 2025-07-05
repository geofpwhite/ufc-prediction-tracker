use crate::{api, Route};
use dioxus::prelude::*;
use log;

#[component]
pub fn Home() -> Element {
    let mut event_list = use_signal(|| Vec::<(String, String, usize)>::new());

    // Use an effect to fetch and update events on mount
    use_effect(move || {
        spawn({
            async move {
                match api::get_upcoming_events().await {
                    Ok(events) => {
                        for event in &events {
                            println!("Upcoming Event: {}-{}-{}", event.0, event.1, event.2);
                        }
                        event_list.write().extend(events.into_iter())
                    }
                    Err(e) => println!("Failed to add upcoming events: {}", e),
                }
            }
        });
    });
    rsx! {
        div { class: "container mx-auto",
            h1 { class: "text-2xl font-bold", "Events" }
            div { class: "my-4" }
            ul { class: "divide-y divide-gray-200 rounded-lg border border-gray-200 shadow-md mt-4",
                {
                    event_list()
                        .iter()
                        .map(|event| {
                            let mut title_date = event
                                .0
                                .trim_matches(|char| char == '|')
                                .split("||");
                            let title = title_date.next().expect("");
                            let date = title_date.next().expect("");
                            rsx! {
                                Link {
                                    to: Route::Predict {
                                        id: event.2,
                                        link: event.1.clone(),
                                    },
                                    class: "block",
                                    li { class: "p-4 text-black-100 hover:bg-blue-500 cursor-pointer transition-colors w-full flex justify-between items-center gap-4",
                                        div { class: "flex-1 rounded-l-lg px-4 py-2 font-bold", "{title}" }
                                        div { class: "rounded-r-lg px-4 py-2 text-gray-600 text-sm", "{date}" }
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}
