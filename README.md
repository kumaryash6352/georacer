# What is Georacer?

Georacer is a real, world-space scavenger hunting game, where you and other
players compete to be the first to take an image of an object or area.

When you join a lobby and start the game, all players are given the same random
object from a database of objects.  This is a zoomed in image of the object or area,
which will progressively zoom out over time. The first person to find the object or area
and take a picture of it wins.

We use the Gemini 2.5 Flash model to tell if the image taken by the user, which
may be from a different perspective or in different lighting, is the same as the
target in the database.

The game is accessed through a website using React, which is rendered to a
static website. MongoDB is used as the backing database. A hosted server written
in Rust using Axum coordinates and matchmakes. The website also includes the
ability to register your own objects to create your own playlist, saving both
the taken image and the GPS coordinates. Websockets are used to communicate
within a game lobby.

Keys are in `.env`. Use dotenv or source the .env file to load them.

## UI Flow
On first open, users are given a text description of the game.

"GeoRacer is a fast paced game about knowing your home, street, or town. Be the
first to find and snap a picture of the object to score!"

At the bottom are two buttons: "Create Lobby" or "Join Lobby".

Creating a lobby generates a QR code, which can be scanned by another person to
join your lobby.  From the create lobby screen, you can see who's joined, game
settings, and you can start the game.

Game settings are:
1. Amount of points to win.
   
   Players are given points when they find the object. Players earn 1 point for
   every player they beat to the object.

2. Target list.

   For this proof of concept, the target set is fixed.
   
3. Players that can score per object.

   After this many players find the object, a new object is selected.

### Game Loop
The game is fast paced.

1. Every player receives a countdown of 3 seconds.
2. An image of the object to hunt for is displayed.

   In this screen, there's the image of the object at the top row. There's a count of
   players who have found it and time taken on the next row. The middle,
   larger row has the camera feed with a button to submit a picture of the object.
   The bottom most row has a hot or cold meter to the "canonical" object: the saved GPS coordinates.
   Your current position is compared to where you were 5 seconds ago, and if you got closer to the
   saved GPS coordinates of the object, it shows "Hotter!".

3. Once enough players find the object, the phone buzzes and a new object is
   selected. Goto 2.
4. Once a player wins, the game ends and a leaderboard is displayed. Players
   have the option to rejoin the lobby.
