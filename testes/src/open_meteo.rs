use reqwest::blocking::Client;
use std::time::Instant;

const BASE: &str = "https://api.open-meteo.com/v1/forecast";

fn client() -> Client {
    Client::new()
}

#[test]
fn schema_valido() -> Result<(), Box<dyn std::error::Error>> {
    let res = client()
        .get(BASE)
        .query(&[("latitude", "-1.46"), ("longitude", "-48.50"), ("hourly", "temperature_2m")])
        .send()?
        .json::<serde_json::Value>()?;

    for chave in ["hourly", "hourly_units", "latitude", "longitude", "timezone"] {
        assert!(!res[chave].is_null(), "Chave '{chave}' ausente");
    }
    Ok(())
}

#[test]
fn erro_sem_parametro_hourly() -> Result<(), Box<dyn std::error::Error>> {
    let res = client()
        .get(BASE)
        .query(&[("latitude", "-1.46"), ("longitude", "-48.50")])
        .send()?;

    let body = res.json::<serde_json::Value>()?;
    // A API retorna 200 mas sem o campo "hourly" quando o parâmetro é omitido
    assert!(
        body["hourly"].is_null(),
        "Sem o parâmetro 'hourly', o campo 'hourly' não deve estar presente. body={body}"
    );
    Ok(())
}

#[test]
fn temperature_2m_tem_168_itens() -> Result<(), Box<dyn std::error::Error>> {
    let res = client()
        .get(BASE)
        .query(&[("latitude", "-1.46"), ("longitude", "-48.50"), ("hourly", "temperature_2m")])
        .send()?
        .json::<serde_json::Value>()?;

    let arr = res["hourly"]["temperature_2m"].as_array()
        .expect("temperature_2m deve ser array");

    assert_eq!(arr.len(), 168, "Esperava 168 itens (7 dias x 24h)");
    assert!(arr.iter().all(|v| v.is_number() || v.is_null()));
    Ok(())
}

#[test]
fn multiplas_variaveis_tem_unidades() -> Result<(), Box<dyn std::error::Error>> {
    let res = client()
        .get(BASE)
        .query(&[
            ("latitude", "-1.46"),
            ("longitude", "-48.50"),
            ("hourly", "temperature_2m,precipitation,windspeed_10m"),
        ])
        .send()?
        .json::<serde_json::Value>()?;

    for key in ["temperature_2m", "precipitation", "windspeed_10m"] {
        let unit = res["hourly_units"][key].as_str().unwrap_or("");
        assert!(!unit.is_empty(), "Unidade vazia para {key}");
    }
    Ok(())
}

#[test]
fn cache_segunda_requisicao_nao_e_muito_mais_lenta() -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{BASE}?latitude=-1.46&longitude=-48.50&hourly=temperature_2m");

    let t1 = Instant::now();
    client().get(&url).send()?;
    let d1 = t1.elapsed();

    let t2 = Instant::now();
    client().get(&url).send()?;
    let d2 = t2.elapsed();

    assert!(d2 <= d1 * 2, "2ª req ({d2:?}) muito mais lenta que 1ª ({d1:?})");
    Ok(())
}