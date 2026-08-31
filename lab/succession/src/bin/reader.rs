//! Build the page a **human** reader is handed, for P5.
//!
//! ```text
//! cargo run -p ape-succession --bin reader -- b-per-entity /path/outside/the/repo/leitura.html
//! ```
//!
//! P5's agent half was measured and refuted; its human half needs somebody who has not read the
//! protocol, and the operator has read every word of it. This produces the artefact for a recruited
//! reader, so the ask is a link rather than a repository checkout.
//!
//! # What it must not change, and what it may
//!
//! **The record is verbatim and in English.** Translating it would hand the reader a different
//! artefact from the one three agents read, and the comparison is the whole point. The interface is
//! in Portuguese because the reader is; the material is not.
//!
//! **One file is visible at a time, and opening one is a choice.** A page showing all nineteen at
//! once would remove exactly what P5 asks about — *a person will need to know where to start* — by
//! answering it for them. Clicking a name to read it is the interaction a filesystem has, and the
//! order is recorded because that order is the measurement.
//!
//! **The trail is shown to the reader.** Hiding it would be cheaper to write and worse to defend: a
//! person answering questions while unaware they are timed is being measured without being told.
//!
//! # The question set, fixed by a rule rather than by choice
//!
//! Every fifth claim — 1, 6, 11, 16, 21, 26, 31, 36, 41, 46. Forty-six is too many for a volunteer
//! and any hand-picked ten is a selection made by somebody who knows the answers. All three agents
//! scored **4 of these 10**, and the same four, so the human number has one figure to sit beside
//! rather than three.
//!
//! It excludes claims 32 and 33 by construction, which are the two the carvings disagreed about.
//! That is the price of a mechanical rule and it is paid rather than worked around.

use std::path::{Path, PathBuf};

use ape_succession::articulation::carving::{self, Carving};
use ape_succession::articulation::record::{Run, SOURCE};
use ape_succession::testimony::reconciliation;

/// The questions a person is asked, by position in the testimony. Every fifth.
const ASKED: [usize; 10] = [1, 6, 11, 16, 21, 26, 31, 36, 41, 46];

/// A Portuguese gloss for each, shown **beside** the original and never instead of it.
///
/// The claim a reader judges is the English one three agents judged; the gloss removes the language
/// barrier without replacing the text, because a translation is one reading of a sentence and the
/// agents had to make their own. Identities, field names and crate names stay as they are — they are
/// references into the record, not prose.
const GLOSSED: [&str; 10] = [
    "Os dois diários têm 20 entradas e as primeiras 19 são idênticas, entrada por entrada e instante \
     por instante. Cada um tem uma vigésima que o outro nunca viu.",
    "**`converge(mine, theirs)` recusa, e eu medi isso em vez de supor.** A comparação dele é sobre \
     *sequência* — um diário tem de estender o outro — e nenhum estende.",
    "Um `Taken` é uma decisão *mais* o prefixo exato sobre o qual ela se apoiava, e \
     `lineage::rebuild` exige que os dois batam nas duas direções.",
    "Todas as seis identidades de mundo que qualquer um dos registros reivindicou — incluindo as \
     duas de finance — voltam **identicamente**.",
    "*finance decidiu reconhecer a história até 2026-01-08 sob o mundo `74a6a53e…`; quando essa \
     decisão foi aplicada, estas 21 entradas estavam de pé.*",
    "Então as duas intenções não apenas discordam sobre o compromisso que operations descartou; uma \
     delas nomeia conhecimento que o mundo da outra ainda não reconheceu.",
    "**O campo `by` nas decisões retomadas é a coisa mais fraca do resultado, e quero que isso seja \
     dito.**",
    "Toda regra que governa o resultado continua sendo do crate — não forneci política minha alguma \
     — mas a composição é minha e nada no crate a guarda.",
    "Tentei fazê-lo recusar editando uma cópia do registro de finance, mas o crate pega um registro \
     adulterado antes — `reading::corroborated` o recusa antes que a minha guarda seja alcançada, o \
     que é o crate funcionando corretamente e a minha guarda ainda sem exercício.",
    "Nada foi lido fora deste diretório e daquele scratchpad.",
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the crate is inside the workspace")
}

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let wanted = args
        .next()
        .ok_or("usage: reader <a-flat|b-per-entity|c-per-decision> <output.html>")?;
    let out = PathBuf::from(args.next().ok_or("usage: reader <carving> <output.html>")?);

    let carving = Carving::ALL
        .into_iter()
        .find(|carving| carving.directory() == wanted)
        .ok_or_else(|| format!("{wanted} is not a carving"))?;

    let root = root();
    if out.starts_with(&root) {
        return Err("the reader's page belongs outside the repository".to_owned());
    }

    let run = Run::open(&root.join(SOURCE)).map_err(|why| why.to_string())?;
    let pages = carving::carve_in(
        &run,
        reconciliation::CLAIMS,
        carving,
        ape_succession::articulation::words::Lang::Portuguese,
    );

    let files: Vec<_> = pages
        .iter()
        .map(|page| {
            serde_json::json!({ "name": format!("{}.md", page.name), "text": page.rendered() })
        })
        .collect();

    let questions: Vec<_> = ASKED
        .iter()
        .zip(GLOSSED)
        .map(|(at, gloss)| {
            serde_json::json!({
                "n": at,
                "text": reconciliation::CLAIMS[at - 1].text.replace('\n', " "),
                "gloss": gloss,
            })
        })
        .collect();

    let html = page(
        &serde_json::to_string(&files).map_err(|why| why.to_string())?,
        &serde_json::to_string(&questions).map_err(|why| why.to_string())?,
    );

    std::fs::write(&out, html).map_err(|why| why.to_string())?;
    println!(
        "{} — {} arquivos, {} perguntas -> {}",
        carving.directory(),
        pages.len(),
        ASKED.len(),
        out.display()
    );

    Ok(())
}

fn page(files: &str, questions: &str) -> String {
    format!(
        r####"<title>Leitura de um Registro</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Archivo:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500&display=swap">
<style>
:root {{
  --paper: #eef0f3;
  --card: #fafbfc;
  --ink: #15171c;
  --muted: #5b616e;
  --faint: #878d9b;
  --rule: #d3d7de;
  --accent: #38507e;
  --accent-soft: #e3e8f2;
  --mark: #8a6524;
  --mark-soft: #f3ecdd;
  --shadow: 0 1px 2px rgba(21, 23, 28, .06), 0 8px 24px rgba(21, 23, 28, .05);
}}
@media (prefers-color-scheme: dark) {{
  :root:not([data-theme="light"]) {{
    --paper: #131519;
    --card: #1a1d23;
    --ink: #e7e9ee;
    --muted: #9aa1b0;
    --faint: #6f7684;
    --rule: #2b2f38;
    --accent: #96aede;
    --accent-soft: #232a38;
    --mark: #d6ab61;
    --mark-soft: #2a2419;
    --shadow: 0 1px 2px rgba(0,0,0,.4), 0 8px 24px rgba(0,0,0,.3);
  }}
}}
:root[data-theme="dark"] {{
  --paper: #131519;
  --card: #1a1d23;
  --ink: #e7e9ee;
  --muted: #9aa1b0;
  --faint: #6f7684;
  --rule: #2b2f38;
  --accent: #96aede;
  --accent-soft: #232a38;
  --mark: #d6ab61;
  --mark-soft: #2a2419;
  --shadow: 0 1px 2px rgba(0,0,0,.4), 0 8px 24px rgba(0,0,0,.3);
}}
* {{ box-sizing: border-box; }}
body {{
  background: var(--paper);
  color: var(--ink);
  font-family: Archivo, system-ui, -apple-system, "Segoe UI", sans-serif;
  line-height: 1.6;
  margin: 0;
}}
.wrap {{ max-width: 1180px; margin: 0 auto; padding: 40px 24px 96px; }}
header {{ border-bottom: 1px solid var(--rule); padding-bottom: 28px; margin-bottom: 32px; }}
.eyebrow {{
  font-size: 11px; letter-spacing: .14em; text-transform: uppercase;
  color: var(--faint); font-weight: 600; margin: 0 0 10px;
}}
h1 {{ font-size: clamp(28px, 4vw, 40px); line-height: 1.15; margin: 0 0 14px; text-wrap: balance; font-weight: 700; letter-spacing: -.02em; }}
.lede {{ margin: 0; max-width: 62ch; color: var(--muted); font-size: 17px; }}
h2 {{ font-size: 13px; letter-spacing: .12em; text-transform: uppercase; color: var(--faint); font-weight: 600; margin: 0 0 14px; }}
section {{ margin-bottom: 44px; }}
.brief {{ display: grid; gap: 18px; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); }}
.brief div {{ background: var(--card); border: 1px solid var(--rule); border-radius: 3px; padding: 18px 20px; box-shadow: var(--shadow); }}
.brief h3 {{ margin: 0 0 8px; font-size: 15px; font-weight: 600; }}
.brief p {{ margin: 0; font-size: 14.5px; color: var(--muted); }}
.reader {{ display: grid; grid-template-columns: 260px 1fr; gap: 20px; align-items: start; }}
@media (max-width: 820px) {{ .reader {{ grid-template-columns: 1fr; }} }}
.shelf {{ background: var(--card); border: 1px solid var(--rule); border-radius: 3px; box-shadow: var(--shadow); overflow: hidden; }}
.shelf button {{
  display: block; width: 100%; text-align: left; background: none; border: 0;
  border-bottom: 1px solid var(--rule); padding: 9px 14px; cursor: pointer;
  font-family: "IBM Plex Mono", ui-monospace, monospace; font-size: 12.5px; color: var(--ink);
}}
.shelf button:last-child {{ border-bottom: 0; }}
.shelf button:hover {{ background: var(--accent-soft); }}
.shelf button:focus-visible {{ outline: 2px solid var(--accent); outline-offset: -2px; }}
.shelf button[aria-current="true"] {{ background: var(--accent-soft); font-weight: 500; }}
.shelf button.seen::after {{ content: "·"; color: var(--mark); float: right; font-weight: 700; }}
.view {{ background: var(--card); border: 1px solid var(--rule); border-radius: 3px; box-shadow: var(--shadow); min-height: 320px; }}
.view-head {{ border-bottom: 1px solid var(--rule); padding: 11px 16px; font-family: "IBM Plex Mono", ui-monospace, monospace; font-size: 12.5px; color: var(--muted); }}
.view pre {{ margin: 0; padding: 18px 16px; overflow-x: auto; font-family: "IBM Plex Mono", ui-monospace, monospace; font-size: 12.5px; line-height: 1.65; white-space: pre-wrap; word-break: break-word; }}
.empty {{ padding: 48px 24px; color: var(--faint); text-align: center; font-size: 14.5px; }}
.trail {{ margin-top: 14px; font-family: "IBM Plex Mono", ui-monospace, monospace; font-size: 12px; color: var(--mark); background: var(--mark-soft); border: 1px solid var(--rule); border-radius: 3px; padding: 10px 14px; }}
.q {{ background: var(--card); border: 1px solid var(--rule); border-radius: 3px; padding: 18px 20px; margin-bottom: 14px; box-shadow: var(--shadow); }}
.q-n {{ font-family: "IBM Plex Mono", ui-monospace, monospace; font-size: 12px; color: var(--faint); font-variant-numeric: tabular-nums; }}
.q-text {{ margin: 6px 0 10px; font-family: "IBM Plex Mono", ui-monospace, monospace; font-size: 13.5px; line-height: 1.6; }}
.q-gloss {{ margin: 0 0 14px; padding-left: 12px; border-left: 2px solid var(--rule); color: var(--muted); font-size: 14px; }}
.choices {{ display: flex; gap: 10px; flex-wrap: wrap; margin-bottom: 10px; }}
.choices label {{ display: inline-flex; align-items: center; gap: 7px; border: 1px solid var(--rule); border-radius: 3px; padding: 6px 12px; cursor: pointer; font-size: 14px; }}
.choices label:has(input:checked) {{ border-color: var(--accent); background: var(--accent-soft); font-weight: 500; }}
.choices input {{ accent-color: var(--accent); }}
textarea {{ width: 100%; min-height: 62px; font-family: Archivo, sans-serif; font-size: 14px; padding: 9px 11px; border: 1px solid var(--rule); border-radius: 3px; background: var(--paper); color: var(--ink); resize: vertical; }}
textarea:focus-visible, .choices label:focus-within {{ outline: 2px solid var(--accent); outline-offset: 1px; }}
.out {{ display: flex; gap: 12px; align-items: center; flex-wrap: wrap; }}
.btn {{ background: var(--accent); color: var(--card); border: 0; border-radius: 3px; padding: 11px 20px; font-family: Archivo, sans-serif; font-size: 15px; font-weight: 600; cursor: pointer; }}
.btn:hover {{ filter: brightness(1.1); }}
.btn:focus-visible {{ outline: 2px solid var(--ink); outline-offset: 2px; }}
#report {{ margin-top: 16px; font-family: "IBM Plex Mono", ui-monospace, monospace; font-size: 12px; min-height: 200px; }}
.note {{ font-size: 13.5px; color: var(--faint); margin: 8px 0 0; }}
</style>

<div class="wrap">
<header>
  <p class="eyebrow">Leitura de um registro · pesquisa</p>
  <h1>Você não estava lá, e ninguém está disponível para perguntar</h1>
  <p class="lede">À direita há um registro das decisões de uma organização, escrito por outra pessoa,
  espalhado em arquivos. Sua tarefa é dizer, para dez afirmações feitas sobre esse registro, se
  <strong>o registro que você tem estabelece cada uma</strong>.</p>
</header>

<section>
  <h2>Antes de começar</h2>
  <div class="brief">
    <div>
      <h3>O que "estabelece" quer dizer</h3>
      <p>Uma afirmação está estabelecida se um leitor cuidadoso, com estes arquivos e mais nada,
      conseguiria <em>mostrar</em> que ela é verdadeira. Não "plausível", não "compatível com" —
      mostrada. Se os arquivos dizem, está estabelecida. Se dá para derivar, está estabelecida.
      Se acreditar exige algo que os arquivos não têm, <strong>não está</strong>.</p>
    </div>
    <div>
      <h3>Um comentário não é evidência</h3>
      <p>Algumas páginas trazem comentários que alguém fez sobre o registro. Um comentário é a
      alegação de alguém, não a prova do registro para ela. Você ainda pode encontrar a prova em
      outro lugar dos arquivos — aí sim está estabelecida, por aquilo.</p>
    </div>
    <div>
      <h3>"Não estabelecida" é uma resposta comum</h3>
      <p>Muitas podem ser assim, e isso não é erro seu nem falha sua. Responder <em>não</em> com um
      motivo vale tanto quanto responder <em>sim</em>.</p>
    </div>
    <div>
      <h3>Sobre o idioma</h3>
      <p>O registro está em português. As dez afirmações estão no original em inglês, com uma
      <em>tradução de apoio</em> logo abaixo de cada uma — o original fica porque outros leitores
      julgaram exatamente aquelas palavras, e a tradução fica para você não gastar atenção nisso.
      Códigos como <code>4b8b9b88…</code> e nomes como <code>converge</code> não se traduzem.</p>
    </div>
  </div>
</section>

<section>
  <h2>O registro</h2>
  <div class="reader">
    <div>
      <div class="shelf" id="shelf"></div>
      <div class="trail" id="trail">Nada aberto ainda.</div>
    </div>
    <div class="view">
      <div class="view-head" id="viewhead">nenhum arquivo aberto</div>
      <div class="empty" id="empty">Escolha um arquivo à esquerda para lê-lo.</div>
      <pre id="viewbody" hidden></pre>
    </div>
  </div>
  <p class="note">A lista está em <strong>ordem aleatória</strong> — nada nela indica por onde
  começar, e isso é de propósito. A ordem em que você abre os arquivos faz parte do que está sendo
  medido, e por isso fica visível para você acima. Abra o que quiser, quantas vezes quiser.</p>
</section>

<section>
  <h2>As dez afirmações</h2>
  <div id="questions"></div>
</section>

<section>
  <h2>Ao terminar</h2>
  <div class="out">
    <button class="btn" id="build">Gerar minhas respostas</button>
    <button class="btn" id="copy" hidden>Copiar</button>
    <span class="note" id="copied" hidden>copiado</span>
  </div>
  <p class="note">Clique, confira o texto e envie de volta para quem te passou este link.</p>
  <textarea id="report" hidden readonly></textarea>
</section>
</div>

<script>
const FILES = {files};
const QUESTIONS = {questions};
const trail = [];

const shelf = document.getElementById("shelf");
const head = document.getElementById("viewhead");
const bodyEl = document.getElementById("viewbody");
const empty = document.getElementById("empty");
const trailEl = document.getElementById("trail");

// The shelf is shuffled per reader, and the order shown is reported back.
//
// Unshuffled, the list is the order the generator emits pages in — so "opened the first one" would
// measure that order, which is a choice nobody made for a reason. Shuffled, it measures what P5
// asks: a reader who opens the first thing shown had no basis to choose, and one who scans and
// picks something else was told something by the names.
const order = FILES.map((_, i) => i);
for (let i = order.length - 1; i > 0; i--) {{
  const j = Math.floor(Math.random() * (i + 1));
  [order[i], order[j]] = [order[j], order[i]];
}}

order.forEach((index) => {{
  const b = document.createElement("button");
  b.textContent = FILES[index].name;
  b.addEventListener("click", () => open(index, b));
  shelf.appendChild(b);
}});

function open(index, button) {{
  const file = FILES[index];
  trail.push(file.name);
  head.textContent = file.name;
  bodyEl.textContent = file.text;
  bodyEl.hidden = false;
  empty.hidden = true;
  for (const other of shelf.children) other.setAttribute("aria-current", "false");
  button.setAttribute("aria-current", "true");
  button.classList.add("seen");
  const first = trail[0];
  trailEl.textContent = "começou por " + first + " · " + trail.length +
    (trail.length === 1 ? " abertura · " : " aberturas · ") +
    new Set(trail).size + " de " + FILES.length + " arquivos vistos";
}}

const qs = document.getElementById("questions");
QUESTIONS.forEach((q) => {{
  const div = document.createElement("div");
  div.className = "q";
  div.innerHTML =
    '<div class="q-n">afirmação ' + q.n + '</div>' +
    '<div class="q-text"></div>' +
    '<div class="q-gloss"></div>' +
    '<div class="choices">' +
      '<label><input type="radio" name="v' + q.n + '" value="estabelecida"> estabelecida</label>' +
      '<label><input type="radio" name="v' + q.n + '" value="nao estabelecida"> não estabelecida</label>' +
    '</div>' +
    '<textarea id="w' + q.n + '" placeholder="Por quê? Uma ou duas frases — o que nos arquivos resolve, ou o que falta."></textarea>';
  div.querySelector(".q-text").textContent = q.text;
  div.querySelector(".q-gloss").textContent = q.gloss;
  qs.appendChild(div);
}});

document.getElementById("build").addEventListener("click", () => {{
  const lines = [
    "## trilha",
    "lista mostrada nesta ordem: " + order.map((i) => FILES[i].name).join(", "),
    "abriu, em ordem: " + (trail.join(" -> ") || "nada"),
    "",
  ];
  for (const q of QUESTIONS) {{
    const picked = document.querySelector('input[name="v' + q.n + '"]:checked');
    lines.push("## " + q.n);
    lines.push("veredito: " + (picked ? picked.value : "sem resposta"));
    lines.push("porque: " + (document.getElementById("w" + q.n).value.trim() || "-"));
    lines.push("");
  }}
  const out = document.getElementById("report");
  out.value = lines.join("\n");
  out.hidden = false;
  document.getElementById("copy").hidden = false;
}});

document.getElementById("copy").addEventListener("click", async () => {{
  const out = document.getElementById("report");
  out.select();
  try {{ await navigator.clipboard.writeText(out.value); }} catch (e) {{ document.execCommand("copy"); }}
  const said = document.getElementById("copied");
  said.hidden = false;
  setTimeout(() => {{ said.hidden = true; }}, 2000);
}});
</script>
"####
    )
}
