use mensula_key::Key;

use crate::api::migrate;

#[derive(Debug, clap::Parser)]
// #[clap(author, version, about)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Debug, clap::Subcommand)]
pub enum CliCommand {
    Create(CreateCommand),
    Migrate(MigrateCommand),
}

impl CliCommand {
    pub fn run(&self) -> std::io::Result<()> {
        match self {
            CliCommand::Create(command) => command.run(),
            CliCommand::Migrate(command) => command.run(),
        }
    }
}

#[derive(Debug, clap::Args)]
pub struct CreateCommand {
    #[clap(subcommand)]
    subcommand: CreateSubcommand,
}

impl CreateCommand {
    pub fn run(&self) -> std::io::Result<()> {
        match self.subcommand {
            CreateSubcommand::User => cli_create_user().map(|_| ()),
        }
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum CreateSubcommand {
    User,
}

pub fn cli_create_user() -> std::io::Result<Key> {
    panic!("currently not implemented due to privatization of server");
    // println!("Creating new user");
    // let stdin = std::io::stdin();

    // println!("Username:");
    // let mut name = String::new();
    // stdin.read_line(&mut name)?;
    // name = name.trim().to_owned();
    // while let Err(err) = User::check_name(&name) {
    //     println!("{}", err);
    //     stdin.read_line(&mut name)?;
    //     name = name.trim().to_owned();
    // }

    // println!("Display name:");
    // let mut display_name = String::new();
    // stdin.read_line(&mut display_name)?;
    // display_name = display_name.trim().to_owned();
    // while let Err(err) = User::check_display_name(&display_name) {
    //     println!("{}", err);
    //     stdin.read_line(&mut display_name)?;
    //     display_name = display_name.trim().to_owned();
    // }

    // println!("Password:");
    // let mut password = rpassword::read_password()?;
    // while let Err(err) = User::check_password(&password) {
    //     println!("{}", err);
    //     password = rpassword::read_password()?;
    // }

    // println!("Repeat Password:");
    // let mut password_repeat = rpassword::read_password()?;
    // while password != password_repeat {
    //     println!("passwords did not match");
    //     password_repeat = rpassword::read_password()?;
    // }

    // println!("creating user '{}' ({})", name, display_name);
    // let db = get_db();
    // let user = match User::create(name, display_name, password) {
    //     Ok(user) => user,
    //     Err(err) => {
    //         return Err(Error::new(ErrorKind::Other, err.to_string()));
    //     }
    // };
    // let user_key = db.insert(user);
    // if let Some(user_key) = &user_key {
    //     println!("user added with id {:?}", user_key);
    // } else {
    //     println!("user insert failed");
    // }

    // user_key.ok_or(Error::new(ErrorKind::Other, "failed to create user"))
}

#[derive(Debug, clap::Args)]
pub struct MigrateCommand {
    old_file: String,
}

impl MigrateCommand {
    pub fn run(&self) -> std::io::Result<()> {
        migrate::migrate(&self.old_file);

        Ok(())
    }
}