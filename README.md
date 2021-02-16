# rainfall 

It detects rainy weather and notifies your slack.

## How to use.

```
git clone git@github.com:katsusuke/rainfall.git
cd rainfall
cargo run -- -i YOUR_YAHOO_JAPAN_APPID -s xoxb-YOUR_SLACK_BOT_USER_OAUTH_ACCESS_TOKEN -c 139.7508397,35.6835749 -w
```

## -i: Yahoo! JAPAN APPID

Generate appid by following link.
https://e.developer.yahoo.co.jp/register

## -s SLACK_API_KEY

TODO:

API Test

```
cargo run -- -t -s xoxb-YOUR_SLACK_BOT_USER_OAUTH_ACCESS_TOKEN
```

Post a "Hello Slack" message to #weather channel .

## -c location of your house.

Separate the latitude and longitude with a `,`.

## -w watching

Start the process and check it every 10 minutes.
Once it detects rain, it will not notify you until 6 hours later.

