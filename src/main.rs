mod person;

use crate::person::PersonRepository;

#[tokio::main]
async fn main() {
    let person_repository = PersonRepository::connect("./test.db").await.unwrap();
    // let id = person_repository.insert("Homer", "Simpson").await.unwrap();
    // println!("id: {}", id);

    let person = person_repository.find_by_id(1).await.unwrap();

    println!("{:#?}", person)
}
