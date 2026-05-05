use reqwest::blocking::Client;

const BASE: &str = "https://pokeapi.co/api/v2";

fn client() -> Client {
    Client::new()
}

#[test]
fn recursos_basicos_bulbasaur() -> Result<(), Box<dyn std::error::Error>> {
    let poke = client()
        .get(format!("{BASE}/pokemon/bulbasaur"))
        .send()?
        .json::<serde_json::Value>()?;

    assert_eq!(poke["id"], 1);
    assert!(poke["base_experience"].as_i64().unwrap_or(0) > 0);
    assert!(!poke["abilities"].as_array().unwrap().is_empty());
    Ok(())
}

#[test]
fn paginacao_offset_e_correto() -> Result<(), Box<dyn std::error::Error>> {
    let page = client()
        .get(format!("{BASE}/pokemon?limit=20&offset=40"))
        .send()?
        .json::<serde_json::Value>()?;

    assert_eq!(page["results"].as_array().unwrap().len(), 20);
    assert!(page["previous"].as_str().unwrap_or("").contains("offset=20"));
    assert!(page["next"].as_str().unwrap_or("").contains("offset=60"));
    Ok(())
}

#[test]
fn recurso_aninhado_move_do_charizard() -> Result<(), Box<dyn std::error::Error>> {
    let charizard = client()
        .get(format!("{BASE}/pokemon/charizard"))
        .send()?
        .json::<serde_json::Value>()?;

    let move_url = charizard["moves"][0]["move"]["url"]
        .as_str()
        .expect("URL do move não encontrada");

    let move_data = client().get(move_url).send()?.json::<serde_json::Value>()?;

    for key in ["name", "pp", "type"] {
        assert!(!move_data[key].is_null(), "Chave '{key}' ausente no move");
    }
    Ok(())
}

#[test]
fn nao_encontrado_retorna_404() -> Result<(), Box<dyn std::error::Error>> {
    let r1 = client().get(format!("{BASE}/pokemon/pokemon-que-nao-existe")).send()?;
    assert_eq!(r1.status(), 404);

    let r2 = client().get(format!("{BASE}/berry/999999")).send()?;
    assert_eq!(r2.status(), 404);
    Ok(())
}

#[test]
fn dados_agregados_primeiros_5_pokemons() -> Result<(), Box<dyn std::error::Error>> {
    for id in 1..=5u32 {
        let p = client()
            .get(format!("{BASE}/pokemon/{id}"))
            .send()?
            .json::<serde_json::Value>()?;

        assert!(p["base_experience"].as_i64().unwrap_or(0) > 0, "base_experience inválido para id={id}");
        assert!(p["height"].as_i64().unwrap_or(0) > 0, "height inválido para id={id}");
    }
    Ok(())
}