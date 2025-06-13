# *clenv* - Clean environment
This is a small command line tool which is used to maintain many environmental files and configurations. 

# Why?
This project was created with the intent of having many contributors on a single project which struggle maintaining keys, specific configurations, or wrangling particuarly fickle  repositories. 

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
This configuration file can be found in the binary's location.
| name | example | description |
| --- | --- | --- |
| db | db="/path/to/db/" | Location of rocksdb folder |
| pub | pub="/path/to/.pub" | Location of public key on local machine |
| priv | priv="/path/to/.crt" | Location of the private key on local machine |

## Arguments and Examples
| name | interface | example | description |
| --- | --- | --- | --- |
| init | clenv init [none] | clenv init --name name_of_db | creates a new environmental database |
| add | clenv add [path to .pub file] | clenv add keys.pub | adds a public key to envrpt the receipients' keys | 
| show | clenv show [none] | clenv show | shows the currently selected database, users who have access, and available namespaces | 
| cfg | clenv cfg [name of key] [argument for keys] | clenv cfg --db "path/to/db" | changes a specific configuration which is located |
| dump | clenv dump [name space name] [name of ouptut file] | clenv dump name_of_db "out.env" | dumps all blocks into individual env files to current working directory |
| write | clenv write [names space name] [name of blob] | clenv write ns |
