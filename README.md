# Arrrasv2
A new version of arras.io. Community built, hosted, and coded. Read README to help creating.

Due to the current state of the arras.io game, we have decided to take matters in our own hands. The goal is to make a backend based on rust/ rust wrapped c++/ node js (avoid if possible, or use deno/bun if js is absolutely needed),and a frontend with canvas or equivalent (update : pixijs seems promising, due to easy implementation of shapes, textures, etc and use of both canvas2D and webgl). Communication will be engine.io. To cover the problem of servers, we will make one permanent server (looking for a host ?) that manages captchas,cookies, middlewares, etc, and redirects to several individual servers (which anyone can set up on their computer if they want, set a player cap, and set the mode). The main server will display a list of available servers to go on. We are looking for developpers and server hosts, any contribution is greatly appreciated.

As a starting point, check Lucrehulk's reverse engineered arras. It will help for game physics and graphics.

Current state : made some base communication functions, which must be debugged. The rust part is very dubious. There seems to be a minor problem in the graphics for serverjoin. A dependecy lock file must be added too.
Rust will likely be wrapped with neon to make server part easier. Node js will only be used for sockets, requests (and encrypting/captchas), and server setup. Backend (game mechanics, data storage, maze generation) will be in rust, and pixijs frontend to display based on coordinates. Captchas are to be rendered server side and changed every half second at least.

Current state on 11th july : switched to full rust, the image captcha works. The game data structure is very nearly done (except drones), the socket handling is done. The next test is physics loop.
