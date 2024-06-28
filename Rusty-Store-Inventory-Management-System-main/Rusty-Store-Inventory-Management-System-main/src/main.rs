
struct  Products{
    name:String,
    description:String,
    price: f32,
    quantity: u32,
}
trait  Inventory{
    fn add_product(&mut self,product: Products) ;
    fn edit_product(&mut self,index: usize,product: Products) ;
    fn delete_product(&mut self,index: usize) ;
}

impl Inventory for Vec<Products> {
    fn add_product(&mut self, product: Products)  {
       self.push(product);
    }
    fn edit_product(&mut self,index:usize, products: Products)  {
        if let Some(p) = self.get_mut(index) {
            *p = products;
        } else {
            println!("Product index out of bounds");
        }
    }
    fn delete_product(&mut self, index: usize)  {
        if index < self.len() {
            self.remove(index);
        }else {
            println!("Prouct index out of bounds");
        }
    }

}

fn main(){
    let mut inventory: Vec<Products> = Vec::new();
    inventory.add_product(Products {
        name: "oil".to_string(),
        description: "refined soya oil".to_string(),
        price: 110.0,
        quantity: 100,
    });
    inventory.add_product(Products {
        name: "masala".to_string(),
        description: "chana-masala".to_string(),
        price: 10.0,
        quantity: 50,
    });
    println!("Inventory:");
    for (index, product) in inventory.iter().enumerate() {
        println!("{}. {} - {} - Price: ${:.2} - Quantity: {}", index + 1, product.name, product.description, product.price, product.quantity);
    }
}


