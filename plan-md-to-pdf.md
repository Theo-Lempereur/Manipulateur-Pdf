# Plan : Feature Markdown → PDF

## Objectif

Ajouter la conversion de fichiers Markdown (.md) en PDF, accessible via le CLI et le GUI.

---

## Outil utilisé

**Pandoc** (v3.9, deja installe) via `std::process::Command`.

### Probleme : moteur PDF

Pandoc ne genere pas de PDF seul. Il a besoin d'un **moteur PDF** externe. Options :

| Moteur | Taille | Installation | Qualite |
|--------|--------|-------------|---------|
| **pdflatex** (MiKTeX/TinyTeX) | ~200MB+ | `winget install MiKTeX.MiKTeX` | Excellente, standard academique |
| **typst** | ~30MB | `winget install typst` | Bonne, moderne, leger |
| **wkhtmltopdf** | ~50MB | installateur .exe | Correcte, via HTML |

**Recommandation** : utiliser ce qui est disponible, avec un ordre de priorite :
1. `pdflatex` (si installe)
2. `typst` (si installe)
3. Sinon, erreur claire indiquant quel moteur installer

Le code detéctera automatiquement le moteur disponible.

---

## Fichiers modifies/crees

### 1. `pdftool-core/src/md_to_pdf.rs` (nouveau)

```rust
// Detection du moteur PDF disponible
fn find_pdf_engine() -> Result<String, Error>
    // Cherche pdflatex, puis typst, puis erreur

// Fonction principale
pub fn md_to_pdf(input: &Path, output: &Path) -> Result<(), Error>
    // Appelle: pandoc input.md -o output.pdf --pdf-engine=<detected>
```

Recherche de pandoc : bundled a cote de l'exe → PATH systeme (meme pattern que pdftotext/ghostscript).

### 2. `pdftool-core/src/lib.rs`

- Ajouter `mod md_to_pdf;`
- Ajouter `pub use md_to_pdf::md_to_pdf;`

### 3. `pdftool-cli/src/main.rs`

Nouvelle sous-commande :

```
pdftool md-to-pdf input.md -o output.pdf
```

- `input` : fichier Markdown (obligatoire)
- `-o` / `--output` : fichier PDF de sortie (defaut : `input.pdf`)

### 4. `src-tauri/src/main.rs`

- Nouvelle commande Tauri `cmd_md_to_pdf(input, output_dir, output_name)`
- Ajout dans `invoke_handler`

### 5. `ui/index.html`

Nouvelle option dans l'onglet **Convert** :

- Ajouter un dropdown "Source format" en haut du panel Convert :
  - `PDF → Images` (comportement actuel)
  - `Markdown → PDF` (nouveau)
- Quand "Markdown → PDF" est selectionne :
  - Le browse n'accepte que les `.md`
  - Les champs Format/DPI sont masques
  - Le bouton dit "Convert to PDF"
  - Le hint du nom de sortie passe a `.pdf`

### 6. `ui/main.js`

- Logique de switch pour le mode Convert (similaire au switch Extract)
- Nouvelle fonction `runMdToPdf()`

---

## Deroulement

1. Creer la branche `feature/md-to-pdf`
2. Implementer `md_to_pdf.rs` dans le core
3. Ajouter la sous-commande CLI
4. Ajouter la commande Tauri
5. Modifier le UI (Convert tab avec dropdown)
6. Build + test
7. Commit, push, PR, merge

---

## Pre-requis utilisateur

L'utilisateur doit avoir installe :
- **Pandoc** (deja OK)
- **Un moteur PDF** parmi : `pdflatex`, `typst`, ou `wkhtmltopdf`

Si aucun moteur n'est trouve, le message d'erreur indiquera :
> "No PDF engine found. Install one of: pdflatex (MiKTeX), typst, or wkhtmltopdf"
