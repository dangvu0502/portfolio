# Navbar Component - AI Generation Guide

**Component:** Navigation Bar
**Priority:** High (Site navigation)
**Estimated Time:** 10-15 minutes

---

## Wireframe

```
┌─────────────────────────────────────────────────────────────┐
│  [Logo/Name]               Home  Projects  Contact  [GitHub] │
└─────────────────────────────────────────────────────────────┘
                              ↑
                   Fixed to top, semi-transparent
```

### Layout Details

**Desktop (> 1024px):**
- Fixed position at top of viewport
- Height: 64px
- Max-width: 1280px, centered with horizontal padding
- Logo/name on left, nav links centered/right
- Semi-transparent background with backdrop blur

**Mobile (< 640px):**
- Height: 56px (slightly shorter)
- Logo/name on left
- Essential links only (Home, Projects, Contact)
- Consider hamburger menu if more than 3 links
- Smaller font sizes

---

## Visual Design Preview

**Color Scheme:**
- Background: rgba(15, 17, 22, 0.8) with backdrop-blur-md
- Logo/Name: White (#ffffff), bold
- Nav Links: Light gray (#b4b8c5)
- Nav Links Hover: Bright blue (#3b82f6)
- Active Link: Bright blue with underline
- Border Bottom: Subtle (#2d3139)

**Typography Hierarchy:**
```
[LOGO/NAME] ← 20-24px, Bold (Brand)
Nav Links ← 16px, Medium (Navigation)
```

**Spacing:**
- Horizontal padding: 24px (mobile), 48px (desktop)
- Between nav links: 32px
- Logo to nav links: auto (flexbox space-between)
- Vertical padding: 16px

---

## Interaction States

```
State         Appearance
Default       Links in light gray (#b4b8c5)
Hover         Link color shifts to accent (#3b82f6), smooth transition
Active        Blue color with 2px underline below
Focus         2px blue outline ring for keyboard nav
Scrolled      May increase background opacity (optional)
```

**Interaction Properties:**
- Smooth scroll to sections on link click
- Active section highlighting based on viewport position
- Duration: 200ms
- Easing: ease-in-out

---

## AI Generation Prompt

### Copy and paste this entire prompt ⬇️

```
HIGH-LEVEL GOAL:
Create a fixed, responsive navigation bar for a portfolio website that provides quick access to key sections. The navbar should be minimal, professional, and stay visible at the top while scrolling. It should adapt seamlessly to mobile devices and include smooth scroll behavior.

DETAILED INSTRUCTIONS:

1. Create a fixed navbar component (position: fixed, top: 0, width: 100%) with semi-transparent background
2. Use dark background with backdrop blur effect: rgba(15, 17, 22, 0.8) + backdrop-filter: blur(12px)
3. Add subtle bottom border (1px solid #2d3139) to separate from content
4. Implement the following layout (left to right):
   - Logo/Brand name on left (your name or initials)
   - Navigation links in center or right (Home, Projects, Contact)
   - Optional: GitHub icon link on far right
5. Add smooth scroll behavior when clicking navigation links
6. Highlight active section based on scroll position
7. Responsive behavior:
   - Desktop (> 1024px): Full horizontal layout, generous spacing (32px between links)
   - Tablet (640-1024px): Comfortable spacing (24px between links)
   - Mobile (< 640px): Compact spacing (16px), consider hamburger if > 3 links
8. Add smooth hover states:
   - Nav links: color shifts from #b4b8c5 to #3b82f6
   - Logo: subtle scale (1.02) on hover
   - GitHub icon: scale (1.1) and color shift

CODE EXAMPLES & CONSTRAINTS:

Color Palette (use exact values):
- Background: rgba(15, 17, 22, 0.8)
- Backdrop Blur: 12px
- Border: 1px solid #2d3139
- Logo/Name: #ffffff
- Nav Links Default: #b4b8c5
- Nav Links Hover/Active: #3b82f6
- Focus Ring: #3b82f6

Content:
- Logo/Name: "Your Name" or "YN" (initials)
- Nav Links:
  - "Home" (scrolls to #hero or top)
  - "Projects" (scrolls to #projects)
  - "Contact" (scrolls to #contact)
- Optional GitHub icon on far right

Typography:
- Logo: 1.25-1.5rem, font-weight: 700
- Nav Links: 1rem, font-weight: 500
- Use system font stack: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif

Navbar Specifications:
- Height: 64px (desktop), 56px (mobile)
- Horizontal padding: 48px (desktop), 24px (mobile)
- z-index: 50 (stays above content)
- Max-width: 1280px, centered with margin: 0 auto
- Smooth transitions: 200ms ease-in-out for color changes

Active Link Indicator:
- Use 2px bottom border or underline in accent color (#3b82f6)
- Detect active section using Intersection Observer or scroll position
- Smooth transition when switching active links

Smooth Scroll Implementation:
- Use `scroll-behavior: smooth` in CSS or smooth scrolling JavaScript
- Offset scroll position by navbar height (64px) so content isn't hidden behind navbar
- Duration: 800ms, easing: ease-in-out

Accessibility:
- All links must have visible focus states (2px solid outline)
- Use semantic <nav> element
- Logo link should have aria-label="Home" or "Navigate to homepage"
- Ensure minimum touch target size (44x44px) on mobile

DO NOT:
- Use any external CSS frameworks except Tailwind CSS (if tool supports it)
- Add complex dropdown menus (keep it simple)
- Include search functionality (not needed for portfolio)
- Use hamburger menu unless absolutely necessary (3 links fit comfortably)
- Forget to handle z-index for navbar to stay above content

STRICT SCOPE:

Create ONLY the navbar component. This should be:
- A single React component named `Navbar.tsx` (or similar)
- Self-contained with all styles (inline Tailwind or CSS-in-JS)
- Fixed positioning with proper z-index
- Exported as default for easy integration

DO NOT create:
- Footer component (separate)
- Page sections (separate components)
- Routing logic (just smooth scroll to anchors)
- Global styles (only navbar-specific)
```

---

## Iteration Checklist

After generating, verify:

- [ ] Navbar stays fixed at top when scrolling
- [ ] Semi-transparent background with blur effect works
- [ ] All links scroll smoothly to their sections
- [ ] Active link is highlighted correctly
- [ ] Hover states work on all links
- [ ] Logo is clickable and returns to top
- [ ] Focus states visible for keyboard navigation
- [ ] Navbar doesn't cover content (sections have proper padding-top)
- [ ] Mobile layout fits all links comfortably
- [ ] GitHub icon (if included) opens in new tab

---

## Common Refinements

If the AI output needs adjustment, use these follow-up prompts:

**"Navbar covers content when scrolling to sections"**
```
Add scroll-margin-top: 80px to all section elements to offset scroll position by navbar height.
```

**"Background blur doesn't work"**
```
Ensure the navbar has backdrop-filter: blur(12px) and -webkit-backdrop-filter: blur(12px) for Safari support.
```

**"Active link detection doesn't work"**
```
Implement Intersection Observer to detect when sections enter viewport, and update active link state accordingly. Threshold should be 0.5 (50% visible).
```

**"Links too close together on mobile"**
```
Reduce horizontal spacing to 16px on screens smaller than 640px. Consider stacking vertically if more than 3 links.
```

**"Need hamburger menu for mobile"**
```
Add a hamburger icon button (3 horizontal lines) that toggles a mobile menu overlay. Menu should slide in from top or right with smooth animation (300ms).
```

---

## Integration Notes

**Section Padding:**
When integrating navbar with other sections, ensure the first section (Hero) has proper padding or margin to account for fixed navbar:

```css
/* Hero section should start below navbar */
.hero-section {
  padding-top: 64px; /* Desktop navbar height */
}

@media (max-width: 640px) {
  .hero-section {
    padding-top: 56px; /* Mobile navbar height */
  }
}

/* All sections should have scroll-margin for smooth scroll offset */
section {
  scroll-margin-top: 80px;
}
```

---

## Ready to Build?

1. ✅ Copy the prompt above
2. ✅ Paste into v0.dev or Lovable
3. ✅ Review generated code
4. ✅ Test smooth scrolling behavior
5. ✅ Verify active link detection
6. ✅ Port to Dioxus when satisfied

**Next:** Generate footer component with `06-footer.md`
