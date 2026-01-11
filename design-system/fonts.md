# Typography Configuration

## Google Fonts Import

Add to your HTML `<head>` or CSS:

### HTML Import
```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500&display=swap" rel="stylesheet">
```

### CSS Import
```css
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500&display=swap');
```

## Font Usage

### Inter (Headings and Body)

Inter is a versatile sans-serif font designed for computer screens. It has excellent readability and a modern, professional appearance.

- **Headings:** `font-family: 'Inter', sans-serif;`
- **Body text:** `font-family: 'Inter', sans-serif;`
- **UI elements:** `font-family: 'Inter', sans-serif;`

Weight usage:
- 400: Regular body text
- 500: Medium emphasis, subheadings
- 600: Semi-bold, emphasis
- 700: Bold, headings

### IBM Plex Mono (Code and Technical)

IBM Plex Mono is a monospaced font designed for code and technical content.

- **Code blocks:** `font-family: 'IBM Plex Mono', monospace;`
- **Data displays:** `font-family: 'IBM Plex Mono', monospace;`
- **Technical labels:** `font-family: 'IBM Plex Mono', monospace;`

Weight usage:
- 400: Regular code
- 500: Emphasis in code

## Example Typography Scale

```css
/* Heading styles */
h1 { font-family: 'Inter', sans-serif; font-weight: 700; font-size: 2rem; line-height: 1.2; }
h2 { font-family: 'Inter', sans-serif; font-weight: 600; font-size: 1.5rem; line-height: 1.3; }
h3 { font-family: 'Inter', sans-serif; font-weight: 600; font-size: 1.25rem; line-height: 1.4; }
h4 { font-family: 'Inter', sans-serif; font-weight: 600; font-size: 1.125rem; line-height: 1.4; }

/* Body styles */
body { font-family: 'Inter', sans-serif; font-weight: 400; font-size: 1rem; line-height: 1.6; }
p { font-family: 'Inter', sans-serif; font-weight: 400; font-size: 1rem; line-height: 1.6; }

/* Small text */
small { font-family: 'Inter', sans-serif; font-size: 0.875rem; line-height: 1.5; }
.caption { font-family: 'Inter', sans-serif; font-size: 0.75rem; line-height: 1.4; }

/* Code and data */
code { font-family: 'IBM Plex Mono', monospace; font-size: 0.875em; }
.data { font-family: 'IBM Plex Mono', monospace; font-size: 0.875rem; }
```

## Tailwind CSS v4 Usage

In Tailwind CSS v4, fonts are configured via CSS custom properties. Add to your CSS:

```css
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=IBM+Plex+Mono:wght@400;500&display=swap');

@theme {
  --font-sans: 'Inter', ui-sans-serif, system-ui, sans-serif;
  --font-mono: 'IBM Plex Mono', ui-monospace, monospace;
}
```

Then use utility classes:
```jsx
// Heading
<h1 className="font-sans font-bold text-2xl">Heading</h1>

// Body
<p className="font-sans text-base">Body text</p>

// Code
<code className="font-mono text-sm">code</code>

// Data display
<span className="font-mono text-xs">192.168.1.100</span>
```
