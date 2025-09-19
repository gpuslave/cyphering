use std::collections::HashMap;
use std::fmt;

// Represents the permissions for a user on an object.
#[derive(Clone, Copy, Debug)]
struct Permissions {
    read: bool,
    write: bool,
    execute: bool,
    owner: bool,
}

impl Permissions {
    // Create a new Permissions struct
    fn new(read: bool, write: bool, execute: bool, owner: bool) -> Self {
        Self { read, write, execute, owner }
    }
}

// Implement Display to print permissions in "rwxo" format.
impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}",
            if self.read { 'r' } else { '-' },
            if self.write { 'w' } else { '-' },
            if self.execute { 'x' } else { '-' },
            if self.owner { 'o' } else { '-' }
        )
    }
}

fn main() {
    let users: Vec<&str> = vec!["Admin", "Aleksei", "Hacker"];
    let files: Vec<String> = vec![
        "document.txt".to_string(),
        "image.png".to_string(),
        "secret_plan.docx".to_string(),
    ];

    // The access matrix is a HashMap where the key is a tuple (user_index, file_index)
    // and the value is the Permissions struct.
    let mut access_matrix: HashMap<(usize, usize), Permissions> = HashMap::new();

    // --- Populate the access matrix with some initial permissions ---

    // Admin (user 0) has full control over all files.
    for i in 0..files.len() {
        access_matrix.insert((0, i), Permissions::new(true, true, true, true));
    }

    // Aleksei (user 1) owns document.txt and can read image.png.
    access_matrix.insert((1, 0), Permissions::new(true, true, true, true)); // rwxo for document.txt
    access_matrix.insert((1, 1), Permissions::new(true, false, false, false)); // r--- for image.png

    // Hacker (user 2) has no permissions by default. We don't need to insert anything
    // for the Hacker, as the absence of a key implies no permissions.

    // --- Print the access matrix ---
    println!("Access Control Matrix:");
    println!("{:<10} | {:<20} | Permissions", "User", "File");
    println!("{:-<11}|{:-<22}|{:-<12}", "", "", "");

    for (user_index, user) in users.iter().enumerate() {
        for (file_index, file) in files.iter().enumerate() {
            let perms = access_matrix.get(&(user_index, file_index))
                .cloned()
                .unwrap_or(Permissions::new(false, false, false, false)); // Default to no permissions
            println!("{:<10} | {:<20} | {}", user, file, perms);
        }
        if user_index < users.len() - 1 {
            println!("{:-<11}|{:-<22}|{:-<12}", "", "", "");
        }
    }
}