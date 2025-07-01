use crate::api;
use dioxus::prelude::*;

#[component]
pub fn PastPredictions() -> Element {
    let mut events = use_signal(|| Vec::<(usize, String, String)>::new()); // (id, name, date)

    use_effect(move || {
        let mut events = events.clone();
        spawn(async move {
            // Fetch all events with predictions
            if let Ok(predicted_events) = api::get_events_with_predictions().await {
                // Filter for past events (date < today, date in mm/dd/yyyy)
                let today = chrono::Utc::now().naive_utc().date();
                let past_events = predicted_events
                    .into_iter()
                    .filter(|(_id, _name, date)| {
                        chrono::NaiveDate::parse_from_str(date, "%m/%d/%Y")
                            .map(|d| d < today)
                            .unwrap_or(false)
                    })
                    .collect::<Vec<_>>();
                events.write().clear();
                events.write().extend(past_events);
            }
        });
    });

    rsx! [
        div { class: "container mx-auto",
            h1 { class: "text-2xl font-bold mb-4", "Past Predictions" }
            ul { class: "divide-y divide-gray-200 rounded-lg border border-gray-200 shadow-md mt-4",
                {events().iter().map(|(_id, name, date)| rsx! {
                    li { class: "p-4 flex flex-col md:flex-row md:justify-between items-start md:items-center",
                        span { class: "font-semibold text-lg", "{name}" }
                        span { class: "text-gray-500 ml-2", "{date}" }
                    }
                })}
            }
                // (if events().is_empty() {
        //     rsx!(div { class: "text-gray-400 mt-8 text-center", "No past predictions found." })
        // } else {
        //     rsx!()
        // })
        }
    ]
}
