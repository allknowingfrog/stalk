# Stalk

A proof-of-concept project, inspired by beanstalkd.

# Usage

## Server

```
cargo run
```

## Client

Example terminal session. There are clearly some kinks to work out...

```
jeremiah@kurt:~$ echo "PUT first" | nc -N 127.0.0.1 7878
CREATED
jeremiah@kurt:~$ echo "PUT second" | nc -N 127.0.0.1 7878
CREATED
jeremiah@kurt:~$ echo "GET" | nc -N 127.0.0.1 7878
ERR
jeremiah@kurt:~$ echo "GET " | nc -N 127.0.0.1 7878
FOUND
first
jeremiah@kurt:~$ echo "GET " | nc -N 127.0.0.1 7878
FOUND
second
jeremiah@kurt:~$ echo "GET " | nc -N 127.0.0.1 7878
NONEjeremiah@kurt:~$
```
