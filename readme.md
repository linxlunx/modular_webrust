# modular_webrust

Using [Nickel](https://github.com/nickel-org/nickel.rs) to build modular web service in Rust. I am using MySQL database for this case. You can generate the database first and sample data with [dbmigrate](https://github.com/Keats/dbmigrate). I only create the login function to show how to deal with the module. You can use this command to test the login function.

```
$ curl -d "username=linggar&password=password" http://localhost:3000/users/login
```

## Installation
- Copy env.ini.sample to env.ini
- Fill the database detail in env.ini file
- Run build command 
```
$ cargo build
```
- Let's rock
```
$ ./target/debug/modular_webrust
```
### TO DO
A lot of things to do :p

### NOTES TO RUST DEVELOPER
*Never Give Up*

