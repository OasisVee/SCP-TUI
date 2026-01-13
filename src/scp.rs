use anyhow::{Context, Result};
use directories::BaseDirs;
use rand::Rng;
use regex::Regex;
use reqwest::blocking::Client;
use scraper::{ElementRef, Html, Selector};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

const MAX_SCP: i32 = 9999;
const WIKI_BASE_URL: &str = "https://scp-wiki.wikidot.com/scp-";

pub struct ScpManager {
    storage_dir: PathBuf,
    client: Client,
}

impl ScpManager {
    pub fn new() -> Result<Self> {
        let base_dirs = BaseDirs::new().context("Could not determine home directory")?;
        let storage_dir = base_dirs
            .home_dir()
            .join(".local")
            .join("share")
            .join("scipindex-tui")
            .join("saved_entries");

        fs::create_dir_all(&storage_dir)?;

        let client = Client::builder()
            .user_agent("Mozilla/5.0 (compatible; SCP-TUI/1.0)")
            .build()?;

        Ok(Self {
            storage_dir,
            client,
        })
    }

    pub fn get_scp_path(&self, number: i32) -> PathBuf {
        self.storage_dir.join(format!("SCP-{:03}.txt", number))
    }

    pub fn get_scp(&self, number: i32) -> Result<String> {
        let path = self.get_scp_path(number);

        if path.exists() {
            let mut content = String::new();
            File::open(&path)?.read_to_string(&mut content)?;
            return Ok(content);
        }

        let url = format!("{}{:03}", WIKI_BASE_URL, number);
        let resp = self.client.get(&url).send()?;

        if !resp.status().is_success() {
            return Ok(format!(
                "Failed to fetch SCP-{:03}. Status: {}",
                number,
                resp.status()
            ));
        }

        let body = resp.text()?;
        let parsed_text = self.parse_html(&body, number)?;

        let mut file = File::create(&path)?;
        file.write_all(parsed_text.as_bytes())?;

        Ok(parsed_text)
    }

    pub fn get_random_scp(&self) -> Result<(i32, String)> {
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..=MAX_SCP);
        let content = self.get_scp(number)?;
        Ok((number, content))
    }

    fn parse_html(&self, html: &str, number: i32) -> Result<String> {
        let document = Html::parse_document(html);
        let content_selector = Selector::parse("#page-content").unwrap();

        let content_div = match document.select(&content_selector).next() {
            Some(div) => div,
            None => {
                return Ok("Could not find page content. This page might not exist.".to_string())
            }
        };

        let text = content_div.text().collect::<Vec<_>>().join(" ");
        if text.contains("This page doesn't exist") {
            return Ok(format!("SCP-{:03} does not exist.", number));
        }

        let mut output = Vec::new();
        output.push(format!("SCP-{:03}", number));
        output.push("=".repeat(40));
        output.push(String::new());

        let blocks = self.collect_blocks(content_div);
        output.extend(blocks);

        Ok(output.join("\n\n"))
    }

    fn collect_blocks(&self, element: ElementRef) -> Vec<String> {
        let mut blocks = Vec::new();
        let mut current_inline = String::new();

        let exclude_classes = [
            "licensebox",
            "page-rate-widget-box",
            "authorlink-wrapper",
            "creditRate",
            "heritage-rating-module",
            "scp-transcription",
            "info-container",
            "u-credit-view",
            "u-credit-box",
            "rate-widget-hl",
            "scp-image-caption",
            "credit-pane",
            "credit-back",
            "info-pane-contents",
            "footer-wikiwalk-nav",
        ];

        for node in element.children() {
            if let Some(text) = node.value().as_text() {
                let s = text.trim();
                if !s.is_empty() {
                    current_inline.push_str(s);
                    current_inline.push(' ');
                }
            } else if let Some(el) = ElementRef::wrap(node) {
                if let Some(classes) = el.value().attr("class") {
                    if exclude_classes.iter().any(|c| classes.contains(c)) {
                        continue;
                    }
                }
                if let Some(id) = el.value().attr("id") {
                    if id == "u-credit-view" || id == "u-credit-box" {
                        continue;
                    }
                }

                let tag = el.value().name();
                if self.is_block_element(tag) {
                    if !current_inline.trim().is_empty() {
                        blocks.push(current_inline.trim().to_string());
                        current_inline.clear();
                    }

                    match tag {
                        "div" => {
                            if let Some(classes) = el.value().attr("class") {
                                if classes.contains("scp-image-block") {
                                    if let Some(img) =
                                        el.select(&Selector::parse("img").unwrap()).next()
                                    {
                                        let src = img.value().attr("src").unwrap_or("");
                                        blocks.push(format!("[IMAGE: {}]", src));
                                    }
                                    continue;
                                }
                            }
                            blocks.extend(self.collect_blocks(el));
                        }
                        "p" => {
                            let text = self.clean_text(el);
                            if !text.starts_with("rating:") && !text.is_empty() {
                                blocks.push(text);
                            }
                        }
                        "blockquote" => {
                            let text = self.clean_text(el);
                            blocks.push(self.format_blockquote(&text));
                        }
                        "h1" | "h2" | "h3" | "h4" => {
                            let text = self.clean_text(el);
                            blocks.push(format!("# {}", text));
                        }
                        "ul" | "ol" => {
                            for li in el.select(&Selector::parse("li").unwrap()) {
                                let text = self.clean_text(li);
                                blocks.push(format!("- {}", text));
                            }
                        }
                        "li" => {
                            let text = self.clean_text(el);
                            blocks.push(format!("- {}", text));
                        }
                        _ => {
                            blocks.extend(self.collect_blocks(el));
                        }
                    }
                } else {
                    match tag {
                        "br" => current_inline.push('\n'),
                        _ => {
                            let text = self.clean_text(el);
                            current_inline.push_str(&text);
                            current_inline.push(' ');
                        }
                    }
                }
            }
        }

        if !current_inline.trim().is_empty() {
            blocks.push(current_inline.trim().to_string());
        }

        blocks
    }

    fn is_block_element(&self, tag: &str) -> bool {
        matches!(
            tag,
            "p" | "div"
                | "blockquote"
                | "ul"
                | "ol"
                | "li"
                | "h1"
                | "h2"
                | "h3"
                | "h4"
                | "table"
                | "pre"
                | "hr"
        )
    }

    fn clean_text(&self, element: ElementRef) -> String {
        let raw = element.text().collect::<Vec<_>>().join("");
        let re = Regex::new(r"\s+").unwrap();
        let cleaned = re.replace_all(&raw, " ").trim().to_string();
        cleaned
    }

    fn format_blockquote(&self, text: &str) -> String {
        let width = 60;
        let options = textwrap::Options::new(width);
        let wrapped = textwrap::fill(text, &options);

        let lines: Vec<&str> = wrapped.lines().collect();
        let mut out = String::new();
        let border = format!("+{}+", "-".repeat(width + 2));

        out.push_str(&border);
        out.push('\n');
        for line in lines {
            out.push_str(&format!("| {:<width$} |\n", line, width = width));
        }
        out.push_str(&border);
        out
    }
}
