# Hero to Projects Spacing Fix

## Problem Statement
Current layout has excessive vertical gap (~300-400px) between Hero section and Featured Projects section, creating visual disconnection and poor flow.

## Visual Wireframe

```
┌─────────────────────────────────────────────────────────┐
│                     HERO SECTION                        │
│                                                         │
│          Frontend Developer specializing in AI          │
│     Building performant, AI-powered web experiences     │
│                                                         │
│         [View Projects]  [Contact Me]                   │
│                                                         │
│            [GitHub] [LinkedIn] [Email]                  │
│                                                         │
├─────────────────────────────────────────────────────────┤
│                   ↓                                     │  ← SCROLL INDICATOR
│              ┈┈┈┈┈┈┈┈┈┈┈                               │  ← SUBTLE DIVIDER (optional)
│                                                         │
│  ← 100px gap (reduced from 400px)                      │
│                                                         │
├─────────────────────────────────────────────────────────┤
│              Featured Projects                          │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐               │
│  │ AI Chat │  │E-Commerce│  │Portfolio│               │
│  │Assistant│  │Dashboard │  │Generator│               │
└─────────────────────────────────────────────────────────┘
```

## Implementation Instructions

### Step 1: Locate Current Spacing
**File:** `src/views/home.rs` or `assets/styling/main.css`

Find the CSS class that controls spacing between hero and projects sections.

**What to look for:**
- Class names like `.hero`, `.projects-section`, or container classes
- Properties: `margin-bottom`, `padding-bottom` (on hero) or `margin-top`, `padding-top` (on projects)
- Current values likely: `20rem`, `25rem`, or `400px`

### Step 2: Reduce Vertical Gap
**Change the spacing value:**

```css
/* BEFORE (example) */
.hero-section {
    padding-bottom: 25rem; /* TOO MUCH */
}

/* AFTER */
.hero-section {
    padding-bottom: 6rem; /* 96px - better flow */
}
```

**OR if spacing is on projects section:**

```css
/* BEFORE */
.projects-section {
    margin-top: 25rem;
}

/* AFTER */
.projects-section {
    margin-top: 6rem; /* 96px */
}
```

**Responsive adjustments:**
```css
/* Mobile */
@media (max-width: 768px) {
    .hero-section {
        padding-bottom: 4rem; /* 64px on mobile */
    }
}
```

### Step 3: Add Scroll Indicator (Optional but Recommended)

**Add to HTML/Rust component** (in `src/components/hero.rs`):

```rust
// Add this at the bottom of the hero section, after social icons

html! {
    <div class="scroll-indicator">
        <svg class="scroll-chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor">
            <polyline points="6 9 12 15 18 9"></polyline>
        </svg>
    </div>
}
```

**Add CSS** (in `assets/styling/main.css` or appropriate CSS file):

```css
.scroll-indicator {
    margin-top: 3rem;
    display: flex;
    justify-content: center;
    opacity: 0.5;
    animation: bounce 2s infinite;
}

.scroll-chevron {
    width: 2rem;
    height: 2rem;
    stroke-width: 2;
    color: #60a5fa; /* Matches your blue accent */
}

@keyframes bounce {
    0%, 20%, 50%, 80%, 100% {
        transform: translateY(0);
    }
    40% {
        transform: translateY(-10px);
    }
    60% {
        transform: translateY(-5px);
    }
}

/* Hide on mobile if needed */
@media (max-width: 768px) {
    .scroll-indicator {
        display: none;
    }
}
```

### Step 4: Optional Subtle Divider

Add after scroll indicator for visual separation:

```css
.section-divider {
    width: 100px;
    height: 1px;
    background: linear-gradient(
        90deg,
        transparent,
        rgba(96, 165, 250, 0.3),
        transparent
    );
    margin: 2rem auto;
}
```

```rust
// In hero.rs
html! {
    <div class="section-divider"></div>
}
```

## Testing Checklist

- [ ] Gap reduced to ~100px (6rem)
- [ ] Scroll indicator visible and animating
- [ ] Spacing looks good on desktop (1920px, 1440px, 1280px)
- [ ] Spacing looks good on tablet (768px, 1024px)
- [ ] Spacing looks good on mobile (375px, 414px)
- [ ] Visual flow feels natural when scrolling
- [ ] No awkward empty space
- [ ] Sections feel connected but distinct

## Expected Results

**Before:**
- 300-400px empty gap
- Disconnected feel
- User hesitation to scroll

**After:**
- Clean 96px breathing room
- Natural visual flow
- Animated scroll hint guides user
- Professional, polished appearance

## Quick Reference Values

| Screen Size | Recommended Gap |
|-------------|-----------------|
| Desktop     | 6rem (96px)     |
| Tablet      | 5rem (80px)     |
| Mobile      | 4rem (64px)     |

## Files to Modify

1. **CSS File:** `assets/styling/main.css` or `assets/tailwind.css`
   - Reduce gap spacing
   - Add scroll indicator styles
   - Add animation

2. **Hero Component:** `src/components/hero.rs`
   - Add scroll indicator HTML
   - Add optional divider

3. **Test:** Run `trunk serve` and check at multiple breakpoints

---

**Pro Tip:** Use browser DevTools to live-adjust spacing values before committing. Press F12, inspect the hero section, and modify padding/margin values in real-time to find the perfect spacing.
