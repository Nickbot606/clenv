# *clenv* - Clean environment
This is a small command line tool which is used to maintain many environmental files and configurations. 

# Why?
This project was created with the intent of having many contributors on a single project which struggle maintaining keys, specific configurations, or wrangling particuarly fickle highly modular highly configurable repositories. 

# Installation

```bash
curl -sSf https://https://github.com/Nickbot606/clenv/install.sh | sh
```

# Usage/Nomanclature

The general usage is stored into three basic categories 
**Namespaces:** These are rocks db's column families
**Collections:** Collctions are essentially a basic schema with .envs associated with them. 
**Env:** Finally, each .env file/secrets file is encrypted as individual blob

## Best Practice
The way I like to use this tool is to use the Namespace as a project, then each of my collections are a branch or speicfic way to configure a large section of a monorepository, then each blob/env being specific to the section/part of the application that I need configured.

# Usage and arguments/features

| description | example | description |
| --- | --- | --- |
| init | clenv init --name first.db | creates a new environmental database |
| add | clenv add keys.pub | adds a public key to envrpt the users' keys | 
| show | clenv show * | shows the keys for a specific keys | 