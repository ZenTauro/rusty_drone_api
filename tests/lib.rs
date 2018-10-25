extern crate drone_api;
extern crate serde_json;
extern crate serde;

#[cfg(test)]
mod tests {
    use std::io::Read;
    use drone_api::structs::*;
    use std::env::var;

    /// This test checks that the response coming from the server matches
    /// a previous one. It takes `DRONE_TOKEN` and `DRONE_SERVER` from
    /// env variables and compares the serialized response with the one
    /// stored in a file.
    ///
    /// It only compares the commit for simplicity.
    #[test]
    fn check_with_serialized() {
        let mut file = std::fs::File::open("./tests/data/feed.json").unwrap();
        let token    = var("DRONE_TOKEN").unwrap();
        let client   = drone_api::Client::new(token, var("DRONE_SERVER").unwrap());
        let response = client.get_current_user_feed().unwrap();

        let mut feed_elem_str = String::new();
        let _result           = file.read_to_string(&mut feed_elem_str);
        let feed_elem: Vec<FeedElement> = serde_json::from_str(&feed_elem_str).unwrap();

        assert_eq!(
            feed_elem.first().unwrap().commit,
            response. first().unwrap().commit
        );
    }
}
