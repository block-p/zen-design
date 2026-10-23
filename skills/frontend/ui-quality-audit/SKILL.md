---
name: ui-design-system
description: >
  Utilise ce skill pour tout ce qui touche à la qualité visuelle d'une interface :
  audit "cheap detector", choix typographiques/couleurs/motion, prototypage haute fidélité,
  et validation visuelle via Playwright MCP. Couvre les 5 outils du stack design avancé :
  Impeccable, UI UX Pro Max, Taste, Huashu Design, Playwright.
---

# UI Design System — Stack Complet

Ce skill encode une culture de design exigeante pour Claude et Claude Code.
Il remplace les décisions arbitraires par des règles et des réflexes précis.

## MODULE 1 — Audit "Cheap Detector" (Impeccable)

Avant de générer ou valider une UI, scanner systématiquement ces signaux de médiocrité :

### Signaux visuels qui font "cheap"
- **Gradients génériques** : purple→pink, blue→teal sans intention. Interdit sauf usage éditorial assumé.
- **Espacements cassés** : padding incohérent entre composants, marges asymétriques non intentionnelles, line-height trop serré sur les body textes (< 1.5).
- **Animations trop lisses** : ease ou linear par défaut. Résultat : mouvement plastique sans âme. Utiliser des courbes de Bézier personnalisées ou des springs.
- **Ombres plates** : `box-shadow: 0 2px 4px rgba(0,0,0,0.1)` — c'est le signe distinctif du template Bootstrap. Soit supprimer, soit travailler avec plusieurs couches d'ombres (layered shadows).
- **Bordures radius uniformes** : `border-radius: 8px` partout = aucune hiérarchie. Varier intentionnellement (0px pour éléments data, 4px pour inputs, 999px pour tags).
- **Couleurs de texte par défaut** : noir pur (#000) sur blanc pur (#fff) = trop de contraste, fatigue. Utiliser #1a1a1a / #f5f5f5 minimum.
- **Icônes génériques** : FontAwesome sans customisation. Préférer Lucide, Phosphor, ou des SVG sur mesure.
- **Hover states absents** : si rien ne réagit au hover, l'interface paraît morte.
- **États de chargement ignorés** : skeleton screens > spinners génériques.
- **Typographie non travaillée** : Inter + 16px + regular = aucun caractère. Voir Module 3.

### Checklist audit rapide (pré-livraison)
```
□ Pas de gradient purple→pink / blue→teal
□ Espacements cohérents (système 4px/8px)
□ Line-height body ≥ 1.5
□ Animations avec courbes de Bézier personnalisées
□ Ombres en couches multiples OU supprimées
□ Border-radius variés intentionnellement
□ Pas de #000 sur #fff
□ Icônes customisées (Lucide/Phosphor/SVG)
□ Hover states sur tous les éléments interactifs
□ Skeleton screens pour les chargements
□ Typographie travaillée (voir Module 3)
```

## MODULE 2 — Règles UX Embarquées (UI UX Pro Max)

### Les 10 règles fondamentales

**Hiérarchie & Lisibilité**
1. Maximum 3 niveaux de taille typographique par écran. Au-delà = hiérarchie cassée.
2. Un seul CTA primaire par vue. Deux CTA primaires = zéro CTA primaire.
3. La règle du F-pattern : placer l'info critique en haut à gauche. Les yeux ne lisent pas, ils scannent.

**Feedback & États**
4. Tout élément interactif doit avoir 4 états : default, hover, active, disabled. Pas d'exception.
5. Un formulaire sans validation inline perd 40% de ses completions. Valider champ par champ, pas à la soumission.
6. Les messages d'erreur doivent être prescriptifs, pas descriptifs. "Email invalide" < "Utilisez le format nom@domaine.com".

**Performance perçue**
7. Afficher quelque chose en < 100ms. Pas forcément le contenu — un skeleton, une animation, un placeholder.
8. Les transitions entre pages/vues : 200–300ms max. Au-delà, l'utilisateur pense que ça rame.

**Mobile & Touch**
9. Zone de tap minimum : 44×44px (Apple HIG) / 48×48dp (Material). En dessous = frustration garantie.
10. Le thumb zone : 75% des interactions mobiles se font dans le tiers inférieur de l'écran. Placer les actions primaires là.

### Palettes recommandées par contexte

| Contexte | Palette type |
|----------|-------------|
| Finance / Data | Monochromatique + accent cyan ou amber |
| Santé / Wellness | Vert sauge, beige chaud, blanc cassé |
| Tech / SaaS | Dark avec accents électriques (electric blue, neon lime) |
| Editorial / Blog | Noir typographique + papier + 1 couleur éditoriale |
| Dashboard Trading | Fond anthracite, rouge/vert signals, gris data |
| Créatif / Portfolio | Contraste brutal, typographie expressive, couleur unique |

## MODULE 3 — Culture Visuelle (Taste)

### Typographie — règles de caractère

**Ne jamais utiliser** (sans raison éditoriale explicite) :
- Inter, Roboto, Arial, System-ui comme police principale
- Open Sans (mort créativement)
- Montserrat seul (trop vu)

**Combinaisons qui fonctionnent :**
```
Rubik (headings) + Nunito Sans (body) → Moderne, chaleureux, tech
Playfair Display (headings) + Inter (body) → Éditorial, luxe
DM Serif Display (headings) + DM Sans (body) → Élégant, intemporel
Space Grotesk (headings) + Inter (body) → Tech, bold, startup
```

**Règles de taille** (échelle modulaire × 1.25 ou × 1.333) :
```
h1: 2.5rem (40px)  → poids 700–900
h2: 2rem (32px)    → poids 600–700
h3: 1.5rem (24px)  → poids 600
body: 1rem (16px)  → poids 400, line-height 1.5–1.75
small: 0.875rem    → poids 400
```

### Motion — culture d'animation

**Mauvaises animations** (à bannir) :
- `transition: all 0.3s ease` — trop global, trop prévisible
- Fade in/out seul sur tout — paresseux
- Animations simultanées sur tous les éléments — chaos visuel

**Bonnes animations :**
```css
/* Courbe de qualité pour les entrées */
.reveal {
  animation: reveal 0.5s cubic-bezier(0.22, 1, 0.36, 1) forwards;
}

/* Spring pour les interactions */
.button-press {
  transition: transform 0.15s cubic-bezier(0.2, 0.9, 0.4, 1.1);
}
```

**Principe du stagger** : les éléments de liste n'apparaissent jamais ensemble. Délai progressif de 40–80ms par item.

### Couleur — approche systémique

**Structure d'une palette cohérente :**
```
Primary:    #2563EB (actions principales, liens)
Secondary:  #3B82F6 (survols, accents légers)
Accent:     #059669 (succès, CTAs secondaires)
Surface:    #F8FAFC (fond clair)
Muted:      #F1F5FD (fond grisé)
Border:     #E4ECFC (séparations)
Foreground: #0F172A (texte principal)
```

## MODULE 4 — Prototypage Haute Fidélité (Huashu Design)

### Principes d'implémentation exacte

Quand l'utilisateur fournit une référence visuelle (screenshot, moodboard, description précise) :
1. **Extraction pixel** : identifier les valeurs exactes avant de coder. Couleur hex, taille px, spacing.
2. **Zéro interprétation libre** : si c'est montré, c'est reproduit. Pas de "j'ai adapté légèrement".
3. **Composants isolés** : chaque élément UI = un composant réutilisable avec ses props explicites.
4. **Variables CSS systémiques** : tout ce qui se répète devient une variable. Jamais de valeur en dur répétée.

### Template de composant haute fidélité (React)
```jsx
const Card = ({
  title,
  description,
  image,
  tags = [],
  cta = { label: 'Learn more', href: '#' },
  variant = 'default'
}) => {
  const variants = {
    default: 'bg-white border-gray-200',
    elevated: 'bg-white shadow-lg border-transparent',
    dark: 'bg-gray-900 text-white border-gray-800'
  };

  return (
    <article className={`rounded-2xl p-6 border ${variants[variant]}`}>
      {image && (
        <div className="aspect-video rounded-xl overflow-hidden mb-4">
          <img src={image} alt={title} className="w-full h-full object-cover" />
        </div>
      )}
      <div className="space-y-3">
        <h3 className="font-display font-bold text-xl">{title}</h3>
        <p className="text-dim leading-relaxed">{description}</p>
        {tags.length > 0 && (
          <div className="flex gap-2 flex-wrap">
            {tags.map(tag => (
              <span key={tag} className="px-2.5 py-1 bg-muted rounded-full text-xs font-medium">
                {tag}
              </span>
            ))}
          </div>
        )}
        <a href={cta.href} className="inline-flex items-center gap-1.5 text-primary font-semibold text-sm hover:underline mt-2">
          {cta.label} <span aria-hidden="true">→</span>
        </a>
      </div>
    </article>
  );
};
```

### Variables CSS système de base
```css
:root {
  --color-primary: #2563EB;
  --color-accent: #059669;
  --color-surface: #F8FAFC;
  --color-muted: #F1F5FD;
  --color-border: #E4ECFC;
  --color-foreground: #0F172A;
  --color-dim: #64748B;

  --space-xs: 4px;
  --space-sm: 8px;
  --space-md: 16px;
  --space-lg: 24px;
  --space-xl: 32px;
  --space-2xl: 48px;

  --radius-sm: 4px;
  --radius-md: 8px;
  --radius-lg: 12px;
  --radius-xl: 16px;
  --radius-full: 999px;

  --text-xs: 0.75rem;
  --text-sm: 0.875rem;
  --text-base: 1rem;
  --text-lg: 1.125rem;
  --text-xl: 1.25rem;
  --text-2xl: 1.5rem;
  --text-3xl: 1.875rem;
  --text-4xl: 2.25rem;

  --leading-tight: 1.25;
  --leading-normal: 1.5;
  --leading-relaxed: 1.65;
}
```

## MODULE 5 — Validation Visuelle (Playwright MCP)

Playwright MCP permet à Claude Code de voir l'interface générée dans un vrai navigateur et de valider le rendu.

### Workflow de validation post-génération
```
1. browser_navigate → URL de l'interface générée
2. browser_take_screenshot → capture plein écran
3. Analyse visuelle : couleurs, espacements, typographie
4. browser_click → tester les interactions (hover, formulaires)
5. browser_resize → vérifier responsive (mobile, tablet)
6. Correction des anomalies détectées
7. Nouveau screenshot → comparaison avant/après
```

### Commandes Playwright MCP clés (Claude Code)
```
browser_navigate     → Aller sur une URL
browser_take_screenshot → Capturer l'écran
browser_snapshot     → Capture texte + ARIA de la page
browser_click        → Cliquer un élément
browser_hover        → Survoler un élément
browser_resize       → Redimensionner la fenêtre
browser_press_key    → Appuyer sur une touche
browser_type         → Taper du texte
browser_fill_form    → Remplir un formulaire
browser_evaluate     → Exécuter du JS dans la page
```

### Configuration MCP (claude_desktop_config.json ou Claude Code)
```json
{
  "mcpServers": {
    "playwright": {
      "command": "npx",
      "args": ["-y", "@anthropic/mcp-playwright"]
    }
  }
}
```

### Checklist de validation visuelle
```
□ Les couleurs correspondent à la palette définie
□ Les espacements sont cohérents (système 4px/8px)
□ La typographie est correcte (police, taille, graisse, line-height)
□ Les animations fonctionnent (entrée, hover, clic)
□ Le responsive est correct (mobile 375px, tablet 768px)
□ Les états interactifs sont visibles (hover, focus, active, disabled)
□ Les formulaires ont une validation inline
□ Les messages d'erreur sont prescriptifs
□ Les skeletons/placeholders apparaissent pendant le chargement
□ Le contraste est suffisant (WCAG AA minimum)
```

## USAGE RÉSUMÉ

| Situation | Module à activer |
|-----------|-----------------|
| "Est-ce que mon UI fait cheap ?" | Module 1 — Audit |
| "Choisis une palette / typographie" | Modules 2 & 3 |
| "Reproduis exactement ce mockup" | Module 4 |
| "Vérifie que ça rend bien en vrai" | Module 5 |
| Génération UI from scratch | Modules 1+2+3+4 en séquence |
| Livraison finale | Module 1 audit + Module 5 validation |

---

**Skill généré pour ATLAS NEXUS**
Sources : Impeccable, UI UX Pro Max, Taste, Huashu Design, Playwright MCP.
Compatible : Claude Code · ChatGPT · DeepSeek · Cursor · tout LLM avec instructions système.

## COMPATIBILITÉ MULTI-PLATEFORME

Ce skill est conçu pour fonctionner avec n'importe quel LLM disposant d'instructions système (system prompt). Les modules 1-4 sont universels. Le Module 5 (Playwright) est spécifique à Claude Code.

### Claude Code
Placer `SKILL.md` dans `.claude/skills/ui-design-system/`. Chargé automatiquement.
```bash
mkdir -p .claude/skills/ui-design-system
curl -o .claude/skills/ui-design-system/SKILL.md \
  https://raw.githubusercontent.com/AtlasNexusTech/ui-design-system/main/SKILL.md
```

### ChatGPT
**Option A — Custom Instructions** (Settings → Personalization → Custom Instructions) :
Copier le contenu de `adapters/chatgpt-custom-instructions.md` dans le champ "How would you like ChatGPT to respond?".

**Option B — Custom GPT** :
Créer un GPT avec les instructions de `adapters/chatgpt-custom-instructions.md`. Le Module 5 peut utiliser le navigateur intégré de ChatGPT (GPT-4o avec browsing) pour la validation visuelle — demander des screenshots et les analyser.

### DeepSeek
Utiliser le contenu de `adapters/deepseek-system-prompt.md` comme system prompt dans l'API DeepSeek, ou en première instruction de conversation. Le Module 5 devient une checklist de validation manuelle — l'assistant énumère les points à vérifier visuellement.

### Cursor / Windsurf / autres IDE
Copier `SKILL.md` dans le dossier de règles du projet (`.cursorrules`, `.windsurfrules`, etc.) ou l'inclure dans le contexte du projet.

### Mode universel (tout LLM)
Envoyer le contenu de `adapters/universal-design-prompt.md` en premier message de la conversation. Les 4 premiers modules s'appliquent immédiatement. Le Module 5 est une checklist textuelle.
