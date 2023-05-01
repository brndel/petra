use crate::tables::{
  category::{CategoryGroupInsert, CategoryInsert},
  payment::PaymentInsert,
  payment::{PaymentCategoryLinkInsert, PaymentUserLinkInsert},
  user::{User, UserInsert},
};
use chrono::{DateTime, Days, Local};
use data::{query::SelectQuery, Database, Filter, FilterValue, PrimKey};
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

fn insert_user(database: &Database, display_name: String, password: Option<&'static str>) -> i64 {
  let name = display_name.to_lowercase().replace(" ", "_");
  database
    .insert(UserInsert {
      name: name.to_owned(),
      display_name,
      auth_hash: password
        .map(|s| sha256::digest(format!("{name}:{s}").as_str()))
        .unwrap_or_default(),
    })
    .unwrap()
}

fn insert_payment(
  database: &Database,
  name: String,
  owner_id: i64,
  now: DateTime<Local>,
  categories: &Vec<PrimKey>,
) -> i64 {
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

  let category_id = categories.choose(&mut rng).unwrap().to_owned();

  database.insert(PaymentCategoryLinkInsert {
    payment_id,
    category_id,
  });

  payment_id
}

fn insert_category_group(database: &Database, name: String, icon: String) -> PrimKey {
  database.insert(CategoryGroupInsert { name, icon }).unwrap()
}

fn insert_category(
  database: &Database,
  name: String,
  icon: String,
  group_id: PrimKey,
  categories: &mut Vec<PrimKey>,
) {
  let id = database
    .insert(CategoryInsert {
      name,
      icon,
      group_id,
    })
    .unwrap();
  categories.push(id);
}

pub fn insert_foo(database: &Database) {
  if SelectQuery::new()
    .filter(User::id().eq(ADMIN_USER_ID))
    .get_first::<PrimKey>(&database)
    .is_some()
  {
    return;
  }

  let group_shopping =
    insert_category_group(database, "Einkauf".to_owned(), "shopping_cart".to_owned());
  let group_home = insert_category_group(database, "Wohnung".to_owned(), "home".to_owned());

  let mut categories = Vec::new();

  insert_category(
    database,
    "Essen".to_owned(),
    "restaurant".to_owned(),
    group_shopping,
    &mut categories,
  );
  insert_category(
    database,
    "Haushalt".to_owned(),
    "cleaning_services".to_owned(),
    group_shopping,
    &mut categories,
  );
  insert_category(
    database,
    "Kleidung".to_owned(),
    "dry_cleaning".to_owned(),
    group_shopping,
    &mut categories,
  );

  insert_category(
    database,
    "Miete".to_owned(),
    "real_estate_agent".to_owned(),
    group_home,
    &mut categories,
  );
  insert_category(
    database,
    "Warmkosten".to_owned(),
    "bolt".to_owned(),
    group_home,
    &mut categories,
  );
  insert_category(
    database,
    "Möbel".to_owned(),
    "chair".to_owned(),
    group_home,
    &mut categories,
  );

  let now = Local::now();

  let mut rng = rand::thread_rng();

  for u in 0..FOO_USER_COUNT {
    let user_name = if u == 0 {
      ("Admin".to_string(), Some("abc"))
    } else {
      (format!("User {}", u), None)
    };
    let user_id = insert_user(database, user_name.0, user_name.1);

    for _ in 0..FOO_PAYMENT_COUNT {
      let name_shop = FOO_SHOP_NAMES.choose(&mut rng).unwrap();
      let name_item = FOO_SHOP_ITEMS.choose(&mut rng).unwrap();
      insert_payment(
        database,
        format!("{} {}", name_shop, name_item),
        user_id,
        now,
        &categories,
      );
    }
  }
}
