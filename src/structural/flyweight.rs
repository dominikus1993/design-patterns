use std::collections::HashMap;


#[derive(Debug,)]
struct Company<'a> {
    name: &'a str,
}

struct CompanyFactory<'a> {
    companies: HashMap<&'a str, Company<'a>>,
}   

struct Product<'a> {
    id: i32,
    name: String,
    price: f64,
    company: Company<'a>,
}


impl<'a> CompanyFactory<'a> {
    fn new() -> CompanyFactory<'a> {
        CompanyFactory{companies: HashMap::new()}
    }

    fn get_company(&mut self, name: &'a str) -> &Company<'a> {
        if !self.companies.contains_key(name) {
            self.companies.insert(name, Company{name});
        }
        self.companies.get(name).unwrap()
    }
} 

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_builder() {
        let mut factory = CompanyFactory::new();
        let company_name = "name";

        let product = factory.get_company(&company_name);

        assert_eq!(product.name, company_name);

        let second_product = factory.get_company(&company_name);

        assert_eq!(second_product.name, company_name);
    }
}