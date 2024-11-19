use crate::api::regexword::regexword_dbo::RegexWordDboState;
use crate::core::regexword::data::states::regexword::RegexWord;
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
            RegexWordDboState::RegexWordDbo { kind, data  } =>
                RegexWordStates::RegexWord(
                    RegexWord {
                        kind,
                        data: data.into(),
                    }
                )
        }
    }
}

impl From<RegexWordStates> for RegexWordDboState {
    fn from(value: RegexWordStates) -> Self {
        match value {
            RegexWordStates::RegexWord(data) => {
                RegexWordDboState::RegexWordDbo {
                    kind: data.kind,
                    data: data.data.into(),
                }
            }
        }
    }
}
