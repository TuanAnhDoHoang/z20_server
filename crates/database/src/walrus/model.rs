use getset::{Getters, MutGetters, Setters};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Getters, Setters, MutGetters)]
// Tạo các hàm pub name(), pub set_name(), pub name_mut()
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[validate(length(min = 1))]
    user_id: String, // Đã bỏ pub

    #[validate(length(min = 2))]
    name: String,

    #[validate(length(min = 10))]
    idea_description: String,

    guided_questions: Vec<Questions>,

    prototype_code: String,

    project_files: Vec<ProjectFile>,

    quilt: Quilt,

    site: Site,

    created_at: DateTime,

    updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Getters, Setters, MutGetters)]
// Tạo các hàm pub name(), pub set_name(), pub name_mut()
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Quilt{
    id: String,
    identifier: String,
    file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Getters, Setters, MutGetters)]
// Tạo các hàm pub name(), pub set_name(), pub name_mut()
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Site{
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default, Getters, Setters, MutGetters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[serde(rename_all = "camelCase")]
pub struct Questions {
    #[validate(length(min = 1))]
    question: String,
    answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default, Getters, Setters, MutGetters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[serde(rename_all = "camelCase")]
pub struct ProjectFile {
    #[validate(length(min = 1))]
    filename: String,
    content: Vec<u8>,
    path: String,
}
