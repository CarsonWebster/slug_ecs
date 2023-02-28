use bevy::prelude::*;

#[derive(Component, Debug)]
struct Price(f32);

#[derive(Component, Debug)]
enum SlugType {
    Fiat, 
    Grateful,
    Strong,
}

#[derive(Component, Debug)]
struct Slug {
    slug_type: SlugType,
    price: Price,
}

fn spawn_random_slug(mut commands: Commands) {
    let slug_type = match rand::random::<u8>() % 3 {
        0 => SlugType::Fiat,
        1 => SlugType::Grateful,
        2 => SlugType::Strong,
        _ => unreachable!(),
    };
    let price = match slug_type {
        SlugType::Fiat => Price(10.0),
        SlugType::Grateful => Price(50.0),
        SlugType::Strong => Price(999.0),
    };
    commands
        .spawn( Slug { slug_type, price } );
}

fn print_my_slugs(query: Query<&Slug>) {
    for slug in query.iter() {
        println!("You pulled a...");
        match slug.slug_type {
            SlugType::Fiat => println!("Fiat Slug! It's worth ${}!", slug.price.0),
            SlugType::Grateful => println!("Grateful Slug! It's worth ${}!", slug.price.0),
            SlugType::Strong => println!("
            Strong Slug! It's worth ${}!
            This one must be an extra rare find!"
            , slug.price.0),
        }
    }
}

fn main() {
    App::new()
        .add_startup_system(spawn_random_slug)
        .add_system(print_my_slugs)
        .run();
}