# rwdbms
Robotics website dev-branch management system.

### What is it
This project is a web server that receives websocket messages from GitHub, and then copies it into the `static/` directory. At the same time, it is a HTTP Server that serves that directory.

This project solves the problem of having multiple dev branches for the robotics website. Essentially, it runs a websocket server that GitHub sends requests to when branches are created or pushed to. When the server receives these, it updates the static files in `static/`.


### Setup instructions
#### Development
`cargo run`

### (Production) Structure
| Directory | Description |
| --- | --- |
| static | statically served content |
| repos | git repos |
