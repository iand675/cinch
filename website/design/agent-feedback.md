# Reviewing the site with Agentation

`npm run dev` adds [Agentation](https://agentation.com)'s annotation toolbar to every page (landing, platform pages, and the docs). Production builds contain none of it: `FeedbackToolbar.astro` loads `src/lib/feedback-toolbar.ts` behind `import.meta.env.DEV`, so the import is removed at build time. Check with `grep -rli agentation dist` after `npm run build`; it should find nothing.

## Copy and paste (works with any agent, including cloud sessions)

1. `npm run dev` and open the page.
2. Click the toolbar icon in the bottom-right corner, then click an element (or select text, or drag an area) and write the note.
3. Copy. The markdown includes the page, a CSS selector for each element, nearby text, and your note. Paste it into the agent's chat.

Annotating a `data-*` element gives the agent a stable handle: the landing gadgets carry attributes such as `data-runnable`, `data-printout`, `data-agent-loop`, and `data-contract-screen`.

## Direct to a local agent (MCP)

When the agent runs on the same machine as the browser:

```sh
claude mcp add agentation -- npx -y agentation-mcp server
PUBLIC_AGENTATION_ENDPOINT=http://localhost:4747 npm run dev
```

The toolbar then syncs annotations to the server on port 4747, and the agent can list and resolve them (`agentation_get_all_pending`). A cloud agent session cannot reach `localhost` on your machine, so use copy and paste with one.

## For agents receiving annotations

Treat an annotation as a pointer plus a request. Find the element from the selector or `data-*` attribute, read the owning component listed in [AGENTS.md](../AGENTS.md), and apply the website design rules as usual; an annotation does not override them. If a note asks for something the handbook rules out, say so rather than doing it silently.

Licence: Agentation is PolyForm Shield 1.0.0. It is a development dependency for internal review and is not shipped with the site.
