use bevy::prelude::*;

use crate::{
    currency::{Currency, PlayerCurrencies, setup_currency},
    state::GameState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Playing),
        spawn_currency_ui.after(setup_currency),
    );
    app.add_systems(
        Update,
        update_currency_ui
            .after(setup_currency)
            .run_if(in_state(GameState::Playing)),
    );
}

#[derive(Component)]
pub struct CurrencyUi;

#[derive(Component)]
pub struct CurrencyInfo(Currency);

pub fn update_currency_ui(
    currencies: Res<PlayerCurrencies>,
    mut texts: Query<(&mut Text, &CurrencyInfo)>,
) {
    for (currency, value) in currencies.currencies.iter() {
        let Some((mut text, _)) = texts.iter_mut().find(|(_, info)| info.0 == *currency) else {
            continue;
        };

        *text = Text::new(format!("{currency:?}: {value}"));
    }
}

pub fn spawn_currency_ui(mut commands: Commands, currencies: Res<PlayerCurrencies>) {
    commands
        .spawn((
            CurrencyUi,
            Name::new("Currency UI"),
            Node {
                position_type: PositionType::Absolute,
                top: px(0),
                right: px(0),
                width: px(150),
                height: px(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: px(12),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.07, 0.9)),
            DespawnOnExit(GameState::Playing),
        ))
        .with_children(|parent| {
            for (currency, amount) in currencies.currencies.iter() {
                parent.spawn((
                    Name::new(format!("{currency:?} currency info")),
                    Text::new(format!("{currency:?}: {amount}")),
                    TextFont::from_font_size(18.0),
                    TextColor(Color::WHITE),
                    CurrencyInfo(*currency),
                    Node {
                        bottom: px(15),
                        right: px(15),
                        ..default()
                    },
                ));
            }
        });
}
