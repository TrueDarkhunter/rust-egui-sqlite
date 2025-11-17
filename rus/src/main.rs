use rusqlite::{params, Connection, Result};

//Printer Errors ud
fn main() {
    if let Err(e) = run() {
        eprintln!("Fejl: {}", e);
    }
}

//Tager input fra bruger og kører funktionen baseret på inputet
fn run() -> Result<()> {


    let mut whileloop = true;



    while whileloop {

        println!("LektieCafe admin
        Vælg en mulighed:
        1. Lav lektier database
        2. Tilføj lektie
        3. Læs lektier
        4. Ændre lektie
        5. Slet lektie
        6. Afslut

        ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading input");
        
        if input.trim() == "1" {
            lav_lektier()?;
        } else if input.trim() == "2" {
            tilfoj_lektie()?;
        } else if input.trim() == "3" {
            los_lektier()?;
        } else if input.trim() == "4" {
            opdater_lektie()?;
        } else if input.trim() == "5" {
            slet_lektie()?;
        } else if input.trim() == "6" {
            println!("Afslutter");
            whileloop = false;
        } else {
            println!("Ugyldigt valg");
        }
    }
    Ok(())
}


//Opretter database tabel og indsætter nogle værdier
fn lav_lektier() -> Result<()> {
    let conn = Connection::open("lektier.sqlite")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS lektier (
            id INTEGER PRIMARY KEY,
            navn TEXT NOT NULL,
            klasse TEXT NOT NULL,
            fravor INTEGER,
            kommet INTEGER
        )",
        [],
    )?;
        let data = vec![
        ("Alice", "2.a", 2, 15),
        ("Bob", "2.b", 1, 10),
        ("Charlie", "2.c", 3, 20),
        ("Diana", "2.d", 2, 5),
        ("Eve", "2.e", 4, 30),
        ("Frank", "2.f", 1, 0),
        ("Grace", "2.g", 2, 25),
    ];
    for (navn, klasse, fravor, kommet) in data {
        conn.execute(
        "INSERT INTO lektier (navn, klasse, fravor, kommet) VALUES (?1, ?2, ?3, ?4)",
        params![navn, klasse, fravor, kommet],
        )?;
    }
    println!("Lektier database oprettet");
    Ok(())
}


//Tilføjer til database ved at bruger input
fn tilfoj_lektie() -> Result<()> {
    let conn = Connection::open("lektier.sqlite")?;
    println!("Tilføj person til lektier database:
    Skriv først navn");
    let mut navn = String::new();
    std::io::stdin().read_line(&mut navn).expect("Error reading input");
    println!("Skriv så klasse");
    let mut klasse = String::new();
    std::io::stdin().read_line(&mut klasse).expect("Error reading input");
    println!("Skriv så fravær");
    let mut fravor_str = String::new();
    std::io::stdin().read_line(&mut fravor_str).expect("Error reading input");
    println!("Skriv så om antalle gange de er kommet");
    let mut kommet_str = String::new();
    std::io::stdin().read_line(&mut kommet_str).expect("Error reading input");
    let fravor1: i32 = fravor_str.trim().parse().unwrap();
    let kommet1: i32 = kommet_str.trim().parse().unwrap();

    conn.execute(
        "INSERT INTO lektier (navn, klasse, fravor, kommet) VALUES (?1, ?2, ?3, ?4)",
        params![navn.trim(), klasse.trim(), fravor1, kommet1],
    )?;
println!("{} tilføjet på databasen", navn.trim());
Ok(())
}

//printer databasen ud
fn los_lektier() -> Result<()> {
    let conn = Connection::open("lektier.sqlite")?;
    println!("Lektier liste:");
    let mut stmt = conn.prepare("SELECT id, navn, klasse, fravor, kommet FROM lektier")?;
    let lektie_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,      // id
            row.get::<_, String>(1)?,   // navn
            row.get::<_, String>(2)?,   // klasse
            row.get::<_, i32>(3)?,      // fravær (fravor)
            row.get::<_, i32>(4)?,      // antal gange mødt op (kommet)
        ))
    })?;
    println!("ID | Navn | Klasse | Fravær | Antal gange mødt op | Fraværsprocent");
    for lektie_res in lektie_iter {
        let (id, navn, klasse, fravor, kommet) = lektie_res?; 
        let fravorprocent = fravor as f32 / (fravor + kommet) as f32 * 100.0;
        println!("{} | {} | {} | {} | {} | {}", id, navn, klasse, fravor, kommet, fravorprocent);
    }
    Ok(())
}

//Ændre database ved at bruger vælger hvad der skal ændres
fn opdater_lektie() -> Result<()> {
    let conn = Connection::open("lektier.sqlite")?;
    println!("Ændre person i lektier database:
    Skriv navn");
    let mut navn = String::new();
    std::io::stdin().read_line(&mut navn).expect("Error reading input");
    let navn = navn.trim();

    // Query existing data for that name
    let mut stmt = conn.prepare("SELECT id, navn, klasse, fravor, kommet FROM lektier WHERE navn = ?1")?;
    let mut rows = stmt.query(params![navn])?;
    
    if let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let navn_db: String = row.get(1)?;
        let klasse: String = row.get(2)?;
        let fravor: i32 = row.get(3)?;
        let kommet: i32 = row.get(4)?;
        
        println!("Eksisterende data:");
        println!("{} | {} | {} | {} | {}", id, navn_db, klasse, fravor, kommet);

        println!("vælg hvad du vil opdatere for {}:
        1. navn: {}
        2. klasse: {}
        3. fravær: {}
        4. Antal gange mødt op: {}", navn, navn, klasse, fravor, kommet);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading input");
        if input.trim() == "1" {
            println!("Skriv nyt navn:");
            let mut nyt_navn = String::new();
            std::io::stdin().read_line(&mut nyt_navn).expect("Error reading input");
            let nyt_navn = nyt_navn.trim();
            conn.execute(
                "UPDATE lektier SET navn = ?1 WHERE id = ?2",
                params![nyt_navn, id],
            )?;
            println!("Navn opdateret til {}", nyt_navn);
        } else if input.trim() == "2" {
            println!("Skriv ny klasse:");
            let mut ny_klasse = String::new();
            std::io::stdin().read_line(&mut ny_klasse).expect("Error reading input");
            let ny_klasse = ny_klasse.trim();
            conn.execute(
                "UPDATE lektier SET klasse = ?1 WHERE id = ?2",
                params![ny_klasse, id],
            )?;
            println!("Klasse opdateret til {}", ny_klasse);
        } else if input.trim() == "3" {
            println!("Skriv nyt fravær:");
            let mut fravor_str = String::new();
            std::io::stdin().read_line(&mut fravor_str).expect("Error reading input");
            let fravor_ny: i32 = fravor_str.trim().parse().unwrap();
            conn.execute(
                "UPDATE lektier SET fravor = ?1 WHERE id = ?2",
                params![fravor_ny, id],
            )?;
            println!("Fravær opdateret til {}", fravor_ny);
        } else if input.trim() == "4" {
            println!("Skriv nyt Antal gange mødt op:");
            let mut kommet_str = String::new();
            std::io::stdin().read_line(&mut kommet_str).expect("Error reading input");
            let kommet_ny: i32 = kommet_str.trim().parse().unwrap();
            conn.execute(
                "UPDATE lektier SET kommet = ?1 WHERE id = ?2",
                params![kommet_ny, id],
            )?;
            println!("Antal gange mødt op opdateret til {}", kommet_ny);
        } else {
            println!("Ugyldigt valg");
        }
    } else {
        println!("Ingen person fundet med navn: {}", navn);
    }
    Ok(())
}


//ud fra et navn scanner den databasen og sletter den hvis man vil
fn slet_lektie() -> Result<()> {
    let conn = Connection::open("lektier.sqlite")?;
    println!("Slet person fra lektier database:
    Skriv navn på person du vil slette");
    let mut navn = String::new();
    std::io::stdin().read_line(&mut navn).expect("Error reading input");
    let navn = navn.trim();

    let mut stmt = conn.prepare("SELECT id, navn, klasse, fravor, kommet FROM lektier WHERE navn = ?1")?;
    let mut rows = stmt.query(params![navn])?;
    
    if let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let navn_db: String = row.get(1)?;
        let klasse: String = row.get(2)?;
        let fravor: i32 = row.get(3)?;
        let kommet: i32 = row.get(4)?;
        
        println!("Du er ved at slette:");
        println!("{} | {} | {} | {} | {}", id, navn_db, klasse, fravor, kommet);
        println!("Er du sikker? (ja/nej)");
        let mut bekraeft = String::new();
        std::io::stdin().read_line(&mut bekraeft).expect("Error reading input");
        
        if bekraeft.trim().to_lowercase() == "ja" {
            conn.execute(
                "DELETE FROM lektier WHERE id = ?1",
                params![id],
            )?;
            println!("{} slettet fra databasen", navn);
        } else {
            println!("Sletning annulleret");
        }
    } else {
        println!("Ingen person fundet med navn: {}", navn);
    }
    Ok(())

}