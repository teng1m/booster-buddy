#[derive(Clone, Debug)]
pub struct PlayerInfo {
    wg_auth_token: String,
    discord_id: String,
    auth_expiry: u64,
}

pub fn load_from_db() -> Option<Vec<PlayerInfo>> {
    match rusqlite::Connection::open("./player_info.db3") {
        Ok(connection) => {
            match connection.prepare("SELECT discord_id, auth_token, auth_expiry FROM players") {
                Ok(mut statement) => match statement.query_map([], |row| {
                    Ok(PlayerInfo {
                        discord_id: row.get(0)?,
                        wg_auth_token: row.get(1)?,
                        auth_expiry: row.get(2)?,
                    })
                }) {
                    Ok(player_iter) => Some(player_iter.filter_map(|player| player.ok()).collect()),
                    Err(error) => {
                        println!("Error reading DB: {}", error);
                        None
                    }
                },
                Err(e) => {
                    println!("Error preparing SQL statement: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            println!("Error opening SQL Connection: {}", e);
            None
        }
    }
}

pub fn insert_player(player: PlayerInfo) {
    if let Ok(connection) = rusqlite::Connection::open("./player_info.db3") {
        if connection
            .execute(
                "INSERT INTO players (discord_id, auth_token, auth_expiry) VALUES (?1, ?2, ?3)",
                (
                    &player.discord_id,
                    &player.wg_auth_token,
                    &player.auth_expiry,
                ),
            )
            .is_err()
        {
            println!("Error inserting player {:?} into DB", player);
        }
    } else {
        println!("Error opening DB connection");
    }
}

//                Discord ID
pub fn drop_player(player: String) {
    if let Ok(connection) = rusqlite::Connection::open("./player_info.db3") {
        if connection
            .execute("DELETE FROM players WHERE discord_id=?1", [&player])
            .is_err()
        {
            println!("Error dropping player with ID {:?} from DB", player)
        }
    } else {
        println!("Error opening DB connection");
    }
}

#[cfg(test)]
mod tests {
    use super::PlayerInfo;

    #[test]
    pub fn create_db() {
        let connection = rusqlite::Connection::open("./player_info.db3").unwrap();

        let _ = connection.execute(
            "CREATE TABLE players (
                discord_id    TEXT PRIMARY KEY NOT NULL,
                auth_token    TEXT NOT NULL,
                auth_expiry   BIGINT NOT NULL
            )",
            (),
        );
    }

    #[test]
    pub fn get_entries() {
        let connection = rusqlite::Connection::open("./player_info.db3").unwrap();

        let mut statement = connection
            .prepare("SELECT discord_id, auth_token, auth_expiry FROM players")
            .unwrap();

        let player_iter = statement
            .query_map([], |row| {
                Ok(PlayerInfo {
                    discord_id: row.get(0)?,
                    wg_auth_token: row.get(1)?,
                    auth_expiry: row.get(2)?,
                })
            })
            .unwrap();

        for player in player_iter {
            println!("Found Player: {:?}", player.unwrap());
        }
    }

    #[test]
    pub fn insert_entry() {
        let me = PlayerInfo {
            discord_id: "ashikgaediuahgroai".to_string(),
            wg_auth_token: "asdkhgasduikgherasukigheasrg".to_string(),
            auth_expiry: 235798,
        };

        rusqlite::Connection::open("./player_info.db3")
            .unwrap()
            .execute(
                "INSERT INTO players (discord_id, auth_token, auth_expiry) VALUES (?1, ?2, ?3)",
                (&me.discord_id, &me.wg_auth_token, &me.auth_expiry),
            )
            .unwrap();
    }

    #[test]
    pub fn drop_entries() {
        rusqlite::Connection::open("./player_info.db3")
            .unwrap()
            .execute("DELETE FROM players", ())
            .unwrap();
    }
}
