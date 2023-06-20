use core_api_rs::{Api,FilterOperator, Query};

use chrono::{Utc, Duration, DateTime, NaiveDate};
use std::cell::RefCell;
use std::fmt::Write;


enum Test {
    Execute,
    Log
}

fn main() {
    let todo = Test::Execute;
    let dates = match DateIterator::new("2020-08-20") {
        Ok(c) => c,
        Err(e) => {
            println!("Error creating date iterator: {:#?}", e);
            return;
        }
    };
    let mut today = dates.next();
    let mut tommorow = dates.next();
    let api = Api::from("DCrZJjaUtFd1KHg3zqbRTYelO9Xs26IM");

    for _ in 0..10 {
        let setup_query = api.paged(3, 1)
            .and(FilterOperator::Exists("doi".to_string()))
            .and(FilterOperator::Bigger("createdDate".into(), &today))
            .and(FilterOperator::Smaller("createdDate".into(), &tommorow));
        let query = Query::SearchWorks(setup_query);
    
        match todo {
            Test::Execute => {
                let resp = api.execute_query(query);
                if let Err(e) = resp {
                    println!("{:#?}", e);
                } else {
                    println!("Trys left: {:#?}", resp.unwrap().ratelimit_remaining);
                }
            },
            Test::Log => {
                let resp = query.parse_request();
                println!("{:#?}", resp);
            },
        }
        today = tommorow;
        tommorow = dates.next();
    } 

    
    
    
}



struct DateIterator {
    current_date: RefCell<DateTime<Utc>>,
}

impl DateIterator {
    fn new(start_date: &str) -> Result<Self, chrono::ParseError> {
        let date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;
        let datetime = DateTime::from_utc(date.and_hms(0, 0, 0), Utc);
        Ok(Self {
            current_date: RefCell::new(datetime),
        })
    }

    fn next(&self) -> String {
        let date = self.current_date.borrow().date();
        *self.current_date.borrow_mut() = (date + Duration::days(1)).and_hms(0, 0, 0);
        self.current_date.borrow().format("%Y-%m-%dT00:00:00Z").to_string()
    }
}