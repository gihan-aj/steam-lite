### SteamSpy Data

This is an API for Steam Spy. It accepts requests in a GET string and returns data in JSON arrays.

  *IMPORTANT:* some things have changed, please, read this document through!

  The data is refreshed once a day, there is no reason to request the same information more than once every 24 hours.

  Allowed poll rate - 1 request per second for most requests, 1 request per 60 seconds for the *all* requests.

  ## Examples: ##
   
  * steamspy.com/api.php?request=appdetails&appid=730 - returns data for Counter-Strike: Global Offensive
  * steamspy.com/api.php?request=top100in2weeks - return Top 100 apps by players in the last two weeks
  * steamspy.com/api.php?request=all&page=1 - return apps 1,000-1,999 of all apps.


  ## Common parameters: ##
 
  * request - code for API request call.
  * appid - Application ID (a number).


  ## Accepted requests: ##
  
  ### appdetails ###

  Returns details for the specific application. Requires *appid* parameter.  

  ### genre ###

  Returns games in this particular genre. Requires *genre* parameter and works like this:
  
  * steamspy.com/api.php?request=genre&genre=Early+Access

  ### tag ###

  Returns games with this particular tag. Requires *tag* parameter and works like this:
  
  * steamspy.com/api.php?request=tag&tag=Early+Access

  ### top100in2weeks ###

  Returns Top 100 games by players in the last two weeks.

  ### top100forever ###

  Returns Top 100 games by players since March 2009.

  ### top100owned ###

  Returns Top 100 games by owners.

  ### all ###

  Returns all games with owners data sorted by owners. Returns 1,000 entries per page.
  * page - page number for the list (starts at 0)


  ## Return format for an app: ##

  * appid - Steam Application ID. If it's 999999, then data for this application is hidden on developer's request, sorry.
  * name - game's name
  * developer - comma separated list of the developers of the game
  * publisher - comma separated list of the publishers of the game
  * score_rank - score rank of the game based on user reviews
  * owners - owners of this application on Steam as a range.
  * average_forever - average playtime since March 2009. In minutes.
  * average_2weeks - average playtime in the last two weeks. In minutes.
  * median_forever - median playtime since March 2009. In minutes.
  * median_2weeks - median playtime in the last two weeks. In minutes.
  * ccu - peak CCU yesterday.
  * price - current US price in cents.
  * initialprice - original US price in cents.
  * discount - current discount in percents.
  * tags - game's tags with votes in JSON array.
  * languages - list of supported languages.
  * genre - list of genres.


  ## Questions? ##

  Contact me by e-mail: *sergey at galyonkin dot com*.

## Endpoints

***GET:https://steamspy.com/api.php?request=appdetails&appid=1462040***

```json
{
    "appid": 1462040,
    "name": "FINAL FANTASY VII REMAKE INTERGRADE",
    "developer": "Square Enix",
    "publisher": "Square Enix",
    "score_rank": "",
    "positive": 33079,
    "negative": 4228,
    "userscore": 0,
    "owners": "1,000,000 .. 2,000,000",
    "average_forever": 0,
    "average_2weeks": 0,
    "median_forever": 0,
    "median_2weeks": 0,
    "price": "1399",
    "initialprice": "3999",
    "discount": "65",
    "ccu": 761,
    "languages": "English, French, Italian, German, Spanish - Spain, Spanish - Latin America, Portuguese - Brazil, Simplified Chinese, Traditional Chinese, Japanese, Korean",
    "genre": "Action, Adventure, RPG",
    "tags": {
        "RPG": 385,
        "Action RPG": 350,
        "JRPG": 347,
        "Singleplayer": 307,
        "Action-Adventure": 299,
        "Party-Based RPG": 270,
        "Spectacle fighter": 245,
        "Third Person": 240,
        "Action": 236,
        "Controller": 225,
        "Adventure": 223,
        "3D": 219,
        "Fantasy": 217,
        "3D Platformer": 165,
        "Magic": 163,
        "Story Rich": 136,
        "Cinematic": 130,
        "Realistic": 120,
        "Drama": 117,
        "Sci-fi": 102
    }
}
```

***GET:https://steamspy.com/api.php?request=all&page=1***

```json
{
    "9450": {
        "appid": 9450,
        "name": "Warhammer 40,000: Dawn of War - Soulstorm",
        "developer": "Relic Entertainment",
        "publisher": "SEGA",
        "score_rank": "",
        "positive": 16764,
        "negative": 847,
        "userscore": 0,
        "owners": "1,000,000 .. 2,000,000",
        "average_forever": 0,
        "average_2weeks": 0,
        "median_forever": 0,
        "median_2weeks": 0,
        "price": "0",
        "initialprice": "0",
        "discount": "0",
        "ccu": 673
    },
    "490220": {
        "appid": 490220,
        "name": "Prismata",
        "developer": "Lunarch Studios",
        "publisher": "Lunarch Studios",
        "score_rank": "",
        "positive": 970,
        "negative": 217,
        "userscore": 0,
        "owners": "1,000,000 .. 2,000,000",
        "average_forever": 0,
        "average_2weeks": 0,
        "median_forever": 0,
        "median_2weeks": 0,
        "price": "0",
        "initialprice": "0",
        "discount": "0",
        "ccu": 6
    },
    "1301720": {
        "appid": 1301720,
        "name": "Escape Room - Der kranke Kollege",
        "developer": "Bitbeast Games",
        "publisher": "Bitbeast Games",
        "score_rank": "",
        "positive": 9872,
        "negative": 2233,
        "userscore": 0,
        "owners": "1,000,000 .. 2,000,000",
        "average_forever": 0,
        "average_2weeks": 0,
        "median_forever": 0,
        "median_2weeks": 0,
        "price": "0",
        "initialprice": "0",
        "discount": "0",
        "ccu": 63
    },
    "921060": {
        "appid": 921060,
        "name": "Modern Combat 5",
        "developer": "Gameloft",
        "publisher": "Gameloft",
        "score_rank": "",
        "positive": 3130,
        "negative": 2595,
        "userscore": 0,
        "owners": "1,000,000 .. 2,000,000",
        "average_forever": 0,
        "average_2weeks": 0,
        "median_forever": 0,
        "median_2weeks": 0,
        "price": "0",
        "initialprice": "0",
        "discount": "0",
        "ccu": 1
    }
}
```

***GET:https://steamspy.com/api.php?request=top100in2weeks***
- Top 100 (2 Weeks): request=top100in2weeks (Ranked by active players over the past fortnight).
- Top 100 (Forever): request=top100forever (Ranked by active players historically since March 2009).
- Top 100 (Owned): request=top100owned (Ranked by highest total volume of owners).

```json
{
    "730": {
        "appid": 730,
        "name": "Counter-Strike: Global Offensive",
        "developer": "Valve",
        "publisher": "Valve",
        "score_rank": "",
        "positive": 7642084,
        "negative": 1173003,
        "userscore": 0,
        "owners": "100,000,000 .. 200,000,000",
        "average_forever": 0,
        "average_2weeks": 0,
        "median_forever": 0,
        "median_2weeks": 0,
        "price": "0",
        "initialprice": "0",
        "discount": "0",
        "ccu": 1013936
    },
    "1172470": {
        "appid": 1172470,
        "name": "Apex Legends",
        "developer": "Respawn",
        "publisher": "Electronic Arts",
        "score_rank": "",
        "positive": 668053,
        "negative": 326926,
        "userscore": 0,
        "owners": "100,000,000 .. 200,000,000",
        "average_forever": 0,
        "average_2weeks": 0,
        "median_forever": 0,
        "median_2weeks": 0,
        "price": "0",
        "initialprice": "0",
        "discount": "0",
        "ccu": 124262
    },
    "578080": {
        "appid": 578080,
        "name": "PUBG: BATTLEGROUNDS",
        "developer": "PUBG Corporation",
        "publisher": "KRAFTON, Inc.",
        "score_rank": "",
        "positive": 1520457,
        "negative": 1037487,
        "userscore": 0,
        "owners": "100,000,000 .. 200,000,000",
        "average_forever": 0,
        "average_2weeks": 0,
        "median_forever": 0,
        "median_2weeks": 0,
        "price": "0",
        "initialprice": "0",
        "discount": "0",
        "ccu": 314682
    }
}
```