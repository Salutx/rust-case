# 📈 **Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore (RUST)**<br/>

Este projeto implementa um sistema de busca eficiente e escalável para o catálogo de produtos da MegaStore. Utilizando a linguagem Rust, a solução busca oferecer desempenho e precisão mesmo com um grande volume de dados.

**Algoritmos e Estruturas de Dados Utilizados**<br/>
A implementação do sistema de busca para o catálogo de produtos da "MegaStore" foi realizada com foco em garantir eficiência, escalabilidade e precisão. Para isso, adotou-se uma estratégia centrada na utilização de tabelas hash (HashMaps em Rust) como principal estrutura de dados, complementada por técnicas de pré-processamento textual. As principais tecnologias utilizadas foram as crates regex, serde, serde_json e unicode-normalization, fundamentais para o processamento e a normalização dos dados.

**Tabelas Hash (HashMap)**<br/>
A estrutura central do sistema é o HashMap da biblioteca padrão do Rust, que foi utilizada para indexar rapidamente os produtos com base em palavras-chave extraídas de seus atributos (nome, marca, categoria, descrição, etc.). Cada entrada do HashMap associa um termo de busca normalizado a uma lista de identificadores de produtos correspondentes. Essa abordagem permitiu: <br/>
- Acesso em tempo constante O(1), na média, ao recuperar produtos relacionados a um termo de busca. <br/>
- Escalabilidade, mesmo com um catálogo contendo milhões de produtos. <br/>
- Facilidade de manutenção e atualização do índice, já que o HashMap oferece operações rápidas de inserção e remoção. <br/> <br/>

**Algoritmos de Busca** <br/>
O processo de busca no sistema foi baseado em um algoritmo simples e eficaz: <br/>
1. Normalização e Tokenização do termo de busca inserido pelo usuário. <br/>
2. Consulta direta ao HashMap para obter os IDs dos produtos relevantes. <br/>
3. Interseção dos conjuntos de IDs quando a busca envolvia múltiplos termos (AND lógico). <br/>
Esse fluxo garante resultados precisos e rápidos, minimizando a necessidade de varreduras extensivas no catálogo. <br/> <br/>

**Pré-processamento de Texto** <br/>
Para otimizar ainda mais a eficiência e a precisão:<br/>
- Utilizou-se a crate `regex` para remover pontuações e tokenizar os textos de entrada.<br/>
- A crate `unicode-normalization` foi empregada para normalizar os textos, removendo acentos e padronizando caracteres Unicode, melhorando a robustez das buscas.<br/>
- A crate `serde_json` permitiu a leitura e serialização eficiente dos dados de produtos a partir de arquivos JSON.<br/>
Essas técnicas de pré-processamento garantiram que termos similares ou grafias alternativas fossem corretamente mapeados ao mesmo índice no HashMap.<br/>

## 🛠️ **Como rodar o projeto**<br/>

**Pré-requisitos**<br/>

- Ter o Rust instalado: https://www.rust-lang.org/tools/install<br/>
- Ter as dependências do projeto (se encontram no Cargo.toml):
  - regex (1.11.1)
  - serde (1.0, feat: derive)
  - serde_json (1.0.140)
  - unicode-normalization (0.1)

**Passos:**<br/>

**1. Clone o projeto**<br/>

- `git clone https://github.com/Salutx/rust-case.git`<br/>
- `cd rust-case`<br/>

**2. Compile o projeto**<br/>

- `cargo build`<br/>

**3. Execute com os dados mockados**<br/>

- `cargo run`<br/>

## 🧪 Testes<br/>

Para rodar os testes unitários:

- `cargo test`

🎥 Vídeo Explicativo: https://www.youtube.com/watch?v=zJCYiyYJQP4 <br/>
📺 Acesse o vídeo explicando o sistema e as decisões de projeto

🧠 Autor
Desenvolvido por Salutx (Lucas Matos) 📧 lucas.costa.92648@a.fecaf.com.br
