use crate::api::regexword::regexword_dbo::RegexWordDboState;
use crate::core::regexword::data::states::regexword_activate::RegexWordActivate;
use crate::core::regexword::data::states::regexword_disable::RegexWordDisable;
use crate::core::regexword::data::states::RegexWordStates;
use framework_cqrs_lib::cqrs::infra::repositories::mongo_entity_repository::CanTransform;


impl CanTransform<RegexWordStates> for RegexWordDboState {
    fn transform_into_other(&self) -> RegexWordStates {
        self.clone().into()
    }
}

impl CanTransform<RegexWordDboState> for RegexWordStates {
    fn transform_into_other(&self) -> RegexWordDboState {
        self.clone().into()
    }
}


impl From<RegexWordDboState> for RegexWordStates {
    fn from(value: RegexWordDboState) -> Self {
        match value {
            RegexWordDboState::RegexWordDisableDbo { kind, data } =>
                RegexWordStates::RegexWordDisable(
                    RegexWordDisable {
                        kind,
                        data: data.into(),
                    }
                ),
            RegexWordDboState::RegexWordActivateDbo { kind, data, date_activate } =>
                RegexWordStates::RegexWordActivate(
                    RegexWordActivate {
                        kind,
                        data: data.into(),
                        date_activate
                    }
                )
        }
    }
}

impl From<RegexWordStates> for RegexWordDboState {
    fn from(value: RegexWordStates) -> Self {
        match value {
            RegexWordStates::RegexWordActivate(data) => {
                RegexWordDboState::RegexWordActivateDbo {
                    kind: data.kind,
                    data: data.data.into(),
                    date_activate: data.date_activate
                }
            }
            RegexWordStates::RegexWordDisable(data) => {
                RegexWordDboState::RegexWordDisableDbo {
                    kind: data.kind,
                    data: data.data.into(),
                }
            }
        }
    }
}
