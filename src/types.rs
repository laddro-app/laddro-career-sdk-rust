use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct ResumeSummary {
    pub id: String,
    #[serde(rename = "resumeId")]
    pub resume_id: String,
    pub title: String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedList<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    #[serde(rename = "atsScore")]
    pub ats_score: u32,
    #[serde(rename = "layoutType")]
    pub layout_type: String,
    #[serde(rename = "supportsProfileImage")]
    pub supports_profile_image: bool,
    pub defaults: TemplateDefaults,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateDefaults {
    #[serde(rename = "pageSize")]
    pub page_size: String,
    pub spacing: u32,
    #[serde(rename = "fontSize")]
    pub font_size: u32,
    pub font: String,
    #[serde(rename = "pageNumbering")]
    pub page_numbering: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateColor {
    pub id: String,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    #[serde(rename = "backgroundPartColor")]
    pub background_part_color: Option<String>,
    #[serde(rename = "underlineColor")]
    pub underline_color: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "textMuted")]
    pub text_muted: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateFont {
    pub family: String,
    pub label: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateDetail {
    #[serde(flatten)]
    pub template: Template,
    #[serde(rename = "availableColors")]
    pub available_colors: Vec<TemplateColor>,
    #[serde(rename = "availableFonts")]
    pub available_fonts: Vec<TemplateFont>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub recommended: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelProvider {
    pub provider: String,
    pub name: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub models: Vec<Model>,
    #[serde(rename = "keyPrefix")]
    pub key_prefix: String,
    #[serde(rename = "docsUrl")]
    pub docs_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Language {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RenderOptions {
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "colorId", skip_serializing_if = "Option::is_none")]
    pub color_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,
    #[serde(rename = "fontSize", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    #[serde(rename = "pageNumbering", skip_serializing_if = "Option::is_none")]
    pub page_numbering: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TailorRequest {
    #[serde(rename = "resumeId", skip_serializing_if = "Option::is_none")]
    pub resume_id: Option<String>,
    #[serde(rename = "positionName")]
    pub position_name: String,
    #[serde(rename = "jobDescription", skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    #[serde(rename = "jobUrl", skip_serializing_if = "Option::is_none")]
    pub job_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "includeCoverLetter", skip_serializing_if = "Option::is_none")]
    pub include_cover_letter: Option<bool>,
    #[serde(rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "colorId", skip_serializing_if = "Option::is_none")]
    pub color_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CoverLetterSummary {
    pub id: String,
    #[serde(rename = "coverLetterId")]
    pub cover_letter_id: String,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateCoverLetterRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "jobTitle", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "companyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "hiringManager", skip_serializing_if = "Option::is_none")]
    pub hiring_manager: Option<String>,
    #[serde(rename = "letterContent")]
    pub letter_content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateCoverLetterResponse {
    #[serde(rename = "coverLetterId")]
    pub cover_letter_id: String,
    pub title: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GenerateCoverLetterRequest {
    #[serde(rename = "resumeId", skip_serializing_if = "Option::is_none")]
    pub resume_id: Option<String>,
    #[serde(rename = "positionName")]
    pub position_name: String,
    #[serde(rename = "jobDescription", skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    #[serde(rename = "jobUrl", skip_serializing_if = "Option::is_none")]
    pub job_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportRequest {
    #[serde(rename = "resumeId")]
    pub resume_id: String,
    #[serde(rename = "templateId", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "colorId", skip_serializing_if = "Option::is_none")]
    pub color_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,
    #[serde(rename = "fontSize", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    #[serde(rename = "pageNumbering", skip_serializing_if = "Option::is_none")]
    pub page_numbering: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AISettings {
    pub provider: String,
    pub model: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "hasApiKey")]
    pub has_api_key: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SettingsResponse {
    pub ai: Option<AISettings>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateAISettingsRequest {
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "apiKey")]
    pub api_key: String,
}
