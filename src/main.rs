use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Hotspot {
    name: String,
    access_type: String,
    has_captive_portal: bool,
    location: Location,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DmsCoordinate {
    degrees: u32,
    minutes: u32,
    seconds: f32,
    direction: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Location {
    latitude: DmsCoordinate,
    longitude: DmsCoordinate,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("hotspots.json")?;
    let mut hotspots: Vec<Hotspot> = serde_json::from_str(&contents)?;
    let mut startup = false;

    clear_screen();
    println!("\n=====================================");
    println!("📡 WELCOME TO THE WIFI MAPPER CLI 📡");
    println!("=====================================");
    
    loop {
        if !startup {
            startup = true;
        } else {
            clear_screen();
        }
        println!("\n--- Main Menu ---");
        println!("1. List all hotspots");
        println!("2. Search hotspots by name");
        println!("3. Add a new hotspot");
        println!("4. Edit an existing hotspot");
        println!("5. Delete a hotspot");         
        println!("6. Exit application");
        print!("Choose an option (1-6): ");
        
        io::stdout().flush()?; 

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            // List all hotspots
            "1" => {
                println!("\n--- Current Hotspots ({}) ---", hotspots.len());
                // Pass the index `i` so hotspots are numbered 1, 2, 3...
                for (i, spot) in hotspots.iter().enumerate() {
                    // print the index + 1 for user-friendly numbering
                    print!("[{}] ", i + 1); 
                    print_hotspot(spot);
                }
                pause();
            }
            // Search hotspots by name
            "2" => {
                print!("\nEnter search keyword: ");
                io::stdout().flush()?;

                // create a query as a String
                let mut query = String::new();
                // read the line into the query
                io::stdin().read_line(&mut query)?;
                // format the query to be machine-friendly
                let query = query.trim().to_lowercase();

                println!("\n--- Search Results ---");
                let mut found = false;
                // iterate through the hotspots and see if the name has the query
                for spot in &hotspots {
                    // format the data to be machine-friendly
                    if spot.name.to_lowercase().contains(&query) {
                        print_hotspot(spot);
                        found = true;
                    }
                }
                if !found {
                    println!("No hotspots found matching '{}'", query);
                }
                pause();
            }
            // Add a new hotspot
            "3" => {
                println!("\n--- Add New Hotspot ---\nPlease enter the information as it appears in the options.");
                // basic information
                let name = prompt_user("Enter Wifi Name: ")?;
                let access_type = prompt_user("Enter Access Type (Password Required/None/Certificate): ")?;
                let portal_input = prompt_user("Has Captive Portal? (yes/no): ")?;
                // check if the input starts with 'y' or 'Y' for yes, otherwise it's considered no
                let has_captive_portal = portal_input.to_lowercase().starts_with('y');

                // for now, this is manual, but we may add a map in the future
                println!("\n--- Enter Latitude ---");
                let lat_deg: u32 = prompt_user("Degrees: ")?.parse().unwrap_or(0);
                let lat_min: u32 = prompt_user("Minutes: ")?.parse().unwrap_or(0);
                let lat_sec: f32 = prompt_user("Seconds: ")?.parse().unwrap_or(0.0);
                let lat_dir = prompt_user("Direction (N/S): ")?.to_uppercase();

                println!("\n--- Enter Longitude ---");
                let lon_deg: u32 = prompt_user("Degrees: ")?.parse().unwrap_or(0);
                let lon_min: u32 = prompt_user("Minutes: ")?.parse().unwrap_or(0);
                let lon_sec: f32 = prompt_user("Seconds: ")?.parse().unwrap_or(0.0);
                let lon_dir = prompt_user("Direction (E/W): ")?.to_uppercase();

                // construct the DmsCoordinate
                let latitude = DmsCoordinate { degrees: lat_deg, minutes: lat_min, seconds: lat_sec, direction: lat_dir };
                let longitude = DmsCoordinate { degrees: lon_deg, minutes: lon_min, seconds: lon_sec, direction: lon_dir };

                // construct the Hotspot and add it to the vector
                hotspots.push(Hotspot {
                    name,
                    access_type,
                    has_captive_portal,
                    // we construct the Location here
                    location: Location { latitude, longitude },
                });

                // save using a reference to the vector to not move the data
                match save_hotspots(&hotspots) {
                    Ok(_) => println!("\nHotspot saved successfully!"),
                    Err(e) => println!("\nError saving file: {}", e),
                }
                pause();
            }

            "4" => {
                clear_screen();
                println!("--- Edit Hotspot ---");

                if hotspots.is_empty() {
                    println!("There are no hotspots saved yet to edit.");
                } else {
                    // show the numbered list
                    for (i, spot) in hotspots.iter().enumerate() {
                        println!("[{}] {}", i + 1, spot.name);
                    }

                    // later, add a search feature for user-friendly editing
                    print!("\nEnter the number of the hotspot to edit: ");
                    io::stdout().flush()?;
                    let mut index_input = String::new();
                    io::stdin().read_line(&mut index_input)?;

                    // parse and validate the index selection
                    if let Ok(user_num) = index_input.trim().parse::<usize>() {
                        let target_index = user_num - 1;

                        if target_index < hotspots.len() {
                            // get a mutable reference to the chosen hotspot
                            let spot = &mut hotspots[target_index];
                            println!("\nEditing: '{}'", spot.name);
                            println!("(Press Enter without typing to keep the current value)\n");

                            // edit basic strings
                            let new_name = prompt_user(&format!("Name [{}] : ", spot.name))?;
                            if !new_name.is_empty() { spot.name = new_name; }

                            let new_access = prompt_user(&format!("Access Type [{}] : ", spot.access_type))?;
                            if !new_access.is_empty() { spot.access_type = new_access; }

                            let new_portal = prompt_user(&format!("Has Captive Portal? [{}] (yes/no): ", if spot.has_captive_portal { "yes" } else { "no" }))?;
                            if !new_portal.is_empty() {
                                spot.has_captive_portal = new_portal.to_lowercase().starts_with('y');
                            }

                            // edit latitude
                            println!("\n--- Edit Latitude ---");
                            let lat_deg = prompt_user(&format!("Degrees [{}] : ", spot.location.latitude.degrees))?;
                            if !lat_deg.is_empty() { spot.location.latitude.degrees = lat_deg.parse().unwrap_or(spot.location.latitude.degrees); }

                            let lat_min = prompt_user(&format!("Minutes [{}] : ", spot.location.latitude.minutes))?;
                            if !lat_min.is_empty() { spot.location.latitude.minutes = lat_min.parse().unwrap_or(spot.location.latitude.minutes); }

                            let lat_sec = prompt_user(&format!("Seconds [{}] : ", spot.location.latitude.seconds))?;
                            if !lat_sec.is_empty() { spot.location.latitude.seconds = lat_sec.parse().unwrap_or(spot.location.latitude.seconds); }

                            let lat_dir = prompt_user(&format!("Direction [{}] : ", spot.location.latitude.direction))?;
                            if !lat_dir.is_empty() { spot.location.latitude.direction = lat_dir.to_uppercase(); }

                            // edit longitude
                            println!("\n--- Edit Longitude ---");
                            let lon_deg = prompt_user(&format!("Degrees [{}] : ", spot.location.longitude.degrees))?;
                            if !lon_deg.is_empty() { spot.location.longitude.degrees = lon_deg.parse().unwrap_or(spot.location.longitude.degrees); }

                            let lon_min = prompt_user(&format!("Minutes [{}] : ", spot.location.longitude.minutes))?;
                            if !lon_min.is_empty() { spot.location.longitude.minutes = lon_min.parse().unwrap_or(spot.location.longitude.minutes); }

                            let lon_sec = prompt_user(&format!("Seconds [{}] : ", spot.location.longitude.seconds))?;
                            if !lon_sec.is_empty() { spot.location.longitude.seconds = lon_sec.parse().unwrap_or(spot.location.longitude.seconds); }

                            let lon_dir = prompt_user(&format!("Direction [{}] : ", spot.location.longitude.direction))?;
                            if !lon_dir.is_empty() { spot.location.longitude.direction = lon_dir.to_uppercase(); }

                            // save changes back to the JSON
                            match save_hotspots(&hotspots) {
                                Ok(_) => println!("\nHotspot updated successfully!"),
                                Err(e) => println!("\nError saving file: {}", e),
                            }
                        } else {
                            println!("\nError: No hotspot exists at number [{}]", user_num);
                        }
                    } else {
                        println!("\nInvalid input. Please type a valid number.");
                    }
                }
                pause();
            }

            "5" => {
                println!("\n--- Delete Hotspot ---");
                if hotspots.is_empty() {
                    println!("There are no hotspots saved yet to delete.");
                } else {
                    for (i, spot) in hotspots.iter().enumerate() {
                        println!("[{}] {}", i + 1, spot.name);
                    }

                    // later, add a search feature for user-friendly editing
                    print!("\nEnter the number of the hotspot to delete: ");
                    io::stdout().flush()?;
                    let mut index_input = String::new();
                    io::stdin().read_line(&mut index_input)?;

                    if let Ok(user_num) = index_input.trim().parse::<usize>() {
                        let target_index = user_num - 1;

                        
                        if target_index < hotspots.len() {
                            let removed_spot = hotspots.remove(target_index);
                            println!("\nSuccessfully deleted: '{}'", removed_spot.name);

                            match save_hotspots(&hotspots) {
                                Ok(_) => println!("Database updated on disk."),
                                Err(e) => println!("Error saving updates to file: {}", e),
                            }
                        } else {
                            println!("\nError: No hotspot exists at number [{}]", user_num);
                        }
                    } else {
                        println!("\nInvalid input. Please type a valid number.");
                    }
                }
                pause();
            }
            "6" => {
                println!("\nGoodbye!");
                break; 
            }
            _ => {
                println!("Error: Invalid choice, please enter 1-6.");
                pause();
            }
        }
    }

    Ok(())
}

fn print_hotspot(spot: &Hotspot) {
    println!(
        "📍 Name: {}\n   Type: {}\n   Portal: {}\n   Location: {}°{}'{}\"{} {}°{}'{}\"{}\n",
        spot.name,
        spot.access_type,
        if spot.has_captive_portal { "Yes" } else { "No" },
        spot.location.latitude.degrees,
        spot.location.latitude.minutes,
        spot.location.latitude.seconds,
        spot.location.latitude.direction,
        spot.location.longitude.degrees,
        spot.location.longitude.minutes,
        spot.location.longitude.seconds,
        spot.location.longitude.direction
    );
}

fn prompt_user(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn save_hotspots(hotspots: &Vec<Hotspot>) -> Result<(), Box<dyn std::error::Error>> {
    let json_string = serde_json::to_string_pretty(hotspots)?;
    std::fs::write("hotspots.json", json_string)?;
    Ok(())
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let _ = io::stdout().flush();
}

fn pause() {
    print!("\nPress Enter to return to the Main Menu...");
    let _ = io::stdout().flush();
    let mut discard = String::new();
    let _ = io::stdin().read_line(&mut discard);
}
