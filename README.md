---

# MITRE ATT&CK Matrix TTPs to SQLite

This application converts the TTPs from the MITRE ATT&CK Matrix into an SQLite database.

## Prerequisites

Before running this application, you need to create the SQLite database.

## Instructions

### Step 1: Create the Database

To create the database, use the `sqlite3.exe` command followed by the name of the database file. For example, to create a database named `mitre_matrix.db`, run the following command:

```sh
sqlite3.exe mitre_matrix.db
```

### Step 2: Create the Tables

Once the database is created, you need to create the necessary tables. Use the following SQL commands to create the tables:

```sql
CREATE TABLE IF NOT EXISTS Tactics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Techniques (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Tactics_Techniques (
    tactic_id INTEGER,
    technique_id INTEGER,
    FOREIGN KEY (tactic_id) REFERENCES Tactics(id),
    FOREIGN KEY (technique_id) REFERENCES Techniques(id),
    PRIMARY KEY (tactic_id, technique_id)
);
```

You can execute these commands within the SQLite command line interface. After running `sqlite3.exe mitre_matrix.db`, paste and execute each of the `CREATE TABLE` commands above.

### Example

Here is a step-by-step example:

1. Open your terminal or command prompt.
2. Run the command to create the database:

   ```sh
   sqlite3.exe mitre_matrix.db
   ```

3. In the SQLite command line interface, execute the following commands:

   ```sql
   CREATE TABLE IF NOT EXISTS Tactics (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       name TEXT NOT NULL
   );

   CREATE TABLE IF NOT EXISTS Techniques (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       name TEXT NOT NULL
   );

   CREATE TABLE IF NOT EXISTS Tactics_Techniques (
       tactic_id INTEGER,
       technique_id INTEGER,
       FOREIGN KEY (tactic_id) REFERENCES Tactics(id),
       FOREIGN KEY (technique_id) REFERENCES Techniques(id),
       PRIMARY KEY (tactic_id, technique_id)
   );
   ```

4. Exit the SQLite command line interface by typing `.exit`.

Your SQLite database is now ready with the required tables. You can now proceed to use the application.

---

Feel free to customize the application.
