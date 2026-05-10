use laddro_career::*;

fn api_key() -> String {
    std::env::var("LADDRO_API_KEY").expect("Set LADDRO_API_KEY")
}

#[tokio::test]
async fn test_list_templates() {
    let client = Client::new("");
    let templates = client.list_templates().await.unwrap();
    assert_eq!(templates.len(), 22);
}

#[tokio::test]
async fn test_get_template() {
    let client = Client::new("");
    let detail = client.get_template("GRAPHITE").await.unwrap();
    assert_eq!(detail.template.id, "GRAPHITE");
    assert!(!detail.available_colors.is_empty());
}

#[tokio::test]
async fn test_list_fonts() {
    let client = Client::new("");
    let fonts = client.list_fonts().await.unwrap();
    assert_eq!(fonts.len(), 21);
}

#[tokio::test]
async fn test_list_languages() {
    let client = Client::new("");
    let langs = client.list_languages().await.unwrap();
    assert_eq!(langs.len(), 14);
}

#[tokio::test]
async fn test_list_models() {
    let client = Client::new("");
    let models = client.list_models().await.unwrap();
    assert_eq!(models.len(), 10);
}

#[tokio::test]
async fn test_list_resumes() {
    let client = Client::new(api_key());
    let list = client.list_resumes(5, 0).await.unwrap();
    assert!(!list.items.is_empty());
}

#[tokio::test]
async fn test_get_resume() {
    let client = Client::new(api_key());
    let list = client.list_resumes(1, 0).await.unwrap();
    let id = &list.items[0].resume_id;
    let resume = client.get_resume(id).await.unwrap();
    assert_eq!(&resume.resume_id, id);
}

#[tokio::test]
async fn test_render_resume() {
    let client = Client::new(api_key());
    let list = client.list_resumes(1, 0).await.unwrap();
    let id = &list.items[0].resume_id;
    let pdf = client.render_resume(id, &RenderOptions {
        template_id: "GRAPHITE".to_string(),
        locale: None, color_id: None, font: None,
        spacing: None, margin: None, font_size: None, page_numbering: None,
    }).await.unwrap();
    assert!(pdf.len() > 1000);
}

#[tokio::test]
async fn test_export_pdf() {
    let client = Client::new(api_key());
    let list = client.list_resumes(1, 0).await.unwrap();
    let id = &list.items[0].resume_id;
    let pdf = client.export_pdf(&ExportRequest {
        resume_id: id.clone(),
        template_id: Some("COBALT".to_string()),
        locale: None, color_id: None, font: None,
        spacing: None, margin: None, font_size: None, page_numbering: None,
    }).await.unwrap();
    assert!(pdf.len() > 1000);
}

#[tokio::test]
async fn test_get_settings() {
    let client = Client::new(api_key());
    let _s = client.get_settings().await.unwrap();
}

#[tokio::test]
async fn test_delete_settings() {
    let client = Client::new(api_key());
    let s = client.delete_ai_settings().await.unwrap();
    assert!(s.ai.is_none());
}

#[tokio::test]
async fn test_list_cover_letters() {
    let client = Client::new(api_key());
    let _list = client.list_cover_letters(5, 0).await.unwrap();
}

#[tokio::test]
async fn test_create_cover_letter() {
    let client = Client::new(api_key());
    let resp = client.create_cover_letter(&CreateCoverLetterRequest {
        full_name: "Rust Test".to_string(),
        letter_content: "<p>From Rust SDK.</p>".to_string(),
        title: None, job_title: None, address: None, email: None,
        phone: None, company_name: None, hiring_manager: None,
    }).await.unwrap();
    assert!(!resp.cover_letter_id.is_empty());
}

#[tokio::test]
async fn test_auth_error() {
    let client = Client::new("laddro_live_invalid");
    let err = client.list_resumes(1, 0).await.unwrap_err();
    match err {
        Error::Api(e) => assert!(e.is_auth_error()),
        _ => panic!("expected API error"),
    }
}

#[tokio::test]
async fn test_not_found() {
    let client = Client::new(api_key());
    let err = client.get_resume("00000000-0000-0000-0000-000000000000").await.unwrap_err();
    match err {
        Error::Api(e) => assert!(e.is_not_found()),
        _ => panic!("expected API error"),
    }
}
