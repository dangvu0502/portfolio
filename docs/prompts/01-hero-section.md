# Hero Section - AI Generation Guide

**Component:** Hero Section
**Priority:** High (First impression)
**Estimated Time:** 15-20 minutes

---

## Wireframe

```
┌─────────────────────────────────────────────────────────────┐
│                                                               │
│                                                               │
│                                                               │
│                       [Your Name]                            │
│                                                               │
│           Frontend Developer specializing in AI              │
│                                                               │
│        Building performant, AI-powered web experiences       │
│          with modern frameworks. Currently exploring         │
│                    Rust and WebAssembly.                     │
│                                                               │
│                                                               │
│            [View Projects]    [Contact Me]                   │
│                                                               │
│                                                               │
│               [GitHub]  [LinkedIn]  [Email]                  │
│                                                               │
│                                                               │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Layout Details

**Desktop (> 1024px):**
- Full viewport height (100vh)
- All content centered vertically and horizontally
- Max-width: 1024px container
- Buttons side-by-side
- Social icons in a row

**Mobile (< 640px):**
- 80vh height (slightly shorter)
- Content stacked vertically
- Buttons stack on very small screens (< 400px)
- Reduced spacing and font sizes

---

## Visual Design Preview

**Color Scheme:**
- Background: Very dark charcoal (#0f1116)
- Name: Pure white, large and bold
- Role: Light gray, medium weight
- Tagline: Muted gray, regular weight
- Primary button: Bright blue (#3b82f6)
- Secondary button: Outlined, gray border

**Typography Hierarchy:**
```
[YOUR NAME] ← 40-64px, Bold (H1)
Frontend Developer... ← 32-48px, Semibold (H2)
Building performant... ← 18-20px, Regular (paragraph)
```

**Spacing:**
- Between name and role: 16px
- Between role and tagline: 24px
- Between tagline and buttons: 40px
- Between buttons and social: 48px
- Button gap: 16px
- Social icon gap: 24px

---

## Animation Sequence

```
Time    Element         Animation
0ms     (page loads)    -
400ms   Name            Fade up from 20px below
500ms   Role            Fade up from 20px below
600ms   Tagline         Fade up from 20px below
700ms   CTA Buttons     Fade up from 20px below
800ms   Social Icons    Fade in (no movement)
```

**Animation Properties:**
- Duration: 400ms
- Easing: ease-out
- Transform: translateY(20px) → 0
- Opacity: 0 → 1

---

## AI Generation Prompt

### Copy and paste this entire prompt ⬇️

```
HIGH-LEVEL GOAL:
Create a responsive, minimal hero section for a web developer portfolio that immediately communicates value proposition with clear CTAs. The design should be modern, professional, and optimized for both recruiters and technical interviewers.

DETAILED INSTRUCTIONS:

1. Create a full-viewport-height hero section (min-height: 100vh) with centered content
2. Use a dark background (#0f1116) with white text for high contrast
3. Implement the following content hierarchy (top to bottom, centered):
   - Main headline with developer name (H1, large bold text)
   - Role/specialization subheadline (H2, medium weight)
   - Brief value proposition tagline (paragraph, lighter text)
   - Two CTA buttons side-by-side (primary and secondary)
   - Row of social icons (GitHub, LinkedIn, Email) below buttons
4. Add staggered fade-up animations on page load:
   - Name appears at 400ms
   - Role appears at 500ms
   - Tagline appears at 600ms
   - Buttons appear at 700ms
   - Social icons appear at 800ms
5. Make it fully responsive:
   - On mobile (< 640px): stack buttons vertically, reduce font sizes
   - On tablet (640-1024px): comfortable spacing
   - On desktop (> 1024px): max-width 1024px container, generous spacing
6. Add smooth hover states:
   - Primary button: lift up 2px, increase shadow, slight color shift
   - Secondary button: border color changes to accent color
   - Social icons: scale to 1.1, color shifts to accent

CODE EXAMPLES & CONSTRAINTS:

Color Palette (use exact values):
- Background: #0f1116
- Text Primary: #ffffff
- Text Secondary: #b4b8c5
- Text Muted: #6b7280
- Accent Primary: #3b82f6
- Accent Hover: #60a5fa

Content (customize with your info):
- Name: "Your Name"
- Role: "Frontend Developer specializing in AI Integration"
- Tagline: "Building performant, AI-powered web experiences with modern frameworks. Currently exploring Rust and WebAssembly."
- Primary CTA: "View Projects" (smooth scrolls to #projects)
- Secondary CTA: "Contact Me" (smooth scrolls to #contact or mailto:)
- Social Links: GitHub, LinkedIn, Email (use icon library like Lucide React or Heroicons)

Typography:
- H1: 3-4rem (desktop), 2-2.5rem (mobile), font-weight: 700
- H2: 1.5-2rem (desktop), 1.25-1.5rem (mobile), font-weight: 500
- Tagline: 1.125rem, font-weight: 400, max-width: 600px, line-height: 1.75
- Use system font stack: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif

Button Specifications:
- Primary: bg=#3b82f6, padding=1rem 2rem, rounded corners (0.5rem), white text, semibold
- Secondary: transparent bg, border=1px solid #404552, same padding/rounding, #b4b8c5 text
- Both: min-height 44px for accessibility, smooth transitions (250ms)

Animations:
- Use CSS keyframes for fade-up: translateY(20px) → 0, opacity 0 → 1
- Duration: 400ms, easing: ease-out
- Respect prefers-reduced-motion media query (disable animations if set)

DO NOT:
- Use any external CSS frameworks except Tailwind CSS (if tool supports it)
- Add unnecessary decorative elements or complex backgrounds
- Include auto-playing videos or heavy animations
- Use fixed pixel widths that break responsiveness
- Forget alt text or aria-labels for accessibility

STRICT SCOPE:

Create ONLY the hero section component. This should be:
- A single React component named `Hero.tsx` (or similar)
- Self-contained with all styles (inline Tailwind or CSS-in-JS)
- No dependencies on other components
- Exported as default for easy integration

DO NOT create:
- Navbar component (separate)
- Other page sections
- Routing logic
- Global styles (only hero-specific)
```

---

## Iteration Checklist

After generating, verify:

- [ ] Full viewport height on desktop
- [ ] All text is readable (good contrast)
- [ ] Buttons are at least 44x44px (touch-friendly)
- [ ] Animations work smoothly
- [ ] Hover states work on all interactive elements
- [ ] Mobile layout doesn't overflow horizontally
- [ ] Social links open in new tabs
- [ ] "View Projects" scrolls smoothly to projects section
- [ ] Respects reduced motion preference

---

## Common Refinements

If the AI output needs adjustment, use these follow-up prompts:

**"The animations are too fast"**
```
Slow down all animations to 600ms duration and use a gentler easing (ease-in-out).
```

**"Buttons are too close together on mobile"**
```
On screens smaller than 400px, stack the buttons vertically with 1rem gap between them.
```

**"Need more spacing at the top"**
```
Add padding-top: 4rem (64px) to push content away from navbar area.
```

---

## Ready to Build?

1. ✅ Copy the prompt above
2. ✅ Paste into v0.dev or Lovable
3. ✅ Review generated code
4. ✅ Test on mobile/tablet/desktop
5. ✅ Port to Dioxus when satisfied

**Next:** Once hero looks good, move to `02-projects-section.md`
