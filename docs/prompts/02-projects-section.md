# Projects Section - AI Generation Guide

**Component:** Projects Showcase Section
**Priority:** High (Core portfolio content)
**Estimated Time:** 25-30 minutes

---

## Wireframe

```
┌─────────────────────────────────────────────────────────────┐
│                                                               │
│                    Featured Projects                         │
│                                                               │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │            │  │            │  │            │            │
│  │  [Image]   │  │  [Image]   │  │  [Image]   │            │
│  │            │  │            │  │            │            │
│  ├────────────┤  ├────────────┤  ├────────────┤            │
│  │Project 1   │  │Project 2   │  │Project 3   │            │
│  │Description │  │Description │  │Description │            │
│  │here...     │  │here...     │  │here...     │            │
│  │            │  │            │  │            │            │
│  │[Tech] [JS] │  │[Tech] [TS] │  │[Tech] [AI] │            │
│  │[React] ... │  │[Next] ...  │  │[API] ...   │            │
│  │            │  │            │  │            │            │
│  │[Live][Git] │  │[Live][Git] │  │[Live][Git] │            │
│  └────────────┘  └────────────┘  └────────────┘            │
│                                                               │
│  ┌────────────┐  ┌────────────┐                             │
│  │            │  │            │                             │
│  │  [Image]   │  │  [Image]   │                             │
│  │            │  │            │                             │
│  ├────────────┤  ├────────────┤                             │
│  │Project 4   │  │Project 5   │                             │
│  │...         │  │...         │                             │
│  └────────────┘  └────────────┘                             │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Layout Details

**Desktop (> 1024px):**
- 3-column grid
- Grid gap: 24px
- Max-width: 1280px container
- Equal height cards

**Tablet (640-1024px):**
- 2-column grid
- Grid gap: 20px
- Padding: 24px horizontal

**Mobile (< 640px):**
- 1-column layout
- Cards full width (minus padding)
- Padding: 16px horizontal

---

## Card Structure

```
┌─────────────────────┐
│                     │
│   Project Image     │ ← 16:9 aspect ratio, ~200px height
│   (16:9 ratio)      │
│                     │
├─────────────────────┤
│                     │
│ Project Title       │ ← H3, semibold, white
│                     │
│ Short description   │ ← 2-3 lines max
│ about the project   │   Gray text
│ and what it does.   │
│                     │
│ [React] [TypeScript]│ ← Tech badges
│ [Tailwind] [API]    │   Wrap to multiple rows
│                     │
│ [Live Demo] [GitHub]│ ← Action buttons
│                     │
└─────────────────────┘
```

**Card Hover Effect:**
```
Default State:
- Border: 1px solid #2d3139 (subtle gray)
- Shadow: Medium drop shadow
- Transform: none

Hover State:
- Lift up 4px (translateY(-4px))
- Border: 1px solid #3b82f6 (blue accent)
- Shadow: Large, more prominent
- Smooth transition (250ms)
```

---

## Component Breakdown

### Main Section
- ID: `projects` (for smooth scroll)
- Background: #0f1116 (matches page)
- Padding: 96px vertical, 24px horizontal
- Section title: "Featured Projects"

### Project Card
- Background: #24272f (elevated surface)
- Border: 1px solid #2d3139
- Border radius: 12px (0.75rem)
- Overflow: hidden (for image)
- Padding: 24px (content area, not image)

### Project Image
- Aspect ratio: 16:9
- Height: 200px
- Object-fit: cover
- Loading: lazy (performance)
- Placeholder: Gray background while loading

### Tech Badges
- Background: #1a1d24
- Border: 1px solid #404552
- Padding: 4px 12px
- Border radius: 4px
- Font size: 12px (0.75rem)
- Color: #6b7280 (muted)
- Gap: 8px between badges
- Limit: 3-5 tags per project

### Action Buttons
- **Live Demo:** Solid blue (#3b82f6), white text
- **GitHub:** Ghost style, gray border (#404552), gray text
- Both: Padding 8px 16px, rounded corners
- Icons + text (use Lucide or Heroicons)
- Open in new tab with rel="noopener noreferrer"

---

## Sample Data Structure

```typescript
interface Project {
  id: number;
  title: string;
  description: string;
  thumbnail: string;
  techStack: string[];
  liveUrl?: string;
  githubUrl?: string;
}

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
  // Add 2-4 more projects...
];
```

---

## AI Generation Prompt

### Copy and paste this entire prompt ⬇️

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
  thumbnail: string;
  techStack: string[];
  liveUrl?: string;
  githubUrl?: string;
}

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
  {
    id: 2,
    title: "E-Commerce Dashboard",
    description: "Admin dashboard for managing products, orders, and analytics with real-time updates.",
    thumbnail: "/projects/dashboard.jpg",
    techStack: ["Next.js", "TypeScript", "Prisma", "PostgreSQL"],
    liveUrl: "https://example.com/dashboard",
    githubUrl: "https://github.com/username/dashboard"
  },
  {
    id: 3,
    title: "Portfolio Generator",
    description: "Tool to generate beautiful developer portfolios from GitHub profiles using AI.",
    thumbnail: "/projects/portfolio-gen.jpg",
    techStack: ["React", "OpenAI", "Tailwind", "Vite"],
    liveUrl: "https://example.com/portfolio-gen",
    githubUrl: "https://github.com/username/portfolio-gen"
  }
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
- Limit description to 3 lines with line-clamp or ellipsis

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
1. ProjectsSection component - Main section wrapper
2. ProjectCard component - Individual card
3. Sample project data (at least 3 projects)

DO NOT create:
- Modal/lightbox for project details (v2 feature)
- Backend API integration (static data for now)
- Project detail pages (cards link directly to live demos)
- Search or filter functionality
```

---

## Iteration Checklist

After generating, verify:

- [ ] Grid displays correctly (3 cols → 2 cols → 1 col)
- [ ] Cards have equal heights in each row
- [ ] Images load lazily
- [ ] Hover effect lifts card smoothly
- [ ] Border changes to blue on hover
- [ ] Tech badges wrap nicely
- [ ] Both buttons are clickable
- [ ] Links open in new tabs
- [ ] Alt text on all images
- [ ] Mobile layout looks clean
- [ ] No horizontal scrolling

---

## Common Refinements

**"Cards have different heights"**
```
Set display: grid on the parent container and use grid-auto-rows: 1fr to make all cards equal height.
```

**"Images take too long to load"**
```
Add loading="lazy" to all img tags and use optimized WebP images with JPEG fallback.
```

**"Need more projects visible"**
```
Increase max-width to 1400px and adjust grid-template-columns to fit 4 columns on large screens.
```

**"Description text overflows"**
```
Use CSS line-clamp to limit description to 3 lines:
display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden;
```

---

## Ready to Build?

1. ✅ Copy the prompt above
2. ✅ Paste into v0.dev or Lovable
3. ✅ Replace sample project data with your real projects
4. ✅ Test responsive behavior
5. ✅ Port to Dioxus when satisfied

**Next:** Once projects look good, move to `03-skills-section.md`
