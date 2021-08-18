# Tenki(天気)

## How to use

1. Clone this repo.

2. Get your weather API key from <https://www.weatherapi.com>.

3. Rename `.env.template` to `.env`.

4. Replace `YourApiKey` with your actual weather API key.

5. And then run with `cargo run <city> -n[=<number>] -a` or `cargo run <city> --days[=<number>] -a`. Alternatively, you can also use `zip code` or possibly `IP address` instead of `city`, and it still should work.

    - For example,

    ```bash
    # The following will generate the forecast for the next 3 days including the current day,
    # and the current air quality information will not be given.
    cargo run seattle -n=3

    # This will generate the same information above as well as air quality information.
    cargo run new york --days=3 -a
    ```

# Updates

Now you can pass `-c` or `--cels` to show temperatures in Celsius.
