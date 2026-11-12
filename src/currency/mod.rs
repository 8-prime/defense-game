use bevy::prelude::*;
use std::collections::HashMap;

use bevy::ecs::resource::Resource;

use crate::state::GameState;

mod ui;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup_currency);
    ui::plugin(app);
}

pub fn setup_currency(mut commands: Commands) {
    commands.insert_resource(PlayerCurrencies::default());
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Currency {
    Red,
    Green,
    Blue,
}

#[derive(Resource, Clone)]
pub struct PlayerCurrencies {
    pub currencies: HashMap<Currency, u64>,
}

impl PlayerCurrencies {
    pub fn from(red: u64, green: u64, blue: u64) -> Self {
        let mut map = HashMap::new();

        map.insert(Currency::Red, red);
        map.insert(Currency::Green, green);
        map.insert(Currency::Blue, blue);

        return Self { currencies: map };
    }
}

pub struct Price {
    pub currency: Currency,
    pub amount: u64,
}

impl Default for PlayerCurrencies {
    fn default() -> Self {
        let mut currencies = HashMap::new();
        currencies.insert(Currency::Green, 10);
        Self {
            currencies: currencies,
        }
    }
}

impl PlayerCurrencies {
    pub fn try_update_balance(&mut self, price: &Price) -> bool {
        let Some(value) = self.currencies.get_mut(&price.currency) else {
            return false;
        };

        if *value < price.amount {
            return false;
        }

        *value = value.saturating_sub(price.amount);
        return true;
    }

    pub fn deposit(&mut self, player_currencies: &PlayerCurrencies) {
        for (currency, value) in self.currencies.iter_mut() {
            let Some(deposit_value) = player_currencies.currencies.get(currency) else {
                continue;
            };

            *value = value.saturating_add(*deposit_value);
        }
    }
}

pub fn purchase(player_currencies: &mut PlayerCurrencies, price: &Price) -> bool {
    return player_currencies.try_update_balance(price);
}
