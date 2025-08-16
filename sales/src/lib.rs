#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,

}
impl Cart {
    pub fn new() -> Cart {
        Cart{
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let mut price = 0.0;
        for product in &s.products{
            if product.0 == ele {
                price = product.1;
            } 
        }
        self.items.push((ele, price))
    }
     pub fn get_prices(&self) -> Vec<f32> {
        self.items.iter().map(|(_, p)| *p).collect()
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices = self.get_prices();
        let cal = self.items.len() / 3; 
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let v: Vec<f32> = prices[cal..].to_vec();

        let percentage: f32 = v.iter().sum::<f32>() * 100.0 / prices.iter().sum::<f32>();

        self.receipt = prices
            .iter()
            .map(|price| round_two(price * percentage / 100.0))
            .collect();

        self.receipt.clone()
    }
}


fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}