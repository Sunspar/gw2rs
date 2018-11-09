- Go through the existing endpoints / structs and make sure any identifiers are properly newtyped
- Standardize the trait implementations on structs exposed by the library

Endpoints as presented from https://api.guildwars2.com/v2
- [ ] /v2/account [a]
- [ ] /v2/account/achievements [a]
- [ ] /v2/account/bank [a]
- [ ] /v2/account/dungeons [a]
- [ ] /v2/account/dyes [a]
- [ ] /v2/account/finishers [a]
- [ ] /v2/account/gliders [a]
- [ ] /v2/account/home/cats [a]
- [ ] /v2/account/home/nodes [a]
- [ ] /v2/account/inventory [a]
- [ ] /v2/account/mail [d,a]
- [ ] /v2/account/mailcarriers [a]
- [ ] /v2/account/masteries [a]
- [ ] /v2/account/mastery/points [a]
- [ ] /v2/account/materials [a]
- [ ] /v2/account/minis [a]
- [ ] /v2/account/outfits [a]
- [ ] /v2/account/pvp/heroes [a]
- [ ] /v2/account/raids [a]
- [ ] /v2/account/recipes [a]
- [ ] /v2/account/skins [a]
- [ ] /v2/account/titles [a]
- [ ] /v2/account/wallet [a]
- [x] /v2/achievements [l]
- [X] /v2/achievements/categories [l]
- [x] /v2/achievements/daily
- [x] /v2/achievements/daily/tomorrow
- [X] /v2/achievements/groups [l]
- [-] /v2/adventures [l,d]
- [-] /v2/adventures/:id/leaderboards [d]
- [-] /v2/adventures/:id/leaderboards/:board/:region [d]
- [x] /v2/backstory/answers [l]
- [x] /v2/backstory/questions [l]
- [x] /v2/build
- [x] /v2/cats
- [ ] /v2/characters [a]
- [ ] /v2/characters/:id/backstory [a]
- [ ] /v2/characters/:id/core [a]
- [ ] /v2/characters/:id/crafting [a]
- [-] /v2/characters/:id/dungeons [d,a]
- [ ] /v2/characters/:id/equipment [a]
- [ ] /v2/characters/:id/heropoints [a]
- [ ] /v2/characters/:id/inventory [a]
- [ ] /v2/characters/:id/recipes [a]
- [ ] /v2/characters/:id/sab [a]
- [ ] /v2/characters/:id/skills [a]
- [ ] /v2/characters/:id/specializations [a]
- [ ] /v2/characters/:id/training [a]
- [x] /v2/colors [l]
- [x] /v2/commerce/delivery [a]
- [x] /v2/commerce/exchange
- [x] /v2/commerce/listings
- [x] /v2/commerce/prices
- [x] /v2/commerce/transactions [a]
- [ ] /v2/continents [l]
- [x] /v2/currencies [l]
- [ ] /v2/dungeons [l]
- [ ] /v2/emblem
- [-] /v2/events [l,d]
- [-] /v2/events-state [d]
- [ ] /v2/files
- [ ] /v2/finishers [l]
- [-] /v2/gemstore/catalog [l,d]
- [x] /v2/gliders [l]
- [ ] /v2/guild/:id [a]
- [ ] /v2/guild/:id/log [a]
- [ ] /v2/guild/:id/members [a]
- [ ] /v2/guild/:id/ranks [a]
- [ ] /v2/guild/:id/stash [a]
- [ ] /v2/guild/:id/storage [a]
- [ ] /v2/guild/:id/teams [a]
- [ ] /v2/guild/:id/treasury [a]
- [ ] /v2/guild/:id/upgrades [a]
- [ ] /v2/guild/permissions [l]
- [ ] /v2/guild/search
- [ ] /v2/guild/upgrades [l]
- [ ] /v2/items [l]
- [ ] /v2/itemstats [l]
- [x] /v2/legends
- [ ] /v2/mailcarriers [l]
- [ ] /v2/maps [l]
- [ ] /v2/masteries [l]
- [ ] /v2/materials [l]
- [ ] /v2/minis [l]
- [ ] /v2/nodes
- [ ] /v2/outfits [l]
- [ ] /v2/pets [l]
- [ ] /v2/professions [l]
- [ ] /v2/pvp
- [ ] /v2/pvp/amulets [l]
- [ ] /v2/pvp/games [a]
- [ ] /v2/pvp/heroes [l]
- [ ] /v2/pvp/ranks [l]
- [-] /v2/pvp/rewardtracks [l,d]
- [-] /v2/pvp/runes [l,d]
- [ ] /v2/pvp/seasons [l]
- [ ] /v2/pvp/seasons/:id/leaderboards
- [ ] /v2/pvp/seasons/:id/leaderboards/:board/:region
- [-] /v2/pvp/sigils [l,d]
- [ ] /v2/pvp/standings [a]
- [ ] /v2/pvp/stats [a]
- [ ] /v2/quaggans
- [ ] /v2/races [l]
- [ ] /v2/raids [l]
- [ ] /v2/recipes
- [ ] /v2/recipes/search
- [ ] /v2/skills [l]
- [ ] /v2/skins [l]
- [x] /v2/specializations [l]
- [x] /v2/stories [l]
- [x] /v2/stories/seasons [l]
- [x] /v2/titles [l]
- [X] /v2/tokeninfo [a]
- [x] /v2/traits [l]
- [-] /v2/vendors [l,d]
- [x] /v2/worlds [l]
- [x] /v2/wvw/abilities [l]
- [x] /v2/wvw/matches
- [ ] /v2/wvw/matches/overview
- [ ] /v2/wvw/matches/scores
- [ ] /v2/wvw/matches/stats
- [ ] /v2/wvw/matches/stats/:id/guilds/:guild_id
- [ ] /v2/wvw/matches/stats/:id/teams/:team/top/kdr
- [ ] /v2/wvw/matches/stats/:id/teams/:team/top/kills
- [x] /v2/wvw/objectives [l]
- [x] /v2/wvw/ranks [l]
- [-] /v2/wvw/rewardtracks [l,d]
- [x] /v2/wvw/upgrades [l]