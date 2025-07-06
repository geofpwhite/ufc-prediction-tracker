use crate::api;
use dioxus::prelude::*;

#[component]
pub fn Predict(id: usize, link: String) -> Element {
    let mut fights = use_signal(|| Vec::<(String, String)>::new());
    let mut selected = use_signal(|| Vec::<Option<usize>>::new());
    let id = use_signal(|| id);
    // println!("{link} link");
    // println!("{id} id");
    use_effect(move || {
        let link = link.clone();

        spawn(async move {
            match api::get_fights(link).await {
                Ok(fights_vec) => {
                    selected.write().resize(fights_vec.len(), None);
                    fights.write().extend(fights_vec.clone());
                    spawn(async move {
                        let fights_vec_clone = fights_vec.clone();
                        match api::get_predictions(*id.read()).await {
                            Ok(preds) => {
                                let mut sel = selected.write();
                                for (i, (f1, f2)) in fights_vec_clone.iter().enumerate() {
                                    if let Some(pred) = preds.iter().find(|p| {
                                        (p.0 == *f1 && p.1 == *f2) || (p.0 == *f2 && p.1 == *f1)
                                    }) {
                                        sel[i] = if pred.0 == *f1 { Some(0) } else { Some(1) };
                                    }
                                }
                            }
                            Err(e) => log::error!("Failed to fetch predictions: {}", e),
                        }
                    });
                }
                Err(e) => log::error!("Failed to fetch fights: {}", e),
            }
        });
    });
    rsx! {
        div { class: "container mx-auto",
            h1 { class: "text-2xl font-bold mb-4", "Predict Fights" }
            ul { class: "divide-y divide-gray-200 rounded-lg border border-gray-200 shadow-md mt-4",
                {
                    fights()
                        .iter()
                        .enumerate()
                        .map(|(i, (f1, f2))| {
                            let sel = selected().get(i).cloned().unwrap_or(None);
                            let f1_btn = f1.clone();
                            let f2_btn = f2.clone();
                            let f1_btn2 = f1.clone();
                            let f2_btn2 = f2.clone();
                            rsx! {
                                li { class: "p-6 grid grid-cols-3 items-center gap-4 justify-items-center hover:bg-gray-50 transition-all duration-200 w-full border border-gray-100 shadow-sm rounded-lg my-4", // add justify-items-center
                                    button {
                                        class: format!(
                                            "{} border border-gray-200 px-6 py-3 rounded-lg text-xl font-bold shadow-md transition-colors duration-200 w-full min-w-0 min-h-[56px] flex items-center justify-center ",
                                            if sel == Some(0) {
                                                "bg-green-500 text-white scale-105"
                                            } else {
                                                "bg-gray-200 text-gray-700 hover:bg-green-100"
                                            },
                                        ),
                                        style: "width: 100%; max-width:40vw",
                                        onclick: move |_| {
                                            selected.write()[i] = Some(0);
                                            let winner = f1_btn.clone();
                                            let loser = f2_btn.clone();
                                            let event_id = id;
                                            spawn(async move {
                                                let _ = api::predict(*event_id.read(), winner, loser).await;
                                            });
                                        },
                                        "{f1_btn}"
                                    }
                                    span { class: "mx-6 text-3xl font-extrabold text-gray-400 text-center w-20vw flex items-center justify-center",
                                        "vs"
                                    } // add flex and justify-center
                                    button {
                                        class: format!(
                                            "border border-gray-200 px-6 py-3 rounded-lg text-xl font-bold shadow-md transition-colors duration-200 w-full min-w-0 min-h-[56px] flex items-center justify-center {}",
                                            if sel == Some(1) {
                                                "bg-green-500 text-white scale-105"
                                            } else {
                                                "bg-gray-200 text-gray-700 hover:bg-green-100"
                                            },
                                        ),
                                        style: "width: 100%; min-width: 0; max-width: 40vw; ",
                                        onclick: move |_| {
                                            selected.write()[i] = Some(1);
                                            let loser = f1_btn2.clone();
                                            let winner = f2_btn2.clone();
                                            let event_id = id;
                                            spawn(async move {
                                                let _ = api::predict(*event_id.read(), winner, loser).await;
                                            });
                                        },
                                        "{f2_btn2}"
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}
