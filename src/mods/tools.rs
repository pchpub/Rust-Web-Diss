
pub async fn reg_fit(url: &str) -> Result<Vec<String>,()> {
    let mut return_data: Vec<String> = Vec::with_capacity(3);
    let mut index_begin = 0;
    let mut index_end = 0;
    for item in url.chars() {
        if item == '.' {
            if url[index_begin..index_end].len() >= 5 && &url[index_begin..index_end][..4] == "xn--"{
                return_data.push(idna::domain_to_unicode(&url[index_begin..index_end]).0); 
            }else{
                return_data.push((&url[index_begin..index_end]).to_string());
            }
            index_begin = index_end + 1;
        }
        index_end += 1;
    }
    if url[index_begin..].len() >= 5 && &url[index_begin..][..4] == "xn--"{
        return_data.push(idna::domain_to_unicode(&url[index_begin..]).0); 
    }else{
        return_data.push((&url[index_begin..]).to_string());
    }

    Ok(return_data)
}