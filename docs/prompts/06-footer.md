# Footer Component - AI Generation Guide

**Component:** Site Footer
**Priority:** Medium (Completes the page)
**Estimated Time:** 10-15 minutes

---

## Wireframe

```
┌─────────────────────────────────────────────────────────────┐
│                                                               │
│                    Built with Dioxus & Rust                  │
│                                                               │
│              [GitHub]  [LinkedIn]  [Email]  [Twitter]        │
│                                                               │
│              © 2025 Your Name. All rights reserved.          │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Layout Details

**Desktop (> 1024px):**
- Full-width section with max-width: 1280px container
- Centered content with generous vertical padding (48px)
- Social links in a row with 24px gaps
- Copyright text below social links

**Mobile (< 640px):**
- Reduced vertical padding (32px)
- Social icons slightly smaller (20px instead of 24px)
- Stacked layout if needed
- Smaller font sizes for copyright

---

## Visual Design Preview

**Color Scheme:**
- Background: Slightly lighter than body (#1a1d24)
- Tech Stack Text: Accent color (#3b82f6)
- Social Icons: Muted gray (#6b7280)
- Social Icons Hover: Bright blue (#3b82f6)
- Copyright Text: Very muted (#6b7280)
- Divider Line: Subtle border (#2d3139)

**Typography Hierarchy:**
```
Built with... ← 14-16px, Medium (Featured text)
Copyright ← 12-14px, Regular (Legal text)
```

**Spacing:**
- Vertical padding: 48px (desktop), 32px (mobile)
- Between tech text and social: 24px
- Between social and copyright: 24px
- Social icon gaps: 24px
- Top border: 1px solid #2d3139

---

## Content Sections

```
Section               Content
Top Border            1px divider line
Tech Stack            "Built with Dioxus & Rust" or "Crafted with ❤️ and Rust"
Social Links          GitHub, LinkedIn, Email, Twitter/X (icons)
Copyright             "© 2025 Your Name. All rights reserved."
Optional              "Made with coffee ☕ in [City]" (personality touch)
```

---

## AI Generation Prompt

### Copy and paste this entire prompt ⬇️

```
HIGH-LEVEL GOAL:
Create a clean, minimal footer for a portfolio website that includes social links, a tech stack callout, and copyright information. The footer should feel complete without being cluttered and maintain the professional aesthetic of the rest of the site.

DETAILED INSTRUCTIONS:

1. Create a footer section with slightly lighter background than main content (#1a1d24)
2. Add a subtle top border (1px solid #2d3139) to separate from content above
3. Use centered layout with max-width: 1280px container
4. Implement the following content (top to bottom, centered):
   - Tech stack callout: "Built with Dioxus & Rust" or "Crafted with ❤️ and Rust"
   - Row of social icons: GitHub, LinkedIn, Email, Twitter/X (optional)
   - Copyright notice: "© 2025 Your Name. All rights reserved."
5. Make it fully responsive:
   - Desktop (> 1024px): Generous padding (48px vertical), comfortable spacing
   - Tablet (640-1024px): Moderate padding (40px vertical)
   - Mobile (< 640px): Compact padding (32px vertical), slightly smaller icons
6. Add smooth hover states:
   - Social icons: scale to 1.1, color shifts from muted gray to accent blue
   - Tech stack text: subtle color shift or glow effect (optional)

CODE EXAMPLES & CONSTRAINTS:

Color Palette (use exact values):
- Background: #1a1d24
- Top Border: 1px solid #2d3139
- Tech Stack Text: #3b82f6 (accent color)
- Social Icons Default: #6b7280
- Social Icons Hover: #3b82f6
- Copyright Text: #6b7280

Content (customize with your info):
- Tech Stack: "Built with Dioxus & Rust" or "Crafted with ❤️ and Rust" or "Powered by Rust & WebAssembly"
- Social Links:
  - GitHub: https://github.com/yourusername
  - LinkedIn: https://linkedin.com/in/yourprofile
  - Email: mailto:your.email@example.com
  - Twitter/X: https://twitter.com/yourusername (optional)
- Copyright: "© 2025 Your Name. All rights reserved."
- Optional personality: "Made with coffee ☕ in San Francisco" or similar

Typography:
- Tech Stack: 0.875-1rem (14-16px), font-weight: 500, color: accent blue
- Copyright: 0.75-0.875rem (12-14px), font-weight: 400, color: muted gray
- Use system font stack: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif

Footer Specifications:
- Vertical padding: 48px (desktop), 32px (mobile)
- Horizontal padding: 48px (desktop), 24px (mobile)
- Max-width: 1280px, centered with margin: 0 auto
- Top border: 1px solid #2d3139
- Background: #1a1d24 (slightly lighter than body)

Social Icon Specifications:
- Icon size: 24px (desktop), 20px (mobile)
- Gap between icons: 24px
- Default color: #6b7280 (muted gray)
- Hover: color #3b82f6, scale 1.1
- Transition: 200ms ease-in-out
- Use icon library like Heroicons or Lucide React
- All icons should be outline style (not solid)

Spacing:
- Between tech stack text and social icons: 24px
- Between social icons and copyright: 24px
- All text elements centered horizontally

Accessibility:
- Social icons must have aria-label: "Visit my GitHub", "Connect on LinkedIn", etc.
- All links open in new tabs (target="_blank" rel="noopener noreferrer")
- Ensure minimum touch target size (44x44px) for social icons on mobile
- Use semantic <footer> element
- Copyright symbol: Use HTML entity &copy; or Unicode ©

DO NOT:
- Add multiple columns or complex grid layouts (keep it simple)
- Include newsletter signup or forms (not needed for portfolio)
- Add sitemap or extensive link lists
- Use auto-playing animations or distracting effects
- Forget alt text or aria-labels for accessibility

STRICT SCOPE:

Create ONLY the footer component. This should be:
- A single React component named `Footer.tsx` (or similar)
- Self-contained with all styles (inline Tailwind or CSS-in-JS)
- No dependencies on other components except icon library
- Exported as default for easy integration

DO NOT create:
- Navbar component (separate)
- Other page sections
- Newsletter forms or CTAs
- Global styles (only footer-specific)
```

---

## Iteration Checklist

After generating, verify:

- [ ] Footer has clear visual separation from content above
- [ ] Tech stack text is easily readable and properly styled
- [ ] All social icons are aligned and evenly spaced
- [ ] Social links open in new tabs
- [ ] Hover states work smoothly on all icons
- [ ] Copyright year is current (2025)
- [ ] Footer looks balanced on mobile
- [ ] Touch targets are at least 44px on mobile
- [ ] Focus states visible for keyboard navigation
- [ ] Aria-labels are present on all icon links

---

## Common Refinements

If the AI output needs adjustment, use these follow-up prompts:

**"Social icons are too small on mobile"**
```
Increase mobile icon size to 22px and add more padding around each icon for easier tapping. Ensure touch targets are at least 44x44px.
```

**"Footer feels too cramped"**
```
Increase vertical padding to 64px on desktop and 40px on mobile. Add 32px gap between each content section (tech text, social icons, copyright).
```

**"Want to emphasize the Rust/Dioxus tech stack more"**
```
Add a subtle gradient or glow effect to the tech stack text. Consider adding Rust and Dioxus logos as small inline icons next to the text.
```

**"Copyright text is too prominent"**
```
Reduce font size to 0.75rem (12px) and lower opacity to 0.7. This makes it less visually dominant while still being readable.
```

**"Need additional links (blog, resume, etc.)"**
```
Add a row of text links above the social icons: "Resume | Blog | Uses" with 16px gaps between them. Style in muted gray with hover to accent color.
```

---

## Integration Notes

**Sticky Footer:**
If you want the footer to stick to the bottom of the viewport when content is short, use this approach:

```css
/* Main layout wrapper */
.page-wrapper {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

/* Main content grows to fill space */
.main-content {
  flex: 1;
}

/* Footer naturally sticks to bottom */
.footer {
  /* Your footer styles */
}
```

**Content Above Footer:**
Ensure the section immediately above the footer (typically Contact section) has adequate bottom padding:

```css
.contact-section {
  padding-bottom: 96px; /* Desktop */
}

@media (max-width: 640px) {
  .contact-section {
    padding-bottom: 64px; /* Mobile */
  }
}
```

---

## Personalization Ideas

Add personality to your footer while keeping it professional:

**Option 1: Location + Fun Fact**
```
"Built with Dioxus & Rust"
[Social Icons]
"Made with coffee ☕ in San Francisco"
© 2025 Your Name
```

**Option 2: Tech Stack Emphasis**
```
"Powered by Rust 🦀 & WebAssembly"
[Social Icons]
© 2025 Your Name • All rights reserved
```

**Option 3: Minimalist**
```
[Social Icons]
© 2025 Your Name
```

**Option 4: Call-to-Action**
```
"Let's build something amazing together"
[Social Icons]
© 2025 Your Name • Available for opportunities
```

---

## Ready to Build?

1. ✅ Copy the prompt above
2. ✅ Paste into v0.dev or Lovable
3. ✅ Review generated code
4. ✅ Customize content with your info
5. ✅ Test on mobile and desktop
6. ✅ Port to Dioxus when satisfied

**Next:** Update README.md to include navbar (00) and footer (06) in the build order
