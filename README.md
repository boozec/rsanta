# rsanta - make your Secret Santa

You must set environments variables for smtp:
```
export EMAIL=youremail@tld.com
export PASSWORD=passsword
export HOST=smtp_host
```

Create a members file with the following syntax:
```
// file_for_members.txt
Name Surname <email@tld.com>
Name2 Surname2 <email2@tld.com>
```

Then build the source code:
```
cargo build
```

Then call the program with the file previously created as `-f` param:
```
/target/debug/rsanta -f <file_for_members.txt>
```
