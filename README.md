# Mira

Mira is a taggable image-board implemented in Rust with a React front-end.

## Building and Running

The backend API is written in Rust, and depends on a running PostgreSQL instance. Start by exporting a `DATABASE_URL`
environment variable. If you don't already have the `diesel_cli`, install it with
```sh
$ cargo install diesel_cli --no-default-features --features postgres
```

Then, go ahead and run the set up and migrations with

```sh
$ diesel setup
$ diesel migration run
```

Once the database is set up, you should be able to run the backend server with

```sh
$ cargo run
```

The frontend is built with React in JavaScript, so ensure you have node and `npm` installed, then run

```sh
$ npm install
$ npm run start
```

in the `mira-web` directory. There should be no extra set up necessary for the frontend and backend to communicate with one another.
