use crate::tables::{
  payment::PaymentInsert,
  payment::PaymentUserLinkInsert,
  user::{User, UserInsert},
};
use chrono::{DateTime, Days, Local};
use data::{query::SelectQuery, Database, PrimKey};
use rand::{seq::SliceRandom, Rng};

pub const ADMIN_USER_ID: i64 = 1;
const FOO_USER_COUNT: i64 = 5;
const FOO_PAYMENT_COUNT: i64 = 10;

// Written by Chat-GPT
const FOO_SHOP_NAMES: &'static [&'static str] = &[
  "Rewe",
  "Edeka",
  "Aldi",
  "Lidl",
  "DM",
  "Rossmann",
  "Thalia",
  "H&M",
  "Zara",
  "Douglas",
  "Media Markt",
  "Saturn",
  "IKEA",
  "Depot",
  "Toom",
  "Bauhaus",
  "SportScheck",
  "Decathlon",
  "Fressnapf",
  "Apotheke",
  "Sonnenbrillen-Shop",
  "Uhren-Shop",
];

const FOO_SHOP_ITEMS: &'static [&'static str] = &[
  "Milch",
  "Brot",
  "Eier",
  "Käse",
  "Joghurt",
  "Butter",
  "Obst",
  "Gemüse",
  "Fleisch",
  "Fisch",
  "Nudeln",
  "Reis",
  "Mehl",
  "Zucker",
  "Salz",
  "Pfeffer",
  "Öl",
  "Essig",
  "Tee",
  "Kaffee",
  "Müsli",
  "Haferflocken",
  "Marmelade",
  "Honig",
  "Nüsse",
  "Schokolade",
  "Chips",
  "Getränke",
  "Klopapier",
  "Seife",
  "Shampoo",
  "Bücher",
  "Zeitschriften",
  "Musik-CDs",
  "DVDs",
  "Kleidung",
  "Schuhe",
  "Schmuck",
  "Kosmetik",
  "Parfum",
  "Handtaschen",
  "Geschenkkarten",
  "Sportausrüstung",
  "Elektronikgeräte",
  "Möbel",
  "Dekorationsartikel",
  "Kunstwerke",
  "Spielzeug",
  "Spiele",
  "Instrumente",
  "Werkzeuge",
  "Autozubehör",
  "Fahrradzubehör",
  "Reiseausrüstung",
  "Bürobedarf",
  "Schulbedarf",
  "Pflanzen",
  "Tierbedarf",
  "Medikamente",
  "Sonnenbrillen",
  "Uhren",
  "Telefone",
  "Computer",
  "Drucker",
  "Kamera",
  "Kopfhörer",
  "Lautsprecher",
  "Fernseher",
  "Spielkonsolen",
  "Smart Home Geräte",
  "Fitnessgeräte",
];

fn insert_user(database: &Database, display_name: String) -> i64 {
  let name = display_name
    .to_lowercase()
    .chars()
    .filter(|c| c != &' ')
    .collect::<String>();
  database.insert(UserInsert { name, display_name }).unwrap()
}

fn insert_payment(database: &Database, name: String, owner_id: i64, now: DateTime<Local>) -> i64 {
  let mut rng = rand::thread_rng();
  let amount = rng.gen_range(-10000..10000);
  let timestamp = now
    .checked_sub_days(Days::new(rng.gen_range(0..100)))
    .unwrap()
    .to_rfc3339();
  let payment_id = database
    .insert(PaymentInsert {
      name,
      amount,
      timestamp,
      owner_id,
    })
    .unwrap();

  database
    .insert(PaymentUserLinkInsert {
      payment_id,
      user_id: owner_id,
    })
    .unwrap();

  database
    .insert(PaymentUserLinkInsert {
      payment_id,
      user_id: ADMIN_USER_ID,
    })
    .unwrap();

  payment_id
}

pub fn insert_foo(database: &Database) {
  if SelectQuery::new()
    .filter(User::id().eq(ADMIN_USER_ID))
    .get_first::<PrimKey>(&database)
    .is_some()
  {
    return;
  }
  let now = Local::now();

  let mut rng = rand::thread_rng();

  for u in 0..FOO_USER_COUNT {
    let user_name = if u == 0 {
      "Admin".to_string()
    } else {
      format!("User {}", u)
    };
    let user_id = insert_user(database, user_name);

    for _ in 0..FOO_PAYMENT_COUNT {
      let name_shop = FOO_SHOP_NAMES.choose(&mut rng).unwrap();
      let name_item = FOO_SHOP_ITEMS.choose(&mut rng).unwrap();
      insert_payment(
        database,
        format!("{} {}", name_shop, name_item),
        user_id,
        now,
      );
    }
  }
}
