pub struct Item {
    pub id: u8,
    pub name: String,
    pub weight: i32,
    pub value: u16,
    pub is_present: bool
}

pub struct Backpack {
    pub size: u8,
    pub max_weight: i32,
    pub items: [Item; 6]
}

impl Backpack {
    pub fn new(size: u8, max_weight:i32) -> Backpack {
        

        //Propably want that from a file or smth
        let items = [
            Item::new(0, "book"     ,8, 300, false),
            Item::new(1, "phone"    ,5, 500, false),
            Item::new(2, "hat"      ,1, 100, false),
            Item::new(3, "watch"    ,2, 350, false),
            Item::new(4, "pen"      ,1, 50,  false),
            Item::new(5, "wallet"   ,5, 800, false)
        ];
    
        Backpack {
            size,
            max_weight,
            items
        }
    }

    pub fn print_content(&self){
        println!("Backpack content: ");
        for item in self.items.iter(){
            if item.is_present {
                print!("ID: {}\t| Item : {}\t| Weight : {}\t| Value {}\t|" , item.id, item.name, item.weight, item.value)
            }
            println!();
        }
    }

    pub fn items_max_weight(&self) -> i32 {
      let mut weight: i32 = 0;
      for item in self.items.iter(){
        weight += item.weight;
      }
      weight
    }

}

impl Item {
    pub fn new(id: u8, name: &str, weight: i32, value: u16, is_present: bool) -> Item {
        Item {id, name: name.to_string(), weight, value, is_present }
    }
}