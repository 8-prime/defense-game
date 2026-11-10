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

#[derive(Resource)]
pub struct PlayerCurrencies {
    pub currencies: HashMap<Currency, u64>,
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
}

pub fn purchase(player_currencies: &mut PlayerCurrencies, price: &Price) -> bool {
    return player_currencies.try_update_balance(price);
}

pub fn setup_currency_ui(mut commands: Commands) {}
