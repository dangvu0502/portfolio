# Contact Section - AI Generation Guide

**Component:** Contact/CTA Section
**Priority:** High (Conversion point)
**Estimated Time:** 10-15 minutes

---

## Wireframe

```
┌─────────────────────────────────────────────────────────────┐
│                                                               │
│                                                               │
│                    Let's Work Together                       │
│                                                               │
│         I'm currently seeking full-time opportunities        │
│         in frontend development. Open to remote or           │
│                     [Location] positions.                    │
│                                                               │
│                                                               │
│     ┌──────────────────┐  ┌──────────┐  ┌──────────┐       │
│     │  📧 Email Me     │  │ LinkedIn │  │  GitHub  │       │
│     └──────────────────┘  └──────────┘  └──────────┘       │
│                                                               │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Layout Details

**All Screen Sizes:**
- Centered content
- Max-width: 768px (narrower for focused CTA)
- Padding: 96px vertical, 24px horizontal
- Background: #0f1116 (matches main background)

**Responsive Behavior:**
- Desktop: Buttons in a row
- Mobile (< 640px): Stack buttons vertically, max-width 320px

---

## Button Design

```
Primary Button (Email):
┌─────────────────────┐
│  📧  Email Me       │ ← Blue background (#3b82f6)
└─────────────────────┘   White text, semibold
                          Icon + text
                          Larger size (1.125rem text)

Hover State:
- Background: #60a5fa (lighter blue)
- Shadow increases
- Scale: 1.02 (spring animation)

Secondary Buttons (LinkedIn, GitHub):
┌─────────────────────┐
│  🔗  LinkedIn       │ ← Transparent background
└─────────────────────┘   Gray border (#404552)
                          Gray text (#b4b8c5)

Hover State:
- Border: #3b82f6 (blue)
- Text: #ffffff (white)
```

---

## Content Structure

### Headline
- Text: "Let's Work Together" or "Get In Touch"
- Font: 2-3rem (32-48px)
- Weight: 700 (bold)
- Color: #ffffff
- Margin-bottom: 1rem

### Subtext
- Text: Brief availability statement (customize to your situation)
- Font: 1.125rem (18px)
- Weight: 400 (regular)
- Color: #b4b8c5 (secondary text)
- Line-height: 1.75
- Max-width: 600px
- Margin-bottom: 2.5rem (40px)

### Buttons Container
- Display: flex
- Gap: 1rem (16px)
- Justify-content: center
- Flex-wrap: wrap (for mobile)

---

## AI Generation Prompt

### Copy and paste this entire prompt ⬇️

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
  Email Me
</a>
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
1. ContactSection component - Complete contact section
2. Include all buttons and text inline (no separate components needed)

DO NOT create:
- Contact form with validation
- Backend API for form submission
- Email verification logic
- Newsletter signup (out of scope)
```

---

## Iteration Checklist

After generating, verify:

- [ ] Section is visually centered
- [ ] Headline is attention-grabbing
- [ ] Subtext clearly states availability
- [ ] Email button is most prominent
- [ ] Email opens mailto: link correctly
- [ ] Subject line is pre-filled in mailto:
- [ ] LinkedIn/GitHub buttons are styled as secondary
- [ ] All external links open in new tabs
- [ ] Hover effects work smoothly
- [ ] Spring animation on email button (feels bouncy)
- [ ] Mobile layout stacks buttons vertically
- [ ] All buttons have focus states
- [ ] Icons are visible and properly aligned

---

## Common Refinements

**"Email button is too big"**
```
Reduce padding to 0.875rem 1.75rem and font-size to 1rem to match secondary buttons.
```

**"Want to add a resume download button"**
```
Add a third secondary button with download icon and link to your resume PDF.
```

**"Spring animation is too aggressive"**
```
Change easing to cubic-bezier(0.4, 0, 0.2, 1) for a gentler effect.
```

**"Buttons are too close on mobile"**
```
When stacked vertically, increase gap to 1.25rem (20px) between buttons.
```

---

## Content Tips

### Headline Options
- "Let's Work Together" (collaborative)
- "Get In Touch" (direct)
- "Ready to Chat?" (casual)
- "Let's Build Something" (creative)

### Subtext Examples
**Actively job hunting:**
> "I'm currently seeking full-time opportunities in frontend development. Open to remote or San Francisco positions."

**Open to opportunities:**
> "I'm always interested in hearing about new projects and opportunities. Feel free to reach out!"

**Freelance focus:**
> "Available for freelance projects and consulting. Let's discuss how I can help bring your ideas to life."

---

## Mailto Link Tips

Pre-fill subject line for better conversion:
```html
mailto:you@example.com?subject=Opportunity from Portfolio
```

You can also pre-fill body (but keep it short):
```html
mailto:you@example.com?subject=Opportunity&body=Hi, I'd like to discuss...
```

---

## Ready to Build?

1. ✅ Copy the prompt above
2. ✅ Paste into v0.dev or Lovable
3. ✅ Customize with your real email/LinkedIn/GitHub
4. ✅ Test mailto: link opens your email client
5. ✅ Test on mobile (buttons should stack)
6. ✅ Port to Dioxus when satisfied

**Next:** Once contact section looks good, move to `05-complete-layout.md`
