#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct WalletMetrics {
    pub total_inflow: u128,
    pub total_outflow: u128,
    pub tx_count: u32,
    pub risk_count: u32,
    pub credit_score: u32,
}

#[contracttype]
pub enum DataKey {
    Metrics(Address),
}

#[contract]
pub struct BankingAnalyticsContract;

#[contractimpl]
impl BankingAnalyticsContract {
    pub fn record_inflow(env: Env, user: Address, amount: u128) -> WalletMetrics {
        user.require_auth();

        let mut metrics = Self::get_or_create_metrics(&env, &user);

        if Self::is_risky_transaction(&metrics, amount) {
            metrics.risk_count += 1;
        }

        metrics.total_inflow += amount;
        metrics.tx_count += 1;
        metrics.credit_score = Self::calculate_score(&metrics);

        env.storage()
            .persistent()
            .set(&DataKey::Metrics(user), &metrics);

        metrics
    }

    pub fn record_outflow(env: Env, user: Address, amount: u128) -> WalletMetrics {
        user.require_auth();

        let mut metrics = Self::get_or_create_metrics(&env, &user);

        if Self::is_risky_transaction(&metrics, amount) {
            metrics.risk_count += 1;
        }

        metrics.total_outflow += amount;
        metrics.tx_count += 1;
        metrics.credit_score = Self::calculate_score(&metrics);

        env.storage()
            .persistent()
            .set(&DataKey::Metrics(user), &metrics);

        metrics
    }

    pub fn get_report(env: Env, user: Address) -> WalletMetrics {
        Self::get_or_create_metrics(&env, &user)
    }

    pub fn reset_my_data(env: Env, user: Address) -> WalletMetrics {
        user.require_auth();

        let metrics = WalletMetrics {
            total_inflow: 0,
            total_outflow: 0,
            tx_count: 0,
            risk_count: 0,
            credit_score: 50,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Metrics(user), &metrics);

        metrics
    }

    fn get_or_create_metrics(env: &Env, user: &Address) -> WalletMetrics {
        env.storage()
            .persistent()
            .get(&DataKey::Metrics(user.clone()))
            .unwrap_or(WalletMetrics {
                total_inflow: 0,
                total_outflow: 0,
                tx_count: 0,
                risk_count: 0,
                credit_score: 50,
            })
    }

    fn is_risky_transaction(metrics: &WalletMetrics, amount: u128) -> bool {
        if metrics.tx_count == 0 {
            return false;
        }

        let total_volume = metrics.total_inflow + metrics.total_outflow;
        let average_amount = total_volume / metrics.tx_count as u128;

        amount > average_amount * 3
    }

    fn calculate_score(metrics: &WalletMetrics) -> u32 {
        let mut score: u32 = 100;

        if metrics.total_outflow > metrics.total_inflow {
            score = score.saturating_sub(25);
        }

        if metrics.tx_count < 3 {
            score = score.saturating_sub(15);
        }

        let risk_penalty = metrics.risk_count.saturating_mul(20);
        score = score.saturating_sub(risk_penalty);

        if metrics.tx_count >= 10 && metrics.risk_count == 0 {
            score = (score + 5).min(100);
        }

        score
    }
}