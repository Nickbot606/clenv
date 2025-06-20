# *clenv* - Clean environment
This is a small command line tool which is used to maintain many environmental files and configurations. 

# Motivation
This project was created with the intent of alleviating the stress on teams which have an offline first /locally test attitude with many configurations and branches with interface drift. 

# Installation

```bash
curl -sSf https://https://github.com/Nickbot606/clenv/install.sh | sh
```

# Usage/Nomanclature

The general usage is stored into three basic categories 
- **Namespaces:** These are rocks db's column families
- **Collections:** Collctions are essentially a basic schema with .envs associated with them. 
- **Env:** Finally, each .env file/secrets file is encrypted as individual blob

## Best Practice
The way I like to use this tool is to use the Namespace as a project, then each of my collections are a branch or speicfic way to configure a large section of a monorepository, then each blob/env being specific to the section/part of the application that I need configured.

# Usage
## Configuration
*clenv* is not free from having confgiruations itself, however these are kept to a minimum in order to maintain a semblance of state management. All of these keys can be changed remotely via the `cfg` argument.
This configuration file can be found in `~/.config/clenv`

| name | example | description |
| --- | --- | --- |
| db | db="/path/to/db/" | Location of rocksdb folder |
| ns | ns="current_namespace" | Currently selected namepace (this can also be changed with the `clenv ns` command) |
| priv | priv="/path/to/.crt" | Location of the private key on local machine |

## Arguments and Examples
| name | interface | example | description |
| --- | --- | --- | --- |
| add | clenv add [name] | clenv add "keys.pub" | adds a public key to add the users' keys | 
| show | clenv show [none] | clenv show | shows the currently selected database, users who have access, and available namespaces | 
| cfg | clenv cfg [name of key] [argument for keys] | clenv cfg --db "path/to/db" | changes a specific configuration remotely. (Putting just clenv cfg will list current configuration) |
| ns | clenv ns [name of namespace] | clenv ns "ns" | changes the currently selected namespace. Putting no  |
| dump | clenv dump [name space name] [name of ouptut file] | clenv dump name_of_db | dumps all blocks into individual env files from the namespace to current working directory |
| write | clenv write [path to .env] [name of blob] | clenv write "./.env" "default_env" | writes the selected file to the currently selected namespace with a name of your choosing. If you match a blob name exactly, it will overwrite said blob | 

