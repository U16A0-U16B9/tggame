use crate::game::win_conditions::WinConditions;
use crate::schema::roles;
use crate::services::database::establish_connection;
use diesel::prelude::*;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Display, PartialEq, EnumString)]
pub enum Roles {
    Villager,
    Tanner,
    Seer,
    Werewolf,
}

#[derive(Queryable, Insertable)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub win_condition: String,
}

impl Role {
    pub fn create(role: Roles, description: String, win_condition: WinConditions) -> QueryResult<Role> {
        let connection = &mut establish_connection();
        let new_role = Role {
            id: Uuid::new_v4(),
            name: role.to_string(),
            description,
            win_condition: win_condition.to_string(),
        };

        diesel::insert_into(roles::table)
            .values(&new_role)
            .get_result(connection)
    }

    pub fn seed() {
        let connection = &mut establish_connection();
        diesel::delete(roles::table).execute(connection);

        Role::create(
            Roles::Villager,
            "Just an average folk".to_string(),
            WinConditions::EliminateWerewolves,
        )
        .expect(format!("Error seeding role {}", Roles::Villager.to_string()).as_str());

        Role::create(
            Roles::Tanner,
            "You hate your job and your life".to_string(),
            WinConditions::Die,
        )
        .expect(format!("Error seeding role {}", Roles::Tanner.to_string()).as_str());

        Role::create(
            Roles::Seer,
            "Each night choose a player to learn if he is Villager or Werewolf".to_string(),
            WinConditions::EliminateWerewolves,
        )
        .expect(format!("Error seeding role {}", Roles::Seer.to_string()).as_str());

        Role::create(
            Roles::Werewolf,
            "Each night choose a player to eliminate".to_string(),
            WinConditions::EliminateVillagers,
        )
        .expect(format!("Error seeding role {}", Roles::Werewolf.to_string()).as_str());
    }
}
