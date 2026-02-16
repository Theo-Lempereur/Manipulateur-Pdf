# PDFTool — Simple PDF Tool (Rust)

## 🎯 Objectif du projet

PDFTool est un petit logiciel qui permet à n’importe qui de manipuler facilement des fichiers PDF, sans connaissance technique.

Fonctionnalités principales :

* extraire certaines pages d’un PDF
* compresser un PDF pour réduire sa taille
* convertir un PDF en images (PNG ou JPEG)

Le programme est conçu pour :

* fonctionner via un exécutable unique
* être utilisable depuis un terminal simple
* ne nécessiter aucune connaissance en programmation pour l’utilisateur final

Exemples d’usage :

```bash
pdftool extract input.pdf --pages 2-5 -o out.pdf
pdftool compress input.pdf -o compressed.pdf
pdftool convert input.pdf --format png
```

L’objectif est d’avoir un outil minimal, rapide, portable, et compréhensible.

---

## 🧠 Philosophie

Ce projet n’essaie PAS de réimplémenter un moteur PDF.

Il s’appuie volontairement sur un outil existant très robuste : Ghostscript.

Rust est utilisé comme langage principal pour :

* gérer l’interface utilisateur (CLI)
* orchestrer les commandes
* gérer les fichiers
* produire un exécutable final

Ghostscript fait le travail lourd (PDF).

---

## 🧱 Stack imposée

### Langage principal

* Rust (édition stable)

### IDE

* Microsoft Visual Studio Code

Extensions recommandées :

* rust-analyzer
* CodeLLDB

---

### Moteur PDF

* Ghostscript

Ghostscript est utilisé pour :

* compression PDF
* extraction de pages
* conversion PDF → images

Le programme Rust appellera Ghostscript via la ligne de commande (`std::process::Command`).

Ghostscript doit être installé sur la machine.

Commande de vérification :

```bash
gs --version
```

---

## ✅ Fonctionnalités prioritaires

Ordre strict :

1. Extraction de pages
2. Compression
3. Conversion PDF → images

Pas de conversion vers Word / HTML.
Pas d’interface graphique pour le moment. 

Interface CLI uniquement.

---

## 📦 Résultat attendu

* un binaire généré via :

```bash
cargo build --release
```

* exécutable utilisable sans Rust installé
* fonctionnement multiplateforme (Windows priorité)

---

# ⚙️ Section Agent IA

Cette section est destinée à un agent automatisé chargé du développement.

---

## Contraintes

* langage : Rust
* IDE cible : VS Code
* moteur PDF : Ghostscript
* interface : CLI uniquement
---

## Liberté accordée

L’agent peut choisir :

* l’architecture interne
* les crates Rust (clap recommandé mais non imposé)
* organisation des modules
* gestion des erreurs
* format des logs

