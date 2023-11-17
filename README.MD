# rwdbms
Robotics website dev-branch management system.

### What is it
This project is a web server that receives websocket messages from GitHub, and then copies it into the `static/` directory. At the same time, it is a HTTP Server that serves that directory.

### Setup instructions
`cargo run`

### (Production) Structure
| Directory | Description |
| --- | --- |
| static | statically served content |
| repos | git repos |
