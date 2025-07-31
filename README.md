# chianti

Collect info about the youtube videos you watch and view the data in the form of charts.

# Chianti Components

| Component         | Description                                                    | Code location       |
| ----------------- | -------------------------------------------------------------- | ------------------- |
| Server            | Recive data from browser extension and store it                | ./src               |
| web               | Display data stored on the server in form of charts and tables | ./web               |
| Browser extension | Send youtube video info to server                              | ./browser-extension |

# Installation

## Browser extension

1. Run `./build-browser-extension.sh` with argument `firefox` or `chrome` or `all` to build the extension and package it.
2. Use the final `.xpi` or `.zip` file to install the extension in your browser

## Server

### Docker

1. Build the Docker Image using `./build-image.sh`
2. Run the Docker Image using `docker run -d --restart always -p 8080:8080 <IMAGE>` - You may add a volume to make the database persistent

### Local

- Run `./installer.sh install` - By default the server will be installed in `${XDG_DATA_HOME}/chianti`
                                  If `XDG_DATA_HOME` is set, otherwise it will be installed in `/usr/share/chianti`

# Uninstallation

## Browser extension

- Go to installed extensions page in your browser and click remove/uninstall

## Server

### Docker

1. Stop the Docker container
2. Remove the Docker container
3. Remove the Docker image

### Local

- Run `./installer.sh uninstall`
