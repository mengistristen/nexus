# nexus

A Markdown note-taking command-line tool.

## Todo

- allow searching notes by their name as well as their file name
- introduce some caching mechanism to avoid reading all metadata from files when we want to traverse the graph
- add ```nexus list``` to list all notes
- add ```nexus consolidate``` (subcommand name TBD) to consolidate notes of a particular branch into a single file
- update hashes stored in metadata when loading a note if we notice that the content has changed
- check for cycles in the graph of notes and refuse linking if a new link would introduce one
- introduce an indexing algorithm to allow for searching note contents
