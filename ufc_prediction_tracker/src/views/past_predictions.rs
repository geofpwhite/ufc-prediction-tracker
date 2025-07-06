use std::collections::HashMap;

use crate::{api, Route};
use dioxus::prelude::*;

#[component]
pub fn PastPredictions() -> Element {
    let mut events = use_signal(|| Vec::<(usize, String, String, String)>::new()); // (id, name, date)
    let mut correct = use_signal(|| 0 as i64);
    let mut incorrect = use_signal(|| 0 as i64);
    use_effect(move || {
        // let mut correct = correct.clone();
        // let mut incorrect = incorrect.clone();
        spawn(async move {
            // Fetch all events with predictions
            if let Ok(predicted_events) = api::get_events_with_predictions().await {
                // Filter for past events (date < today, date in mm/dd/yyyy)
                let past_events = predicted_events.into_iter().collect::<Vec<_>>();
                events.write().clear();
                events.write().extend(past_events);
            }
        });
        spawn(async move {
            // Fetch all events with predictions
            if let Ok(counts) = api::get_total_prediction_correctness().await {
                // Filter for past events (date < today, date in mm/dd/yyyy)
                let c = counts.0;
                let i = counts.1;
                *incorrect.write() = i;
                *correct.write() = c;
            }
        });
    });

    rsx! [
        div { class: "container mx-auto",
            ul { class: "divide-y  rounded-lg border border-gray-200 shadow-md mt-4",
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
    let mut fights = use_signal(|| Vec::<(String, String)>::new());
    let mut predictions: Signal<Vec<(String, String)>> =
        use_signal(|| Vec::<(String, String)>::new());
    let mut correctMap = use_signal(|| HashMap::<(String, String), bool>::new());
    use_effect(move || {
        // let mut fights = fights.clone();
        // let mut predictions = predictions.clone();
        // let mut correctMap = correctMap.clone();
        let link = link.clone();
        spawn(async move {
            if let Ok(result) = api::scrape_results(link, id).await {
                fights.write().clear();
                fights.write().extend(result);
            }
            if let Ok(predicted_fights) = api::get_predictions(id).await {
                predictions.write().clear();
                predictions.write().extend(predicted_fights);
            }
            fights.read().iter().for_each(|(f1, f2)| {
                let f1 = f1.clone();
                let f2 = f2.clone();
                spawn(async move {
                    if let Ok(_resultid) = api::add_result(id, f1.clone(), f2.clone()).await {
                        println!("result added");
                    }
                    if let Some(pred) = predictions
                        .read()
                        .iter()
                        .find(|p| (p.0 == *f1 && p.1 == *f2) || (p.0 == *f2 && p.1 == *f1))
                    {
                        if pred.0 == *f1 {
                            correctMap.write().insert((f1.clone(), f2.clone()), true);
                            println!("winner: {f1} loser: {f2}");
                        } else {
                            correctMap.write().insert((f1.clone(), f2.clone()), false);
                        }
                    }
                });
            })
        });
        spawn(async move {
            if let Ok(predicted_fights) = api::get_predictions(id).await {
                predictions.write().clear();
                predictions.write().extend(predicted_fights);
            }
        });
    });

    rsx! {
        div { class: "container mx-auto",
            h1 { class: "text-2xl font-bold mb-4", "Event Results" }
            ul { class: "rounded-lg shadow-md mt-4",
                {fights().iter().map(|(winner, loser)| rsx! {
                    li { class: "p-6 grid grid-cols-3 items-center gap-4 justify-items-center w-full border shadow-sm rounded-lg my-4",
                        button {
                            class: "bg-green-500 text-white border border-gray-200 px-6 py-3 rounded-lg text-xl font-bold shadow-md w-full min-w-0 min-h-[56px] flex items-center justify-center cursor-not-allowed",
                            style: "width: 100%; max-width:40vw",
                            disabled: true,
                            "{winner}"
                        }
                        span { class: "mx-6 text-3xl font-extrabold text-gray-400 text-center w-20vw flex flex-col items-center justify-center",
                            "vs"
                            {
                                let correct_map_ref = correctMap();
                                let val = correct_map_ref.get(&(winner.clone(), loser.clone()));
                                if let Some(value) = val {
                                    if *value {
                                        rsx! {
                                            span {
                                                class: "block text-xs text-green-400 mt-1 text-center col-span-3",
                                                style: "font-size: 0.75rem;",
                                                "Correct"
                                            }
                                        }
                                    } else {
                                        rsx! {
                                            span {
                                                class: "block text-xs text-red-400 mt-1 text-center col-span-3",
                                                style: "font-size: 0.75rem;",
                                                "Incorrect"
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        span {
                                            class: "block text-xs text-gray-400 mt-1 text-center col-span-3",
                                            style: "font-size: 0.75rem;",
                                            "No Prediction"
                                        }
                                    }
                                }
                            }
                        }
                        button {
                            class: format!(
                                "bg-gray-200  {} text-gray-700 px-6 py-3 rounded-lg text-xl font-bold shadow-md w-full min-w-0 min-h-[56px] flex items-center justify-center cursor-not-allowed",
                                {
                                    let correct_map_ref = correctMap();
                                    let val = correct_map_ref.get(&(winner.clone(), loser.clone()));
                                    if let Some(value) = val {
                                        if *value == true {
                                            "border-1 text-gray-700"
                                        } else {
                                            "border-5 text-red-500"
                                        }
                                    } else {
                                        "1"
                                    }
                                },
                            ),
                            style: "width: 100%; min-width: 0; max-width: 40vw; ",
                            disabled: false,
                            "{loser}"
                        }
                    }
                })}
            }
        }
    }
}
