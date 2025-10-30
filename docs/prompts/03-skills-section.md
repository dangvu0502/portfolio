# Skills/Tech Stack Section - AI Generation Guide

**Component:** Skills & Tech Stack Section
**Priority:** Medium (Shows breadth of knowledge)
**Estimated Time:** 15 minutes

---

## Wireframe

```
┌─────────────────────────────────────────────────────────────┐
│                                                               │
│                        Tech Stack                            │
│                                                               │
│  Frontend                                                     │
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌──────────────┐       │
│  │ React  │ │TypeScript│ │Next.js │ │ Tailwind CSS │       │
│  └────────┘ └──────────┘ └────────┘ └──────────────┘       │
│  ┌────────────┐ ┌──────┐                                     │
│  │ JavaScript │ │ HTML │                                     │
│  └────────────┘ └──────┘                                     │
│                                                               │
│  AI Integration                                               │
│  ┌───────────┐ ┌──────────┐ ┌─────────────────┐            │
│  │ OpenAI API│ │Langchain │ │Vector Databases │            │
│  └───────────┘ └──────────┘ └─────────────────┘            │
│  ┌─────┐ ┌────────────────────┐                             │
│  │ RAG │ │Prompt Engineering  │                             │
│  └─────┘ └────────────────────┘                             │
│                                                               │
│  Currently Learning                                           │
│  ┌──────┐ ┌────────┐ ┌─────────────┐                        │
│  │ Rust │ │ Dioxus │ │ WebAssembly │                        │
│  └──────┘ └────────┘ └─────────────┘                        │
│  ┌─────────────────────┐                                     │
│  │Systems Programming  │                                     │
│  └─────────────────────┘                                     │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Layout Details

**All Screen Sizes:**
- Centered content
- Max-width: 1024px
- Padding: 96px vertical, 24px horizontal
- Background: #1a1d24 (alternate from main sections)

**Category Layout:**
- Category title: Left-aligned
- Badges: Flex wrap, left-aligned
- Margin between categories: 40px

---

## Badge Design

```
Default State:
┌──────────────┐
│   React      │ ← Background: #24272f
└──────────────┘   Border: 1px solid #404552
                   Text: #b4b8c5
                   Padding: 12px 20px

Hover State:
┌──────────────┐
│   React      │ ← Border: #3b82f6 (blue)
└──────────────┘   Text: #ffffff (white)
                   Scale: 1.05
                   Transition: 150ms
```

**Badge Specifications:**
- Background: #24272f
- Border: 1px solid #404552
- Padding: 0.75rem 1.25rem (12px 20px)
- Border radius: 0.5rem (8px)
- Font size: 1rem (16px)
- Font weight: 500 (medium)
- Gap: 1rem (16px) between badges
- Flex wrap enabled

---

## Category Structure

### Frontend (Primary Expertise)
Skills to include:
- React
- TypeScript
- Next.js
- Tailwind CSS
- JavaScript
- HTML/CSS

### AI Integration (Specialization)
Skills to include:
- OpenAI API
- Langchain
- Vector Databases
- RAG (Retrieval-Augmented Generation)
- Prompt Engineering

### Currently Learning (Growth Mindset)
Skills to include:
- Rust
- Dioxus
- WebAssembly
- Systems Programming

---

## AI Generation Prompt

### Copy and paste this entire prompt ⬇️

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
- Keyboard focusable if badges will be interactive in future
- Consider adding aria-label to section: "Technical skills and expertise"

DO NOT:
- Add skill level indicators (bars, percentages) - they're subjective and distracting
- Use company logos instead of text (less accessible, harder to maintain)
- Include endorsement counts or certifications (belongs on LinkedIn/resume)
- Add year of experience per skill (focus on what you know, not how long)
- Create more than 3-4 categories (causes decision paralysis for reader)

STRICT SCOPE:

Create:
1. SkillsSection component - Main section
2. SkillBadge component - Individual badge (reusable)
3. Skills data structure (inline or separate file)

DO NOT create:
- Skill filtering functionality (not needed for portfolio v1)
- Tooltips with skill descriptions (keep it simple)
- Links to projects per skill (v2 feature)
- Skill endorsement system
```

---

## Iteration Checklist

After generating, verify:

- [ ] Section has alternate background color (#1a1d24)
- [ ] Section title is centered
- [ ] 3 categories are clearly separated
- [ ] Category titles are left-aligned
- [ ] Badges wrap to multiple rows gracefully
- [ ] Hover effect works (scale + border color change)
- [ ] Focus states visible for keyboard navigation
- [ ] Mobile layout doesn't break
- [ ] No more than 6-8 badges per category
- [ ] Text is readable (good contrast)

---

## Common Refinements

**"Too many badges in one category"**
```
Prioritize the 5-6 most relevant technologies and remove the rest. More isn't better.
```

**"Badges look too cramped"**
```
Increase gap between badges to 1.25rem (20px) for more breathing room.
```

**"Category titles blend in"**
```
Increase font-weight to 700 (bold) and add margin-top: 0.5rem to category titles.
```

**"Want to make badges clickable to filter projects"**
```
That's a v2 feature. For now, keep it simple and non-interactive.
```

---

## Content Tips

**What to include:**
- Technologies you're actively using
- Skills relevant to job you're seeking
- Recent learning (shows growth)
- Technologies in your portfolio projects

**What NOT to include:**
- Every tool you've touched once
- Outdated technologies (unless job-specific)
- Soft skills (those go in resume/interview)
- Generic terms like "Problem Solving"

---

## Ready to Build?

1. ✅ Copy the prompt above
2. ✅ Paste into v0.dev or Lovable
3. ✅ Customize skills to match your actual expertise
4. ✅ Test hover interactions
5. ✅ Port to Dioxus when satisfied

**Next:** Once skills look good, move to `04-contact-section.md`
