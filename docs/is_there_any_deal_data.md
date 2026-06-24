### IsThereAnyDeal Price History Data

Following JSON structure represents a game's ITAD UUID and basic details.


***GET:https://api.isthereanydeal.com/games/lookup/v1?key={itad_api_key}&appid=221040***

```json
{
    "found": true,
    "game": {
        "id": "018d937f-5b12-7380-a77e-8754efb603d0",
        "slug": "final-fantasy-vii-remake-intergrade",
        "title": "FINAL FANTASY VII REMAKE INTERGRADE",
        "type": "game",
        "mature": false,
        "assets": {
            "boxart": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/boxart.jpg?t=1773897908",
            "banner145": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/banner145.jpg?t=1773897908",
            "banner300": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/banner300.jpg?t=1773897908",
            "banner400": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/banner400.jpg?t=1773897908",
            "banner600": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/banner600.jpg?t=1773897908"
        }
    }
}
```

Following JSON structure represents the `historylow` endpoint returns a list of game IDs that have had their price history updated in the last 24 hours.

***POST:https://api.isthereanydeal.com/games/historylow/v1?key={itad_api_key}&country=lk***

Body:
["018d937f-5b12-7380-a77e-8754efb603d0", "018d937f-6b99-7077-9cf6-eacfce97587f", "018d937e-fd75-7351-be7d-a0be822b98a1"]

```json
[
    {
        "id": "018d937e-fd75-7351-be7d-a0be822b98a1",
        "low": {
            "shop": {
                "id": 26,
                "name": "GamesPlanet UK"
            },
            "price": {
                "amount": 3.37,
                "amountInt": 337,
                "currency": "USD"
            },
            "regular": {
                "amount": 25.34,
                "amountInt": 2534,
                "currency": "USD"
            },
            "cut": 87,
            "timestamp": "2023-12-18T00:55:03+01:00"
        }
    },
    {
        "id": "018d937f-5b12-7380-a77e-8754efb603d0",
        "low": {
            "shop": {
                "id": 61,
                "name": "Steam"
            },
            "price": {
                "amount": 13.99,
                "amountInt": 1399,
                "currency": "USD"
            },
            "regular": {
                "amount": 39.99,
                "amountInt": 3999,
                "currency": "USD"
            },
            "cut": 65,
            "timestamp": "2026-06-11T19:25:59+02:00"
        }
    },
    {
        "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
        "low": {
            "shop": {
                "id": 26,
                "name": "GamesPlanet UK"
            },
            "price": {
                "amount": 3.35,
                "amountInt": 335,
                "currency": "USD"
            },
            "regular": {
                "amount": 16.8,
                "amountInt": 1680,
                "currency": "USD"
            },
            "cut": 80,
            "timestamp": "2022-10-03T10:10:03+02:00"
        }
    }
]
```

***https://api.isthereanydeal.com/games/overview/v2?key={itad_api_key}&country=lk&shops=61***

Body:
["018d937f-5b12-7380-a77e-8754efb603d0"]

```json
{
    "prices": [
        {
            "id": "018d937f-5b12-7380-a77e-8754efb603d0",
            "current": {
                "shop": {
                    "id": 61,
                    "name": "Steam"
                },
                "price": {
                    "amount": 13.99,
                    "amountInt": 1399,
                    "currency": "USD"
                },
                "regular": {
                    "amount": 39.99,
                    "amountInt": 3999,
                    "currency": "USD"
                },
                "cut": 65,
                "voucher": null,
                "flag": "H",
                "drm": [],
                "platforms": [
                    {
                        "id": 1,
                        "name": "Windows"
                    }
                ],
                "timestamp": "2026-06-11T19:25:59+02:00",
                "expiry": "2026-06-25T19:00:00+02:00",
                "url": "https://itad.link/018d9386-63d7-7309-b125-fcda6ae0e965/"
            },
            "lowest": {
                "shop": {
                    "id": 61,
                    "name": "Steam"
                },
                "price": {
                    "amount": 13.99,
                    "amountInt": 1399,
                    "currency": "USD"
                },
                "regular": {
                    "amount": 39.99,
                    "amountInt": 3999,
                    "currency": "USD"
                },
                "cut": 65,
                "timestamp": "2026-06-11T19:25:59+02:00"
            },
            "bundled": 0,
            "urls": {
                "game": "https://isthereanydeal.com/game/id:018d937f-5b12-7380-a77e-8754efb603d0/"
            }
        }
    ],
    "bundles": []
}
```

***GET:https://api.isthereanydeal.com/games/history/v2?key={itad_api_key}&id=018d937f-5b12-7380-a77e-8754efb603d0&country=lk&shops=61&since=***

```json
[
    {
        "timestamp": "2026-06-11T19:25:59+02:00",
        "shop": {
            "id": 61,
            "name": "Steam"
        },
        "deal": {
            "price": {
                "amount": 13.99,
                "amountInt": 1399,
                "currency": "USD"
            },
            "regular": {
                "amount": 39.99,
                "amountInt": 3999,
                "currency": "USD"
            },
            "cut": 65
        }
    },
    {
        "timestamp": "2026-05-07T19:20:18+02:00",
        "shop": {
            "id": 61,
            "name": "Steam"
        },
        "deal": {
            "price": {
                "amount": 39.99,
                "amountInt": 3999,
                "currency": "USD"
            },
            "regular": {
                "amount": 39.99,
                "amountInt": 3999,
                "currency": "USD"
            },
            "cut": 0
        }
    },
    {
        "timestamp": "2026-04-23T19:22:02+02:00",
        "shop": {
            "id": 61,
            "name": "Steam"
        },
        "deal": {
            "price": {
                "amount": 19.99,
                "amountInt": 1999,
                "currency": "USD"
            },
            "regular": {
                "amount": 39.99,
                "amountInt": 3999,
                "currency": "USD"
            },
            "cut": 50
        }
    },
    {
        "timestamp": "2026-03-26T18:48:15+01:00",
        "shop": {
            "id": 61,
            "name": "Steam"
        },
        "deal": {
            "price": {
                "amount": 39.99,
                "amountInt": 3999,
                "currency": "USD"
            },
            "regular": {
                "amount": 39.99,
                "amountInt": 3999,
                "currency": "USD"
            },
            "cut": 0
        }
    },
    {
        "timestamp": "2026-03-22T12:44:37+01:00",
        "shop": {
            "id": 61,
            "name": "Steam"
        },
        "deal": {
            "price": {
                "amount": 19.99,
                "amountInt": 1999,
                "currency": "USD"
            },
            "regular": {
                "amount": 39.99,
                "amountInt": 3999,
                "currency": "USD"
            },
            "cut": 50
        }
    }
]
```

***GET:https://api.isthereanydeal.com/games/info/v2?key={itad_api_key}&id=018d937f-5b12-7380-a77e-8754efb603d0***

```json
{
    "id": "018d937f-5b12-7380-a77e-8754efb603d0",
    "slug": "final-fantasy-vii-remake-intergrade",
    "title": "FINAL FANTASY VII REMAKE INTERGRADE",
    "type": "game",
    "mature": false,
    "assets": {
        "boxart": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/boxart.jpg?t=1773897908",
        "banner145": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/banner145.jpg?t=1773897908",
        "banner300": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/banner300.jpg?t=1773897908",
        "banner400": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/banner400.jpg?t=1773897908",
        "banner600": "https://assets.isthereanydeal.com/018d937f-5b12-7380-a77e-8754efb603d0/banner600.jpg?t=1773897908"
    },
    "earlyAccess": false,
    "achievements": true,
    "tradingCards": true,
    "appid": 1462040,
    "tags": [
        "RPG",
        "Action RPG",
        "JRPG",
        "Singleplayer",
        "Action-Adventure"
    ],
    "releaseDate": "2021-12-16",
    "developers": [
        {
            "id": 112312,
            "name": "Square Enix."
        },
        {
            "id": 101,
            "name": "Square Enix"
        }
    ],
    "publishers": [
        {
            "id": 101,
            "name": "Square Enix"
        }
    ],
    "reviews": [
        {
            "score": 88,
            "source": "Steam",
            "count": 38963,
            "url": "https://store.steampowered.com/app/1462040/"
        },
        {
            "score": 86,
            "source": "Metascore",
            "count": 23,
            "url": "https://metacritic.com/game/final-fantasy-vii-remake-intergrade/critic-reviews/?platform=pc"
        },
        {
            "score": 84,
            "source": "Metacritic User Score",
            "count": 2610,
            "url": "https://metacritic.com/game/final-fantasy-vii-remake-intergrade/user-reviews/?platform=pc"
        },
        {
            "score": 87,
            "source": "OpenCritic",
            "count": 164,
            "url": "https://opencritic.com/game/11370/final-fantasy-vii-remake-intergrade"
        }
    ],
    "stats": {
        "rank": 209,
        "waitlisted": 18739,
        "collected": 10903
    },
    "players": {
        "recent": 4791,
        "day": 7304,
        "week": 10243,
        "peak": 12844
    },
    "urls": {
        "game": "https://isthereanydeal.com/game/final-fantasy-vii-remake-intergrade/"
    }
}
```