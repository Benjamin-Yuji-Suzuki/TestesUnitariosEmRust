# Testes de Integração em Rust

Projeto prático desenvolvido como parte da disciplina de **Testes de Integração** (Nível de Unidade 2 — APIs, Contratos e Estratégias Modernas), por Isaac Souza Elgrably.

Implementa os três desafios práticos do curso usando Rust com `reqwest` e `cargo test`.

---

## 📝 Auditoria da IA

* **AI Assistance:** A lógica, a arquitetura e o desenvolvimento desta biblioteca foram construídos com a assistência de **Claude.ai** (Anthropic). Eles foram usados como ferramentas avançadas de programação em par para garantir um código Rust de alta qualidade, seguro e idiomático.

---

## Estrutura do Projeto

```
testes/
├── Cargo.toml
└── src/
    ├── lib.rs           # Declara os módulos
    ├── open_meteo.rs    # Desafio 1 — Open-Meteo Weather API
    ├── pokeapi.rs       # Desafio 2 — PokeAPI
    └── github_api.rs    # Desafio Avançado — GitHub REST API
```

---

## Desafios Implementados

### Desafio 1 — Open-Meteo (`open_meteo.rs`)

API pública de previsão do tempo, sem autenticação.

| Teste | O que valida |
|-------|-------------|
| `schema_valido` | GET com lat/lon de Belém/PA retorna as chaves `hourly`, `hourly_units`, `latitude`, `longitude`, `timezone` |
| `erro_sem_parametro_hourly` | Requisição sem `hourly` retorna resposta sem o campo de previsão |
| `temperature_2m_tem_168_itens` | `hourly.temperature_2m` tem 168 itens (7 dias × 24h), todos `float` ou `null` |
| `multiplas_variaveis_tem_unidades` | `temperature_2m`, `precipitation` e `windspeed_10m` têm unidades não vazias em `hourly_units` |
| `cache_segunda_requisicao_nao_e_muito_mais_lenta` | Segunda requisição idêntica é no máximo 2× mais lenta que a primeira |

### Desafio 2 — PokeAPI (`pokeapi.rs`)

API pública de Pokémon, sem autenticação. Rate limit de ~100 req/min.

| Teste | O que valida |
|-------|-------------|
| `recursos_basicos_bulbasaur` | `id == 1`, `base_experience > 0`, `abilities` com ao menos 1 item |
| `paginacao_offset_e_correto` | `results` tem 20 itens, `previous` aponta para `offset=20`, `next` para `offset=60` |
| `recurso_aninhado_move_do_charizard` | Extrai URL do primeiro move do Charizard e valida que tem `name`, `pp`, `type` |
| `nao_encontrado_retorna_404` | Pokémon e berry inexistentes retornam 404 |
| `dados_agregados_primeiros_5_pokemons` | IDs 1–5 têm `base_experience > 0` e `height` inteiro positivo |

### Desafio Avançado — GitHub REST API (`github_api.rs`)

Requer autenticação via Personal Access Token com scopes `repo` e `read:user`.

| Teste | O que valida |
|-------|-------------|
| `autenticacao_e_identidade` | `GET /user` retorna `login` e `email` válidos |
| `rate_limit_autenticado_e_5000` | Header `X-RateLimit-Limit` é 5000 para usuário autenticado |
| `busca_repositorios_python_mais_de_50k_stars` | Primeiro resultado da busca tem `stargazers_count > 50000` e `language == Python` |
| `criacao_e_exclusao_de_repositorio` | Cria repositório privado (201) e remove em seguida (204) |
| `paginacao_via_link_header` | Header `Link` contém `rel="next"` e a segunda página retorna 10 issues |

---

## Como Rodar Localmente

### Pré-requisitos

- [Rust](https://rustup.rs/) (edição estável)

### Todos os testes

```bash
cd testes
cargo test
```

### Apenas um módulo

```bash
cargo test open_meteo
cargo test pokeapi
cargo test github_api
```

### GitHub API — variável de ambiente obrigatória

Os testes do GitHub precisam de um Personal Access Token com os scopes `repo` e `read:user`:

```bash
export GH_TOKEN_PESSOAL=ghp_seuTokenAqui
cargo test github_api
```

Ou em linha única:

```bash
GH_TOKEN_PESSOAL=ghp_seuTokenAqui cargo test github_api
```

> **Onde gerar o token:** GitHub → Settings → Developer settings → Personal access tokens → Generate new token  
> Scopes necessários: `repo` (criar/deletar repositórios) e `read:user` (ler perfil)

---

## CI/CD — GitHub Actions

O workflow em `.github/workflow/integration.yml` roda automaticamente em todo push e pull request para `main`.

O token é injetado via secret do repositório. Para configurar:

1. Vá em **Settings → Secrets and variables → Actions**
2. Crie um secret chamado `GH_TOKEN_PESSOAL` com o valor do seu token

---

## Dependências

```toml
[dependencies]
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde_json = "1"
```

---

## Referências

- Martin Fowler — [Test Pyramid](https://martinfowler.com/bliki/TestPyramid.html)
- [Open-Meteo API](https://open-meteo.com)
- [PokeAPI](https://pokeapi.co)
- [GitHub REST API](https://docs.github.com/en/rest)
- Pressman, R. — *Engenharia de Software*, Cap. Testes