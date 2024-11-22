use chrono::{DateTime, Utc};
use framework_cqrs_lib::cqrs::core::context::Context;
use framework_cqrs_lib::cqrs::core::data::Entity;
use framework_cqrs_lib::cqrs::models::errors::ResultErr;
use framework_cqrs_lib::cqrs::models::views::entities::EntityView;
use crate::core::helpers::context::{ctx_is_after_datetime, give_date_time_with_hours};
use crate::core::regexword::data::states::RegexWordStates;
use crate::models::regexword::views::{RegexPartDisableView, RegexPartEnableView, RegexPartView, SelectedWordView, WordInfoView};


pub fn regexword_to_entity_hidden_view(entity: &Entity<RegexWordStates, String>, ctx: &Context) -> ResultErr<EntityView<SelectedWordView>> {

    let word = entity.data.get_word();

    from_state_to_view(&entity.data, ctx)
        .map(|regex_parts| {
            EntityView {
                r#type: "regexword-hidden".to_string(),
                id: entity.entity_id.clone(),
                links: None,
                attributes: SelectedWordView {
                    niveau_difficulte: "simple".to_string(), // cette donner partira
                    regex_parts,
                    word_info: WordInfoView {
                        first_letter: word.chars().take(1).collect::<String>().to_uppercase(),
                        size: word.len() as u32
                    }
                }
            }
        })
}

fn from_state_to_view(state: &RegexWordStates, ctx: &Context) -> ResultErr<Vec<RegexPartView>> {
    let hours = vec![
        give_date_time_with_hours(7, ctx)?,
        give_date_time_with_hours(11, ctx)?,
        give_date_time_with_hours(15, ctx)?,
    ]
        .into_iter()
        .map(|date_apparition| {
            let is_hide = ctx_is_after_datetime(&date_apparition, ctx);
            (is_hide, date_apparition)
        })
        .fold::<ResultErr<Vec<(bool, DateTime<Utc>)>>, _>(Ok(vec![]), |acc, current| {
            match current {
                (Ok(is_hide), date) => {
                    acc.map(|datas| {
                        let add = vec![(is_hide, date.clone())];
                        [datas.as_slice(), add.as_slice()].concat()
                    })
                },
                (Err(e), _) => {
                    Err(e)
                }
            }
        })?;

    let zipped: Vec<(String, (bool, DateTime<Utc>))> = state.get_regex_parts().into_iter().zip(hours.into_iter()).collect::<Vec<_>>();

    let res: Vec<RegexPartView> = zipped
        .into_iter()
        .map(|(regex, (is_hide, active_at))| {
            if is_hide {
                RegexPartView::Disabled(RegexPartDisableView { active_at })
            } else {
                RegexPartView::Enabled(RegexPartEnableView { regex })
            }
        })
        .collect::<Vec<_>>();

    Ok(res)
}