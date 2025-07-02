use crate::{api, Route};
use dioxus::prelude::*;

#[component]
pub fn PastPredictions() -> Element {
    let events = use_signal(|| Vec::<(usize, String, String, String)>::new()); // (id, name, date)

    use_effect(move || {
        let mut events = events.clone();
        spawn(async move {
            // Fetch all events with predictions
            if let Ok(predicted_events) = api::get_events_with_predictions().await {
                // Filter for past events (date < today, date in mm/dd/yyyy)
                let today = chrono::Utc::now().naive_utc().date();
                let past_events = predicted_events.into_iter().collect::<Vec<_>>();
                past_events
                    .clone()
                    .into_iter()
                    .for_each(|(a, b, c, d)| println!("{a},{b},{c}{d}"));
                println!("{:?}", past_events.clone());
                events.write().clear();
                events.write().extend(past_events);
            }
        });
    });

    rsx! [
        div { class: "container mx-auto",
            h1 { class: "text-2xl font-bold mb-4", "Past Predictions" }
            ul { class: "divide-y divide-gray-200 rounded-lg border border-gray-200 shadow-md mt-4",
                {
                    events()
                        .iter()
                        .map(|(id, name, date, link)| {
                            rsx! {
                                Link {
                                    to: Route::PastEvent {
                                        id: id.clone(),
                                        link: link.clone(),
                                    },
                                    li { class: "p-4 flex flex-col md:flex-row md:justify-between items-start md:items-center cursor-pointer hover:bg-gray-100 transition",
                                        span { class: "font-semibold text-lg", "{name}" }
                                        span { class: "text-gray-500 ml-2", "{date}" }
                                    }
                                }
                            }
                        })
                }
            }
                // (if events().is_empty() {
        //     rsx!(div { class: "text-gray-400 mt-8 text-center", "No past predictions found." })
        // } else {
        //     rsx!()
        // })
        }
    ]
}

#[component]
pub fn PastEvent(id: usize, link: String) -> Element {
    let fights = use_signal(|| Vec::<(String, String)>::new());

    use_effect(move || {
        let mut fights = fights.clone();
        let link = link.clone();
        spawn(async move {
            if let Ok(predicted_events) = api::get_results(link, id).await {
                fights.write().clear();
                fights.write().extend(predicted_events);
            }
        });
    });

    rsx! {
        div { class: "container mx-auto",
            h1 { class: "text-2xl font-bold mb-4", "Event Results" }
            ul { class: "divide-y divide-gray-200 rounded-lg border border-gray-200 shadow-md mt-4",
                {fights().iter().map(|(winner, loser)| rsx! {
                    li { class: "p-6 grid grid-cols-3 items-center gap-4 justify-items-center w-full border border-gray-100 shadow-sm rounded-lg my-4",
                        button {
                            class: "bg-green-500 text-white border border-gray-200 px-6 py-3 rounded-lg text-xl font-bold shadow-md w-full min-w-0 min-h-[56px] flex items-center justify-center cursor-not-allowed",
                            style: "width: 100%; max-width:40vw",
                            disabled: true,
                            "{winner}"
                        }
                        span { class: "mx-6 text-3xl font-extrabold text-gray-400 text-center w-20vw flex items-center justify-center",
                            "vs"
                        }
                        button {
                            class: "bg-gray-200 text-gray-700 border border-gray-200 px-6 py-3 rounded-lg text-xl font-bold shadow-md w-full min-w-0 min-h-[56px] flex items-center justify-center cursor-not-allowed",
                            style: "width: 100%; min-width: 0; max-width: 40vw; ",
                            disabled: true,
                            "{loser}"
                        }
                    }
                })}
            }
        }
    }
}
