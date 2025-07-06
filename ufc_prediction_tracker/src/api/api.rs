use crate::db;
use dioxus::prelude::*;
use tracing;
#[server]
pub async fn get_upcoming_events() -> Result<Vec<(String, String, usize)>, ServerFnError> {
    // Simulate fetching upcoming events from a database or an API
    // get from http://ufcstats.com/statistics/events/upcoming?page=all
    let response = reqwest::get("http://ufcstats.com/statistics/events/upcoming?page=all")
        .await?
        .text()
        .await?;
    let doc: scraper::Html = scraper::Html::parse_document(&response);

    let result = (doc
        .select(&scraper::Selector::parse("tr.b-statistics__table-row").unwrap())
        .filter_map(|event| {
            event
                .select(&scraper::Selector::parse("td.b-statistics__table-col").unwrap())
                .next()
                .map(|col| col.text().collect::<String>())
        })
        .collect::<Vec<_>>());
    let links = (doc
        .select(&scraper::Selector::parse("tr.b-statistics__table-row").unwrap())
        .filter_map(|event| {
            event
                .select(&scraper::Selector::parse("a.b-link.b-link_style_black").unwrap())
                .next()
                .map(|link| {
                    //println!("link {}", link.value().attr("href").unwrap().to_string());
                    link.value().attr("href").unwrap().to_string()
                })
        })
        .collect::<Vec<String>>());
    let res = result
        .clone()
        .into_iter()
        .map(|event| {
            event
                .trim()
                .split('\n')
                .filter(|s| !s.trim().is_empty())
                .map(|s| "|".to_string() + s.trim() + "|")
                .collect()
        })
        .collect::<Vec<String>>();
    let zipped = res
        .iter()
        .zip(links.iter())
        .collect::<Vec<(&String, &String)>>();
    for i in zipped.clone() {
        //println!("{}{}", i.0, i.1);
    }
    //println!("Upcoming Events:");
    let mut ids: Vec<usize> = vec![];
    &result.iter().zip(links.iter()).for_each(|(event, link)| {
        let mut split: Vec<&str> = event
            .trim()
            .split('\n')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim())
            .collect();

        let conn = db::get_db_connection().expect("Failed to connect to database");
        let s1: &str = &split[0];
        let s2: &str = &split[1];
        let id = db::add_event(&conn, s1, s2, link).expect("");
        ids.push(id.clone());
        //println!("{}", id);
    });
    Ok(zipped
        .into_iter()
        .zip(ids.iter())
        .map(|((a, b), c)| (a.clone(), b.clone(), c.clone()))
        .collect())
}

#[server]
pub async fn get_fights(link: String) -> Result<Vec<(String, String)>, ServerFnError> {
    // Fetch the event page
    let response = reqwest::get(&link)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch event page: {}", e)))?
        .text()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get event page text: {}", e)))?;
    let doc = scraper::Html::parse_document(&response);
    let row_selector = scraper::Selector::parse("tr.b-fight-details__table-row").unwrap();
    let name_selector = scraper::Selector::parse("a.b-link.b-link_style_black").unwrap();

    let fights = doc
        .select(&row_selector)
        .filter_map(|row| {
            let mut names = row
                .select(&name_selector)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .filter(|s| s.clone().trim() != "View Matchup" && !s.is_empty())
                .collect::<Vec<_>>();
            if names.len() == 2 {
                Some((names[0].clone(), names[1].clone()))
            } else {
                None
            }
        })
        .collect::<Vec<(String, String)>>();
    Ok(fights)
}

#[server]
pub async fn predict(
    event_id: usize,
    winner: String,
    loser: String,
) -> Result<bool, ServerFnError> {
    println!("{}{}{}", event_id, winner, loser);
    let conn = db::get_db_connection().expect("Failed to connect to database");
    match db::add_or_update_prediction(&conn, event_id, &winner, &loser) {
        Ok(f) => Ok(true),
        Err(e) => Err(e.into()),
    }
}

#[server]
pub async fn get_predictions(event_id: usize) -> Result<Vec<(String, String)>, ServerFnError> {
    let conn = db::get_db_connection().expect("Failed to connect to database");
    Ok(db::get_predictions(&conn, event_id)?)
}

#[server]
pub async fn add_result(
    event_id: usize,
    winner: String,
    loser: String,
) -> Result<usize, ServerFnError> {
    let conn = db::get_db_connection().expect("Failed to connect to database");
    match db::add_or_update_result(&conn, event_id, &winner, &loser) {
        Ok(f) => Ok(f),
        Err(e) => Err(e.into()),
    }
}

#[server]
pub async fn get_events_with_predictions(
) -> Result<Vec<(usize, String, String, String)>, ServerFnError> {
    let conn = db::get_db_connection().expect("Failed to connect to database");
    Ok(db::get_past_events_with_predictions(&conn)?)
}

#[server]
pub async fn scrape_results(
    event_link: String,
    event_id: usize,
) -> Result<Vec<(String, String)>, ServerFnError> {
    let conn = db::get_db_connection().expect("Failed to connect to database");
    let response = reqwest::get(&event_link)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch event page: {}", e)))?
        .text()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get event page text: {}", e)))?;
    let doc = scraper::Html::parse_document(&response);
    let row_selector = scraper::Selector::parse("tr.b-fight-details__table-row").unwrap();
    let name_selector = scraper::Selector::parse("a.b-link.b-link_style_black").unwrap();

    let fights = doc
        .select(&row_selector)
        .filter_map(|row| {
            let mut names = row
                .select(&name_selector)
                .map(|el| el.text().collect::<String>().trim().to_string())
                .filter(|s| s.clone().trim() != "View Matchup" && !s.is_empty())
                .collect::<Vec<_>>();
            if names.len() == 2 {
                // Ensure the winner is the first element in the tuple
                Some((names[0].clone(), names[1].clone()))
            } else {
                None
            }
        })
        .collect::<Vec<(String, String)>>();

    Ok(fights)
}

#[server]
pub async fn get_total_prediction_correctness() -> Result<(i64, i64), ServerFnError> {
    let conn = db::get_db_connection().expect("");
    Ok(db::get_my_predictions_correctness(&conn)?)
}
#[server]
pub async fn get_prediction_correctness_for_event(id: usize) -> Result<(i64, i64), ServerFnError> {
    Ok((1, 1))
}
