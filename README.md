# api_album
API for albums

## Framework
* actix-web : web framework

## Other packages
* actix-rt : Actix runtime
* actix-service : used for middleware  
* serde : se-des-rialization framework
* chrono : Date and time library
* jsonwebtoken : Create and parse JWT 
* log : logging facade
* simplelog : Logger
* dotenv : to use variables from .env file
* futures : asynchronous tasks
* diesel : ORM
* r2d2 : connection pool

## Installation

### Rust

Install Rustup + Cargo :

https://www.rust-lang.org/learn/get-started

### Database

Install PostgreSQL : 

https://www.postgresql.org/

Install PgAddmin : 

https://www.postgresql.org/


## Configuration

### Dotenv

Create in root folder the file `.env` containing : 
```
JWT_SECRET=jwt_secret
LOG_FILE=/your/log_file.log
DATABASE_URL=postgres://username:password@localhost/album
```

For Windows double `\` in path

ex : 
```
LOG_FILE=C:\\Users\\toto\\log.log 
```

### Database

Create the database with `/sql/script.sql`

Create user in BDD :
```SQL
INSERT INTO public."user"("email", "password")
VALUES ('test@test.com', encode(digest('password', 'sha256'), 'base64'))
```

## Run
```Shell
cargo run
```

## Debugging with VSCode
Install extensions :
* Rust (rls)
* CodeLLDB (require python 3.6)

## Testing
http://127.0.0.1:8080/login

## Errors

If you encounter this error at build : 
```
error: linking with `cc` failed: exit code: 1
```

Fedora : Symlink missing for Pgsql
```Shell
sudo ln -s /usr/lib64/libpq.so.5 /usr/lib/libpq.so
```
Windows : Add in Path
```
C:\Program Files\PostgreSQL\11\lib
```