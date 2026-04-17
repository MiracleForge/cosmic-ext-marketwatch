fn clear_cache_for_current(&mut self) {
    let key = self.current_cache_key();
    self.last_fetch_time.remove(&key);
    self.cached_quotes.remove(&key);
    self.cached_news.remove(&key);
}

fn save_cache(&mut self) {
    let key = self.current_cache_key();
    self.last_fetch_time.insert(key.clone(), Instant::now());
    self.cached_quotes.insert(key.clone(), self.market_quotes.clone());
    self.cached_news.insert(key, self.news_items.clone());
}
