use pcre2::bytes::Regex;

pub async fn reg_fit(url: &str) -> Result<Vec<&str>,()> {
    println!("{}", url);
    //let url = format!("{url}.");
    let re = if let Ok(value) = Regex::new(
        r"([\s\S]+?)\.([\s\S]+?)\.([\s\S]+?).",
    ) {
        value
    } else {
        return Err(());
    };
    let caps = if let Ok(value) = re.captures(url.as_bytes()) {
        match value {
            Some(cap) => cap,
            None => return Err(()),
        }
    } else {
        return Err(());
    };
    println!("{:?}", caps);
    let mut return_data = Vec::with_capacity(3);
    let mut index = 1;
    while index <= 3 {
        match &caps.get(index) {
            Some(value) => {
                return_data.push(std::str::from_utf8(value.as_bytes()).unwrap());
            },
            None => (),
        }
        index += 1;
    }
    return Ok(return_data);
}