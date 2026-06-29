# Quality Guidelines

> Code quality standards for frontend development.

---

## Overview

This project uses React + TypeScript + Vite + Tailwind CSS for the frontend.

---

## Forbidden Patterns

- Do not mix Tailwind CSS v3 and v4 syntax. The `@tailwind base/components/utilities` directives are v3 syntax and require `tailwindcss` v3 with `postcss.config.js` using `tailwindcss: {}` plugin. The `@tailwindcss/postcss` plugin is for v4 only.

---

## Required Patterns

- All Vite plugins imported in `vite.config.ts` must be listed in `package.json` devDependencies (e.g., `@vitejs/plugin-react`)
- PostCSS config must match the installed Tailwind version:
  - Tailwind v3: `postcss.config.js` uses `tailwindcss: {}` and `autoprefixer: {}`
  - Tailwind v4: `postcss.config.js` uses `'@tailwindcss/postcss': {}`

---

## Testing Requirements

- Frontend build must pass (`npm run build` → `tsc && vite build`)

---

## Code Review Checklist

- Verify all imports in config files have corresponding dependencies in package.json
- Ensure Tailwind CSS version consistency between package.json and postcss.config.js
