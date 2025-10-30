# AI Frontend Generation Prompts

**Purpose:** Masterful prompts for AI-driven frontend tools (v0, Lovable, Bolt, etc.)
**Project:** Portfolio Website
**Tech Stack:** React/TypeScript for prototyping (to be ported to Dioxus/Rust)
**Created:** October 29, 2025

---

## Usage Instructions

These prompts follow the **Structured Prompting Framework**:

1. **High-Level Goal** - Clear objective statement
2. **Detailed Instructions** - Step-by-step breakdown
3. **Code Examples & Constraints** - Concrete specifications and boundaries
4. **Strict Scope** - What to create and what NOT to touch

**How to use:**
1. Copy the entire prompt for a section
2. Paste into your AI tool (v0.dev, Lovable, etc.)
3. Review generated code
4. Iterate with follow-up prompts if needed
5. Port final design to Dioxus/Rust for production

**Important:** All AI-generated code requires human review, testing, and refinement before production use.

---

## Prompt 1: Hero Section

### Copy This Prompt ⬇️

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

**Why this prompt works:**
- Provides exact color values (prevents guessing)
- Specifies animation timing (avoids generic implementations)
- Includes accessibility requirements (focus states, min-height for touch)
- Defines clear boundaries (only hero section, nothing else)

---

## Prompt 2: Project Showcase Section

### Copy This Prompt ⬇️

```
HIGH-LEVEL GOAL:
Create a responsive project showcase section with a grid of project cards. Each card displays a project thumbnail, title, description, tech stack badges, and action links. Design should be minimal, professional, and optimized for showcasing 3-5 featured projects to potential employers.

DETAILED INSTRUCTIONS:

1. Create a section with ID "projects" for smooth scroll navigation
2. Add a centered section title "Featured Projects" (H2, bold, white text)
3. Implement a responsive grid layout:
   - Mobile (< 640px): 1 column
   - Tablet (640px - 1024px): 2 columns
   - Desktop (> 1024px): 3 columns
   - Gap between cards: 1.5rem (24px)
4. Create a ProjectCard component with the following structure:
   - Project thumbnail image (aspect ratio 16:9, height ~200px)
   - Content area with padding:
     * Project title (H3, semibold, white)
     * Description (2-3 lines, gray text, line-clamp)
     * Tech stack badges (flex wrap, small pills)
     * Action buttons (Live Demo + GitHub)
5. Add hover effects to cards:
   - Entire card lifts up 4px (translateY)
   - Shadow increases (more prominent)
   - Border color shifts from subtle gray to accent blue
   - Smooth transition (250ms ease-in-out)
6. Make cards keyboard accessible:
   - Visible focus ring on tab navigation
   - Links are independently focusable
7. Use an array of project data to generate multiple cards
8. Implement loading="lazy" for images (performance optimization)
9. Ensure all links open in new tabs with rel="noopener noreferrer"

CODE EXAMPLES & CONSTRAINTS:

Project Data Structure:
```typescript
interface Project {
  id: number;
  title: string;
  description: string;
  thumbnail: string; // URL or path
  techStack: string[]; // e.g., ["React", "TypeScript", "Tailwind CSS"]
  liveUrl?: string;   // Optional
  githubUrl?: string; // Optional
}

// Example data:
const projects: Project[] = [
  {
    id: 1,
    title: "AI Chat Assistant",
    description: "Real-time chat interface with OpenAI integration, streaming responses, and conversation history.",
    thumbnail: "/projects/ai-chat.jpg",
    techStack: ["React", "TypeScript", "OpenAI API", "Tailwind CSS"],
    liveUrl: "https://example.com/demo",
    githubUrl: "https://github.com/username/ai-chat"
  },
  // ... add 2-4 more projects
];
```

Card Design Specifications:
- Background: #24272f (elevated surface)
- Border: 1px solid #2d3139 (subtle)
- Border on hover: 1px solid #3b82f6 (accent)
- Border radius: 0.75rem (12px)
- Overflow: hidden (for thumbnail)
- Shadow default: 0 4px 6px rgba(0,0,0,0.4)
- Shadow hover: 0 20px 25px rgba(0,0,0,0.6)

Tech Badge Styling:
- Background: #1a1d24
- Border: 1px solid #404552
- Padding: 0.25rem 0.75rem (4px 12px)
- Border radius: 0.25rem (4px)
- Font size: 0.75rem (12px)
- Color: #6b7280 (muted gray)
- Gap between badges: 0.5rem (8px)

Button Styling:
- Live Demo: background=#3b82f6, text=white, padding=0.5rem 1rem
- GitHub: background=transparent, border=1px solid #404552, text=#b4b8c5, same padding
- Both: rounded (0.375rem), semibold font, smooth transitions
- Hover: Live Demo → #60a5fa, GitHub → border=#3b82f6 + text=#ffffff

Section Layout:
- Padding: 6rem vertical (96px), 1.5rem horizontal (24px)
- Max-width: 1280px (centered)
- Background: #0f1116 (matches page background)

Typography:
- Section title: 2-3rem, font-weight: 700, margin-bottom: 3rem
- Project title: 1.25rem, font-weight: 600, margin-bottom: 0.75rem
- Description: 1rem, font-weight: 400, line-height: 1.5, color: #b4b8c5
- Limit description to 3 lines with text-clamp or ellipsis

Accessibility:
- All images need alt text: "Screenshot of [Project Name]"
- Focus-visible on all interactive elements
- Proper heading hierarchy (section uses H2, cards use H3)
- ARIA labels if needed (e.g., "View live demo of [Project Name]")

DO NOT:
- Use carousel/slider (prefer static grid - better UX)
- Add pagination (3-5 projects is optimal, no more needed)
- Include project filtering (v1 doesn't need this complexity)
- Use complex animations that hurt performance
- Forget responsive behavior (must work on all screen sizes)

STRICT SCOPE:

Create:
1. `ProjectsSection.tsx` - Main section wrapper component
2. `ProjectCard.tsx` - Individual card component
3. Sample project data (inline or separate file)

DO NOT create:
- Navbar or hero components
- Modal/lightbox for project details (v2 feature)
- Backend API integration (static data for now)
- Project detail pages (cards link directly to live demos)
```

**Why this prompt works:**
- Provides complete TypeScript interface (prevents type mismatches)
- Specifies exact hover behavior (avoids generic implementations)
- Includes responsive breakpoints with specific column counts
- Explains rationale for decisions (e.g., no carousel = better UX)

---

## Prompt 3: Skills/Tech Stack Section

### Copy This Prompt ⬇️

```
HIGH-LEVEL GOAL:
Create a clean, scannable tech stack section that displays your technical skills organized by category. Design should be minimal with interactive badge components that are both visually appealing and easily readable by recruiters and technical interviewers.

DETAILED INSTRUCTIONS:

1. Create a section with ID "skills" for smooth scroll navigation
2. Add a centered section title "Tech Stack" (H2, bold, white text)
3. Organize skills into 3 categories:
   - "Frontend" (primary expertise)
   - "AI Integration" (specialization)
   - "Currently Learning" (shows growth mindset)
4. For each category:
   - Display category name as H3 (semibold, left-aligned)
   - Show skills as a flex-wrapped row of badge components
   - Use consistent spacing between badges and categories
5. Create interactive skill badges:
   - Default state: neutral background with border
   - Hover state: border color changes to accent, subtle scale up (1.05)
   - Smooth transition (150ms)
6. Make the section visually distinct:
   - Use alternate background color (#1a1d24) to break up the page
   - Adequate padding for breathing room
7. Keep badge count reasonable (5-8 per category max)
8. Ensure badges are keyboard accessible with visible focus states

CODE EXAMPLES & CONSTRAINTS:

Skills Data Structure:
```typescript
interface SkillCategory {
  category: string;
  skills: string[];
}

const skillCategories: SkillCategory[] = [
  {
    category: "Frontend",
    skills: ["React", "TypeScript", "Next.js", "Tailwind CSS", "HTML/CSS", "JavaScript"]
  },
  {
    category: "AI Integration",
    skills: ["OpenAI API", "Langchain", "Vector Databases", "RAG", "Prompt Engineering"]
  },
  {
    category: "Currently Learning",
    skills: ["Rust", "Dioxus", "WebAssembly", "Systems Programming"]
  }
];
```

Section Design:
- Background: #1a1d24 (secondary background, contrasts with #0f1116)
- Padding: 6rem vertical, 1.5rem horizontal
- Max-width: 1024px (slightly narrower than projects for better readability)
- Center aligned

Category Layout:
- Margin-bottom between categories: 2.5rem (40px)
- Category title color: #ffffff
- Category title font-size: 1.25rem (20px), font-weight: 600
- Margin-bottom after title: 1.25rem (20px)

Skill Badge Design:
- Background: #24272f (elevated surface)
- Border: 1px solid #404552 (medium border)
- Padding: 0.75rem 1.25rem (12px 20px)
- Border radius: 0.5rem (8px)
- Font size: 1rem (16px)
- Font weight: 500 (medium)
- Color: #b4b8c5 (secondary text)
- Gap between badges: 1rem (16px)

Hover State:
- Border color: #3b82f6 (accent primary)
- Text color: #ffffff (white)
- Transform: scale(1.05)
- Transition: all 150ms ease-out
- Cursor: pointer (even if not clickable, for visual feedback)

Focus State (keyboard navigation):
- Outline: 2px solid #3b82f6
- Outline offset: 2px

Typography:
- Section title: 2-3rem, font-weight: 700, margin-bottom: 3rem, centered
- Category title: 1.25rem, font-weight: 600, left-aligned
- Badge text: 1rem, font-weight: 500

Responsive Behavior:
- Mobile (< 640px): Reduce padding to 3rem vertical, badges may wrap to multiple lines
- Tablet: Comfortable spacing as specified
- Desktop: Full spacing, max-width constraint

Accessibility:
- Use semantic HTML (section, h2, h3, ul/li for badges if appropriate)
- Ensure badges have adequate color contrast (already meets WCAG AA)
- Keyboard focusable if badges will be interactive in future (e.g., filtering projects)
- Consider adding aria-label to section: "Technical skills and expertise"

DO NOT:
- Add skill level indicators (bars, percentages) - they're subjective and distracting
- Use company logos instead of text (less accessible, harder to maintain)
- Include endorsement counts or certifications (belongs on LinkedIn/resume)
- Add year of experience per skill (focus on what you know, not how long)
- Create more than 3-4 categories (causes decision paralysis for reader)

STRICT SCOPE:

Create:
1. `SkillsSection.tsx` - Main section component
2. `SkillBadge.tsx` - Individual badge component (reusable)
3. Skills data structure (inline or separate file)

DO NOT create:
- Skill filtering functionality (not needed for portfolio v1)
- Tooltips with skill descriptions (keep it simple)
- Links to projects per skill (v2 feature)
- Skill endorsement system
```

**Why this prompt works:**
- Specifies exactly 3 categories (prevents scope creep)
- Explains what NOT to include with rationale (avoids anti-patterns)
- Provides complete data structure (easy to customize)
- Includes hover states that add polish without complexity

---

## Prompt 4: Contact/CTA Section

### Copy This Prompt ⬇️

```
HIGH-LEVEL GOAL:
Create a compelling, conversion-focused contact section that makes it effortless for hiring managers to reach you. Design should feature a clear headline, brief availability statement, and prominent contact methods (email, LinkedIn, GitHub) with an emphasis on the email CTA.

DETAILED INSTRUCTIONS:

1. Create a section with ID "contact" for smooth scroll navigation
2. Center all content vertically and horizontally
3. Implement the following content hierarchy:
   - Attention-grabbing headline (e.g., "Let's Work Together")
   - Brief subtext about availability/interest
   - Row of contact method buttons (Email as primary, LinkedIn/GitHub as secondary)
4. Make the email button the most prominent:
   - Larger size, solid accent background
   - Opens mailto: link with subject line pre-filled
5. Style secondary buttons (LinkedIn, GitHub) as outlined/ghost style
6. Add icon + text to all buttons for clarity
7. Ensure all external links open in new tabs
8. Include smooth hover effects with spring-like animation
9. Make mobile-friendly (stack buttons on small screens if needed)

CODE EXAMPLES & CONSTRAINTS:

Section Design:
- Background: #0f1116 (matches main background)
- Padding: 6rem vertical, 1.5rem horizontal
- Max-width: 768px (narrower for focused content)
- Text-align: center

Typography:
- Headline (H2): 2-3rem, font-weight: 700, color: #ffffff, margin-bottom: 1rem
- Subtext (p): 1.125rem, font-weight: 400, color: #b4b8c5, line-height: 1.75, max-width: 600px, margin-bottom: 2.5rem

Content (customize with your info):
- Headline: "Let's Work Together" or "Get In Touch"
- Subtext: "I'm currently seeking full-time opportunities in frontend development. Open to remote or [location] positions."
- Email: your.email@example.com
- LinkedIn: https://linkedin.com/in/yourprofile
- GitHub: https://github.com/yourusername

Email Button (Primary CTA):
- Background: #3b82f6 (accent primary)
- Padding: 1rem 2rem (16px 32px)
- Border radius: 0.5rem (8px)
- Font size: 1.125rem (18px)
- Font weight: 600 (semibold)
- Color: #ffffff
- Display: inline-flex, align-items: center, gap: 0.75rem
- Icon: Email/mail icon (24px)
- Box shadow: 0 4px 6px rgba(0,0,0,0.4)
- Hover: background → #60a5fa, shadow increases, scale(1.02)
- Transition: all 250ms cubic-bezier(0.34, 1.56, 0.64, 1) /* spring easing */

Mailto link format:
```html
<a href="mailto:your.email@example.com?subject=Opportunity from Portfolio">
```

Secondary Buttons (LinkedIn, GitHub):
- Background: transparent
- Border: 1px solid #404552
- Same padding/rounding as primary
- Font size: 1rem (16px)
- Color: #b4b8c5
- Gap between text and icon: 0.5rem
- Hover: border-color → #3b82f6, color → #ffffff
- Transition: all 250ms ease-in-out

Button Container:
- Display: flex
- Gap: 1rem (16px) between buttons
- Justify-content: center
- Flex-wrap: wrap (for mobile)

Icons:
- Use Lucide React or Heroicons
- Email: Mail or Envelope icon
- LinkedIn: Brand icon or external link
- GitHub: Brand icon or code/terminal icon
- Size: 20-24px, inline with text

Responsive:
- Desktop: All buttons in a row
- Mobile (< 640px): Stack vertically (flex-direction: column), full width buttons (max-width: 320px)

Accessibility:
- All buttons are proper <a> tags with href
- Descriptive text + icon (don't rely on icon alone)
- Focus-visible ring: 2px solid #3b82f6, offset 2px
- ARIA labels if needed: "Email me at [address]", "Connect on LinkedIn", "View GitHub profile"

DO NOT:
- Include a contact form (adds complexity, maintenance, spam issues - direct email is better)
- Add phone number (unless explicitly desired - most tech hiring is email-first)
- Include social media beyond professional networks (Twitter, etc. - keep it focused)
- Use vague "Submit" buttons (be explicit about action)
- Auto-scroll or force interaction (let user decide)

STRICT SCOPE:

Create:
1. `ContactSection.tsx` - Complete contact section component
2. Include all buttons and text inline (no separate components needed)

DO NOT create:
- Contact form with validation
- Backend API for form submission
- Email verification logic
- Newsletter signup (out of scope)
```

**Why this prompt works:**
- Specifies mailto: format with pre-filled subject (improves conversion)
- Explains why NOT to include contact form (prevents overengineering)
- Provides spring easing function (adds polish)
- Clear mobile behavior (stack vertically vs. overflow)

---

## Prompt 5: Complete Layout Integration

### Copy This Prompt ⬇️

```
HIGH-LEVEL GOAL:
Create a complete single-page portfolio layout that integrates all components (Navbar, Hero, Projects, Skills, Contact, Footer) with smooth scroll navigation, consistent spacing, and responsive behavior. Include a fixed navbar with scroll-triggered backdrop blur and active section highlighting.

DETAILED INSTRUCTIONS:

1. Create a main App component that composes all sections
2. Implement a fixed navbar at the top with:
   - Logo/name on the left
   - Navigation links on the right (Projects, Contact)
   - Backdrop blur effect when scrolled past hero
   - Active section indicator (underline on current section)
3. Add smooth scroll behavior for anchor links
4. Stack all sections vertically: Hero → About (optional) → Projects → Skills → Contact → Footer
5. Use Intersection Observer to detect which section is in viewport (for active nav highlighting)
6. Ensure consistent spacing between sections
7. Add a minimal footer with:
   - "Built with [Tech Stack]" credit
   - Social links (duplicate from hero)
   - Copyright notice
8. Implement scroll-triggered fade-in animations for sections as they enter viewport
9. Test responsive behavior across all breakpoints

CODE EXAMPLES & CONSTRAINTS:

Navbar Design:
- Position: fixed, top: 0, z-index: 50
- Height: 64px (4rem)
- Background: rgba(15, 17, 22, 0.8) with backdrop-filter: blur(12px)
- Border-bottom: 1px solid #2d3139
- Padding: 0 3rem (48px) on desktop, 0 1.5rem (24px) on mobile
- Display: flex, justify-content: space-between, align-items: center

Logo/Name:
- Font size: 1.125rem (18px)
- Font weight: 600 (semibold)
- Color: #ffffff
- Clicking returns to top (smooth scroll)

Nav Links:
- Font size: 0.875rem (14px)
- Font weight: 500 (medium)
- Color: #b4b8c5 (secondary text)
- Gap: 2rem (32px) between links
- Hover: color → #ffffff
- Active: 2px underline, color: #3b82f6, offset 4px below text
- Smooth transitions: all 200ms ease-in-out

Smooth Scroll Implementation:
```css
html {
  scroll-behavior: smooth;
}

/* Respect user preference */
@media (prefers-reduced-motion: reduce) {
  html {
    scroll-behavior: auto;
  }
}
```

Active Section Detection (pseudo-code):
```typescript
// Use Intersection Observer to track which section is visible
const sections = ['hero', 'projects', 'skills', 'contact'];
// When section enters viewport (threshold ~0.3), update active state
// Highlight corresponding nav link
```

Section Spacing:
- Hero: min-height 100vh, no top margin (navbar overlays it)
- All other sections: padding-top adjusted to account for fixed navbar (add ~80px)
- Consistent vertical padding: 6rem (96px) on desktop, 4rem (64px) on mobile

Footer Design:
- Background: #1a1d24 (secondary background)
- Border-top: 1px solid #2d3139
- Padding: 2rem vertical (32px)
- Text-align: center
- Font-size: 0.875rem (14px)
- Color: #6b7280 (muted)

Footer Content:
```
Built with Dioxus + Rust • © 2025
[GitHub Icon] [LinkedIn Icon] [Email Icon]
```

Layout Structure:
```tsx
<div className="min-h-screen bg-[#0f1116] text-white">
  <Navbar />
  <main>
    <Hero />
    <ProjectsSection />
    <SkillsSection />
    <ContactSection />
  </main>
  <Footer />
</div>
```

Scroll-triggered Animations:
- Use Intersection Observer with threshold: 0.1
- When section enters viewport, add class that triggers fade-in
- Animation: translateY(20px) → 0, opacity 0 → 1, duration 600ms, ease-out
- Stagger cards within Projects section (50ms delay each)

Responsive Navbar:
- Desktop: Full navbar as specified
- Mobile (< 640px):
  - Reduce horizontal padding to 1rem
  - Consider hamburger menu if links exceed 2-3 items
  - Logo font-size: 1rem

Accessibility:
- Semantic HTML: <nav>, <main>, <section>, <footer>
- Skip to main content link (optional but nice):
  ```html
  <a href="#main" class="sr-only focus:not-sr-only">Skip to main content</a>
  ```
- Proper heading hierarchy (only one H1 in Hero)
- Focus management: when clicking nav link, focus should move to section
- Keyboard navigation: all interactive elements reachable via Tab

Performance:
- Lazy load images in Projects section
- Debounce scroll event listeners if used (or prefer Intersection Observer)
- Minimize bundle size: only import icons that are used
- Defer non-critical CSS

DO NOT:
- Add page transitions or route animations (single page, no routing)
- Implement complex parallax effects (hurts performance, often distracting)
- Include loading spinners (page should load fast enough without them)
- Add animated backgrounds or particle effects (contradicts minimal aesthetic)
- Create mobile app-style bottom navigation (navbar is sufficient)

STRICT SCOPE:

Create:
1. `App.tsx` - Main layout component
2. `Navbar.tsx` - Fixed navigation component
3. `Footer.tsx` - Simple footer component
4. Import and compose all other sections (Hero, Projects, Skills, Contact)
5. Add scroll behavior and Intersection Observer logic

DO NOT create:
- Routing system (not needed for single page)
- State management library (simple useState is sufficient)
- Complex animation library (use CSS transitions/keyframes)
- Backend integration (static site for v1)
```

**Why this prompt works:**
- Provides complete layout structure (easy to visualize)
- Specifies exact navbar behavior (scroll-triggered backdrop)
- Includes Intersection Observer guidance (active section detection)
- Addresses performance explicitly (lazy loading, debouncing)

---

## Bonus: Iteration Prompts

Once you have generated initial components, use these follow-up prompts to refine:

### Refinement Prompt Template

```
REFINE: [Component Name]

I need to adjust the [specific element] in the [Component Name].

Current behavior: [describe what it currently does]
Desired behavior: [describe what you want instead]

Constraints:
- Keep the existing [color scheme/layout/structure]
- Don't change [specific parts that are working]
- Maintain accessibility standards (focus states, ARIA labels)

Example: "The project card hover effect is too aggressive. Reduce the lift from 8px to 4px and make the shadow more subtle."
```

### Mobile Optimization Prompt

```
OPTIMIZE: Mobile Experience

Review the [Component Name] and optimize for mobile devices (320px - 640px width).

Checklist:
1. Reduce font sizes appropriately (maintain readability)
2. Ensure touch targets are minimum 44x44px
3. Stack elements vertically where horizontal layout is cramped
4. Reduce vertical spacing (mobile screens are taller than wide)
5. Test text wrapping (avoid orphans, ensure line breaks are logical)
6. Simplify animations (reduce motion for performance)
7. Check that nothing requires horizontal scrolling

Don't change: [desktop version, color scheme, content]
```

### Accessibility Audit Prompt

```
AUDIT: Accessibility Compliance

Review the [Component Name] for WCAG 2.1 Level AA compliance.

Check for:
1. Color contrast ratios (4.5:1 for normal text, 3:1 for large text)
2. Keyboard navigation (all interactive elements focusable, logical tab order)
3. Focus indicators (visible, at least 2px outline with offset)
4. Screen reader support (semantic HTML, ARIA labels, alt text)
5. Heading hierarchy (no skipped levels)
6. Touch target sizes (44x44px minimum on mobile)
7. Reduced motion support (prefers-reduced-motion media query)

Fix any violations and explain the changes made.
```

---

## Important Reminders

**Before Porting to Dioxus:**
1. Test generated React components thoroughly in browser
2. Validate responsive behavior on real devices
3. Run Lighthouse audit for performance and accessibility
4. Get feedback from 2-3 people on design
5. Screenshot or export final designs for reference

**When Porting to Dioxus:**
- Convert JSX to Dioxus `rsx!` macro syntax
- Replace React hooks with Dioxus signals/hooks
- Port Tailwind classes to Dioxus class syntax
- Test WASM bundle size (aim for < 200KB gzipped)
- Ensure all interactions work identically

**All AI-generated code requires:**
- Human review for logic errors
- Manual testing for edge cases
- Accessibility validation with real tools
- Performance profiling on target devices
- Security review (especially for any future forms/API calls)

---

**Good luck building! Remember: ship fast, iterate based on real feedback.**
