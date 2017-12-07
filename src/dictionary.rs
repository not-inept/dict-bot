extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;
extern crate quick_xml;

use std::io;
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use serde_json::Value;


// Fetch definition from the dictionary API
pub fn fetch(api_key: String, word: String) -> String {
    // Build client for HTTPS request, a bit overkill but easy peasy
    let mut core = Core::new().unwrap();
    let client_config: hyper::client::Config<hyper::client::UseDefaultConnector, _> = ::hyper::Client::configure().connector(
        hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap(),
    );
    let client = client_config.build(&core.handle());

    // The endpoint for our API request to meriam webster
    let uri = format!("https://www.dictionaryapi.com/api/v1/references/collegiate/xml/{}?key={}", word, api_key);

    // Now let's make the request!
    let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            Ok(res);
        });
    });
    let c = core.run(work);
    c.unwrap()
}