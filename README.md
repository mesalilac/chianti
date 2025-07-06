# chianti

Collect info about the youtube videos you watch and view the data in the form of charts.

# Chianti Components

| Component         | Description                                                    | Code location       |
| ----------------- | -------------------------------------------------------------- | ------------------- |
| Server            | Recive data from browser extension and store it                | ./src               |
| webUI             | Display data stored on the server in form of charts and tables | ./webUI             |
| Browser extension | Send youtube video info to server                              | ./firefox-extension |

# How to build

- Create the docker image for the server and run it
- Build the firefox extension and install it

# Setup

- Open the firefox extension and set the base url in the settings for the api to point to where the server is
