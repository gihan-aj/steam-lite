### IsThereAnyDeal Price History Data

Following JSON structure represents a game's ITAD UUID and basic details.


***GET:https://api.isthereanydeal.com/games/lookup/v1?key={ItadApiKey}&appid=221040***

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

***POST:https://api.isthereanydeal.com/games/historylow/v1?key={ItadApiKey}&country=lk***

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

***https://api.isthereanydeal.com/games/overview/v2?key={ItadApiKey}&country=lk&shops=61***

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
        },
        {
            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
            "current": {
                "shop": {
                    "id": 61,
                    "name": "Steam"
                },
                "price": {
                    "amount": 19.99,
                    "amountInt": 1999,
                    "currency": "USD"
                },
                "regular": {
                    "amount": 19.99,
                    "amountInt": 1999,
                    "currency": "USD"
                },
                "cut": 0,
                "voucher": null,
                "flag": null,
                "drm": [],
                "platforms": [
                    {
                        "id": 1,
                        "name": "Windows"
                    }
                ],
                "timestamp": "2026-06-18T19:18:31+02:00",
                "expiry": null,
                "url": "https://itad.link/018d9386-b8db-71f7-ad8c-f142d86554e3/"
            },
            "lowest": {
                "shop": {
                    "id": 61,
                    "name": "Steam"
                },
                "price": {
                    "amount": 4.99,
                    "amountInt": 499,
                    "currency": "USD"
                },
                "regular": {
                    "amount": 19.99,
                    "amountInt": 1999,
                    "currency": "USD"
                },
                "cut": 75,
                "timestamp": "2017-08-17T19:45:28+02:00"
            },
            "bundled": 10,
            "urls": {
                "game": "https://isthereanydeal.com/game/id:018d937f-6b99-7077-9cf6-eacfce97587f/"
            }
        }
    ],
    "bundles": [
        {
            "id": 15594,
            "title": "Build Your Own Biohazard Bundle",
            "page": {
                "id": 1,
                "name": "Humble Bundle",
                "shopId": 37
            },
            "url": "https://humblebundleinc.sjv.io/c/2545989/2091208/25796?u=https%3A%2F%2Fwww.humblebundle.com%2Fstore%2Fpromo%2Fbuildyourownbiohazard%2F",
            "details": "https://isthereanydeal.com/bundles/15594/",
            "isMature": false,
            "publish": "2025-11-12T20:39:26+01:00",
            "expiry": "2025-11-17T03:00:00+01:00",
            "note": "NOTE: Percentage Discount is from Full Regular Retail Price, not any sale price.",
            "counts": {
                "games": 12,
                "media": 0
            },
            "tiers": [
                {
                    "price": null,
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f541-7208-b213-4b438af2fbb4",
                            "slug": "resident-evil-revelations",
                            "title": "Resident Evil: Revelations",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/boxart.jpg?t=1769127337",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner145.jpg?t=1769127337",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner300.jpg?t=1769127338",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner400.jpg?t=1769127338",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner600.jpg?t=1769127339"
                            }
                        },
                        {
                            "id": "018d937e-f85f-70cb-9323-8554024f67c2",
                            "slug": "resident-evil-0-hd-remaster",
                            "title": "Resident Evil 0 HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/boxart.jpg?t=1768966212",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner145.jpg?t=1768966212",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner300.jpg?t=1768966212",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner400.jpg?t=1768966212",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner600.jpg?t=1768966213"
                            }
                        },
                        {
                            "id": "018d937f-03fd-706b-a71d-252267f114c4",
                            "slug": "resident-evil-6-complete",
                            "title": "Resident Evil 6 Complete",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/boxart.jpg?t=1756056020",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner145.jpg?t=1756056021",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner300.jpg?t=1756056021",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner400.jpg?t=1756056021",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner600.jpg?t=1756056022"
                            }
                        },
                        {
                            "id": "018d937f-1732-71eb-b10f-322e4ec880dc",
                            "slug": "resident-evil-7-biohazard-gold-edition",
                            "title": "RESIDENT EVIL 7 biohazard Gold Edition",
                            "type": "package",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1732-71eb-b10f-322e4ec880dc/boxart.jpg?t=1756569322",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1732-71eb-b10f-322e4ec880dc/banner145.jpg?t=1756569322",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1732-71eb-b10f-322e4ec880dc/banner300.jpg?t=1756569322",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1732-71eb-b10f-322e4ec880dc/banner400.jpg?t=1756569322",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1732-71eb-b10f-322e4ec880dc/banner600.jpg?t=1756569322"
                            }
                        },
                        {
                            "id": "018d937f-1e98-7204-9325-3e3295ce44c9",
                            "slug": "resident-evil-hd-remaster",
                            "title": "Resident Evil HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/boxart.jpg?t=1768959004",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner145.jpg?t=1768959004",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner300.jpg?t=1768959004",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner400.jpg?t=1768959004",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner600.jpg?t=1768959005"
                            }
                        },
                        {
                            "id": "018d937f-261c-7359-8cb1-16c7dd4696e9",
                            "slug": "resident-evil-revelations-2-deluxe-edition",
                            "title": "Resident Evil: Revelations 2 Deluxe Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/boxart.jpg?t=1760735404",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner145.jpg?t=1760735405",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner300.jpg?t=1760735405",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner400.jpg?t=1760735405",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner600.jpg?t=1760735405"
                            }
                        },
                        {
                            "id": "018d937f-3610-735b-93db-f948f52410f7",
                            "slug": "resident-evil-5-gold-edition",
                            "title": "Resident Evil 5 GOLD Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/boxart.jpg?t=1756056005",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner145.jpg?t=1756056006",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner300.jpg?t=1756056006",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner400.jpg?t=1756056006",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner600.jpg?t=1756056006"
                            }
                        },
                        {
                            "id": "018d937f-437a-728f-aa59-d868be98fb65",
                            "slug": "resident-evil-village",
                            "title": "Resident Evil Village",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-437a-728f-aa59-d868be98fb65/boxart.jpg?t=1776927901",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-437a-728f-aa59-d868be98fb65/banner145.jpg?t=1776927902",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-437a-728f-aa59-d868be98fb65/banner300.jpg?t=1776927902",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-437a-728f-aa59-d868be98fb65/banner400.jpg?t=1776927902",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-437a-728f-aa59-d868be98fb65/banner600.jpg?t=1776927902"
                            }
                        },
                        {
                            "id": "018d937f-633c-7166-b3f1-2d9b2bd316a6",
                            "slug": "resident-evil-village-winters-expansion",
                            "title": "Resident Evil Village - Winters' Expansion",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-633c-7166-b3f1-2d9b2bd316a6/boxart.jpg?t=1776928527",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-633c-7166-b3f1-2d9b2bd316a6/banner145.jpg?t=1776928527",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-633c-7166-b3f1-2d9b2bd316a6/banner300.jpg?t=1776928527",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-633c-7166-b3f1-2d9b2bd316a6/banner400.jpg?t=1776928527",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-633c-7166-b3f1-2d9b2bd316a6/banner600.jpg?t=1776928528"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        },
                        {
                            "id": "018d937f-781e-716d-acee-7650541ad533",
                            "slug": "resident-evil-2-2019",
                            "title": "RESIDENT EVIL 2 (2019)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/boxart.jpg?t=1779433512",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner145.jpg?t=1779433512",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner300.jpg?t=1779433512",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner400.jpg?t=1779433512",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner600.jpg?t=1779433512"
                            }
                        },
                        {
                            "id": "018d937f-7829-7173-8d70-703fec94e4d6",
                            "slug": "resident-evil-3-2020",
                            "title": "RESIDENT EVIL 3 (2020)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/boxart.jpg?t=1780363208",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner145.jpg?t=1780363208",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner300.jpg?t=1780363208",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner400.jpg?t=1780363208",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner600.jpg?t=1780363208"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 15131,
            "title": "Build Your Own Capcom Legends Bundle",
            "page": {
                "id": 8,
                "name": "Fanatical",
                "shopId": 6
            },
            "url": "https://www.awin1.com/cread.php?awinmid=118821&awinaffid=235265&ued=https%3A%2F%2Fwww.fanatical.com%2Fen%2Fpick-and-mix%2Fbuild-your-own-capcom-legends-bundle",
            "details": "https://isthereanydeal.com/bundles/15131/",
            "isMature": false,
            "publish": "2025-05-22T19:38:54+02:00",
            "expiry": "2025-06-05T00:56:52+02:00",
            "note": "* (USD) 3+ items = 3.33 each; 5+ items = 3.00 each; 7+ items = 2.85 each.",
            "counts": {
                "games": 20,
                "media": 1
            },
            "tiers": [
                {
                    "price": {
                        "amount": 9.99,
                        "amountInt": 999,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-ea44-71c8-bd9f-9842276df52f",
                            "slug": "mega-man-legacy-collection",
                            "title": "Mega Man Legacy Collection",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/boxart.jpg?t=1769139914",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/banner145.jpg?t=1769139914",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/banner300.jpg?t=1769139914",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/banner400.jpg?t=1769139914",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/banner600.jpg?t=1769139915"
                            }
                        },
                        {
                            "id": "018d937e-effa-732c-848f-5948d6106f81",
                            "slug": "ultra-street-fighter-iv",
                            "title": "Ultra Street Fighter IV",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/boxart.jpg?t=1763093092",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner145.jpg?t=1763093092",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner300.jpg?t=1763093092",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner400.jpg?t=1763093092",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner600.jpg?t=1763093092"
                            }
                        },
                        {
                            "id": "018d937e-f03e-7108-a2cc-8aeb260783ad",
                            "slug": "dead-rising-2",
                            "title": "Dead Rising® 2",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/boxart.jpg?t=1768869302",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner145.jpg?t=1768869302",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner300.jpg?t=1768869302",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner400.jpg?t=1768869303",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner600.jpg?t=1768869304"
                            }
                        },
                        {
                            "id": "018d937e-f4da-726b-aff7-4fde33dab7d0",
                            "slug": "dead-rising-2-off-the-record",
                            "title": "Dead Rising® 2: Off The Record",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/boxart.jpg?t=1768876518",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner145.jpg?t=1768876518",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner300.jpg?t=1768876518",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner400.jpg?t=1768876518",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner600.jpg?t=1768876519"
                            }
                        },
                        {
                            "id": "018d937e-f541-7208-b213-4b438af2fbb4",
                            "slug": "resident-evil-revelations",
                            "title": "Resident Evil: Revelations",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/boxart.jpg?t=1769127337",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner145.jpg?t=1769127337",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner300.jpg?t=1769127338",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner400.jpg?t=1769127338",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner600.jpg?t=1769127339"
                            }
                        },
                        {
                            "id": "018d937e-f85f-70cb-9323-8554024f67c2",
                            "slug": "resident-evil-0-hd-remaster",
                            "title": "Resident Evil 0 HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/boxart.jpg?t=1768966212",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner145.jpg?t=1768966212",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner300.jpg?t=1768966212",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner400.jpg?t=1768966212",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner600.jpg?t=1768966213"
                            }
                        },
                        {
                            "id": "018d937e-ff54-72a0-a5d7-ed10118f50d8",
                            "slug": "remember-me",
                            "title": "Remember Me",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/boxart.jpg?t=1779340806",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner145.jpg?t=1779340806",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner300.jpg?t=1779340806",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner400.jpg?t=1779340806",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner600.jpg?t=1779340807"
                            }
                        },
                        {
                            "id": "018d937f-03fd-706b-a71d-252267f114c4",
                            "slug": "resident-evil-6-complete",
                            "title": "Resident Evil 6 Complete",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/boxart.jpg?t=1756056020",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner145.jpg?t=1756056021",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner300.jpg?t=1756056021",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner400.jpg?t=1756056021",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner600.jpg?t=1756056022"
                            }
                        },
                        {
                            "id": "018d937f-0c4d-71ff-8adc-6a1bf47c362b",
                            "slug": "dmc-devil-may-cry",
                            "title": "DMC - Devil May Cry",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/boxart.jpg?t=1768871716",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner145.jpg?t=1768871717",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner300.jpg?t=1768871717",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner400.jpg?t=1768871717",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner600.jpg?t=1768871717"
                            }
                        },
                        {
                            "id": "018d937f-160c-7273-9785-a320b4eb9af5",
                            "slug": "devil-may-cry-4-special-edition",
                            "title": "Devil May Cry® 4 Special Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/boxart.jpg?t=1768872007",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner145.jpg?t=1768872008",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner300.jpg?t=1768872008",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner400.jpg?t=1768872008",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner600.jpg?t=1768872009"
                            }
                        },
                        {
                            "id": "018d937f-1e98-7204-9325-3e3295ce44c9",
                            "slug": "resident-evil-hd-remaster",
                            "title": "Resident Evil HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/boxart.jpg?t=1768959004",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner145.jpg?t=1768959004",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner300.jpg?t=1768959004",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner400.jpg?t=1768959004",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner600.jpg?t=1768959005"
                            }
                        },
                        {
                            "id": "018d937f-1eed-7294-8720-f38b40ba52d9",
                            "slug": "lost-planet-3",
                            "title": "Lost Planet 3",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/boxart.jpg?t=1763217739",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner145.jpg?t=1763217739",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner300.jpg?t=1763217739",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner400.jpg?t=1763217740",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner600.jpg?t=1763217740"
                            }
                        },
                        {
                            "id": "018d937f-20bf-72c1-a901-d69ffb82750e",
                            "slug": "mega-man-legacy-collection-2",
                            "title": "Mega Man Legacy Collection 2",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-20bf-72c1-a901-d69ffb82750e/boxart.jpg?t=1769139332",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-20bf-72c1-a901-d69ffb82750e/banner145.jpg?t=1769139332",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-20bf-72c1-a901-d69ffb82750e/banner300.jpg?t=1769139332",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-20bf-72c1-a901-d69ffb82750e/banner400.jpg?t=1769139332",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-20bf-72c1-a901-d69ffb82750e/banner600.jpg?t=1769139334"
                            }
                        },
                        {
                            "id": "018d937f-221f-70de-8737-c0c0ad381541",
                            "slug": "dead-rising-3-apocalypse-edition",
                            "title": "Dead Rising 3 Apocalypse Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/boxart.jpg?t=1768870815",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner145.jpg?t=1768870815",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner300.jpg?t=1768870816",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner400.jpg?t=1768870816",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner600.jpg?t=1768870817"
                            }
                        },
                        {
                            "id": "018d937f-2459-7030-a3cd-77a8f13b87e9",
                            "slug": "dragons-dogma-dark-arisen",
                            "title": "Dragon's Dogma: Dark Arisen",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/boxart.jpg?t=1768877406",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner145.jpg?t=1768877406",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner300.jpg?t=1768877406",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner400.jpg?t=1768877406",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner600.jpg?t=1768877407"
                            }
                        },
                        {
                            "id": "018d937f-3095-71c8-8f83-52e4870a240e",
                            "slug": "dead-rising-4",
                            "title": "Dead Rising 4",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3095-71c8-8f83-52e4870a240e/boxart.jpg?t=1768873207",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3095-71c8-8f83-52e4870a240e/banner145.jpg?t=1768873207",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3095-71c8-8f83-52e4870a240e/banner300.jpg?t=1768873207",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3095-71c8-8f83-52e4870a240e/banner400.jpg?t=1768873207",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3095-71c8-8f83-52e4870a240e/banner600.jpg?t=1768873208"
                            }
                        },
                        {
                            "id": "018d937f-3445-7218-beb9-7dea0bb65499",
                            "slug": "resident-evil-revelations-2-episode-one-penal-colony",
                            "title": "Resident Evil: Revelations 2 - Episode One: Penal Colony",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/boxart.jpg?t=1769129116",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner145.jpg?t=1769129116",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner300.jpg?t=1769129116",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner400.jpg?t=1769129116",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner600.jpg?t=1769129117"
                            }
                        },
                        {
                            "id": "018d937f-3610-735b-93db-f948f52410f7",
                            "slug": "resident-evil-5-gold-edition",
                            "title": "Resident Evil 5 GOLD Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/boxart.jpg?t=1756056005",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner145.jpg?t=1756056006",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner300.jpg?t=1756056006",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner400.jpg?t=1756056006",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner600.jpg?t=1756056006"
                            }
                        },
                        {
                            "id": "018d937f-556c-700b-adaa-249a833f300c",
                            "slug": "devil-may-cry-5-and-vergil",
                            "title": "Devil May Cry 5 + Vergil",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-556c-700b-adaa-249a833f300c/boxart.jpg?t=1768971915",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-556c-700b-adaa-249a833f300c/banner145.jpg?t=1768971915",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-556c-700b-adaa-249a833f300c/banner300.jpg?t=1768971915",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-556c-700b-adaa-249a833f300c/banner400.jpg?t=1768971915",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-556c-700b-adaa-249a833f300c/banner600.jpg?t=1768971915"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        }
                    ]
                },
                {
                    "price": null,
                    "addon": false,
                    "games": []
                }
            ]
        },
        {
            "id": 14338,
            "title": "Resident Evil Decades of Horror Village Gold",
            "page": {
                "id": 1,
                "name": "Humble Bundle",
                "shopId": 37
            },
            "url": "https://humblebundleinc.sjv.io/c/2545989/2091208/25796?u=https%3A%2F%2Fwww.humblebundle.com%2Fgames%2Fresident-evil-decades-horror-village-gold",
            "details": "https://isthereanydeal.com/bundles/14338/",
            "isMature": false,
            "publish": "2024-08-21T20:26:11+02:00",
            "expiry": "2024-09-12T02:59:20+02:00",
            "note": "This bundle supports Children's Miracle Network Hospitals and Breakthrough T1D.",
            "counts": {
                "games": 12,
                "media": 0
            },
            "tiers": [
                {
                    "price": {
                        "amount": 3,
                        "amountInt": 300,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f541-7208-b213-4b438af2fbb4",
                            "slug": "resident-evil-revelations",
                            "title": "Resident Evil: Revelations",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/boxart.jpg?t=1769127337",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner145.jpg?t=1769127337",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner300.jpg?t=1769127338",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner400.jpg?t=1769127338",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner600.jpg?t=1769127339"
                            }
                        },
                        {
                            "id": "018d937f-1e98-7204-9325-3e3295ce44c9",
                            "slug": "resident-evil-hd-remaster",
                            "title": "Resident Evil HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/boxart.jpg?t=1768959004",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner145.jpg?t=1768959004",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner300.jpg?t=1768959004",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner400.jpg?t=1768959004",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner600.jpg?t=1768959005"
                            }
                        },
                        {
                            "id": "018d937f-3445-7218-beb9-7dea0bb65499",
                            "slug": "resident-evil-revelations-2-episode-one-penal-colony",
                            "title": "Resident Evil: Revelations 2 - Episode One: Penal Colony",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/boxart.jpg?t=1769129116",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner145.jpg?t=1769129116",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner300.jpg?t=1769129116",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner400.jpg?t=1769129116",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner600.jpg?t=1769129117"
                            }
                        }
                    ]
                },
                {
                    "price": {
                        "amount": 10,
                        "amountInt": 1000,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f85f-70cb-9323-8554024f67c2",
                            "slug": "resident-evil-0-hd-remaster",
                            "title": "Resident Evil 0 HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/boxart.jpg?t=1768966212",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner145.jpg?t=1768966212",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner300.jpg?t=1768966212",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner400.jpg?t=1768966212",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner600.jpg?t=1768966213"
                            }
                        },
                        {
                            "id": "018d937f-03fd-706b-a71d-252267f114c4",
                            "slug": "resident-evil-6-complete",
                            "title": "Resident Evil 6 Complete",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/boxart.jpg?t=1756056020",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner145.jpg?t=1756056021",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner300.jpg?t=1756056021",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner400.jpg?t=1756056021",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner600.jpg?t=1756056022"
                            }
                        },
                        {
                            "id": "018d937f-261c-7359-8cb1-16c7dd4696e9",
                            "slug": "resident-evil-revelations-2-deluxe-edition",
                            "title": "Resident Evil: Revelations 2 Deluxe Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/boxart.jpg?t=1760735404",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner145.jpg?t=1760735405",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner300.jpg?t=1760735405",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner400.jpg?t=1760735405",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner600.jpg?t=1760735405"
                            }
                        },
                        {
                            "id": "018d937f-3610-735b-93db-f948f52410f7",
                            "slug": "resident-evil-5-gold-edition",
                            "title": "Resident Evil 5 GOLD Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/boxart.jpg?t=1756056005",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner145.jpg?t=1756056006",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner300.jpg?t=1756056006",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner400.jpg?t=1756056006",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner600.jpg?t=1756056006"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        }
                    ]
                },
                {
                    "price": {
                        "amount": 20,
                        "amountInt": 2000,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-fa07-73ce-a8ee-b9fa1e674392",
                            "slug": "resident-evil-7-biohazard",
                            "title": "RESIDENT EVIL 7 biohazard",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/boxart.jpg?t=1778466015",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner145.jpg?t=1778466015",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner300.jpg?t=1778466015",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner400.jpg?t=1778466015",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner600.jpg?t=1778466015"
                            }
                        },
                        {
                            "id": "018d937f-781e-716d-acee-7650541ad533",
                            "slug": "resident-evil-2-2019",
                            "title": "RESIDENT EVIL 2 (2019)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/boxart.jpg?t=1779433512",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner145.jpg?t=1779433512",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner300.jpg?t=1779433512",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner400.jpg?t=1779433512",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner600.jpg?t=1779433512"
                            }
                        },
                        {
                            "id": "018d937f-7829-7173-8d70-703fec94e4d6",
                            "slug": "resident-evil-3-2020",
                            "title": "RESIDENT EVIL 3 (2020)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/boxart.jpg?t=1780363208",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner145.jpg?t=1780363208",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner300.jpg?t=1780363208",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner400.jpg?t=1780363208",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner600.jpg?t=1780363208"
                            }
                        }
                    ]
                },
                {
                    "price": {
                        "amount": 30,
                        "amountInt": 3000,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937f-6660-72c2-8fd9-e24e950cf40c",
                            "slug": "resident-evil-village-gold-edition",
                            "title": "Resident Evil Village Gold Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6660-72c2-8fd9-e24e950cf40c/boxart.jpg?t=1756579821",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6660-72c2-8fd9-e24e950cf40c/banner145.jpg?t=1756579821",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6660-72c2-8fd9-e24e950cf40c/banner300.jpg?t=1756579821",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6660-72c2-8fd9-e24e950cf40c/banner400.jpg?t=1756579821",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6660-72c2-8fd9-e24e950cf40c/banner600.jpg?t=1756579821"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 14284,
            "title": "Build Your Own Resident Evil Bundle",
            "page": {
                "id": 8,
                "name": "Fanatical",
                "shopId": 6
            },
            "url": "https://www.awin1.com/cread.php?awinmid=118821&awinaffid=235265&ued=https%3A%2F%2Fwww.fanatical.com%2Fen%2Fpick-and-mix%2Fbuild-your-own-resident-evil-bundle",
            "details": "https://isthereanydeal.com/bundles/14284/",
            "isMature": false,
            "publish": "2024-07-30T19:34:29+02:00",
            "expiry": "2024-08-14T08:59:09+02:00",
            "note": "(USD) - 3+ items = $3.33 each; 5+ items = $3.00 each; 8+ items = $2.90 each.",
            "counts": {
                "games": 8,
                "media": 1
            },
            "tiers": [
                {
                    "price": {
                        "amount": 9.99,
                        "amountInt": 999,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f541-7208-b213-4b438af2fbb4",
                            "slug": "resident-evil-revelations",
                            "title": "Resident Evil: Revelations",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/boxart.jpg?t=1769127337",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner145.jpg?t=1769127337",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner300.jpg?t=1769127338",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner400.jpg?t=1769127338",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner600.jpg?t=1769127339"
                            }
                        },
                        {
                            "id": "018d937e-f85f-70cb-9323-8554024f67c2",
                            "slug": "resident-evil-0-hd-remaster",
                            "title": "Resident Evil 0 HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/boxart.jpg?t=1768966212",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner145.jpg?t=1768966212",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner300.jpg?t=1768966212",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner400.jpg?t=1768966212",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner600.jpg?t=1768966213"
                            }
                        },
                        {
                            "id": "018d937e-fa07-73ce-a8ee-b9fa1e674392",
                            "slug": "resident-evil-7-biohazard",
                            "title": "RESIDENT EVIL 7 biohazard",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/boxart.jpg?t=1778466015",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner145.jpg?t=1778466015",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner300.jpg?t=1778466015",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner400.jpg?t=1778466015",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner600.jpg?t=1778466015"
                            }
                        },
                        {
                            "id": "018d937f-03fd-706b-a71d-252267f114c4",
                            "slug": "resident-evil-6-complete",
                            "title": "Resident Evil 6 Complete",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/boxart.jpg?t=1756056020",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner145.jpg?t=1756056021",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner300.jpg?t=1756056021",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner400.jpg?t=1756056021",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-03fd-706b-a71d-252267f114c4/banner600.jpg?t=1756056022"
                            }
                        },
                        {
                            "id": "018d937f-1e98-7204-9325-3e3295ce44c9",
                            "slug": "resident-evil-hd-remaster",
                            "title": "Resident Evil HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/boxart.jpg?t=1768959004",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner145.jpg?t=1768959004",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner300.jpg?t=1768959004",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner400.jpg?t=1768959004",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner600.jpg?t=1768959005"
                            }
                        },
                        {
                            "id": "018d937f-261c-7359-8cb1-16c7dd4696e9",
                            "slug": "resident-evil-revelations-2-deluxe-edition",
                            "title": "Resident Evil: Revelations 2 Deluxe Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/boxart.jpg?t=1760735404",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner145.jpg?t=1760735405",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner300.jpg?t=1760735405",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner400.jpg?t=1760735405",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner600.jpg?t=1760735405"
                            }
                        },
                        {
                            "id": "018d937f-3610-735b-93db-f948f52410f7",
                            "slug": "resident-evil-5-gold-edition",
                            "title": "Resident Evil 5 GOLD Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/boxart.jpg?t=1756056005",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner145.jpg?t=1756056006",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner300.jpg?t=1756056006",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner400.jpg?t=1756056006",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner600.jpg?t=1756056006"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 13798,
            "title": "Capcom Collection Build Your Own Bundle",
            "page": {
                "id": 8,
                "name": "Fanatical",
                "shopId": 6
            },
            "url": "https://www.awin1.com/cread.php?awinmid=118821&awinaffid=235265&ued=https%3A%2F%2Fwww.fanatical.com%2Fen%2Fpick-and-mix%2Fcapcom-collection-build-your-own-bundle",
            "details": "https://isthereanydeal.com/bundles/13798/",
            "isMature": false,
            "publish": "2024-01-25T19:19:54+01:00",
            "expiry": "2024-02-26T09:00:12+01:00",
            "note": null,
            "counts": {
                "games": 13,
                "media": 1
            },
            "tiers": [
                {
                    "price": {
                        "amount": 9.99,
                        "amountInt": 999,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-ea44-71c8-bd9f-9842276df52f",
                            "slug": "mega-man-legacy-collection",
                            "title": "Mega Man Legacy Collection",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/boxart.jpg?t=1769139914",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/banner145.jpg?t=1769139914",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/banner300.jpg?t=1769139914",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/banner400.jpg?t=1769139914",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-ea44-71c8-bd9f-9842276df52f/banner600.jpg?t=1769139915"
                            }
                        },
                        {
                            "id": "018d937e-effa-732c-848f-5948d6106f81",
                            "slug": "ultra-street-fighter-iv",
                            "title": "Ultra Street Fighter IV",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/boxart.jpg?t=1763093092",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner145.jpg?t=1763093092",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner300.jpg?t=1763093092",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner400.jpg?t=1763093092",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner600.jpg?t=1763093092"
                            }
                        },
                        {
                            "id": "018d937e-f488-72b9-88ec-a6021e428927",
                            "slug": "street-fighter-v",
                            "title": "Street Fighter V",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f488-72b9-88ec-a6021e428927/boxart.jpg?t=1763233933",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f488-72b9-88ec-a6021e428927/banner145.jpg?t=1763233933",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f488-72b9-88ec-a6021e428927/banner300.jpg?t=1763233933",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f488-72b9-88ec-a6021e428927/banner400.jpg?t=1763233933",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f488-72b9-88ec-a6021e428927/banner600.jpg?t=1763233933"
                            }
                        },
                        {
                            "id": "018d937e-f559-73df-9406-3ffe1f88671a",
                            "slug": "bionic-commando-rearmed",
                            "title": "Bionic Commando: Rearmed",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/boxart.jpg?t=1763099694",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner145.jpg?t=1763099694",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner300.jpg?t=1763099694",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner400.jpg?t=1763099694",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner600.jpg?t=1763099694"
                            }
                        },
                        {
                            "id": "018d937e-f85f-70cb-9323-8554024f67c2",
                            "slug": "resident-evil-0-hd-remaster",
                            "title": "Resident Evil 0 HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/boxart.jpg?t=1768966212",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner145.jpg?t=1768966212",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner300.jpg?t=1768966212",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner400.jpg?t=1768966212",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner600.jpg?t=1768966213"
                            }
                        },
                        {
                            "id": "018d937e-fd75-7351-be7d-a0be822b98a1",
                            "slug": "resident-evil-6",
                            "title": "Resident Evil 6",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/boxart.jpg?t=1768963508",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner145.jpg?t=1768963508",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner300.jpg?t=1768963508",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner400.jpg?t=1768963508",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner600.jpg?t=1768963508"
                            }
                        },
                        {
                            "id": "018d937e-feb8-71a3-8819-21b3f9d400b8",
                            "slug": "resident-evil-5",
                            "title": "Resident Evil 5",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/boxart.jpg?t=1768972513",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner145.jpg?t=1768972513",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner300.jpg?t=1768972513",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner400.jpg?t=1768972513",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner600.jpg?t=1768972515"
                            }
                        },
                        {
                            "id": "018d937f-0faa-71eb-85e0-cb8fc653b6a7",
                            "slug": "strider-sutoraida-fei-long",
                            "title": "STRIDER™ / ストライダー飛竜®",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/boxart.jpg?t=1778638812",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner145.jpg?t=1778638813",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner300.jpg?t=1778638813",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner400.jpg?t=1778638813",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner600.jpg?t=1778638814"
                            }
                        },
                        {
                            "id": "018d937f-13dd-7189-96cb-554d2d1f8f92",
                            "slug": "dungeons-and-dragons-chronicles-of-mystara",
                            "title": "Dungeons & Dragons: Chronicles of Mystara",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-13dd-7189-96cb-554d2d1f8f92/boxart.jpg?t=1763093620",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-13dd-7189-96cb-554d2d1f8f92/banner145.jpg?t=1763093620",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-13dd-7189-96cb-554d2d1f8f92/banner300.jpg?t=1763093621",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-13dd-7189-96cb-554d2d1f8f92/banner400.jpg?t=1763093621",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-13dd-7189-96cb-554d2d1f8f92/banner600.jpg?t=1763093621"
                            }
                        },
                        {
                            "id": "018d937f-1e98-7204-9325-3e3295ce44c9",
                            "slug": "resident-evil-hd-remaster",
                            "title": "Resident Evil HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/boxart.jpg?t=1768959004",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner145.jpg?t=1768959004",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner300.jpg?t=1768959004",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner400.jpg?t=1768959004",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner600.jpg?t=1768959005"
                            }
                        },
                        {
                            "id": "018d937f-1eed-7294-8720-f38b40ba52d9",
                            "slug": "lost-planet-3",
                            "title": "Lost Planet 3",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/boxart.jpg?t=1763217739",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner145.jpg?t=1763217739",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner300.jpg?t=1763217739",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner400.jpg?t=1763217740",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner600.jpg?t=1763217740"
                            }
                        },
                        {
                            "id": "018d937f-2459-7030-a3cd-77a8f13b87e9",
                            "slug": "dragons-dogma-dark-arisen",
                            "title": "Dragon's Dogma: Dark Arisen",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/boxart.jpg?t=1768877406",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner145.jpg?t=1768877406",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner300.jpg?t=1768877406",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner400.jpg?t=1768877406",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner600.jpg?t=1768877407"
                            }
                        },
                        {
                            "id": "018d937f-2efc-72b2-8ae1-5f5423ab9ca2",
                            "slug": "bionic-commando",
                            "title": "Bionic Commando",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/boxart.jpg?t=1763150455",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner145.jpg?t=1763150455",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner300.jpg?t=1763150455",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner400.jpg?t=1763150455",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner600.jpg?t=1763150455"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 11939,
            "title": "Resident Evil: Decades of Horror Bundle (2022)",
            "page": {
                "id": 1,
                "name": "Humble Bundle",
                "shopId": 37
            },
            "url": "https://humblebundleinc.sjv.io/c/2545989/2091208/25796?u=https%3A%2F%2Fwww.humblebundle.com%2Fgames%2Fresident-evil-decades-horror",
            "details": "https://isthereanydeal.com/bundles/11939/",
            "isMature": false,
            "publish": "2022-08-10T21:44:12+02:00",
            "expiry": "2022-08-25T02:55:00+02:00",
            "note": "* NOTE: Tier 2 and/or Tier 3 buyers DO NOT receive a separate key for RER2/BHR2 Episode One: Penal Colony because it is considered as part of Resident Evil: Revelations 2 Deluxe Edition.\r\nOnly Tier 1 purchases receive RER2/BHR2 Episode One: Penal Colony.",
            "counts": {
                "games": 11,
                "media": 1
            },
            "tiers": [
                {
                    "price": {
                        "amount": 1,
                        "amountInt": 100,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f541-7208-b213-4b438af2fbb4",
                            "slug": "resident-evil-revelations",
                            "title": "Resident Evil: Revelations",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/boxart.jpg?t=1769127337",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner145.jpg?t=1769127337",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner300.jpg?t=1769127338",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner400.jpg?t=1769127338",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner600.jpg?t=1769127339"
                            }
                        },
                        {
                            "id": "018d937f-1e98-7204-9325-3e3295ce44c9",
                            "slug": "resident-evil-hd-remaster",
                            "title": "Resident Evil HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/boxart.jpg?t=1768959004",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner145.jpg?t=1768959004",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner300.jpg?t=1768959004",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner400.jpg?t=1768959004",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner600.jpg?t=1768959005"
                            }
                        },
                        {
                            "id": "018d937f-3445-7218-beb9-7dea0bb65499",
                            "slug": "resident-evil-revelations-2-episode-one-penal-colony",
                            "title": "Resident Evil: Revelations 2 - Episode One: Penal Colony",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/boxart.jpg?t=1769129116",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner145.jpg?t=1769129116",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner300.jpg?t=1769129116",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner400.jpg?t=1769129116",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner600.jpg?t=1769129117"
                            }
                        }
                    ]
                },
                {
                    "price": {
                        "amount": 10,
                        "amountInt": 1000,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f85f-70cb-9323-8554024f67c2",
                            "slug": "resident-evil-0-hd-remaster",
                            "title": "Resident Evil 0 HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/boxart.jpg?t=1768966212",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner145.jpg?t=1768966212",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner300.jpg?t=1768966212",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner400.jpg?t=1768966212",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner600.jpg?t=1768966213"
                            }
                        },
                        {
                            "id": "018d937e-fd75-7351-be7d-a0be822b98a1",
                            "slug": "resident-evil-6",
                            "title": "Resident Evil 6",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/boxart.jpg?t=1768963508",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner145.jpg?t=1768963508",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner300.jpg?t=1768963508",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner400.jpg?t=1768963508",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner600.jpg?t=1768963508"
                            }
                        },
                        {
                            "id": "018d937f-261c-7359-8cb1-16c7dd4696e9",
                            "slug": "resident-evil-revelations-2-deluxe-edition",
                            "title": "Resident Evil: Revelations 2 Deluxe Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/boxart.jpg?t=1760735404",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner145.jpg?t=1760735405",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner300.jpg?t=1760735405",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner400.jpg?t=1760735405",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-261c-7359-8cb1-16c7dd4696e9/banner600.jpg?t=1760735405"
                            }
                        },
                        {
                            "id": "018d937f-3610-735b-93db-f948f52410f7",
                            "slug": "resident-evil-5-gold-edition",
                            "title": "Resident Evil 5 GOLD Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/boxart.jpg?t=1756056005",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner145.jpg?t=1756056006",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner300.jpg?t=1756056006",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner400.jpg?t=1756056006",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner600.jpg?t=1756056006"
                            }
                        }
                    ]
                },
                {
                    "price": {
                        "amount": 30,
                        "amountInt": 3000,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-fa07-73ce-a8ee-b9fa1e674392",
                            "slug": "resident-evil-7-biohazard",
                            "title": "RESIDENT EVIL 7 biohazard",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/boxart.jpg?t=1778466015",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner145.jpg?t=1778466015",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner300.jpg?t=1778466015",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner400.jpg?t=1778466015",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fa07-73ce-a8ee-b9fa1e674392/banner600.jpg?t=1778466015"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        },
                        {
                            "id": "018d937f-781e-716d-acee-7650541ad533",
                            "slug": "resident-evil-2-2019",
                            "title": "RESIDENT EVIL 2 (2019)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/boxart.jpg?t=1779433512",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner145.jpg?t=1779433512",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner300.jpg?t=1779433512",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner400.jpg?t=1779433512",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-781e-716d-acee-7650541ad533/banner600.jpg?t=1779433512"
                            }
                        },
                        {
                            "id": "018d937f-7829-7173-8d70-703fec94e4d6",
                            "slug": "resident-evil-3-2020",
                            "title": "RESIDENT EVIL 3 (2020)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/boxart.jpg?t=1780363208",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner145.jpg?t=1780363208",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner300.jpg?t=1780363208",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner400.jpg?t=1780363208",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner600.jpg?t=1780363208"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 11133,
            "title": "Capcom Build Your Own Bundle",
            "page": {
                "id": 1,
                "name": "Humble Bundle",
                "shopId": 37
            },
            "url": "https://humblebundleinc.sjv.io/c/2545989/2091208/25796?u=https%3A%2F%2Fwww.humblebundle.com%2Fstore%2Fpromo%2Fcapcom-byob",
            "details": "https://isthereanydeal.com/bundles/11133/",
            "isMature": false,
            "publish": "2022-03-07T22:11:48+01:00",
            "expiry": "2022-03-19T18:00:00+01:00",
            "note": "* NOTE: Discounts applies only to FULL RETAIL PRICE of selected titles.",
            "counts": {
                "games": 27,
                "media": 0
            },
            "tiers": [
                {
                    "price": null,
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-effa-732c-848f-5948d6106f81",
                            "slug": "ultra-street-fighter-iv",
                            "title": "Ultra Street Fighter IV",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/boxart.jpg?t=1763093092",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner145.jpg?t=1763093092",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner300.jpg?t=1763093092",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner400.jpg?t=1763093092",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner600.jpg?t=1763093092"
                            }
                        },
                        {
                            "id": "018d937e-f03e-7108-a2cc-8aeb260783ad",
                            "slug": "dead-rising-2",
                            "title": "Dead Rising® 2",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/boxart.jpg?t=1768869302",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner145.jpg?t=1768869302",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner300.jpg?t=1768869302",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner400.jpg?t=1768869303",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner600.jpg?t=1768869304"
                            }
                        },
                        {
                            "id": "018d937e-f4da-726b-aff7-4fde33dab7d0",
                            "slug": "dead-rising-2-off-the-record",
                            "title": "Dead Rising® 2: Off The Record",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/boxart.jpg?t=1768876518",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner145.jpg?t=1768876518",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner300.jpg?t=1768876518",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner400.jpg?t=1768876518",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner600.jpg?t=1768876519"
                            }
                        },
                        {
                            "id": "018d937e-f559-73df-9406-3ffe1f88671a",
                            "slug": "bionic-commando-rearmed",
                            "title": "Bionic Commando: Rearmed",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/boxart.jpg?t=1763099694",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner145.jpg?t=1763099694",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner300.jpg?t=1763099694",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner400.jpg?t=1763099694",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner600.jpg?t=1763099694"
                            }
                        },
                        {
                            "id": "018d937e-f85f-70cb-9323-8554024f67c2",
                            "slug": "resident-evil-0-hd-remaster",
                            "title": "Resident Evil 0 HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/boxart.jpg?t=1768966212",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner145.jpg?t=1768966212",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner300.jpg?t=1768966212",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner400.jpg?t=1768966212",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner600.jpg?t=1768966213"
                            }
                        },
                        {
                            "id": "018d937e-fd75-7351-be7d-a0be822b98a1",
                            "slug": "resident-evil-6",
                            "title": "Resident Evil 6",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/boxart.jpg?t=1768963508",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner145.jpg?t=1768963508",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner300.jpg?t=1768963508",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner400.jpg?t=1768963508",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner600.jpg?t=1768963508"
                            }
                        },
                        {
                            "id": "018d937e-feb8-71a3-8819-21b3f9d400b8",
                            "slug": "resident-evil-5",
                            "title": "Resident Evil 5",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/boxart.jpg?t=1768972513",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner145.jpg?t=1768972513",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner300.jpg?t=1768972513",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner400.jpg?t=1768972513",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner600.jpg?t=1768972515"
                            }
                        },
                        {
                            "id": "018d937e-ff54-72a0-a5d7-ed10118f50d8",
                            "slug": "remember-me",
                            "title": "Remember Me",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/boxart.jpg?t=1779340806",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner145.jpg?t=1779340806",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner300.jpg?t=1779340806",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner400.jpg?t=1779340806",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner600.jpg?t=1779340807"
                            }
                        },
                        {
                            "id": "018d937f-0884-70af-9932-87109bf4aa9a",
                            "slug": "remember-me-complete-pack",
                            "title": "Remember Me™ Complete Pack",
                            "type": null,
                            "mature": false,
                            "assets": {}
                        },
                        {
                            "id": "018d937f-08a6-7233-9599-69a9dfae2a09",
                            "slug": "marvel-vs-capcom-infinite",
                            "title": "Marvel vs. Capcom: Infinite",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-08a6-7233-9599-69a9dfae2a09/boxart.jpg?t=1761632434",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-08a6-7233-9599-69a9dfae2a09/banner145.jpg?t=1761632434",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-08a6-7233-9599-69a9dfae2a09/banner300.jpg?t=1761632434",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-08a6-7233-9599-69a9dfae2a09/banner400.jpg?t=1761632434",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-08a6-7233-9599-69a9dfae2a09/banner600.jpg?t=1761632436"
                            }
                        },
                        {
                            "id": "018d937f-0e4f-700e-89a5-d67d387ebf87",
                            "slug": "umbrella-corps",
                            "title": "Umbrella Corps™",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/boxart.jpg?t=1761759094",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/banner145.jpg?t=1761759094",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/banner300.jpg?t=1761759094",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/banner400.jpg?t=1761759094",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/banner600.jpg?t=1761759095"
                            }
                        },
                        {
                            "id": "018d937f-1714-7308-8702-057c5ab0ec1c",
                            "slug": "umbrella-corps-deluxe-edition",
                            "title": "Umbrella Corps™ Deluxe Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1714-7308-8702-057c5ab0ec1c/boxart.jpg?t=1756056314",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1714-7308-8702-057c5ab0ec1c/banner145.jpg?t=1756056314",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1714-7308-8702-057c5ab0ec1c/banner300.jpg?t=1756056314",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1714-7308-8702-057c5ab0ec1c/banner400.jpg?t=1756056314",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1714-7308-8702-057c5ab0ec1c/banner600.jpg?t=1756056316"
                            }
                        },
                        {
                            "id": "018d937f-190c-739f-a70a-59910c8d3c69",
                            "slug": "resident-evil-revelations-unveiled-edition",
                            "title": "Resident Evil Revelations Unveiled Edition",
                            "type": null,
                            "mature": false,
                            "assets": {}
                        },
                        {
                            "id": "018d937f-1e98-7204-9325-3e3295ce44c9",
                            "slug": "resident-evil-hd-remaster",
                            "title": "Resident Evil HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/boxart.jpg?t=1768959004",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner145.jpg?t=1768959004",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner300.jpg?t=1768959004",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner400.jpg?t=1768959004",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner600.jpg?t=1768959005"
                            }
                        },
                        {
                            "id": "018d937f-1eed-7294-8720-f38b40ba52d9",
                            "slug": "lost-planet-3",
                            "title": "Lost Planet 3",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/boxart.jpg?t=1763217739",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner145.jpg?t=1763217739",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner300.jpg?t=1763217739",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner400.jpg?t=1763217740",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner600.jpg?t=1763217740"
                            }
                        },
                        {
                            "id": "018d937f-1f5c-717d-bcab-035a7c2005ce",
                            "slug": "ultimate-marvel-vs-capcom-3",
                            "title": "Ultimate Marvel vs. Capcom 3",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1f5c-717d-bcab-035a7c2005ce/boxart.jpg?t=1781665819",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1f5c-717d-bcab-035a7c2005ce/banner145.jpg?t=1781665820",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1f5c-717d-bcab-035a7c2005ce/banner300.jpg?t=1781665820",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1f5c-717d-bcab-035a7c2005ce/banner400.jpg?t=1781665820",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1f5c-717d-bcab-035a7c2005ce/banner600.jpg?t=1781665820"
                            }
                        },
                        {
                            "id": "018d937f-221f-70de-8737-c0c0ad381541",
                            "slug": "dead-rising-3-apocalypse-edition",
                            "title": "Dead Rising 3 Apocalypse Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/boxart.jpg?t=1768870815",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner145.jpg?t=1768870815",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner300.jpg?t=1768870816",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner400.jpg?t=1768870816",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner600.jpg?t=1768870817"
                            }
                        },
                        {
                            "id": "018d937f-2459-7030-a3cd-77a8f13b87e9",
                            "slug": "dragons-dogma-dark-arisen",
                            "title": "Dragon's Dogma: Dark Arisen",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/boxart.jpg?t=1768877406",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner145.jpg?t=1768877406",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner300.jpg?t=1768877406",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner400.jpg?t=1768877406",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-2459-7030-a3cd-77a8f13b87e9/banner600.jpg?t=1768877407"
                            }
                        },
                        {
                            "id": "018d937f-24e7-7076-8aea-621316cbd37b",
                            "slug": "lost-planet-extreme-condition",
                            "title": "Lost Planet™: Extreme Condition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-24e7-7076-8aea-621316cbd37b/boxart.jpg?t=1763147560",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-24e7-7076-8aea-621316cbd37b/banner145.jpg?t=1763147560",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-24e7-7076-8aea-621316cbd37b/banner300.jpg?t=1763147560",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-24e7-7076-8aea-621316cbd37b/banner400.jpg?t=1763147560",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-24e7-7076-8aea-621316cbd37b/banner600.jpg?t=1763147561"
                            }
                        },
                        {
                            "id": "018d937f-2ea5-732b-a55c-a59d5b3dee16",
                            "slug": "dead-rising-4-franks-big-package",
                            "title": "Dead Rising 4 Frank's Big Package",
                            "type": "package",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-2ea5-732b-a55c-a59d5b3dee16/boxart.jpg?t=1756055130",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-2ea5-732b-a55c-a59d5b3dee16/banner145.jpg?t=1756055130",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-2ea5-732b-a55c-a59d5b3dee16/banner300.jpg?t=1756055130",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-2ea5-732b-a55c-a59d5b3dee16/banner400.jpg?t=1756055130",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-2ea5-732b-a55c-a59d5b3dee16/banner600.jpg?t=1756055130"
                            }
                        },
                        {
                            "id": "018d937f-2efc-72b2-8ae1-5f5423ab9ca2",
                            "slug": "bionic-commando",
                            "title": "Bionic Commando",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/boxart.jpg?t=1763150455",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner145.jpg?t=1763150455",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner300.jpg?t=1763150455",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner400.jpg?t=1763150455",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner600.jpg?t=1763150455"
                            }
                        },
                        {
                            "id": "018d937f-3407-739f-b4a7-46ccf6754a20",
                            "slug": "the-bionic-commando-pack",
                            "title": "The Bionic Commando Pack",
                            "type": "package",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3407-739f-b4a7-46ccf6754a20/boxart.jpg?t=1770900629",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3407-739f-b4a7-46ccf6754a20/banner145.jpg?t=1770900629",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3407-739f-b4a7-46ccf6754a20/banner300.jpg?t=1770900630",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3407-739f-b4a7-46ccf6754a20/banner400.jpg?t=1770900630",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3407-739f-b4a7-46ccf6754a20/banner600.jpg?t=1770900630"
                            }
                        },
                        {
                            "id": "018d937f-349f-71cd-8eba-8632260d3f33",
                            "slug": "dead-rising",
                            "title": "Dead Rising",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/boxart.jpg?t=1768871107",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/banner145.jpg?t=1768871107",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/banner300.jpg?t=1768871107",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/banner400.jpg?t=1768871107",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/banner600.jpg?t=1768871108"
                            }
                        },
                        {
                            "id": "018d937f-358c-734b-aa9b-776747448c8b",
                            "slug": "marvel-vs-capcom-infinite-deluxe-edition",
                            "title": "Marvel vs. Capcom: Infinite - Deluxe Edition",
                            "type": "package",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-358c-734b-aa9b-776747448c8b/boxart.jpg?t=1755105618",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-358c-734b-aa9b-776747448c8b/banner145.jpg?t=1755105619",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-358c-734b-aa9b-776747448c8b/banner300.jpg?t=1755105619",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-358c-734b-aa9b-776747448c8b/banner400.jpg?t=1755105619",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-358c-734b-aa9b-776747448c8b/banner600.jpg?t=1755105620"
                            }
                        },
                        {
                            "id": "018d937f-3610-735b-93db-f948f52410f7",
                            "slug": "resident-evil-5-gold-edition",
                            "title": "Resident Evil 5 GOLD Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/boxart.jpg?t=1756056005",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner145.jpg?t=1756056006",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner300.jpg?t=1756056006",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner400.jpg?t=1756056006",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner600.jpg?t=1756056006"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        },
                        {
                            "id": "018d937f-7829-7173-8d70-703fec94e4d6",
                            "slug": "resident-evil-3-2020",
                            "title": "RESIDENT EVIL 3 (2020)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/boxart.jpg?t=1780363208",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner145.jpg?t=1780363208",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner300.jpg?t=1780363208",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner400.jpg?t=1780363208",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-7829-7173-8d70-703fec94e4d6/banner600.jpg?t=1780363208"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 7031,
            "title": "Resident Evil Survivors Pack",
            "page": {
                "id": 8,
                "name": "Fanatical",
                "shopId": 6
            },
            "url": "https://www.awin1.com/cread.php?awinmid=118821&awinaffid=235265&ued=https%3A%2F%2Fwww.fanatical.com%2Fen%2Fbundle%2Fresident-evil-survivors-pack",
            "details": "https://isthereanydeal.com/bundles/7031/",
            "isMature": false,
            "publish": "2019-02-15T19:42:00+01:00",
            "expiry": "2019-02-22T17:00:00+01:00",
            "note": null,
            "counts": {
                "games": 3,
                "media": 0
            },
            "tiers": [
                {
                    "price": {
                        "amount": 17.98,
                        "amountInt": 1798,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-fd75-7351-be7d-a0be822b98a1",
                            "slug": "resident-evil-6",
                            "title": "Resident Evil 6",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/boxart.jpg?t=1768963508",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner145.jpg?t=1768963508",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner300.jpg?t=1768963508",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner400.jpg?t=1768963508",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner600.jpg?t=1768963508"
                            }
                        },
                        {
                            "id": "018d937f-3610-735b-93db-f948f52410f7",
                            "slug": "resident-evil-5-gold-edition",
                            "title": "Resident Evil 5 GOLD Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/boxart.jpg?t=1756056005",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner145.jpg?t=1756056006",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner300.jpg?t=1756056006",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner400.jpg?t=1756056006",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3610-735b-93db-f948f52410f7/banner600.jpg?t=1756056006"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 5452,
            "title": "Capcom X Sega X Atlus Bundle",
            "page": {
                "id": 1,
                "name": "Humble Bundle",
                "shopId": 37
            },
            "url": "https://humblebundleinc.sjv.io/c/2545989/2091208/25796?u=https%3A%2F%2Fwww.humblebundle.com%2Fcapcom-sega-atlus-bundle",
            "details": "https://isthereanydeal.com/bundles/5452/",
            "isMature": false,
            "publish": "2017-09-12T20:25:00+02:00",
            "expiry": "2017-09-26T22:00:00+02:00",
            "note": null,
            "counts": {
                "games": 12,
                "media": 0
            },
            "tiers": [
                {
                    "price": {
                        "amount": 1,
                        "amountInt": 100,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f809-71e8-ba2a-6688edc2d3d8",
                            "slug": "sonic-adventure-2",
                            "title": "Sonic Adventure 2",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f809-71e8-ba2a-6688edc2d3d8/boxart.jpg?t=1762752905",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f809-71e8-ba2a-6688edc2d3d8/banner145.jpg?t=1762752906",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f809-71e8-ba2a-6688edc2d3d8/banner300.jpg?t=1762752906",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f809-71e8-ba2a-6688edc2d3d8/banner400.jpg?t=1762752906",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f809-71e8-ba2a-6688edc2d3d8/banner600.jpg?t=1762752907"
                            }
                        },
                        {
                            "id": "018d937f-0de6-7166-af51-0a0d977db15b",
                            "slug": "rollers-of-the-realm",
                            "title": "Rollers of the Realm",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0de6-7166-af51-0a0d977db15b/boxart.jpg?t=1761612032",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0de6-7166-af51-0a0d977db15b/banner145.jpg?t=1761612032",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0de6-7166-af51-0a0d977db15b/banner300.jpg?t=1761612032",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0de6-7166-af51-0a0d977db15b/banner400.jpg?t=1761612032",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0de6-7166-af51-0a0d977db15b/banner600.jpg?t=1761612033"
                            }
                        },
                        {
                            "id": "018d937f-1633-70fb-8f4f-361fe3b0e59e",
                            "slug": "zeno-clash-2",
                            "title": "Zeno Clash 2",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1633-70fb-8f4f-361fe3b0e59e/boxart.jpg?t=1763420118",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1633-70fb-8f4f-361fe3b0e59e/banner145.jpg?t=1763420118",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1633-70fb-8f4f-361fe3b0e59e/banner300.jpg?t=1763420118",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1633-70fb-8f4f-361fe3b0e59e/banner400.jpg?t=1763420118",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1633-70fb-8f4f-361fe3b0e59e/banner600.jpg?t=1763420119"
                            }
                        },
                        {
                            "id": "018d937f-26a4-708b-a574-e7b02fd6b245",
                            "slug": "citizens-of-earth",
                            "title": "Citizens of Earth",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-26a4-708b-a574-e7b02fd6b245/boxart.jpg?t=1769733965",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-26a4-708b-a574-e7b02fd6b245/banner145.jpg?t=1769733965",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-26a4-708b-a574-e7b02fd6b245/banner300.jpg?t=1769733965",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-26a4-708b-a574-e7b02fd6b245/banner400.jpg?t=1769733965",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-26a4-708b-a574-e7b02fd6b245/banner600.jpg?t=1769733966"
                            }
                        },
                        {
                            "id": "018d937f-2efc-72b2-8ae1-5f5423ab9ca2",
                            "slug": "bionic-commando",
                            "title": "Bionic Commando",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/boxart.jpg?t=1763150455",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner145.jpg?t=1763150455",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner300.jpg?t=1763150455",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner400.jpg?t=1763150455",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner600.jpg?t=1763150455"
                            }
                        }
                    ]
                },
                {
                    "price": null,
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-e9ea-739c-ad3b-6439425fae67",
                            "slug": "renegade-ops-collection",
                            "title": "Renegade Ops Collection",
                            "type": "package",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-e9ea-739c-ad3b-6439425fae67/boxart.jpg?t=1756402517",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-e9ea-739c-ad3b-6439425fae67/banner145.jpg?t=1756402518",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-e9ea-739c-ad3b-6439425fae67/banner300.jpg?t=1756402518",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-e9ea-739c-ad3b-6439425fae67/banner400.jpg?t=1756402518",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-e9ea-739c-ad3b-6439425fae67/banner600.jpg?t=1756402518"
                            }
                        },
                        {
                            "id": "018d937f-0a6f-702f-a6fb-63a947847057",
                            "slug": "sonic-generations-collection",
                            "title": "Sonic Generations Collection",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0a6f-702f-a6fb-63a947847057/boxart.jpg?t=1762753808",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0a6f-702f-a6fb-63a947847057/banner145.jpg?t=1762753809",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0a6f-702f-a6fb-63a947847057/banner300.jpg?t=1762753809",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0a6f-702f-a6fb-63a947847057/banner400.jpg?t=1762753809",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0a6f-702f-a6fb-63a947847057/banner600.jpg?t=1762753810"
                            }
                        },
                        {
                            "id": "018d937f-349f-71cd-8eba-8632260d3f33",
                            "slug": "dead-rising",
                            "title": "Dead Rising",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/boxart.jpg?t=1768871107",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/banner145.jpg?t=1768871107",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/banner300.jpg?t=1768871107",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/banner400.jpg?t=1768871107",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-349f-71cd-8eba-8632260d3f33/banner600.jpg?t=1768871108"
                            }
                        },
                        {
                            "id": "018d937f-3637-7204-aa01-d29f494c2c9b",
                            "slug": "tesla-effect-a-tex-murphy-adventure",
                            "title": "Tesla Effect: A Tex Murphy Adventure",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3637-7204-aa01-d29f494c2c9b/boxart.jpg?t=1763064938",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3637-7204-aa01-d29f494c2c9b/banner145.jpg?t=1763064938",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3637-7204-aa01-d29f494c2c9b/banner300.jpg?t=1763064938",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3637-7204-aa01-d29f494c2c9b/banner400.jpg?t=1763064938",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3637-7204-aa01-d29f494c2c9b/banner600.jpg?t=1763064938"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        }
                    ]
                },
                {
                    "price": {
                        "amount": 12,
                        "amountInt": 1200,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937f-0101-71c0-b711-2420bbbc2816",
                            "slug": "motorsport-manager",
                            "title": "Motorsport Manager",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0101-71c0-b711-2420bbbc2816/boxart.jpg?t=1775831708",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0101-71c0-b711-2420bbbc2816/banner145.jpg?t=1775831709",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0101-71c0-b711-2420bbbc2816/banner300.jpg?t=1775831709",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0101-71c0-b711-2420bbbc2816/banner400.jpg?t=1775831709",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0101-71c0-b711-2420bbbc2816/banner600.jpg?t=1775831709"
                            }
                        },
                        {
                            "id": "018d937f-160c-7273-9785-a320b4eb9af5",
                            "slug": "devil-may-cry-4-special-edition",
                            "title": "Devil May Cry® 4 Special Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/boxart.jpg?t=1768872007",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner145.jpg?t=1768872008",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner300.jpg?t=1768872008",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner400.jpg?t=1768872008",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner600.jpg?t=1768872009"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 5315,
            "title": "Humble Capcom Rising Bundle",
            "page": {
                "id": 1,
                "name": "Humble Bundle",
                "shopId": 37
            },
            "url": "https://humblebundleinc.sjv.io/c/2545989/2091208/25796?u=https%3A%2F%2Fwww.humblebundle.com%2Fcapcom-rising-bundle",
            "details": "https://isthereanydeal.com/bundles/5315/",
            "isMature": false,
            "publish": "2017-07-11T21:00:00+02:00",
            "expiry": "2017-07-25T20:00:00+02:00",
            "note": null,
            "counts": {
                "games": 10,
                "media": 0
            },
            "tiers": [
                {
                    "price": {
                        "amount": 1,
                        "amountInt": 100,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937f-0c4d-71ff-8adc-6a1bf47c362b",
                            "slug": "dmc-devil-may-cry",
                            "title": "DMC - Devil May Cry",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/boxart.jpg?t=1768871716",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner145.jpg?t=1768871717",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner300.jpg?t=1768871717",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner400.jpg?t=1768871717",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner600.jpg?t=1768871717"
                            }
                        },
                        {
                            "id": "018d937f-0e4f-700e-89a5-d67d387ebf87",
                            "slug": "umbrella-corps",
                            "title": "Umbrella Corps™",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/boxart.jpg?t=1761759094",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/banner145.jpg?t=1761759094",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/banner300.jpg?t=1761759094",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/banner400.jpg?t=1761759094",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0e4f-700e-89a5-d67d387ebf87/banner600.jpg?t=1761759095"
                            }
                        },
                        {
                            "id": "018d937f-0faa-71eb-85e0-cb8fc653b6a7",
                            "slug": "strider-sutoraida-fei-long",
                            "title": "STRIDER™ / ストライダー飛竜®",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/boxart.jpg?t=1778638812",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner145.jpg?t=1778638813",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner300.jpg?t=1778638813",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner400.jpg?t=1778638813",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner600.jpg?t=1778638814"
                            }
                        }
                    ]
                },
                {
                    "price": null,
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f4da-726b-aff7-4fde33dab7d0",
                            "slug": "dead-rising-2-off-the-record",
                            "title": "Dead Rising® 2: Off The Record",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/boxart.jpg?t=1768876518",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner145.jpg?t=1768876518",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner300.jpg?t=1768876518",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner400.jpg?t=1768876518",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f4da-726b-aff7-4fde33dab7d0/banner600.jpg?t=1768876519"
                            }
                        },
                        {
                            "id": "018d937e-f85f-70cb-9323-8554024f67c2",
                            "slug": "resident-evil-0-hd-remaster",
                            "title": "Resident Evil 0 HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/boxart.jpg?t=1768966212",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner145.jpg?t=1768966212",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner300.jpg?t=1768966212",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner400.jpg?t=1768966212",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f85f-70cb-9323-8554024f67c2/banner600.jpg?t=1768966213"
                            }
                        },
                        {
                            "id": "018d937e-fd75-7351-be7d-a0be822b98a1",
                            "slug": "resident-evil-6",
                            "title": "Resident Evil 6",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/boxart.jpg?t=1768963508",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner145.jpg?t=1768963508",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner300.jpg?t=1768963508",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner400.jpg?t=1768963508",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner600.jpg?t=1768963508"
                            }
                        },
                        {
                            "id": "018d937f-1d00-71b3-a21c-47746962f65b",
                            "slug": "upgrade-pack-ejentogosutopakku",
                            "title": "Upgrade Pack/エージェントゴーストパック",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1d00-71b3-a21c-47746962f65b/banner145.jpg?t=1732756286",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1d00-71b3-a21c-47746962f65b/banner300.jpg?t=1732756286",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1d00-71b3-a21c-47746962f65b/banner400.jpg?t=1732756286",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1d00-71b3-a21c-47746962f65b/banner600.jpg?t=1732756286"
                            }
                        },
                        {
                            "id": "018d937f-1e98-7204-9325-3e3295ce44c9",
                            "slug": "resident-evil-hd-remaster",
                            "title": "Resident Evil HD Remaster",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/boxart.jpg?t=1768959004",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner145.jpg?t=1768959004",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner300.jpg?t=1768959004",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner400.jpg?t=1768959004",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1e98-7204-9325-3e3295ce44c9/banner600.jpg?t=1768959005"
                            }
                        }
                    ]
                },
                {
                    "price": {
                        "amount": 12,
                        "amountInt": 1200,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f03e-7108-a2cc-8aeb260783ad",
                            "slug": "dead-rising-2",
                            "title": "Dead Rising® 2",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/boxart.jpg?t=1768869302",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner145.jpg?t=1768869302",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner300.jpg?t=1768869302",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner400.jpg?t=1768869303",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f03e-7108-a2cc-8aeb260783ad/banner600.jpg?t=1768869304"
                            }
                        },
                        {
                            "id": "018d937f-221f-70de-8737-c0c0ad381541",
                            "slug": "dead-rising-3-apocalypse-edition",
                            "title": "Dead Rising 3 Apocalypse Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/boxart.jpg?t=1768870815",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner145.jpg?t=1768870815",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner300.jpg?t=1768870816",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner400.jpg?t=1768870816",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-221f-70de-8737-c0c0ad381541/banner600.jpg?t=1768870817"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 3406,
            "title": "Humble Capcom Super Turbo HD Remix! ReBundle",
            "page": {
                "id": 1,
                "name": "Humble Bundle",
                "shopId": 37
            },
            "url": "https://humblebundleinc.sjv.io/c/2545989/2091208/25796?u=https%3A%2F%2Fwww.humblebundle.com%2Fcapcom-bundle-remix",
            "details": "https://isthereanydeal.com/bundles/3406/",
            "isMature": false,
            "publish": "2016-05-10T20:13:00+02:00",
            "expiry": "2016-05-24T21:00:00+02:00",
            "note": null,
            "counts": {
                "games": 20,
                "media": 2
            },
            "tiers": [
                {
                    "price": {
                        "amount": 1,
                        "amountInt": 100,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f559-73df-9406-3ffe1f88671a",
                            "slug": "bionic-commando-rearmed",
                            "title": "Bionic Commando: Rearmed",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/boxart.jpg?t=1763099694",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner145.jpg?t=1763099694",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner300.jpg?t=1763099694",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner400.jpg?t=1763099694",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner600.jpg?t=1763099694"
                            }
                        },
                        {
                            "id": "018d937e-f8a0-70e7-95d3-89920b784119",
                            "slug": "lost-planet-3-punisher-pack",
                            "title": "LOST PLANET® 3 - Punisher Pack",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f8a0-70e7-95d3-89920b784119/banner145.jpg?t=1732226107",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f8a0-70e7-95d3-89920b784119/banner300.jpg?t=1732226107",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f8a0-70e7-95d3-89920b784119/banner400.jpg?t=1732226107",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f8a0-70e7-95d3-89920b784119/banner600.jpg?t=1732226107"
                            }
                        },
                        {
                            "id": "018d937e-fdd0-723a-b35e-4a2a308b1a1f",
                            "slug": "lost-planet-3-freedom-fighter-pack",
                            "title": "LOST PLANET® 3 - Freedom Fighter Pack",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fdd0-723a-b35e-4a2a308b1a1f/banner145.jpg?t=1732239012",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fdd0-723a-b35e-4a2a308b1a1f/banner300.jpg?t=1732239012",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fdd0-723a-b35e-4a2a308b1a1f/banner400.jpg?t=1732239012",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fdd0-723a-b35e-4a2a308b1a1f/banner600.jpg?t=1732239013"
                            }
                        },
                        {
                            "id": "018d937f-05ff-706e-b42a-d235a220c109",
                            "slug": "lost-planet-3-assault-pack",
                            "title": "Lost Planet 3: Assault Pack",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937f-05ff-706e-b42a-d235a220c109/banner145.jpg?t=1732259710",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-05ff-706e-b42a-d235a220c109/banner300.jpg?t=1732259710",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-05ff-706e-b42a-d235a220c109/banner400.jpg?t=1732259710",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-05ff-706e-b42a-d235a220c109/banner600.jpg?t=1732259711"
                            }
                        },
                        {
                            "id": "018d937f-0c8f-72a4-b9aa-93e3bb0a36e5",
                            "slug": "lost-planet-3-map-pack-1",
                            "title": "LOST PLANET® 3 - Map Pack 1",
                            "type": null,
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0c8f-72a4-b9aa-93e3bb0a36e5/banner145.jpg?t=1732224617",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0c8f-72a4-b9aa-93e3bb0a36e5/banner300.jpg?t=1732224617",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0c8f-72a4-b9aa-93e3bb0a36e5/banner400.jpg?t=1732224617",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0c8f-72a4-b9aa-93e3bb0a36e5/banner600.jpg?t=1732224617"
                            }
                        },
                        {
                            "id": "018d937f-0faa-71eb-85e0-cb8fc653b6a7",
                            "slug": "strider-sutoraida-fei-long",
                            "title": "STRIDER™ / ストライダー飛竜®",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/boxart.jpg?t=1778638812",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner145.jpg?t=1778638813",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner300.jpg?t=1778638813",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner400.jpg?t=1778638813",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner600.jpg?t=1778638814"
                            }
                        },
                        {
                            "id": "018d937f-1037-7160-a4af-cf3a492b8a2a",
                            "slug": "lost-planet-3-map-pack-2",
                            "title": "LOST PLANET® 3 - Map Pack 2",
                            "type": null,
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1037-7160-a4af-cf3a492b8a2a/banner145.jpg?t=1732225205",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1037-7160-a4af-cf3a492b8a2a/banner300.jpg?t=1732225205",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1037-7160-a4af-cf3a492b8a2a/banner400.jpg?t=1732225205",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1037-7160-a4af-cf3a492b8a2a/banner600.jpg?t=1732225206"
                            }
                        },
                        {
                            "id": "018d937f-1592-71c6-adec-3806cb0f64cf",
                            "slug": "lost-planet-3-survival-pack",
                            "title": "LOST PLANET® 3 - Survival Pack",
                            "type": null,
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1592-71c6-adec-3806cb0f64cf/banner145.jpg?t=1732281308",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1592-71c6-adec-3806cb0f64cf/banner300.jpg?t=1732281308",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1592-71c6-adec-3806cb0f64cf/banner400.jpg?t=1732281308",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1592-71c6-adec-3806cb0f64cf/banner600.jpg?t=1732281309"
                            }
                        },
                        {
                            "id": "018d937f-1eed-7294-8720-f38b40ba52d9",
                            "slug": "lost-planet-3",
                            "title": "Lost Planet 3",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/boxart.jpg?t=1763217739",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner145.jpg?t=1763217739",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner300.jpg?t=1763217739",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner400.jpg?t=1763217740",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner600.jpg?t=1763217740"
                            }
                        },
                        {
                            "id": "018d937f-1f53-731d-89ec-07d406ebe5d5",
                            "slug": "lost-planet-3-complete",
                            "title": "Lost Planet 3 - Complete",
                            "type": "package",
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1f53-731d-89ec-07d406ebe5d5/banner145.jpg?t=1756056035",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1f53-731d-89ec-07d406ebe5d5/banner300.jpg?t=1756056035",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1f53-731d-89ec-07d406ebe5d5/banner400.jpg?t=1756056035",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1f53-731d-89ec-07d406ebe5d5/banner600.jpg?t=1756056036"
                            }
                        },
                        {
                            "id": "018d937f-3445-7218-beb9-7dea0bb65499",
                            "slug": "resident-evil-revelations-2-episode-one-penal-colony",
                            "title": "Resident Evil: Revelations 2 - Episode One: Penal Colony",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/boxart.jpg?t=1769129116",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner145.jpg?t=1769129116",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner300.jpg?t=1769129116",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner400.jpg?t=1769129116",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner600.jpg?t=1769129117"
                            }
                        },
                        {
                            "id": "018d937f-35ae-70c5-87b3-fadcfebde7da",
                            "slug": "lost-planet-3-map-pack-3",
                            "title": "LOST PLANET® 3 - Map Pack 3",
                            "type": null,
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937f-35ae-70c5-87b3-fadcfebde7da/banner145.jpg?t=1732219525",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-35ae-70c5-87b3-fadcfebde7da/banner300.jpg?t=1732219525",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-35ae-70c5-87b3-fadcfebde7da/banner400.jpg?t=1732219525",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-35ae-70c5-87b3-fadcfebde7da/banner600.jpg?t=1732219526"
                            }
                        }
                    ]
                },
                {
                    "price": null,
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f541-7208-b213-4b438af2fbb4",
                            "slug": "resident-evil-revelations",
                            "title": "Resident Evil: Revelations",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/boxart.jpg?t=1769127337",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner145.jpg?t=1769127337",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner300.jpg?t=1769127338",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner400.jpg?t=1769127338",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner600.jpg?t=1769127339"
                            }
                        },
                        {
                            "id": "018d937e-ff54-72a0-a5d7-ed10118f50d8",
                            "slug": "remember-me",
                            "title": "Remember Me",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/boxart.jpg?t=1779340806",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner145.jpg?t=1779340806",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner300.jpg?t=1779340806",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner400.jpg?t=1779340806",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner600.jpg?t=1779340807"
                            }
                        },
                        {
                            "id": "018d937f-0c4d-71ff-8adc-6a1bf47c362b",
                            "slug": "dmc-devil-may-cry",
                            "title": "DMC - Devil May Cry",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/boxart.jpg?t=1768871716",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner145.jpg?t=1768871717",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner300.jpg?t=1768871717",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner400.jpg?t=1768871717",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner600.jpg?t=1768871717"
                            }
                        },
                        {
                            "id": "018d937f-2efc-72b2-8ae1-5f5423ab9ca2",
                            "slug": "bionic-commando",
                            "title": "Bionic Commando",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/boxart.jpg?t=1763150455",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner145.jpg?t=1763150455",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner300.jpg?t=1763150455",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner400.jpg?t=1763150455",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-2efc-72b2-8ae1-5f5423ab9ca2/banner600.jpg?t=1763150455"
                            }
                        },
                        {
                            "id": "018d937f-3271-70f9-a0ed-47379fe5500f",
                            "slug": "dmc-devil-may-cry-vergils-downfall",
                            "title": "DmC Devil May Cry: Vergil's Downfall",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3271-70f9-a0ed-47379fe5500f/banner145.jpg?t=1732855842",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3271-70f9-a0ed-47379fe5500f/banner300.jpg?t=1732855842",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3271-70f9-a0ed-47379fe5500f/banner400.jpg?t=1732855842"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        }
                    ]
                },
                {
                    "price": {
                        "amount": 15,
                        "amountInt": 1500,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-fd75-7351-be7d-a0be822b98a1",
                            "slug": "resident-evil-6",
                            "title": "Resident Evil 6",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/boxart.jpg?t=1768963508",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner145.jpg?t=1768963508",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner300.jpg?t=1768963508",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner400.jpg?t=1768963508",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fd75-7351-be7d-a0be822b98a1/banner600.jpg?t=1768963508"
                            }
                        },
                        {
                            "id": "018d937f-160c-7273-9785-a320b4eb9af5",
                            "slug": "devil-may-cry-4-special-edition",
                            "title": "Devil May Cry® 4 Special Edition",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/boxart.jpg?t=1768872007",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner145.jpg?t=1768872008",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner300.jpg?t=1768872008",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner400.jpg?t=1768872008",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-160c-7273-9785-a320b4eb9af5/banner600.jpg?t=1768872009"
                            }
                        }
                    ]
                }
            ]
        },
        {
            "id": 2599,
            "title": "Humble Capcom Bundle ",
            "page": {
                "id": 1,
                "name": "Humble Bundle",
                "shopId": 37
            },
            "url": "https://humblebundleinc.sjv.io/c/2545989/2091208/25796?u=https%3A%2F%2Fwww.humblebundle.com%2F",
            "details": "https://isthereanydeal.com/bundles/2599/",
            "isMature": false,
            "publish": "2015-10-13T20:00:00+02:00",
            "expiry": "2015-10-27T19:00:00+01:00",
            "note": null,
            "counts": {
                "games": 11,
                "media": 1
            },
            "tiers": [
                {
                    "price": {
                        "amount": 1,
                        "amountInt": 100,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f559-73df-9406-3ffe1f88671a",
                            "slug": "bionic-commando-rearmed",
                            "title": "Bionic Commando: Rearmed",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/boxart.jpg?t=1763099694",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner145.jpg?t=1763099694",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner300.jpg?t=1763099694",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner400.jpg?t=1763099694",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f559-73df-9406-3ffe1f88671a/banner600.jpg?t=1763099694"
                            }
                        },
                        {
                            "id": "018d937f-0faa-71eb-85e0-cb8fc653b6a7",
                            "slug": "strider-sutoraida-fei-long",
                            "title": "STRIDER™ / ストライダー飛竜®",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/boxart.jpg?t=1778638812",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner145.jpg?t=1778638813",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner300.jpg?t=1778638813",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner400.jpg?t=1778638813",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0faa-71eb-85e0-cb8fc653b6a7/banner600.jpg?t=1778638814"
                            }
                        },
                        {
                            "id": "018d937f-1eed-7294-8720-f38b40ba52d9",
                            "slug": "lost-planet-3",
                            "title": "Lost Planet 3",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/boxart.jpg?t=1763217739",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner145.jpg?t=1763217739",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner300.jpg?t=1763217739",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner400.jpg?t=1763217740",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-1eed-7294-8720-f38b40ba52d9/banner600.jpg?t=1763217740"
                            }
                        },
                        {
                            "id": "018d937f-3445-7218-beb9-7dea0bb65499",
                            "slug": "resident-evil-revelations-2-episode-one-penal-colony",
                            "title": "Resident Evil: Revelations 2 - Episode One: Penal Colony",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/boxart.jpg?t=1769129116",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner145.jpg?t=1769129116",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner300.jpg?t=1769129116",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner400.jpg?t=1769129116",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-3445-7218-beb9-7dea0bb65499/banner600.jpg?t=1769129117"
                            }
                        }
                    ]
                },
                {
                    "price": null,
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-f541-7208-b213-4b438af2fbb4",
                            "slug": "resident-evil-revelations",
                            "title": "Resident Evil: Revelations",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/boxart.jpg?t=1769127337",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner145.jpg?t=1769127337",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner300.jpg?t=1769127338",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner400.jpg?t=1769127338",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-f541-7208-b213-4b438af2fbb4/banner600.jpg?t=1769127339"
                            }
                        },
                        {
                            "id": "018d937e-feb8-71a3-8819-21b3f9d400b8",
                            "slug": "resident-evil-5",
                            "title": "Resident Evil 5",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/boxart.jpg?t=1768972513",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner145.jpg?t=1768972513",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner300.jpg?t=1768972513",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner400.jpg?t=1768972513",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-feb8-71a3-8819-21b3f9d400b8/banner600.jpg?t=1768972515"
                            }
                        },
                        {
                            "id": "018d937e-ff54-72a0-a5d7-ed10118f50d8",
                            "slug": "remember-me",
                            "title": "Remember Me",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/boxart.jpg?t=1779340806",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner145.jpg?t=1779340806",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner300.jpg?t=1779340806",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner400.jpg?t=1779340806",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-ff54-72a0-a5d7-ed10118f50d8/banner600.jpg?t=1779340807"
                            }
                        },
                        {
                            "id": "018d937f-0c4d-71ff-8adc-6a1bf47c362b",
                            "slug": "dmc-devil-may-cry",
                            "title": "DMC - Devil May Cry",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/boxart.jpg?t=1768871716",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner145.jpg?t=1768871717",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner300.jpg?t=1768871717",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner400.jpg?t=1768871717",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-0c4d-71ff-8adc-6a1bf47c362b/banner600.jpg?t=1768871717"
                            }
                        },
                        {
                            "id": "018d937f-6b99-7077-9cf6-eacfce97587f",
                            "slug": "resident-evil-4-2005",
                            "title": "Resident Evil 4 (2005)",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/boxart.jpg?t=1769345721",
                                "banner145": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner145.jpg?t=1769345721",
                                "banner300": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner300.jpg?t=1769345721",
                                "banner400": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner400.jpg?t=1769345721",
                                "banner600": "https://assets.isthereanydeal.com/018d937f-6b99-7077-9cf6-eacfce97587f/banner600.jpg?t=1769345721"
                            }
                        }
                    ]
                },
                {
                    "price": {
                        "amount": 15,
                        "amountInt": 1500,
                        "currency": "USD"
                    },
                    "addon": false,
                    "games": [
                        {
                            "id": "018d937e-effa-732c-848f-5948d6106f81",
                            "slug": "ultra-street-fighter-iv",
                            "title": "Ultra Street Fighter IV",
                            "type": "game",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/boxart.jpg?t=1763093092",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner145.jpg?t=1763093092",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner300.jpg?t=1763093092",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner400.jpg?t=1763093092",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-effa-732c-848f-5948d6106f81/banner600.jpg?t=1763093092"
                            }
                        },
                        {
                            "id": "018d937e-fa14-7020-9111-93ad8d407d3c",
                            "slug": "resident-evil-5-untold-stories-bundle",
                            "title": "Resident Evil 5 - UNTOLD STORIES BUNDLE",
                            "type": "dlc",
                            "mature": false,
                            "assets": {
                                "boxart": "https://assets.isthereanydeal.com/018d937e-fa14-7020-9111-93ad8d407d3c/boxart.jpg?t=1733451330",
                                "banner145": "https://assets.isthereanydeal.com/018d937e-fa14-7020-9111-93ad8d407d3c/banner145.jpg?t=1733451330",
                                "banner300": "https://assets.isthereanydeal.com/018d937e-fa14-7020-9111-93ad8d407d3c/banner300.jpg?t=1733451330",
                                "banner400": "https://assets.isthereanydeal.com/018d937e-fa14-7020-9111-93ad8d407d3c/banner400.jpg?t=1733451330",
                                "banner600": "https://assets.isthereanydeal.com/018d937e-fa14-7020-9111-93ad8d407d3c/banner600.jpg?t=1733451331"
                            }
                        }
                    ]
                }
            ]
        }
    ]
}
```