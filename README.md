# chianti

Collect info about the youtube videos you watch and view the data in the form of charts.

# Chianti Components

| Component         | Description                                                    | Code location       |
| ----------------- | -------------------------------------------------------------- | ------------------- |
| Server            | Recive data from browser extension and store it                | ./src               |
| webUI             | Display data stored on the server in form of charts and tables | ./web               |
| Browser extension | Send youtube video info to server                              | ./browser-extension |

# How to build

- Build the docker image for the server and run it
- Build the firefox extension and install it

# Setup

- Open the firefox extension and set the base url in the settings for the api to point to where the server is

## Build server docker image

### Build docker image

```bash
./build.sh
```

### Run docker image

```bash
docker run -d -p 3241:3241 <CONTAINER>
```

Add a volume to make the database persistent
