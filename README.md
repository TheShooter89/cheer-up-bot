[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://stand-with-ukraine.pp.ua)

# CheerUp Telegram bot

Telegram bot to help coping with long-distance friendships nostalgia 😊️

Friends can upload many Telegram video note messages (aka bubble videos) with greetings, jokes, memes and so on, then when you feel lonely or you miss your friends from abroad you can open CheerUp bot and press a button to get a video note choosen randomly among all the video notes uploaded by your friends to see and hear them 😊️

This project has 4 "moving parts":

- `cheer-up`: this is the main "frontend" bot, that renders a "Cheer me up!" button to get a random video note
- `cheer-up-manager`: a manager bot friends can send video notes to, also used to handle the archived video notes (delete single or multiple notes, i.e.)
- `cheer-up-api`: a minimal REST api server to interact with a database persisting videonotes list and their metadata
- `cheer-up-core`: library to share common code between above parts of the project


## Usage/Examples

This project is handled as a `cargo workspace` with 4 crates corresponding to above mentioned moving parts

- `cheer-up`: binary crate - main bot using `teloxide`
- `cheer-up-manager`: binary crate - manager bot using `teloxide`
- `cheer-up-api`: binary crate - REST api server with SQLite database running locally on `PORT=3000`
- `cheer-up-core`: library crate - common code shared between crates (i.e. `cheer_up_core::utils`) 

#### ⚠️ TODO: update this section to document commands to spin up the entire app from project root
clone this repo and `cd` into it and use `cargo run` to run it

```bash
RUST_ENV=debug cargo run
```

alternatively, if on Linux, you can use `make` to run it

```bash
make install

make run
```


## License

[GPL-3.0](https://choosealicense.com/licenses/gpl-3.0/)


## Authors
written with rusty 💛️💙️ by Tanque

- [@TheShooter89](https://www.github.com/TheShooter89)


## `tanque` Stands With Ukraine 🇺🇦️

    "Freedom doesn't come cheap"

`tanque` stands with people of Ukraine in their fight against the brutal russian aggression and unrightful occupation of their homeland

`tanque` stands with people of Ukraine in their fight for **Freedom**, for **Peace**, for **Self-Determination**, for **Happiness**

`tanque` stands with this generation of young ukrainians robbed away of their youth by the war, who will have to find the strength to get up once again and rebuild from the rubbles

####

_By your side, for as long as it takes_ 💪️

        Slava Ukraini 🇺🇦️

### Donate

Please contribute and donate through official government channels or globally-know remarkable institutions:

- **UNITED24**: Institutinal fundraising, charity and media platform of Ukrainian Government. It's possible to donate for food, medicine, medical assistance, refugees support and more

    [U24 official site](https://u24.gov.ua/)

- **Medecins Sans Frontieres**: Life-saving medical assistance both in war and peace time, all over the world

    [MSF official site](https://www.msf.org/ukraine)

- **Protect A Volunteer**: Independent matching platform to support a Volunteer on the frontline
    
    [Protect a Volunteer site](https://protectavolunteer.com/)

Or use below badge:

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://stand-with-ukraine.pp.ua)

---

    humans die, but IDEAS are bulletproof
    🇺🇦️ ️🇪🇺️ 🏳️‍🌈️
