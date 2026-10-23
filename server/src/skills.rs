//! Dynamic skill matcher and loader for Claude Graphic & Motion Skills.
//!
//! Scans `skills/` and `~/.claude/skills/` for `SKILL.md` packages, matches the user's
//! brief against available graphic and motion skills, and injects the specialized instructions,
//! easing curves, and design patterns into the model prompt when using any API key.

use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Clone, Debug)]
pub struct LoadedSkill {
    pub name: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub instructions: String,
    pub path: PathBuf,
}

fn extract_frontmatter_field(content: &str, field: &str) -> Option<String> {
    if !content.starts_with("---") {
        return None;
    }
    let rest = &content[3..];
    let end_idx = rest.find("---")?;
    let yaml = &rest[..end_idx];

    for line in yaml.lines() {
        let line = line.trim();
        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim();
            if key.eq_ignore_ascii_case(field) {
                let val = line[pos + 1..].trim();
                return Some(val.trim_matches('"').trim_matches('\'').to_string());
            }
        }
    }
    None
}

fn strip_frontmatter(content: &str) -> String {
    if !content.starts_with("---") {
        return content.to_string();
    }
    let rest = &content[3..];
    if let Some(end_idx) = rest.find("---") {
        rest[end_idx + 3..].trim().to_string()
    } else {
        content.to_string()
    }
}

pub fn scan_skills_in_dir(base: &Path, results: &mut Vec<LoadedSkill>) {
    if !base.is_dir() {
        return;
    }

    let entries = match fs::read_dir(base) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        // Check if subdirectories have SKILL.md (e.g. skills/motion/kinetic-type-title/SKILL.md)
        let skill_file = path.join("SKILL.md");
        if skill_file.is_file() {
            if let Ok(content) = fs::read_to_string(&skill_file) {
                let slug = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                let name = extract_frontmatter_field(&content, "name").unwrap_or_else(|| slug.clone());
                let title = extract_frontmatter_field(&content, "title").unwrap_or_else(|| {
                    slug.replace('-', " ")
                        .split_whitespace()
                        .map(|w| {
                            let mut c = w.chars();
                            match c.next() {
                                None => String::new(),
                                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                });
                let category = extract_frontmatter_field(&content, "category").unwrap_or_default();
                let description = extract_frontmatter_field(&content, "description").unwrap_or_default();
                let instructions = strip_frontmatter(&content);

                results.push(LoadedSkill {
                    name,
                    title,
                    category,
                    description,
                    instructions,
                    path: path.clone(),
                });
            }
        } else {
            // Recurse one level down (e.g. skills/graphic/ or skills/motion/)
            scan_skills_in_dir(&path, results);
        }
    }
}

pub fn load_all_skills(workspace_dir: &str) -> Vec<LoadedSkill> {
    let mut skills = Vec::new();
    let ws = Path::new(workspace_dir);

    // 1. Check workspace/skills or repo/skills
    let candidates = [
        ws.join("skills"),
        ws.join("../skills"),
        PathBuf::from("skills"),
        PathBuf::from("../skills"),
    ];

    for candidate in &candidates {
        if candidate.is_dir() {
            scan_skills_in_dir(candidate, &mut skills);
            if !skills.is_empty() {
                break;
            }
        }
    }

    // 2. Check ~/.claude/skills
    if skills.is_empty() {
        if let Ok(home) = std::env::var("HOME") {
            let claude_skills = PathBuf::from(home).join(".claude").join("skills");
            if claude_skills.is_dir() {
                scan_skills_in_dir(&claude_skills, &mut skills);
            }
        }
    }

    // Deduplicate by name
    let mut seen = std::collections::HashSet::new();
    skills.retain(|s| seen.insert(s.name.clone()));

    info!("Loaded {} skills from skills directory", skills.len());
    skills
}

pub fn match_skills<'a>(
    prompt: &str,
    mode: &str,
    skills: &'a [LoadedSkill],
) -> Vec<&'a LoadedSkill> {
    let p_lower = prompt.to_lowercase();
    let mut scored: Vec<(&'a LoadedSkill, usize)> = Vec::new();

    for skill in skills {
        let name_lower = skill.name.to_lowercase();
        let name_spaced = name_lower.replace('-', " ");
        let title_lower = skill.title.to_lowercase();
        let mut score = 0;

        // Exact match in prompt
        if p_lower.contains(&name_lower) || p_lower.contains(&name_spaced) || p_lower.contains(&title_lower) {
            score += 150;
        }

        // Semantic keyword rules
        if mode == "motion" {
            if skill.category.eq_ignore_ascii_case("explainer")
                && (p_lower.contains("explainer") || p_lower.contains("tutorial") || p_lower.contains("توضیح"))
            {
                score += 40;
            }
            if skill.category.eq_ignore_ascii_case("launch")
                && (p_lower.contains("launch") || p_lower.contains("product") || p_lower.contains("رونمایی") || p_lower.contains("معرفی"))
            {
                score += 40;
            }
            if skill.name == "kinetic-type-title"
                && (p_lower.contains("title") || p_lower.contains("headline") || p_lower.contains("typography") || p_lower.contains("تیتر"))
            {
                score += 50;
            }
            if skill.name == "data-chart-story" || skill.name == "data-to-video"
                && (p_lower.contains("chart") || p_lower.contains("data") || p_lower.contains("graph") || p_lower.contains("نمودار"))
            {
                score += 50;
            }
            if skill.name == "feature-grid"
                && (p_lower.contains("bento") || p_lower.contains("grid") || p_lower.contains("features") || p_lower.contains("ویژگی"))
            {
                score += 50;
            }
            if skill.name == "motion-director" {
                score += 20; // baseline motion director
            }
        }

        // Graphic skills keywords
        if skill.name == "canvas-design"
            && (p_lower.contains("canvas") || p_lower.contains("drawing") || p_lower.contains("webgl") || p_lower.contains("بوم"))
        {
            score += 80;
        }
        if skill.name == "algorithmic-art"
            && (p_lower.contains("algorithmic") || p_lower.contains("generative") || p_lower.contains("particle") || p_lower.contains("noise") || p_lower.contains("هنر"))
        {
            score += 80;
        }
        if skill.name == "frontend-design"
            && (p_lower.contains("ui") || p_lower.contains("dashboard") || p_lower.contains("frontend") || p_lower.contains("رابط کاربری"))
        {
            score += 35;
        }
        if skill.name == "slack-gif-creator"
            && (p_lower.contains("gif") || p_lower.contains("loop") || p_lower.contains("گیف"))
        {
            score += 80;
        }
        if skill.name == "theme-factory"
            && (p_lower.contains("theme") || p_lower.contains("palette") || p_lower.contains("color system") || p_lower.contains("پالت"))
        {
            score += 60;
        }

        if score > 0 {
            scored.push((skill, score));
        }
    }

    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored.into_iter().take(2).map(|(s, _)| s).collect()
}

pub fn format_skill_instructions(matched: &[&LoadedSkill]) -> String {
    if matched.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    out.push_str("\n\n================================================================================\n");
    out.push_str("SPECIALIZED CLAUDE SKILLS LOADED FOR THIS BRIEF:\n");
    out.push_str("================================================================================\n");

    for skill in matched {
        out.push_str(&format!(
            "\n--- SKILL: {} ({}) ---\n{}\n",
            skill.title, skill.name, skill.instructions
        ));
    }

    out.push_str("================================================================================\n");
    out.push_str("Follow the above skill guidelines, timing/easing curves, and design patterns strictly.\n\n");
    out
}
