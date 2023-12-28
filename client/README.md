# todos

- [ ] Finish strucuture and sizing of the layouts, blocks for each section of the app.

- [ ] Reconfig useWebSocket.ts
    - I don't like having the datastructure as the file name.



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
