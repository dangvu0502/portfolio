# Portfolio AI Prompts - Quick Start Guide

Welcome! This directory contains **5 ready-to-use AI prompts** to generate your portfolio UI components.

---

## 📁 Files Overview

| File | Component | Priority | Time |
|------|-----------|----------|------|
| `00-navbar.md` | Fixed navigation bar | ⭐⭐⭐ High | 10 min |
| `01-hero-section.md` | Hero with name, role, CTAs | ⭐⭐⭐ High | 15 min |
| `02-projects-section.md` | Project cards grid | ⭐⭐⭐ High | 25 min |
| `03-skills-section.md` | Tech stack badges | ⭐⭐ Medium | 15 min |
| `04-contact-section.md` | Contact CTAs | ⭐⭐⭐ High | 10 min |
| `05-complete-layout.md` | Full integration guide | ⭐⭐⭐ High | 20 min |
| `06-footer.md` | Site footer with social links | ⭐⭐ Medium | 10 min |

**Total Time:** ~105 minutes to generate all components

---

## 🚀 Quick Start (3 Steps)

### Step 1: Choose Your Approach

**Option A: Generate All Components** (Recommended)
- Go through each file in order (01 → 05)
- Generate and test each component
- Best for getting complete portfolio

**Option B: Start with Essentials**
- Generate only: Hero (01) + Projects (02) + Contact (04)
- Add Skills (03) and Layout (05) later
- Faster MVP approach

### Step 2: Use AI Tool

Compatible tools:
- ✅ **v0.dev** by Vercel (best for React/Next.js)
- ✅ **Lovable** (formerly GPT Engineer)
- ✅ **Bolt.new** by StackBlitz
- ✅ **Claude** with Artifacts
- ✅ Any AI code generator that supports prompts

### Step 3: Copy, Paste, Iterate

For each component:
1. Open the `.md` file
2. Copy the entire AI prompt (look for the code block)
3. Paste into your AI tool
4. Review generated code
5. Use refinement prompts if needed
6. Port to Dioxus when satisfied

---

## 📖 File Structure (Each File Contains)

Every prompt file includes:

1. **Wireframe** - ASCII visual of layout
2. **Design Specs** - Colors, spacing, typography
3. **AI Prompt** - Complete, copy-paste ready prompt
4. **Iteration Checklist** - What to verify
5. **Common Refinements** - Follow-up prompts for adjustments

---

## 🎨 Design System (Used Across All)

### Colors
```
Background: #0f1116 (dark)
Accent: #3b82f6 (blue)
Text: #ffffff (white)
Secondary Text: #b4b8c5 (gray)
```

### Typography
```
System font: -apple-system, BlinkMacSystemFont, 'Segoe UI'
Headings: Bold (700)
Body: Regular (400)
```

### Spacing
```
Base unit: 8px
Sections: 96px vertical padding
Cards: 24px internal padding
```

---

## 💡 Usage Tips

### Best Practices

**DO:**
- ✅ Generate one section at a time
- ✅ Test each component before moving to next
- ✅ Customize content (your name, projects, skills)
- ✅ Iterate with refinement prompts
- ✅ Review accessibility checklist

**DON'T:**
- ❌ Generate all at once (harder to debug)
- ❌ Skip testing on mobile
- ❌ Forget to customize with your info
- ❌ Port to production without human review

### Common Workflow

```
1. Generate Hero → Test on mobile/desktop
2. Generate Projects → Add your real projects
3. Generate Skills → Customize tech stack
4. Generate Contact → Add your email/links
5. Generate Layout → Integrate all components
6. Port to Dioxus → Rebuild in Rust
7. Deploy → Share with world!
```

---

## 🔧 Customization Guide

### Personalizing Content

**Hero Section:**
- Change name, role, tagline
- Update CTA button text
- Add your social links

**Projects Section:**
- Replace sample data with your projects
- Update project images
- Add real GitHub/demo links

**Skills Section:**
- List technologies you actually use
- Organize by your specialization
- Remove/add categories as needed

**Contact Section:**
- Update email address
- Change availability statement
- Add LinkedIn/GitHub URLs

---

## 🎯 Recommended Order

### For Complete Portfolio
```
Day 1: Navbar (00) + Hero (01) + Footer (06)
Day 2: Projects (02) + Skills (03)
Day 3: Contact (04) + Layout Integration (05)
Day 4: Port to Dioxus
Day 5: Test + Deploy
```

### For Quick MVP
```
Day 1: Navbar (00) + Hero (01) + Projects (02)
Day 2: Contact (04) + Footer (06)
Day 3: Port to Dioxus + Deploy
Later: Add Skills (03) + Full Layout (05)
```

---

## 🐛 Troubleshooting

### AI Generated Wrong Style
**Problem:** Colors/fonts don't match spec
**Solution:** Add "Use exact color values: #0f1116 for background" to prompt

### Component Doesn't Respond
**Problem:** Not mobile-friendly
**Solution:** Use refinement prompt: "Make fully responsive for mobile (375px width)"

### Hover Effects Don't Work
**Problem:** No transitions
**Solution:** Use refinement: "Add smooth transitions (250ms) to all hover states"

### Text Overflows on Mobile
**Problem:** Font too large
**Solution:** Use refinement: "Reduce font sizes by 25% on screens < 640px"

---

## 🎓 Learning Resources

### Understanding the Prompts
- Each prompt follows **4-part framework**:
  1. High-Level Goal (what to build)
  2. Detailed Instructions (step-by-step)
  3. Code Examples (concrete specs)
  4. Strict Scope (boundaries)

### Porting to Dioxus
- Convert JSX → `rsx!` macro syntax
- Replace React hooks → Dioxus signals
- Use Tailwind classes in Dioxus
- Reference: `/docs/front-end-spec.md` (section: Dioxus Implementation)

---

## ✅ Completion Checklist

Track your progress:

- [ ] Generated Navbar component
- [ ] Generated Hero component
- [ ] Generated Projects section
- [ ] Generated Skills section
- [ ] Generated Contact section
- [ ] Generated Footer component
- [ ] Integrated all components (layout guide)
- [ ] Tested on desktop (1440px)
- [ ] Tested on tablet (768px)
- [ ] Tested on mobile (375px)
- [ ] Verified smooth scrolling works
- [ ] Customized with my content
- [ ] Verified accessibility (keyboard nav)
- [ ] Ported to Dioxus
- [ ] Deployed to production

---

## 🚢 Next Steps

Once you've generated all components:

1. **Review** `/docs/front-end-spec.md` for detailed design system
2. **Port** generated React code to Dioxus/Rust
3. **Test** thoroughly on real devices
4. **Deploy** to Vercel, Netlify, or GitHub Pages
5. **Share** your portfolio URL!

---

## 📞 Need Help?

If you encounter issues:

1. Check the **Iteration Checklist** in each file
2. Try a **Common Refinement** prompt
3. Review the **front-end-spec.md** for detailed specs
4. Ask your AI tool to explain any unclear code

---

**Ready to start?** Open `01-hero-section.md` and copy your first prompt! 🎉
