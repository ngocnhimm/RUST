# StellarFinSight

## Problem

Traditional banking systems often store transaction data in centralized databases, making it difficult for users to transparently track their financial behavior, risk level, and wallet reliability.

## Solution

StellarFinSight is a simple financial analytics demo built on Stellar that records wallet inflow and outflow transactions, then generates a basic transaction report, risk alert count, and wallet credit score.

## Why Stellar

Stellar enables fast, low-cost, and transparent financial transactions on Testnet, making it suitable for building digital banking and financial analytics applications.

## Target User

This project is designed for students, fintech learners, banking data analysts, and developers who want to understand how blockchain data can be used for financial behavior analysis and wallet scoring.

## Main Features

* Record wallet inflow transactions
* Record wallet outflow transactions
* Track total inflow
* Track total outflow
* Count total transactions
* Detect unusually large transactions as risk alerts
* Generate a simple wallet credit score
* Reset wallet analytics data for demo testing

## Smart Contract Functions

| Function                       | Description                        |
| ------------------------------ | ---------------------------------- |
| `record_inflow(user, amount)`  | Records money received by a wallet |
| `record_outflow(user, amount)` | Records money spent by a wallet    |
| `get_report(user)`             | Returns wallet analytics report    |
| `reset_my_data(user)`          | Resets wallet data for testing     |

## Analytics Logic

The smart contract stores basic transaction metrics for each wallet:

* `total_inflow`: total amount received
* `total_outflow`: total amount spent
* `tx_count`: total number of recorded transactions
* `risk_count`: number of risky transactions detected
* `credit_score`: simple wallet reliability score from 0 to 100

A transaction is marked as risky when its amount is more than 3 times the wallet’s previous average transaction amount.

The wallet credit score starts from 100 and decreases based on:

* Outflow greater than inflow
* Low transaction count
* Number of risky transactions

## Demo Scenario

1. Reset wallet data.
2. Record an inflow transaction of `100`.
3. Record an outflow transaction of `30`.
4. View the wallet report.
5. Record a large outflow transaction of `1000`.
6. View the updated report and check the increased risk count and reduced credit score.

## Tech Stack

* Smart Contract: Rust / Soroban SDK v22
* Frontend: HTML / JavaScript / @stellar/stellar-sdk
* Wallet: Freighter
* Network: Stellar Testnet
* Explorer: Stellar Expert Testnet Explorer

## Member

* Hien | [https://www.linkedin.com/in/ngochienvothi/] | [ngochienvo05@gmail.com] | [HUTECH + 2026]

