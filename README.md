# rainfall 

It detects rainy weather and notifies your slack.

## How to use.

```
git clone git@github.com:katsusuke/rainfall.git
cd rainfall
cargo run -- -i YOUR_YAHOO_JAPAN_APPID -s SLACK_API_KEY -c 35.6835749,139.7508397 -w
```

## -i: Yahoo! JAPAN APPID

Generate appid by following link.
https://e.developer.yahoo.co.jp/register

## -s SLACK_API_KEY

TODO:


## -c location of your house.

Separate the latitude and longitude with a `,`.

## -w watching

Start the process and check it every 10 minutes.
Once it detects rain, it will not notify you until 6 hours later.

