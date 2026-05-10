use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{ApiError, Error};
use crate::types::*;

pub struct Client {
    base_url: String,
    api_key: String,
    http: HttpClient,
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            base_url: "https://api.laddro.com".to_string(),
            api_key: api_key.into(),
            http: HttpClient::new(),
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into().trim_end_matches('/').to_string();
        self
    }

    pub async fn list_templates(&self) -> Result<Vec<Template>, Error> {
        #[derive(serde::Deserialize)]
        struct Resp { templates: Vec<Template> }
        let resp: Resp = self.get("/v1/templates").await?;
        Ok(resp.templates)
    }

    pub async fn get_template(&self, id: &str) -> Result<TemplateDetail, Error> {
        self.get(&format!("/v1/templates/{}", id)).await
    }

    pub async fn list_fonts(&self) -> Result<Vec<TemplateFont>, Error> {
        #[derive(serde::Deserialize)]
        struct Resp { fonts: Vec<TemplateFont> }
        let resp: Resp = self.get("/v1/fonts").await?;
        Ok(resp.fonts)
    }

    pub async fn list_languages(&self) -> Result<Vec<Language>, Error> {
        #[derive(serde::Deserialize)]
        struct Resp { languages: Vec<Language> }
        let resp: Resp = self.get("/v1/languages").await?;
        Ok(resp.languages)
    }

    pub async fn list_models(&self) -> Result<Vec<ModelProvider>, Error> {
        #[derive(serde::Deserialize)]
        struct Resp { models: Vec<ModelProvider> }
        let resp: Resp = self.get("/v1/models").await?;
        Ok(resp.models)
    }

    pub async fn list_resumes(&self, limit: u32, offset: u32) -> Result<PaginatedList<ResumeSummary>, Error> {
        let path = format!("/v1/resumes?limit={}&offset={}", limit, offset);
        self.get(&path).await
    }

    pub async fn get_resume(&self, id: &str) -> Result<ResumeSummary, Error> {
        self.get(&format!("/v1/resumes/{}", id)).await
    }

    pub async fn render_resume(&self, id: &str, opts: &RenderOptions) -> Result<Vec<u8>, Error> {
        self.post_binary(&format!("/v1/resumes/{}/render", id), opts).await
    }

    pub async fn tailor(&self, req: &TailorRequest) -> Result<Vec<u8>, Error> {
        self.post_binary("/v1/tailor", req).await
    }

    pub async fn export_pdf(&self, req: &ExportRequest) -> Result<Vec<u8>, Error> {
        self.post_binary("/v1/export", req).await
    }

    pub async fn list_cover_letters(&self, limit: u32, offset: u32) -> Result<PaginatedList<CoverLetterSummary>, Error> {
        let path = format!("/v1/cover-letters?limit={}&offset={}", limit, offset);
        self.get(&path).await
    }

    pub async fn get_cover_letter(&self, id: &str) -> Result<CoverLetterSummary, Error> {
        self.get(&format!("/v1/cover-letters/{}", id)).await
    }

    pub async fn create_cover_letter(&self, req: &CreateCoverLetterRequest) -> Result<CreateCoverLetterResponse, Error> {
        self.post_json("/v1/cover-letters", req).await
    }

    pub async fn generate_cover_letter(&self, req: &GenerateCoverLetterRequest) -> Result<Vec<u8>, Error> {
        self.post_binary("/v1/cover-letters/generate", req).await
    }

    pub async fn render_cover_letter(&self, id: &str, opts: &RenderOptions) -> Result<Vec<u8>, Error> {
        self.post_binary(&format!("/v1/cover-letters/{}/render", id), opts).await
    }

    pub async fn get_settings(&self) -> Result<SettingsResponse, Error> {
        self.get("/v1/settings").await
    }

    pub async fn update_ai_settings(&self, req: &UpdateAISettingsRequest) -> Result<SettingsResponse, Error> {
        self.put_json("/v1/settings/model", req).await
    }

    pub async fn delete_ai_settings(&self) -> Result<SettingsResponse, Error> {
        self.delete("/v1/settings/model").await
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let resp = self.http.get(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    async fn post_json<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T, Error> {
        let resp = self.http.post(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .json(body)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    async fn post_binary<B: Serialize>(&self, path: &str, body: &B) -> Result<Vec<u8>, Error> {
        let resp = self.http.post(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .json(body)
            .send()
            .await?;
        self.handle_binary_response(resp).await
    }

    async fn put_json<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T, Error> {
        let resp = self.http.put(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .json(body)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        let resp = self.http.delete(format!("{}{}", self.base_url, path))
            .header("x-api-key", &self.api_key)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    async fn handle_response<T: DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T, Error> {
        let status = resp.status().as_u16();
        if status >= 400 {
            return Err(self.parse_error(resp, status).await);
        }
        Ok(resp.json().await?)
    }

    async fn handle_binary_response(&self, resp: reqwest::Response) -> Result<Vec<u8>, Error> {
        let status = resp.status().as_u16();
        if status >= 400 {
            return Err(self.parse_error(resp, status).await);
        }
        Ok(resp.bytes().await?.to_vec())
    }

    async fn parse_error(&self, resp: reqwest::Response, status: u16) -> Error {
        #[derive(serde::Deserialize, Default)]
        struct ErrBody {
            error: Option<String>,
            code: Option<String>,
        }
        let body: ErrBody = resp.json().await.unwrap_or_default();
        Error::Api(ApiError {
            message: body.error.unwrap_or_else(|| "Unknown error".to_string()),
            status,
            code: body.code,
        })
    }
}
