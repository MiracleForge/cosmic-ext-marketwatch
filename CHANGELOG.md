# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.0] - 2026-04-22

### Added

- **Company logos:** market items now display company logos for better visual identification
- **Logo visibility toggle:** added option in settings to enable or disable logos

### Changed

- Improved visual layout to accommodate logo display
- Minor UI spacing adjustments for better alignment

## [1.2.0] - 2026-04-18

### Added

- **Market screens:** added support for multiple market views:
  - Most Active
  - Gainers
  - Losers
- **Tab-based navigation:** users can now switch between different market screens directly from the UI

### Changed

- **Caching system redesign:** market data is now cached per screen and per wallet, avoiding unnecessary requests.
- **Treeding:** renamed for Market Overview

### UI Improvements

- **Section Settings and Alerts section headers refined:**
  - improved typography (slightly larger size and semibold weight)
  - now use system accent color for better visual hierarchy
  - Refresh Buttons with spacing 
- **General visual polish:**
  - improved spacing and separation between sections

## [1.0.1] - 2026-03-30

### Changed

This release introduces an important update to the project identity and user experience:

- **Trademark compliance:** the extension has been renamed to align with trademark requirements, ensuring consistency and proper branding going forward.
- **Applet display fix:** resolved the issue where the Marketwatch applet’s interface was being cut off on certain screen sizes.
- **Visual refinements:** improved layout consistency within the COSMIC environment.
- **Stable foundation:** establishes a cleaner base for future updates and enhancements.

---

## [1.0.0] - 2026-03-20

### Added

- Real-time market data from Yahoo Finance (no API key required)
- Current price and percentage variation displayed in panel
- Automatic rotation between multiple assets with configurable interval
- Up to 10 custom wallets with up to 10 assets each
- Price alerts: above, below, variation thresholds, turns positive/negative
- Desktop notifications when alert conditions are met
- Latest news per asset from Yahoo Finance (configurable count per asset)
- Support for multiple currencies: USD, BRL, EUR, GBP, JPY, CHF, CAD, AUD, CNY, INR
- Configurable refresh interval (5, 10, 15, 30 minutes or 1 hour)
- Configurable stock rotation speed
- Horizontal and vertical panel support
- Icon-only mode
- Persistent wallet and alert configuration
- Applet icon that adapts to panel size and system theme
- Scrollable popup for small screens
- Standardized layout constants for consistent spacing and typography
