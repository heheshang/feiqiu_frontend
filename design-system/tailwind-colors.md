# Tailwind Color Configuration

## Color Choices

- **Primary:** `emerald` — Used for buttons, links, key accents, success states, primary navigation
- **Secondary:** `blue` — Used for secondary actions, information highlights, links
- **Neutral:** `slate` — Used for backgrounds, text, borders, UI chrome

## Usage Examples

### Primary Colors (emerald)

Primary button: `bg-emerald-500 hover:bg-emerald-600 text-white`
Primary text: `text-emerald-600 dark:text-emerald-400`
Primary border: `border-emerald-500`

Light variants for backgrounds:
`bg-emerald-50 dark:bg-emerald-900/30`
`bg-emerald-100 dark:bg-emerald-900/30`

Dark variants for hover states:
`hover:bg-emerald-600`
`active:bg-emerald-700`

### Secondary Colors (blue)

Secondary button: `bg-blue-500 hover:bg-blue-600 text-white`
Secondary badge: `bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400`
Secondary link: `text-blue-600 hover:text-blue-700`

### Neutral Colors (slate)

Background: `bg-slate-50 dark:bg-slate-900`
Surface: `bg-white dark:bg-slate-800`
Border: `border-slate-200 dark:border-slate-700`
Text primary: `text-slate-900 dark:text-slate-100`
Text secondary: `text-slate-600 dark:text-slate-400`
Text muted: `text-slate-500 dark:text-slate-500`

### Status Colors

- **Online:** `bg-emerald-500` (green)
- **Away:** `bg-amber-500` (yellow/amber)
- **Busy:** `bg-red-500` (red)
- **Offline:** `bg-slate-400` (gray)

## Tailwind CSS v4 Usage

In Tailwind CSS v4, you can use these colors directly with utility classes:

```jsx
// Primary button
<button className="bg-emerald-500 hover:bg-emerald-600 text-white px-4 py-2 rounded-lg">
  Click me
</button>

// Secondary button
<button className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg">
  Secondary
</button>

// Card component
<div className="bg-white dark:bg-slate-800 rounded-xl shadow-sm border border-slate-200 dark:border-slate-700 p-6">
  Content here
</div>

// Status indicator
<span className="w-2 h-2 rounded-full bg-emerald-500" />

// Input field
<input className="w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-200" />
```

## CSS Custom Properties

The tokens.css file provides CSS custom properties for use in custom CSS:

```css
.component {
  background-color: var(--color-primary-500);
  color: var(--color-neutral-900);
  font-family: var(--font-body);
}
```
