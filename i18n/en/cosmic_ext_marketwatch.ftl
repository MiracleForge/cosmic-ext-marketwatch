app-title = Cosmic Marketwatch
view = View
welcome = Welcome to COSMIC! ✨
page-id = Page { $num }
git-description = Git commit {$hash} on {$date}

overview-title = US Market Overview 

wallet-view = US Market Overview
wallet-name = Wallet name
wallet-updated-at = Updated at { $time }

display-loading = Loading...
display-error = Error 

assets-add = ADD ASSET
assets-limit = Asset limit reached ({ $max } max).
assets-search-by = Search by symbol (e.g. AAPL, PETR4)", search_input
assets-loading = Searching...
assets-not-found = No results found.

portfolio-empty = Your portfolio is empty.
portfolio-your = Your Portfolio

render-loading = Loading...

quotes-loading = Loading market data...

news-last = Latest News
news-colapse = Collapse ▲
news-view = View all ▼
news-not-avaliable = No news available at the moment.

screeners-tab-active = Most Active
screeners-tab-gainers = Gainers
screeners-tab-losers = Losers

alerts = Alerts
alerts-select = Selected Asset
alerts-select-stock = Select a stock from the list to create alerts.
alerts-condition-price-above = Price Above
alerts-condition-price-below = Price Below
alerts-condition-variation-above = Variation Above
alerts-condition-variation-below = Variation Below
alerts-condition-turns-positive = Turns Positive
alerts-condition-turns-negative = Turns Negative
alerts-condition = Condition
alerts-value-placeholder = value
alerts-value-add = Add
alerts-self = Your Alerts
alerts-not-found = No alerts yet.
alerts-label-price-above = Price > { $value }
alerts-label-price-below = Price < { $value }
alerts-label-variation-above = Variation > { $value }%
alerts-label-variation-below = Variation < { $value }%
alerts-label-turns-positive = Turns Positive
alerts-label-turns-negative = Turns Negative

alert-notification =
    { $symbol } — { $description }
    Price: { $price } | Change: { $change } %
alert-description-price-above = Price above { $value }
alert-description-price-below = Price below { $value }
alert-description-variation-above = Variation above { $value }%
alert-description-variation-below = Variation below { $value }%
alert-description-turned-positive = Turned positive
alert-description-turned-negative = Turned negative

settings-title = Configurations
settings-panel = Panel
settings-show-icon-only = Show icon only
settings-show-logos = Show logos in screeners
settings-disable-custom-alerts = Disable custom alerts
settings-display-news = Display news
settings-news-per-asset = News per asset (max 5)

settings-refresh-interval = Stock rotation interval (seconds)
settings-refresh = Refresh

settings-stock-rotation-interval = Stock rotation interval (seconds)
settings-refresh-interval = Refresh interval
settings-min = min

info-about = About
info-dev = Development

info-links = Links
info-repo = Repository
info-sup = Support
info-license-label = License
info-license = GPL-3.0-only
info-tip-kofi = Support development via Ko-fi
info-app-desc = Stock market monitor

time-just-now = just now
time-minutes-ago = { $count } min ago
time-hours-ago = { $count ->
    [one] { $count } hour ago
   *[other] { $count } hours ago
}
time-yesterday = yesterday
time-days-ago = { $count ->
    [one] { $count } day ago
   *[other] { $count } days ago
}
time-weeks-ago = { $count ->
    [one] { $count } week ago
   *[other] { $count } weeks ago
}
