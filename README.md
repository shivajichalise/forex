###### _Nepal Rastra Bank Foreign Exchange Rates_

# Forex

A simple console program that taps into the Nepal Rastra Bank's Foreign Exchange Rates API, providing real-time data on currency buy and sell rates directly to the console.
Say NO! to googling 1 USD to NPR every one and then 😅.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Example](#example)
5. [Self-Promotion](#self-promotion)

## Prerequisites

Before you begin, make sure you have the cargo installed in your system.

## Installation

To get started follow below steps:

1. Clone this repository to your local machine:

   ```bash
   git clone git@github.com:shivajichalise/forex.git
   cd forex
   ```

2. Build the program:

   ```bash
   cargo build
   ```

3. Copy the program to a location in your $PATH:

   ```bash
   cp  target/debug/forex ~/usr/local/bin/forex
   ```

## Usage

```bash
forex -f <iso3 currency code> -a <amount>
```

OR

```bash
forex --from <iso3 currency code> --amount <amount>
```

OR

```bash
forex --from <iso3 currency code> // by default amount is set to 1
```

## Example

```bash
forex -f usd
```

```bash
forex -f aud -a 50
```

Available currencies are:

- Indian Rupee (INR)
- U.S. Dollar (USD)
- European Euro (EUR)
- UK Pound Sterling (GBP)
- Swiss Franc (CHF)
- Australian Dollar (AUD)
- Canadian Dollar (CAD)
- Singapore Dollar (SGD)
- Japanese Yen (JPY)
- Chinese Yuan (CNY)
- Saudi Arabian Riyal (SAR)
- Qatari Riyal (QAR)
- Thai Baht (THB)
- UAE Dirham (AED)
- Malaysian Ringgit (MYR)
- South Korean Won (KRW)
- Swedish Kroner (SEK)
- Danish Kroner (DKK)
- Hong Kong Dollar (HKD)
- Kuwaity Dinar (KWD)
- Bahrain Dinar (BHD)

## Self-Promotion

Star the repository on [Github](https://github.com/shivajichalise/forex)

Follow on [Github](https://github.com/shivajichalise)

