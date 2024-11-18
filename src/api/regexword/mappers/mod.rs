use crate::api::regexword::regexword_dbo::BandeauDataDbo;
use crate::core::regexword::data::regexword_data::RegexWordData;

pub mod states;
pub mod events;

impl From<BandeauDataDbo> for RegexWordData {
    fn from(value: BandeauDataDbo) -> Self {
        Self {
            name: value.name,
            lien: value.lien,
            is_other_tab_link: value.is_other_tab_link,
            url_image: value.url_image,
            order: value.order,
        }
    }
}

impl From<RegexWordData> for BandeauDataDbo {
    fn from(value: RegexWordData) -> Self {
        Self {
            name: value.name,
            lien: value.lien,
            is_other_tab_link: value.is_other_tab_link,
            url_image: value.url_image,
            order: value.order,
        }
    }
}

