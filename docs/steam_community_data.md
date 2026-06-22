### Steam Community Wishlist Data

The following JSON structure represents a user's wishlist data fetched from the Steam Community API.

***https://api.steampowered.com/IWishlistService/GetWishlist/v1/?key={SteamApiKey}&steamid=76561199036840533***

```json
{
    "response": {
        "items": [
            {
                "appid": 221040,
                "priority": 4,
                "date_added": 1781478412
            },
            {
                "appid": 223750,
                "priority": 5,
                "date_added": 1753278865
            },
            {
                "appid": 254700,
                "priority": 2,
                "date_added": 1781570077
            },
            {
                "appid": 3764200,
                "priority": 3,
                "date_added": 1781570323
            },
            {
                "appid": 1462040,
                "priority": 1,
                "date_added": 1782076349
            }
        ]
    }
}
```

Following the API documentation, we can make a GET request to fetch detailed information about each game on Steam. The response will include various details such as the game's name, type, price overview, and categories.


***https://store.steampowered.com/api/appdetails?appids=1462040&cc=lk&filters=basic,genres,price_overview,categories***

```json
{
    "1462040": {
        "success": true,
        "data": {
            "type": "game",
            "name": "FINAL FANTASY VII REMAKE INTERGRADE",
            "steam_appid": 1462040,
            "required_age": 0,
            "is_free": false,
            "dlc": [
                3365650,
                3365670,
                3365690,
                4267260,
                4267470
            ],
            "detailed_description": "<h1>FINAL FANTASY VII REMAKE &amp; REBIRTH Twin Pack</h1><p>This product is a bundle containing FINAL FANTASY VII REMAKE INTERGRADE and FINAL FANTASY VII REBIRTH.<br>FINAL FANTASY VII REMAKE &amp; REBIRTH Digital Deluxe Twin Pack containing the digital content is also available for purchase. Please be wary of duplicate purchases.<br>* Those who already own FINAL FANTASY VII REMAKE INTERGRADE or have purchased FINAL FANTASY VII REBIRTH are also able to purchase this product. Please be wary of duplicate purchases.<br> Please visit the individual product pages for details.<h2 class=\"bb_tag\" >■ Twin Pack Contents:</h2><ul class=\"bb_ul\"><li> FINAL FANTASY VII REMAKE INTERGRADE<br></li><li> FINAL FANTASY VII REBIRTH<br></li><li> Summon materia: Posh Chocobo</li></ul>* FINAL FANTASY VII REMAKE INTERGRADE contains the following content.<br><ul class=\"bb_ul\"><li> FINAL FANTASY VII REMAKE<br></li><li> FF7R EPISODE INTERmission</li></ul></p><br><h1>FINAL FANTASY VII REMAKE &amp; REBIRTH Digital Deluxe Twin Pack</h1><p>This product is a bundle containing FINAL FANTASY VII REMAKE INTERGRADE, FINAL FANTASY VII REBIRTH, and the digital content.<br>FINAL FANTASY VII REMAKE &amp; REBIRTH Twin Pack that does not contain the digital content is also available for purchase. Please be wary of duplicate purchases.<br>* Those who already own FINAL FANTASY VII REMAKE INTERGRADE or have purchased FINAL FANTASY VII REBIRTH are also able to purchase this product. Please be wary of duplicate purchases.<br>Please visit the individual product pages for details.<h2 class=\"bb_tag\" >■ Digital Deluxe Twin Pack Contents:</h2><ul class=\"bb_ul\"><li> FINAL FANTASY VII REMAKE INTERGRADE<br></li><li> FINAL FANTASY VII REBIRTH<br></li><li> Digital Art Book<br></li><li> Digital Mini Soundtrack<br></li><li> Summon materia: Magic Pot<br></li><li> Accessory: Reclaimant Choker<br></li><li> Armor: Orchid Bracelet<br></li><li> Summon materia: Posh Chocobo</li></ul>* FINAL FANTASY VII REMAKE INTERGRADE contains the following content.<br><ul class=\"bb_ul\"><li> FINAL FANTASY VII REMAKE<br></li><li> FF7R EPISODE INTERmission</li></ul>* The digital content may be made available for purchase at a later date.</p><br><h1>About the Game</h1>FINAL FANTASY VII REMAKE is a bold reimagining of the original FINAL FANTASY VII, originally released in 1997, developed under the guidance of the original key developers.<br>This critically-acclaimed game, which mixes traditional command-based combat and real-time action, makes its Steam debut along with FF7R EPISODE INTERmission─a new story arc featuring Yuffie Kisaragi.<br><br><span class=\"bb_img_ctn\"><img class=\"bb_img\" src=\"https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1462040/extras/4465042310caed881656fcae54bc3e4f.avif?t=1773895755\" width=616 height=346 /></span><br><br>■STORY<br><br>By exploiting mako, the life-blood of the planet, through their mako reactors, the Shinra Electric Power Company has all but seized control of the entire world. A ragtag group of idealists, known as Avalanche, are one of the last bastions of resistance.<br><br>Cloud, an elite SOLDIER operative-turned-mercenary takes part in an Avalanche operation to destroy Mako Reactor 1 in the city of Midgar. <br>The bombing plunges the city into fiery chaos, and Cloud is tormented by visions of a bitter enemy long thought dead. <br><br>Once more begins a story that will shape the destiny of an entire world.<br><br>* This game is a remake of FINAL FANTASY VII, first released in 1997.<br>The first title in a multi-part series, it is based on the story from the original game, up to the escape from Midgar, adding in new elements.<br><br><span class=\"bb_img_ctn\"><img class=\"bb_img\" src=\"https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1462040/extras/7b05c4dd523e17b5ae01d62c9bdf857c.avif?t=1773895755\" width=616 height=346 /></span><br><br>■FF7R EPISODE INTERmission (New episode featuring Yuffie Kisaragi)<br><br>FF7R EPISODE INTERmission is a brand-new adventure in the world of FINAL FANTASY VII REMAKE INTERGRADE. Play as Wutaian ninja Yuffie Kisaragi as she infiltrates Midgar and conspires with Avalanche HQ to steal the ultimate materia from the Shinra Electric Power Company.<br><br><span class=\"bb_img_ctn\"><img class=\"bb_img\" src=\"https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1462040/extras/ffce09d0945df423366b1d48c14527a4.avif?t=1773895755\" width=616 height=346 /></span><br><br>■Content included in this product<br><br>FINAL FANTASY VII REMAKE INTERGRADE contains the following content:<br>- FINAL FANTASY VII REMAKE full game<br>- DLC &quot;FF7R EPISODE INTERmission&quot; (New episode featuring Yuffie Kisaragi)<br>- Weapon: Cacstar<br>- Armor: Midgar Bangle<br>- Armor: Shinra Bangle<br>- Armor: Corneo Armlet<br>- Accessory: Superstar Belt<br>- Accessory: Mako Crystal<br>- Accessory: Seraphic Earrings<br>- Summon materia: Carbuncle<br>- Summon materia: Chocobo Chick<br>- Summon materia: Cactuar<br><br>*The additional weapons, armor, accessories and summon materia can be obtained from the Gift Box on the main menu.<br>*The weapon &quot;Cacstar&quot; can only be used in FF7R EPISODE INTERmission (Yuffie episode)",
            "about_the_game": "FINAL FANTASY VII REMAKE is a bold reimagining of the original FINAL FANTASY VII, originally released in 1997, developed under the guidance of the original key developers.<br>This critically-acclaimed game, which mixes traditional command-based combat and real-time action, makes its Steam debut along with FF7R EPISODE INTERmission─a new story arc featuring Yuffie Kisaragi.<br><br><span class=\"bb_img_ctn\"><img class=\"bb_img\" src=\"https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1462040/extras/4465042310caed881656fcae54bc3e4f.avif?t=1773895755\" width=616 height=346 /></span><br><br>■STORY<br><br>By exploiting mako, the life-blood of the planet, through their mako reactors, the Shinra Electric Power Company has all but seized control of the entire world. A ragtag group of idealists, known as Avalanche, are one of the last bastions of resistance.<br><br>Cloud, an elite SOLDIER operative-turned-mercenary takes part in an Avalanche operation to destroy Mako Reactor 1 in the city of Midgar. <br>The bombing plunges the city into fiery chaos, and Cloud is tormented by visions of a bitter enemy long thought dead. <br><br>Once more begins a story that will shape the destiny of an entire world.<br><br>* This game is a remake of FINAL FANTASY VII, first released in 1997.<br>The first title in a multi-part series, it is based on the story from the original game, up to the escape from Midgar, adding in new elements.<br><br><span class=\"bb_img_ctn\"><img class=\"bb_img\" src=\"https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1462040/extras/7b05c4dd523e17b5ae01d62c9bdf857c.avif?t=1773895755\" width=616 height=346 /></span><br><br>■FF7R EPISODE INTERmission (New episode featuring Yuffie Kisaragi)<br><br>FF7R EPISODE INTERmission is a brand-new adventure in the world of FINAL FANTASY VII REMAKE INTERGRADE. Play as Wutaian ninja Yuffie Kisaragi as she infiltrates Midgar and conspires with Avalanche HQ to steal the ultimate materia from the Shinra Electric Power Company.<br><br><span class=\"bb_img_ctn\"><img class=\"bb_img\" src=\"https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1462040/extras/ffce09d0945df423366b1d48c14527a4.avif?t=1773895755\" width=616 height=346 /></span><br><br>■Content included in this product<br><br>FINAL FANTASY VII REMAKE INTERGRADE contains the following content:<br>- FINAL FANTASY VII REMAKE full game<br>- DLC &quot;FF7R EPISODE INTERmission&quot; (New episode featuring Yuffie Kisaragi)<br>- Weapon: Cacstar<br>- Armor: Midgar Bangle<br>- Armor: Shinra Bangle<br>- Armor: Corneo Armlet<br>- Accessory: Superstar Belt<br>- Accessory: Mako Crystal<br>- Accessory: Seraphic Earrings<br>- Summon materia: Carbuncle<br>- Summon materia: Chocobo Chick<br>- Summon materia: Cactuar<br><br>*The additional weapons, armor, accessories and summon materia can be obtained from the Gift Box on the main menu.<br>*The weapon &quot;Cacstar&quot; can only be used in FF7R EPISODE INTERmission (Yuffie episode)",
            "short_description": "Cloud Strife, an ex-SOLDIER operative, descends on the mako-powered city of Midgar. The world of the timeless classic FINAL FANTASY VII is reborn, using cutting-edge graphics technology, a new battle system and an additional adventure featuring Yuffie Kisaragi.",
            "supported_languages": "English<strong>*</strong>, French<strong>*</strong>, Italian, German<strong>*</strong>, Spanish - Spain, Spanish - Latin America, Portuguese - Brazil, Simplified Chinese, Traditional Chinese, Japanese<strong>*</strong>, Korean<br><strong>*</strong>languages with full audio support",
            "header_image": "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1462040/header.jpg?t=1773895755",
            "capsule_image": "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1462040/capsule_231x87.jpg?t=1773895755",
            "capsule_imagev5": "https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/1462040/capsule_184x69.jpg?t=1773895755",
            "website": "https://ffvii-remake-intergrade.square-enix-games.com/en-us/",
            "pc_requirements": {
                "minimum": "<strong>Minimum:</strong><br><ul class=\"bb_ul\"><li>Requires a 64-bit processor and operating system<br></li><li><strong>OS:</strong> Windows® 10 64-bit (ver. 2004 or later)<br></li><li><strong>Processor:</strong> AMD FX-8350 / Intel® Core™ i5-3330<br></li><li><strong>Memory:</strong> 8 GB RAM<br></li><li><strong>Graphics:</strong> AMD Radeon™ RX 480 / NVIDIA® GeForce® GTX 780 / 3GB VRAM required<br></li><li><strong>DirectX:</strong> Version 12<br></li><li><strong>Storage:</strong> 100 GB available space<br></li><li><strong>Additional Notes:</strong> Assuming a Resolution of 1920 x 1080</li></ul>",
                "recommended": "<strong>Recommended:</strong><br><ul class=\"bb_ul\"><li>Requires a 64-bit processor and operating system<br></li><li><strong>OS:</strong> Windows® 10 64-bit (ver. 2004 or later)<br></li><li><strong>Processor:</strong> AMD Ryzen™ 3 3100 / Intel® Core™ i7-3770<br></li><li><strong>Memory:</strong> 12 GB RAM<br></li><li><strong>Graphics:</strong> AMD Radeon™ RX 5700 / NVIDIA® GeForce® GTX 1080 / 8GB VRAM required<br></li><li><strong>DirectX:</strong> Version 12<br></li><li><strong>Storage:</strong> 100 GB available space<br></li><li><strong>Additional Notes:</strong> Assuming a Resolution of 2560 x 1440 (This title supports max resolution of 3840 x 2160)</li></ul>"
            },
            "mac_requirements": {
                "minimum": "<strong>Minimum:</strong><br><ul class=\"bb_ul\"></ul>",
                "recommended": "<strong>Recommended:</strong><br><ul class=\"bb_ul\"></ul>"
            },
            "linux_requirements": {
                "minimum": "<strong>Minimum:</strong><br><ul class=\"bb_ul\"></ul>",
                "recommended": "<strong>Recommended:</strong><br><ul class=\"bb_ul\"></ul>"
            },
            "legal_notice": "© 1997, 2021 SQUARE ENIX CO., LTD. All Rights Reserved.<br />\r\nCHARACTER DESIGN:TETSUYA NOMURA/ROBERTO FERRARI<br />\r\nLOGO ILLUSTRATION:©1997 YOSHITAKA AMANO",
            "price_overview": {
                "currency": "USD",
                "initial": 3499,
                "final": 1224,
                "discount_percent": 65,
                "initial_formatted": "$34.99",
                "final_formatted": "$12.24 USD"
            },
            "categories": [
                {
                    "id": 2,
                    "description": "Single-player"
                },
                {
                    "id": 22,
                    "description": "Steam Achievements"
                },
                {
                    "id": 29,
                    "description": "Steam Trading Cards"
                },
                {
                    "id": 68,
                    "description": "Custom Volume Controls"
                },
                {
                    "id": 78,
                    "description": "Adjustable Difficulty"
                },
                {
                    "id": 75,
                    "description": "Keyboard Only Option"
                },
                {
                    "id": 69,
                    "description": "Stereo Sound"
                },
                {
                    "id": 70,
                    "description": "Surround Sound"
                },
                {
                    "id": 18,
                    "description": "Partial Controller Support"
                },
                {
                    "id": 23,
                    "description": "Steam Cloud"
                },
                {
                    "id": 62,
                    "description": "Family Sharing"
                }
            ],
            "genres": [
                {
                    "id": "1",
                    "description": "Action"
                },
                {
                    "id": "25",
                    "description": "Adventure"
                },
                {
                    "id": "3",
                    "description": "RPG"
                }
            ]
        }
    }
}
```