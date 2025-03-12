# SpringUp

## Still working progress project !!!

### Features that should work:  
    [X] Manually setting the groupId and artifactId  
    [X] Automatically setting the groupId and artifactId from the pom  
    [X] Creating the controller, service, model, and repository directories  
    [X] Creating the model, service, and repository classes with basic template given the name  
    [X] Creating the model with SQL attributes from the .sql file
    [ ] Creating the model with SQL attributes by connecting to the database

### Usage

#### Commands

- `-a`: Parses the `pom.xml` file and sets the `groupId` and `artifactId` from it.
    ```sh
    spb-utils-rust -a
    ```

- `-d`: Creates the necessary directories for the project structure based on the `groupId` and `artifactId` from the configuration file.
    ```sh
    spb-utils-rust -d
    ```

- `-s <groupId> <artifactId>`: Manually sets the `groupId` and `artifactId` in the configuration file.
    ```sh
    spb-utils-rust -s com.example myartifact
    ```

- `-f <name> [-t <sql_file>]`: Creates model files with SQL attributes. If no SQL file is specified, it defaults to `init.sql`.
    ```sh
    spb-utils-rust -f MyModel
    spb-utils-rust -f MyModel -t custom.sql
    ```

#### Examples

- Display help information:
    ```sh
    spb-utils-rust -h
    ```

- Parse `pom.xml` and set `groupId` and `artifactId`:
    ```sh
    spb-utils-rust -a
    ```

- Create project directories:
    ```sh
    spb-utils-rust -d
    ```

- Manually set `groupId` and `artifactId`:
    ```sh
    spb-utils-rust -s com.example myartifact
    ```

- Create model files with SQL attributes from `init.sql`:
    ```sh
    spb-utils-rust -f MyModel
    ```

- Create model files with SQL attributes from a custom SQL file:
    ```sh
    spb-utils-rust -f MyModel -t custom.sql
    ```