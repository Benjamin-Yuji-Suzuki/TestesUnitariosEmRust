use reqwest::blocking::Client;
use reqwest::header;

const BASE: &str = "https://api.github.com";

fn client() -> Client {
    let token = std::env::var("GH_TOKEN_PESSOAL").expect("Defina GH_TOKEN_PESSOAL no ambiente");

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, format!("Bearer {token}").parse().unwrap());
    headers.insert(header::ACCEPT, "application/vnd.github+json".parse().unwrap());
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.insert(header::USER_AGENT, "rust-integration-test".parse().unwrap());

    Client::builder().default_headers(headers).build().unwrap()
}

fn login() -> String {
    client()
        .get(format!("{BASE}/user"))
        .send()
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap()["login"]
        .as_str()
        .unwrap()
        .to_string()
}

#[test]
fn autenticacao_e_identidade() -> Result<(), Box<dyn std::error::Error>> {
    let user = client().get(format!("{BASE}/user")).send()?.json::<serde_json::Value>()?;
    assert!(user["login"].is_string(), "'login' ausente");
    assert!(user["email"].is_string() || user["email"].is_null()); // email pode ser privado
    Ok(())
}

#[test]
fn rate_limit_autenticado_e_5000() -> Result<(), Box<dyn std::error::Error>> {
    let res = client().get(format!("{BASE}/rate_limit")).send()?;
    let limit: i64 = res.headers()["X-RateLimit-Limit"].to_str()?.parse()?;
    let remaining: i64 = res.headers()["X-RateLimit-Remaining"].to_str()?.parse()?;
    assert_eq!(limit, 5000);
    assert!(remaining >= 0);
    Ok(())
}

#[test]
fn criacao_e_exclusao_de_repositorio() -> Result<(), Box<dyn std::error::Error>> {
    let repo_name = "rust-integration-test-tmp";
    let login = login();

    let created = client()
        .post(format!("{BASE}/user/repos"))
        .json(&serde_json::json!({ "name": repo_name, "private": true, "auto_init": true }))
        .send()?;
    let created_status = created.status();
    let created_body = created.json::<serde_json::Value>().unwrap_or_default();
    assert_eq!(created_status, 201, "Criação falhou — body: {created_body}");

    let deleted = client()
        .delete(format!("{BASE}/repos/{login}/{repo_name}"))
        .send()?;
    assert_eq!(deleted.status(), 204, "Deleção falhou");
    Ok(())
}

#[test]
fn paginacao_via_link_header() -> Result<(), Box<dyn std::error::Error>> {
    let res = client()
        .get(format!("{BASE}/repos/microsoft/vscode/issues"))
        .query(&[("per_page", "10")])
        .send()?;

    let link = res.headers()["Link"].to_str()?;
    assert!(link.contains(r#"rel="next""#));

    let next_url = link
        .split(',')
        .find(|p| p.contains(r#"rel="next""#))
        .and_then(|p| p.split('<').nth(1))
        .and_then(|p| p.split('>').next())
        .expect("URL de next não encontrada");

    let page2 = client().get(next_url).send()?.json::<serde_json::Value>()?;
    assert_eq!(page2.as_array().unwrap().len(), 10);
    Ok(())
}