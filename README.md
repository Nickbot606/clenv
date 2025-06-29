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
This configuration file can be found in your operating system's configuration directory.

| name | example | description |
| --- | --- | --- |
| db | db="/path/to/db/" | Location of rocksdb folder |
| ns | ns="current_namespace" | Currently selected namepace (this can also be changed with the `clenv ns` command) |
| private_key | priv="/path/to/.crt" | Location of the private key on local machine |

## Arguments and Examples

### NOTE:
If the arugment or command can take in a file path location, it will first check to see if it is a relative or absolute path. If it is neither, it will assume the file is located within the current directory and use the name provided. This makes it so that you can run a command like:
`clenv store text.txt`

Your configuration file will also be adjusted to reflect the absolute path. So if you start a database in a private directory, it will remove it. 


### cfg
cfg is the inital configuration argument. This argument can be run alone like such: 
`clenv cfg`
This will give you a printed list of all of the current configurations.

To individually set one argument, you can run 
`clenv cfg [argument name] [new arugment value]`
for example: `clenv cfg ns second_namespace`
this will change the namespace to "second_namespace"

if you would like to reset all of your configs instead, use 
`clenv cfg init` and it will reprompt you for your name, private key, and database name.

### store
store will store the currently selected file. 
`clenv store test.txt` will store test.txt form the current directory into the currently selected namespace. This will be a different test.txt than a seperate namespace.

This program will first take in the available file, compress it, then encrypt it with your currently conifgured private key, and add it to your currently selected namespace. If you add a second arugment such as `clenv store test.txt test-dev` it will use that name instead as your current file name. Clenv is designed to maintain your current file extension upon storage as well as store the file as an encrypted binary so in theory it could store any file type.

The database can store multiple files of the same name in seperate files, however, it will overwrite one if you are in a currently selected file.
Also note that if you write a file to a namespace that doesn't exist, it will automatically create said namespace.

**disclaimer**
The CLI uses zstd for file compression and oaep rsa for encryption. It does not encrypt the file extenion nor does it encrypt the namespaces or names of entries. It does encrypt the entireity of the file itself.

### dump
dump will write the env/entry to a file. It will use the name of the entry + the file extension it had upon storing it into the database.
`clenv dump test.txt`

If you have a file which is named identically and you choose to write this file, it will overwrite your current file so please be careful.

### show
show if no other arugments will display all of the currently available namespaces. If you speicfy a namespace after show it will display all the entries for that namespace.
`clenv show`

`clenv show ns`

Note that if you change your current namespace, it will not create said namespace until you have stored at least one file into that space.
Use this function to also see who your recipients are by doing the following: 
`clenv show keyring`

### rm
rm removes the entry from the currently selected namespace. 
`clenv rm test.txt`

### add
adds a user to the keyring. Note that this will not update your config to the new rsa public and private keys. But it will add their private key to your current working directory.
`clenv add alice`

note: You can also use "add" to rotate your key if an identical name is entered.

### remove
remove removes a user from the keyring.
`clenv remove alice`

# Features roadmap
1. Windows version (without the need for wsl)
2. Unit testing/integration testing
3. Database obfuscation so that if you do not have a private key, you cannot read namespaces or entries in the database.
4. Ability to sync with cloud services such as s3 or other online services.
5. Furhter hardening of features and make it more ergonomic to use (more arguments, flags, better error checking and cleanup of code)
6. Colored arguments so errors are easier to read
7. Add properties to recipients (such as read only permissions).
8. Go from single threaded RocksDB to multithreaded.
9. Ability to merge users and keys between databases.
10. Possibly add a TUI or some type of other interactive way to use the toolset?