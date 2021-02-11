extern crate reqwest; // 0.9.18

use std::io::Read;

fn get_site(address: String) -> Result<String, ()> {
    let mut res = reqwest::blocking::get(&address);
    let mut body = String::new();

    let res = match res {
        Ok(response) => response,
        Err(response) => return Err(())
    }
    
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(body)
}