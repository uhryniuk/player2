
# Server Tasks

- [x] Rework RouterInfo for pretty printing paths and methods.
    - Required to get server compiling again.
- [x] Add more structured logging.
    - I have a link for a PROD level config.
    - But for the most part, just do basic level config.
- [ ] Clean up config and have valid options for hostname and ports.
- [ ] Clean up HttpServer, it's a real mess.


# Notes
- Don't try to map knowledge from other languages 1-1 in rust.
    - Just read docs, and learn fresh. The language is very terse.

## Frontend Layout


## Frontend style/theme
some fluid background that's black with lighting.

~~Floating Gameboard, chat, leaderboards,~~
~~All with the glassy style background goodness.~~

Nah, the above is too nice, fuck this project
- we using 98.css
https://unpkg.com/98.css



## Rebuild Game
- Add multiplayer
- Add simulated multiplayer with AI.
- Add chat functionality?
- Auth
- Leaderboards / standings
    - Wins, Losses, Total moves, avg moves, 
    - Add dumbie values as well.
        - Scrap some random usernams from reddit or some shit.

## Infra
- Remove the nginx config, that'll go on linode.
    - Nevermind, the nginx config isn't for deployment now.
    - It's for building a local docker dev container.

- convert all js to ts.
