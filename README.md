# Obsidian Garden

This is a fork of https://github.com/ecarrara/obsidian-garden

Obsidian Garden is a simple CLI tool to transform Obsidian Vault's notes into web
pages

## Installation

If you are on OS X or Linux, you can use the installation script to fetch the
latest release:

```bash
curl https://raw.githubusercontent.com/gskorokhod/obsidian-garden/main/install.sh | sh
```

## Features

- ✅ Convert your notes to static HTML pages
- ❔ Full text search with pagefind
- 🚧 LATEX support
- 🚧 Wikilinks support
- 🔴 Callouts
- 🔴 Backlink support
- 🔴 Ignore files

### Notes 

#### LATEX support 

- Basic LATEX blocks are supported
- Matrices, cases, and other more complex structures may not be rendered correctly
- Inline LATEX doesn't always work
- LATEX inside callouts not supported

#### Wikilinks

- Basic ``[[Wikilinks]]` usually work fine
- Can't link to paragraphs (e.g.: ``[[My Note#Heading 2]]`` won't render correctly)
- Suport for non-ASCII links not guaranteed
- No backlinks

### Legend 

- ✅ - Fully supported
- ❔ - Claimed in the original, but not yet tested
- 🚧 - Partially supported
- 🔴 - Not supported

## Getting Started

1. Navigate to you Vault folder and run `obsidian-garden init`

```bash
cd my-notes/
obsidian-garden init
```

2. Customize your site settings by editing the `.garden/site.yaml` file

```yaml
title: Site name
pagefind: false
topnav:
  links:
    - text: Link 1
      href: https://example.com/link-1
    - text: Link 2
      href: https://example.com/link-2
```

3. Generate a static site from your notes.

```bash
obsidian-garden build
```

4. Optional - Enable pagefind on `.garden/site.yaml` and run
[pagefind](https://pagefind.app) to index your site

```bash
pagefind --source dist
```

## What next?

This tool will generate a directory of HTML files based on your notes,
These will be stored in `dist/` inside your Obsidian vault by default.

You can:

- Host it using your static site hosting solution of choice (such as GitHub Pages, or similar)
- Self-host it with any HTTP server

To preview the generated site:

1. Go to the output directory:
```bash
cd dist
```
2. Run [your favourite HTTP server one-liner](https://gist.github.com/willurd/5720255)
```bash
python -m http.server 8000
```
3. Open [http://localhost:8000] in your web browser
