

# IDEAS
- Instead of a panel, we should let the user, restart, choose the gamemodes and everything through commands in the chat.
    - This would be really cool.

# Development Story

1. New User lands on the page
- Can play against the AI on easy mode only. 
- Cannot access chat or play online at all.
- No stats for leaderboard will be tracked or anything like that.

2. User decides to signup/login.
- Unlock chat, friends, leaderboard and the works.
- User can send match invite links.

## Config options

- local, dev, prod
- admin user, login user, no user.
- is logged in, not logged in.
- diff game difficulties.
- 'clicktostart' b4 a game.
- Waiting for player to join.
    - persistent lobby where user can join in the background.



### Version 1 (MVP)
- Simple login,auth,signup.
- pop up modal for first time users, we can check their IP or something and see.
- Simple Leaderboard.
- Single difficulty.
- simple cmd prompt.
- no online at all, or at least no websockets setup yet.
- no mobile responsiveness.
- Game is fully functional and doesn't need to be difficult.

#### TODOski
- Get Postgres Setup locally, some persistence is needed.
- Make rough schema for User then leaderboard later on.
- Don't delete but disconnect Websockets on frontend.
- Finish the game implementation.

##### Client
- Build a better modal component to spawn whatever content I need.
- Build a stream for adding new 'event' messages to chat for whatever reasons.
- Build simple cmd parser with callback system.
- Finish the game implemntation by converting the old code.
- Build out abtract stuff for method calling, sockets, message, auth.

##### Server
- Rebuild backend to focus only on connect4 and the implementation I am focusing on now.
- Get a dumb easy auth/auth endpoint working.
- Simple endpoint for Leaderboard, just send dumbie data over.
- Get a guard figured out for the auth stuff.
- Consider getting AppState figured out too.
- Try not touching the AI code until the server stuff is sorted out.


### Version 2
- uhhhh

---

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
