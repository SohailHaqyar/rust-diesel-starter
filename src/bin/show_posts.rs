extern crate diesel;
extern crate pes_book;

use self::diesel::prelude::*;
use self::models::*;
use self::pes_book::*;

fn main() {
    use pes_book::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("\n");
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
