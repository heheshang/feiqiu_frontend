# Section-Level Implementation Prompt Template

Use this prompt template to implement each section incrementally.

> **Before starting any section**, ensure you have completed the Foundation milestone (project setup, design system, shell).

---

## Section: `[SECTION_NAME]`

Implement the `[SECTION_NAME]` section of the 飞秋 (FeiQiu) application.

### Section Specification

Refer to `sections/[SECTION_NAME]/spec.md` for detailed requirements.

### Component Files

- Main component: `sections/[SECTION_NAME]/[ComponentName].tsx`
- Sub-components: `sections/[SECTION_NAME]/components/*.tsx`
- Types: `sections/[SECTION_NAME]/types.ts`
- Sample data: `sections/[SECTION_NAME]/data.json`

### Data Models

Use the following types from `types.ts`:
```typescript
// List the relevant types for this section
```

### Implementation Steps

1. **Read the specification** - Understand the feature requirements
2. **Review sample data** - Load `data.json` to understand data structure
3. **Implement main component** - Create the primary container component
4. **Implement sub-components** - Build reusable components
5. **Add interactions** - Wire up callbacks and event handlers
6. **Apply styling** - Use design tokens and dark mode support
7. **Test functionality** - Verify all user flows work correctly

### Styling Guidelines

- **Primary actions**: `bg-emerald-500 hover:bg-emerald-600`
- **Secondary actions**: `bg-blue-500 hover:bg-blue-600`
- **Neutral elements**: `bg-slate-100 dark:bg-slate-800`
- **Text**: `text-slate-900 dark:text-slate-100`
- **Borders**: `border-slate-200 dark:border-slate-700`

### Testing Checklist

- [ ] Component renders with sample data
- [ ] All user interactions work
- [ ] Dark mode displays correctly
- [ ] Responsive layout adapts to screen size
- [ ] Empty states show when no data
- [ ] Loading states display appropriately
- [ ] Error messages are user-friendly

### Integration Points

This section connects to:
- Shell navigation
- [List other sections or services it connects to]

### Sample Usage

```tsx
import { [ComponentName] } from './sections/[SECTION_NAME]/components/[ComponentName]'

function App() {
  return (
    <[ComponentName]
      // Props from sample data
      data={sampleData}
      // Callbacks
      onAction={(id) => console.log('Action:', id)}
    />
  )
}
```

---

**For detailed implementation instructions, refer to `instructions/incremental/[NN]-[SECTION_ID].md`**

**For testing specifications, refer to `sections/[SECTION_NAME]/tests.md`**
